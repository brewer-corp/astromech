use super::*;
use std::io::Write;

/// The commander in the main fixture set.
const CMDR: &str = "F0000001";

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// Everything an import produces: the stored events and every commander's ledgers.
#[derive(Debug, PartialEq)]
struct Snapshot {
    events: Vec<(String, i64, Option<String>, String, String, String)>,
    files: Vec<(String, i64, i64, Option<String>, bool)>,
    commanders: Vec<Commander>,
    ledgers: Vec<(Vec<Visit>, Vec<Earning>, Vec<Ship>, Vec<Scan>)>,
}

fn snapshot(h: &History) -> Snapshot {
    let c = h.conn();
    let mut st = c
        .prepare(
            "SELECT f.name, e.offset, e.cmdr, e.ts, e.event, e.json FROM events e
             JOIN journal_files f ON f.id = e.file_id ORDER BY e.id",
        )
        .unwrap();
    let events = st
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    let mut st = c.prepare("SELECT name, offset, lines, cmdr, legacy FROM journal_files ORDER BY name").unwrap();
    let files = st
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)))
        .unwrap()
        .collect::<rusqlite::Result<_>>()
        .unwrap();
    let commanders = h.commanders().unwrap();
    let ledgers = commanders
        .iter()
        .map(|c| {
            let k = c.cmdr.as_str();
            (h.visits(k).unwrap(), h.earnings(k).unwrap(), h.ships(k).unwrap(), h.scans(k).unwrap())
        })
        .collect();
    Snapshot { events, files, commanders, ledgers }
}

fn clean_import() -> (History, ImportReport) {
    let mut h = History::in_memory().unwrap();
    let r = h.import(&fixtures(), &ImportOptions::default()).unwrap();
    (h, r)
}

fn visit(ts: &str, kind: &str, system: &str, addr: i64, station: Option<(&str, i64)>, jump: Option<f64>) -> Visit {
    Visit {
        ts: ts.into(),
        kind: kind.into(),
        star_system: system.into(),
        system_address: Some(addr),
        station_name: station.map(|s| s.0.into()),
        market_id: station.map(|s| s.1),
        jump_dist: jump,
    }
}

#[test]
fn imports_every_line_once() {
    let (mut h, r) = clean_import();
    assert_eq!(r.files, 3);
    assert_eq!(r.lines, 16 + 16 + 15);
    assert_eq!(r.malformed, 1, "the truncated Bounty line");
    assert_eq!(r.events, 46);
    assert_eq!(h.event_count().unwrap(), 46);
    let total: u64 = journal_logs_size();
    assert_eq!(r.bytes, total);
    // Nothing new: nothing read, nothing stored.
    let again = h.import(&fixtures(), &ImportOptions::default()).unwrap();
    assert_eq!(again, ImportReport::default());
    assert_eq!(h.event_count().unwrap(), 46);
}

fn journal_logs_size() -> u64 {
    astromech_journal::journal_logs(&fixtures()).iter().map(|p| std::fs::metadata(p).unwrap().len()).sum()
}

#[test]
fn visits_ledger() {
    let (h, _) = clean_import();
    let sol = 10477373803;
    let lincoln = Some(("Abraham Lincoln", 128016640));
    let miller = Some(("Miller Depot", 128000001));
    assert_eq!(
        h.visits(CMDR).unwrap(),
        vec![
            visit("2026-09-01T10:00:07Z", "system", "Sol", sol, None, None),
            visit("2026-09-01T10:00:07Z", "station", "Sol", sol, lincoln, None),
            visit("2026-09-01T10:10:00Z", "system", "Alpha Centauri", 1, None, Some(4.38)),
            visit("2026-09-01T10:20:00Z", "system", "Barnard's Star", 2, None, Some(6.5)),
            visit("2026-09-01T10:25:00Z", "station", "Barnard's Star", 2, miller, None),
            // 09-02: logging in docked at Miller Depot again is not a new visit.
            visit("2026-09-02T10:10:00Z", "system", "Sol", sol, None, Some(5.95)),
            visit("2026-09-02T10:20:00Z", "system", "Alpha Centauri", 1, None, Some(4.38)),
            // 09-03: logging in where the last session ended is not a new visit either.
            visit("2026-09-03T10:10:00Z", "station", "Alpha Centauri", 1, Some(("Hutton Orbital", 128000002)), None),
        ]
    );
}

#[test]
fn earnings_ledger() {
    let (h, _) = clean_import();
    let e = |ts: &str, kind: &str, amount: i64, cost: Option<i64>, detail: Option<&str>| Earning {
        ts: ts.into(),
        kind: kind.into(),
        amount,
        cost,
        detail: detail.map(str::to_string),
    };
    assert_eq!(
        h.earnings(CMDR).unwrap(),
        vec![
            e("2026-09-01T10:05:00Z", "trade", 500000, Some(450000), Some("gold")),
            e("2026-09-01T10:26:00Z", "bounty", 120000, None, Some("Barnard's Star Group")),
            e("2026-09-01T10:27:00Z", "mission", 250000, None, Some("Mission_Delivery_name")),
            e("2026-09-03T10:11:00Z", "organic", 5000000, None, None),
            e("2026-09-03T10:12:00Z", "exploration", 100000, None, None),
            e("2026-09-03T10:13:00Z", "search_and_rescue", 40000, None, Some("Occupied Escape Pod")),
            e("2026-09-03T10:14:00Z", "community_goal", 2000000, None, Some("Fixture Goal")),
            e("2026-09-03T10:15:00Z", "micro_resources", 15000, None, None),
            e("2026-09-03T10:16:00Z", "combat_bond", 60000, None, Some("Alliance")),
            e("2026-09-03T10:17:00Z", "voucher", 5000, None, Some("codex")),
            e("2026-09-03T10:18:00Z", "trade", 1500, None, Some("water")),
        ]
    );
}

#[test]
fn ships_ledger() {
    let (h, _) = clean_import();
    let s = |id: i64, ty: &str, name: Option<&str>, ident: Option<&str>, first: &str| Ship {
        ship_id: id,
        ship_type: ty.into(),
        name: name.map(str::to_string),
        ident: ident.map(str::to_string),
        first_seen: first.into(),
        last_flown: None,
        acquired: None,
        sold: None,
    };
    assert_eq!(
        h.ships(CMDR).unwrap(),
        vec![
            Ship {
                last_flown: Some("2026-09-02T10:00:06Z".into()),
                sold: Some("2026-09-02T10:04:00Z".into()),
                ..s(1, "cobramkiii", Some("Rusty"), Some("RS-01"), "2026-09-01T10:00:06Z")
            },
            s(2, "asp", Some("Wanderer"), None, "2026-09-02T10:01:00Z"),
            s(3, "krait_mkii", None, None, "2026-09-02T10:01:00Z"),
            Ship {
                last_flown: Some("2026-09-03T10:00:06Z".into()),
                acquired: Some("2026-09-02T10:02:05Z".into()),
                ..s(4, "python", Some("Big Snake"), Some("BS-04"), "2026-09-02T10:02:05Z")
            },
        ]
    );
}

#[test]
fn scans_ledger() {
    let (h, _) = clean_import();
    let scan = |ts: &str, kind: &str, body: &str, id: i64, class: Option<&str>| Scan {
        ts: ts.into(),
        kind: kind.into(),
        system_address: Some(1),
        star_system: Some("Alpha Centauri".into()),
        body_name: Some(body.into()),
        body_id: Some(id),
        class: class.map(str::to_string),
        landable: None,
        terraformable: None,
        was_discovered: None,
        was_mapped: None,
    };
    assert_eq!(
        h.scans(CMDR).unwrap(),
        vec![
            Scan {
                was_discovered: Some(true),
                was_mapped: Some(false),
                ..scan("2026-09-01T10:11:00Z", "body", "Alpha Centauri A", 1, Some("G"))
            },
            Scan {
                landable: Some(true),
                terraformable: Some(true),
                was_discovered: Some(true),
                was_mapped: Some(false),
                ..scan("2026-09-01T10:12:00Z", "body", "Alpha Centauri A 1", 4, Some("High metal content body"))
            },
            scan("2026-09-01T10:15:00Z", "mapped", "Alpha Centauri A 1", 4, None),
            scan("2026-09-02T10:40:00Z", "organic", "Alpha Centauri A 1", 4, Some("Bacterium Aurasus")),
        ]
    );
}

/// The done-when: kill the import at every possible line, restart it on the same database
/// file, and get exactly what one clean import gives.
#[test]
fn import_killed_anywhere_then_resumed_equals_clean_import() {
    let (clean, report) = clean_import();
    let want = snapshot(&clean);
    let opts = ImportOptions { batch_lines: 4, ..Default::default() };
    for crash_at in 0..report.lines {
        let dir = tempfile::tempdir().unwrap();
        let db = dir.path().join("history.sqlite");
        {
            let mut h = History::open(&db).unwrap();
            let crash = ImportOptions { crash_after_lines: Some(crash_at), ..opts.clone() };
            assert!(matches!(h.import(&fixtures(), &crash), Err(HistoryError::Crashed)), "crash at line {crash_at}");
        }
        let mut h = History::open(&db).unwrap();
        h.import(&fixtures(), &opts).unwrap();
        assert_eq!(snapshot(&h), want, "killed after {crash_at} lines, then resumed");
    }
}

#[test]
fn rebuilt_ledgers_equal_imported_ones() {
    let (mut h, _) = clean_import();
    let want = snapshot(&h);
    h.rebuild_ledgers().unwrap();
    assert_eq!(snapshot(&h), want);
}

/// The game is mid-write: the unfinished line waits, and is imported once it is complete.
#[test]
fn unfinished_line_waits_for_its_newline() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Journal.2026-09-04T100000.01.log");
    let mut f = std::fs::File::create(&path).unwrap();
    // CRLF line endings, as the game writes them, and a byte-order mark on the first line.
    write!(f, "\u{feff}{{\"timestamp\":\"2026-09-04T10:00:00Z\",\"event\":\"Fileheader\"}}\r\n").unwrap();
    write!(f, "{{\"timestamp\":\"2026-09-04T10:00:00Z\",\"event\":\"Commander\",\"FID\":\"F1\",\"Name\":\"A\"}}\r\n")
        .unwrap();
    write!(f, "{{\"timestamp\":\"2026-09-04T10:00:01Z\",\"event\":\"FSDJump\",\"StarSystem\":\"Sol\",").unwrap();
    f.flush().unwrap();
    let mut h = History::in_memory().unwrap();
    let r = h.import(dir.path(), &ImportOptions::default()).unwrap();
    assert_eq!((r.lines, r.events, r.malformed), (2, 2, 0));
    assert!(h.visits("F1").unwrap().is_empty());
    write!(f, "\"SystemAddress\":10477373803,\"JumpDist\":3.5}}\r\n").unwrap();
    f.flush().unwrap();
    let r = h.import(dir.path(), &ImportOptions::default()).unwrap();
    assert_eq!((r.lines, r.events), (1, 1));
    let v = h.visits("F1").unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].star_system, "Sol");
    assert_eq!(v[0].jump_dist, Some(3.5));
}

#[test]
fn commanders_and_legacy_stay_apart() {
    let mut h = History::in_memory().unwrap();
    h.import(&fixtures().join("two-commanders"), &ImportOptions::default()).unwrap();
    let keys: Vec<(String, String, bool)> =
        h.commanders().unwrap().into_iter().map(|c| (c.cmdr, c.name, c.legacy)).collect();
    assert_eq!(
        keys,
        vec![
            ("F0000001".into(), "Fixture".into(), false),
            ("F0000002".into(), "Second".into(), false),
            ("F0000001/legacy".into(), "Fixture".into(), true),
        ]
    );
    // Ship 1 is a different ship for each, and selling the live commander's ship 1 touches
    // neither of the others.
    let ships = |k: &str| -> Vec<(i64, String, Option<String>)> {
        h.ships(k).unwrap().into_iter().map(|s| (s.ship_id, s.ship_type, s.sold)).collect()
    };
    assert_eq!(ships("F0000002"), vec![(1, "sidewinder".into(), None)]);
    assert_eq!(ships("F0000001/legacy"), vec![(1, "anaconda".into(), None)]);
    assert_eq!(ships("F0000001"), vec![(4, "python".into(), None)]);
    let systems = |k: &str| -> Vec<(String, String)> {
        h.visits(k).unwrap().into_iter().map(|v| (v.kind, v.star_system)).collect()
    };
    assert_eq!(systems("F0000002"), vec![("system".into(), "Sol".into()), ("system".into(), "Alpha Centauri".into())]);
    // The live commander's Location isn't compared with the other commander's last system.
    assert_eq!(
        systems("F0000001"),
        vec![("system".into(), "Barnard's Star".into()), ("station".into(), "Barnard's Star".into())]
    );
    assert_eq!(systems("F0000001/legacy"), vec![("system".into(), "Sol".into())]);
    // Fileheader comes before any commander is named.
    let unowned: i64 = h.conn().query_row("SELECT COUNT(*) FROM events WHERE cmdr IS NULL", [], |r| r.get(0)).unwrap();
    assert_eq!(unowned, 2);
}

/// A database written by schema v1 (no commanders) gets its commanders and ledgers back on
/// open, the same as a fresh import, and the next import has nothing to do.
#[test]
fn v1_database_is_migrated() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("history.sqlite");
    {
        let c = astromech_store::migrate(Connection::open(&db).unwrap(), &MIGRATIONS[..1]).unwrap();
        for path in astromech_journal::journal_logs(&fixtures()) {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            let text = std::fs::read_to_string(&path).unwrap();
            c.execute(
                "INSERT INTO journal_files(name, offset, lines) VALUES (?1, ?2, ?3)",
                params![name, text.len() as i64, text.lines().count() as i64],
            )
            .unwrap();
            let file_id = c.last_insert_rowid();
            let mut offset = 0;
            for l in text.split_inclusive('\n') {
                if let Ok(j) = parse_line(l.trim()) {
                    c.execute(
                        "INSERT INTO events(file_id, offset, ts, event, json) VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![file_id, offset, j.timestamp, j.name, j.raw_json],
                    )
                    .unwrap();
                }
                offset += l.len() as i64;
            }
        }
        // A v1 ledger row, mixed across commanders: it must not survive.
        c.execute("INSERT INTO ships(ship_id, ship_type, first_seen) VALUES (99, 'x', 't')", []).unwrap();
    }
    let mut h = History::open(&db).unwrap();
    let (clean, _) = clean_import();
    assert_eq!(snapshot(&h), snapshot(&clean));
    assert_eq!(h.import(&fixtures(), &ImportOptions::default()).unwrap(), ImportReport::default());
    let pending: i64 = h.conn().query_row("SELECT COUNT(*) FROM pending", [], |r| r.get(0)).unwrap();
    assert_eq!(pending, 0);
}

/// A restored backup: the oldest journal arrives after the newer ones. The ledgers end up the
/// same as importing in order.
#[test]
fn older_journal_imported_late_gives_the_same_ledgers() {
    let (clean, _) = clean_import();
    let dir = tempfile::tempdir().unwrap();
    let logs = astromech_journal::journal_logs(&fixtures());
    let copy = |p: &PathBuf| std::fs::copy(p, dir.path().join(p.file_name().unwrap())).unwrap();
    let mut h = History::in_memory().unwrap();
    for p in &logs[1..] {
        copy(p);
    }
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    assert_ne!(snapshot(&h).ledgers, snapshot(&clean).ledgers, "the first file matters");
    copy(&logs[0]);
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    let (got, want) = (snapshot(&h), snapshot(&clean));
    assert_eq!((got.commanders, got.ledgers), (want.commanders, want.ledgers));
    let pending: i64 = h.conn().query_row("SELECT COUNT(*) FROM pending", [], |r| r.get(0)).unwrap();
    assert_eq!(pending, 0);
}

/// Ship rows only take type and name from events newer than the ones they have.
#[test]
fn ship_rows_ignore_older_events() {
    let h = History::in_memory().unwrap();
    let line = |ts: &str, ship: &str, name: &str| {
        parse_line(&format!(
            r#"{{"timestamp":"{ts}","event":"Loadout","Ship":"{ship}","ShipID":7,"ShipName":"{name}","Modules":[]}}"#
        ))
        .unwrap()
    };
    ledgers::derive(h.conn(), 1, "F1", &line("2026-09-02T00:00:00Z", "python", "New")).unwrap();
    ledgers::derive(h.conn(), 2, "F1", &line("2026-09-01T00:00:00Z", "cobramkiii", "Old")).unwrap();
    let rename = parse_line(
        r#"{"timestamp":"2026-09-01T12:00:00Z","event":"SetUserShipName","ShipID":7,"UserShipName":"Older"}"#,
    )
    .unwrap();
    ledgers::derive(h.conn(), 3, "F1", &rename).unwrap();
    let s = &h.ships("F1").unwrap()[0];
    assert_eq!(
        (s.ship_type.as_str(), s.name.as_deref(), s.first_seen.as_str(), s.last_flown.as_deref()),
        ("python", Some("New"), "2026-09-01T00:00:00Z", Some("2026-09-02T00:00:00Z"))
    );
}
