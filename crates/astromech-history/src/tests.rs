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
    let via = match (kind, jump) {
        (_, Some(_)) => "FSDJump",
        _ if ts.ends_with("10:00:07Z") => "Location",
        ("station", None) => "Docked",
        _ => "CarrierJump",
    };
    Visit {
        ts: ts.into(),
        kind: kind.into(),
        star_system: system.into(),
        system_address: Some(addr),
        station_name: station.map(|s| s.0.into()),
        market_id: station.map(|s| s.1),
        jump_dist: jump,
        via: via.into(),
    }
}

#[test]
fn imports_every_line_once() {
    let (mut h, r) = clean_import();
    assert_eq!(r.files, 3);
    assert_eq!(r.lines, 16 + 20 + 17);
    assert_eq!(r.malformed, 1, "the truncated Bounty line");
    assert_eq!(r.events, 52);
    assert_eq!(h.event_count().unwrap(), 52);
    let total: u64 = journal_logs_size();
    assert_eq!((r.bytes, r.bytes_total), (total, total));
    // Nothing new: nothing read, nothing stored.
    let again = h.import(&fixtures(), &ImportOptions::default()).unwrap();
    assert_eq!(again, ImportReport::default());
    assert_eq!(h.event_count().unwrap(), 52);
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
            visit("2026-09-02T10:15:00Z", "system", "Wolf 359", 4, None, None),
            visit("2026-09-02T10:20:00Z", "system", "Alpha Centauri", 1, None, Some(4.38)),
            // 09-03: logging in where the last session ended is not a new visit either.
            visit("2026-09-03T10:10:00Z", "station", "Alpha Centauri", 1, Some(("Hutton Orbital", 128000002)), None),
            visit("2026-09-03T10:25:00Z", "station", "Alpha Centauri", 1, Some(("Hutton Orbital", 128000002)), None),
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
            e("2026-09-02T10:50:00Z", "powerplay", 100000, None, Some("Zachary Hudson")),
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

#[test]
fn import_reports_progress() {
    let mut h = History::in_memory().unwrap();
    let mut seen = vec![];
    let opts = ImportOptions { batch_lines: 10, ..Default::default() };
    let r = h.import_with(&fixtures(), &opts, |p| seen.push(p.clone())).unwrap();
    assert_eq!((seen[0].bytes, seen[0].bytes_total), (0, journal_logs_size()));
    assert!(seen.windows(2).all(|w| w[0].bytes <= w[1].bytes && w[0].lines <= w[1].lines));
    assert!(seen.len() > 4, "after each batch and each file: {}", seen.len());
    assert_eq!(seen.last(), Some(&r));
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

/// The done-when of astromech-fxp0: a search finds a visited station with the right counts.
#[test]
fn logbook_search_finds_places_with_their_counts() {
    let (h, _) = clean_import();
    let b = h.logbook(None, "  HUTTON ", 50).unwrap();
    assert_eq!(b.commander.as_ref().map(|c| c.name.as_str()), Some("Fixture"));
    assert_eq!(b.query, "HUTTON");
    assert_eq!(
        b.places,
        vec![Place {
            kind: "station".into(),
            star_system: "Alpha Centauri".into(),
            system_address: Some(1),
            station_name: Some("Hutton Orbital".into()),
            market_id: Some(128000002),
            visits: 2,
            first: "2026-09-03T10:10:00Z".into(),
            last: "2026-09-03T10:25:00Z".into(),
        }]
    );
    assert_eq!(
        b.timeline.iter().map(|v| v.ts.as_str()).collect::<Vec<_>>(),
        ["2026-09-03T10:25:00Z", "2026-09-03T10:10:00Z"]
    );

    // A system name finds the system and the stations in it; latest visit first.
    let b = h.logbook(None, "alpha", 50).unwrap();
    let got: Vec<(&str, Option<&str>, i64, &str, &str)> = b
        .places
        .iter()
        .map(|p| (p.kind.as_str(), p.station_name.as_deref(), p.visits, p.first.as_str(), p.last.as_str()))
        .collect();
    assert_eq!(
        got,
        vec![
            ("station", Some("Hutton Orbital"), 2, "2026-09-03T10:10:00Z", "2026-09-03T10:25:00Z"),
            ("system", None, 2, "2026-09-01T10:10:00Z", "2026-09-02T10:20:00Z"),
        ]
    );

    // No search: everything, with totals.
    let b = h.logbook(Some(CMDR), "", 2).unwrap();
    assert_eq!(b.totals, Totals { systems: 4, stations: 3, system_visits: 6, station_visits: 4 });
    assert_eq!((b.matches, b.places.len(), b.timeline.len()), (7, 2, 2));
    assert!(h.logbook(None, "nowhere", 50).unwrap().places.is_empty());
}

#[test]
fn read_only_open_reads_what_the_writer_wrote() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("history.sqlite");
    assert!(History::open_read_only(&db).is_err(), "no database yet: nothing is created");
    let mut w = History::open(&db).unwrap();
    let r = History::open_read_only(&db).unwrap();
    w.import(&fixtures(), &ImportOptions::default()).unwrap();
    assert_eq!(r.logbook(None, "miller", 10).unwrap().places[0].visits, 1);
    assert!(r.conn().execute("DELETE FROM visits", []).is_err(), "read only");
}

/// The done-when of astromech-bg0x: fixture sessions give the expected totals.
#[test]
fn session_summary_totals() {
    let (h, _) = clean_import();
    let s = h.session(None, Some("2026-09-02T10:30:00Z")).unwrap().unwrap();
    assert_eq!(
        (s.start.as_str(), s.end.as_str(), s.duration_secs, s.latest),
        ("2026-09-02T10:00:05Z", "2026-09-02T11:00:00Z", 3595, false)
    );
    assert_eq!(s.earnings, vec![EarningTotal { kind: "powerplay".into(), amount: 100000, count: 1 }]);
    assert_eq!((s.total, s.per_hour, s.data_sold), (100000, Some(100139), 0));
    // Two FSD jumps; the carrier jump is counted apart and adds no distance.
    assert_eq!((s.jumps, s.carrier_jumps), (2, 1));
    assert!((s.distance_ly - (5.95 + 4.38)).abs() < 1e-9);
    // Bounties awarded (not yet cashed in): 15,000 + 25,000.
    assert_eq!((s.bounties_awarded, s.bounty_value), (2, 40000));

    let first = h.session(Some(CMDR), Some("2026-09-01T10:20:00Z")).unwrap().unwrap();
    assert_eq!((first.duration_secs, first.jumps, first.total, first.trade_profit), (1795, 2, 870000, 50000));
    let kinds: Vec<(&str, i64)> = first.earnings.iter().map(|e| (e.kind.as_str(), e.amount)).collect();
    assert_eq!(kinds, [("trade", 500000), ("mission", 250000), ("bounty", 120000)]);
    assert_eq!(first.bounties_awarded, 0, "that session's Bounty line is truncated");

    // No time given: the latest session, data sold = exploration + organic.
    let last = h.session(None, None).unwrap().unwrap();
    assert_eq!((last.start.as_str(), last.latest), ("2026-09-03T10:00:05Z", true));
    assert_eq!((last.total, last.data_sold, last.trade_profit), (7221500, 5100000, 1500));
    assert!(h.session(None, Some("2020-01-01T00:00:00Z")).unwrap().is_none());
}

/// Part 2 of a session (`Fileheader` `part: 2` after `Continued`) doesn't name the commander;
/// its lines still belong to the one part 1 named, on import and when commanders are
/// recomputed from the stored lines.
#[test]
fn continuation_part_keeps_the_commander() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("history.sqlite");
    let systems =
        |h: &History| -> Vec<String> { h.visits("F0000003").unwrap().into_iter().map(|v| v.star_system).collect() };
    {
        let mut h = History::open(&db).unwrap();
        h.import(&fixtures().join("continued"), &ImportOptions::default()).unwrap();
        assert_eq!(systems(&h), ["Sol", "Alpha Centauri"]);
        let unowned: i64 =
            h.conn().query_row("SELECT COUNT(*) FROM events WHERE cmdr IS NULL", [], |r| r.get(0)).unwrap();
        assert_eq!(unowned, 1, "only part 1's Fileheader");
        h.conn().execute("INSERT INTO pending VALUES ('commanders')", []).unwrap();
        h.conn().execute("UPDATE events SET cmdr = NULL", []).unwrap();
    }
    let h = History::open(&db).unwrap();
    assert_eq!(systems(&h), ["Sol", "Alpha Centauri"]);
}

/// The game gives a ship bought with a trade-in the id of the ship traded in (astromech-vett):
/// each ownership is its own row, and the new ship isn't marked sold.
#[test]
fn traded_in_ship_id_reused_by_the_new_ship() {
    let dir = tempfile::tempdir().unwrap();
    let lines = [
        r#"{"timestamp":"2026-09-05T10:00:00Z","event":"Commander","FID":"F5","Name":"Trader"}"#,
        r#"{"timestamp":"2026-09-05T10:00:01Z","event":"Loadout","Ship":"CobraMkIII","ShipID":5,"Modules":[]}"#,
        r#"{"timestamp":"2026-09-05T10:05:00Z","event":"ShipyardBuy","ShipType":"python","ShipPrice":100,"SellOldShip":"cobramkiii","SellShipID":5,"SellPrice":10,"MarketID":1}"#,
        r#"{"timestamp":"2026-09-05T10:05:00Z","event":"ShipyardNew","ShipType":"python","NewShipID":5}"#,
        r#"{"timestamp":"2026-09-05T10:05:02Z","event":"Loadout","Ship":"python","ShipID":5,"Modules":[]}"#,
        r#"{"timestamp":"2026-09-05T10:06:00Z","event":"SetUserShipName","Ship":"python","ShipID":5,"UserShipName":"Snake","UserShipId":"SN-05"}"#,
        r#"{"timestamp":"2026-09-05T10:07:00Z","event":"ShipRedeemed","ShipType":"sidewinder","NewShipID":6}"#,
    ];
    std::fs::write(dir.path().join("Journal.2026-09-05T100000.01.log"), lines.join("\r\n") + "\r\n").unwrap();
    let mut h = History::in_memory().unwrap();
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    let rows = |h: &History| -> Vec<(i64, String, Option<String>, Option<String>, Option<String>)> {
        h.ships("F5").unwrap().into_iter().map(|s| (s.ship_id, s.ship_type, s.name, s.acquired, s.sold)).collect()
    };
    let t = |s: &str| Some(format!("2026-09-05T{s}Z"));
    let want = vec![
        (5, "cobramkiii".into(), None, None, t("10:05:00")),
        (5, "python".into(), Some("Snake".into()), t("10:05:00"), None),
        (6, "sidewinder".into(), None, t("10:07:00"), None),
    ];
    assert_eq!(rows(&h), want);
    h.rebuild_ledgers().unwrap();
    assert_eq!(rows(&h), want);
}

/// A ship id seen on another type without its sale (the journal with the sale is gone):
/// the old ship was gone by then, so its row closes and the new ship gets its own.
#[test]
fn ship_id_on_another_type_without_a_sale_is_a_new_ship() {
    let h = History::in_memory().unwrap();
    let loadout = |ts: &str, ship: &str| {
        parse_line(&format!(r#"{{"timestamp":"{ts}","event":"Loadout","Ship":"{ship}","ShipID":7,"Modules":[]}}"#))
            .unwrap()
    };
    ledgers::derive(h.conn(), 1, "F1", &loadout("2026-09-01T00:00:00Z", "cobramkiii")).unwrap();
    ledgers::derive(h.conn(), 2, "F1", &loadout("2026-09-02T00:00:00Z", "python")).unwrap();
    let rows: Vec<(String, Option<String>)> =
        h.ships("F1").unwrap().into_iter().map(|s| (s.ship_type, s.sold)).collect();
    assert_eq!(rows, vec![("cobramkiii".into(), Some("2026-09-02T00:00:00Z".into())), ("python".into(), None)]);
}

const OLD_JOURNAL: (&str, &str) = (
    "Journal.160610140000.01.log",
    r#"{"timestamp":"2016-06-10T14:00:00Z","event":"Fileheader","part":1,"gameversion":"2.1"}
{"timestamp":"2016-06-10T14:00:01Z","event":"LoadGame","Commander":"Old Timer","Ship":"SideWinder","ShipID":1,"Credits":1000,"Loan":0}
{"timestamp":"2016-06-10T14:05:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0,0,0],"JumpDist":5.0}
"#,
);

fn new_journal(fid: &str, name: &str, day: u32) -> (String, String) {
    (
        format!("Journal.2026-09-{day:02}T100000.01.log"),
        format!(
            r#"{{"timestamp":"2026-09-{day:02}T10:00:00Z","event":"Fileheader","part":1,"gameversion":"4.0.0.1900"}}
{{"timestamp":"2026-09-{day:02}T10:00:01Z","event":"Commander","FID":"{fid}","Name":"{name}"}}
{{"timestamp":"2026-09-{day:02}T10:00:02Z","event":"LoadGame","FID":"{fid}","Commander":"{name}","Ship":"Python","ShipID":4,"Credits":1000,"Loan":0}}
{{"timestamp":"2026-09-{day:02}T10:05:00Z","event":"FSDJump","StarSystem":"Sol","SystemAddress":10477373803,"StarPos":[0,0,0],"JumpDist":5.0}}
"#
        ),
    )
}

fn commander_keys(h: &History) -> Vec<String> {
    let mut keys: Vec<String> = h.commanders().unwrap().into_iter().map(|c| c.cmdr).collect();
    keys.sort();
    keys
}

/// Journals from before the Frontier id give the commander `name:<Commander>`; once one FID
/// has that name they are that commander's history (astromech-yrfv), whichever is imported
/// first, after a rebuild, and when commanders are recomputed from the stored lines.
#[test]
fn journals_from_before_fid_join_the_commander_with_that_name() {
    let dir = tempfile::tempdir().unwrap();
    let (new_name, new_text) = new_journal("F7", "Old Timer", 7);
    for in_order in [true, false] {
        let jdir = tempfile::tempdir().unwrap();
        let db = dir.path().join(format!("history-{in_order}.sqlite"));
        let write = |(n, t): (&str, &str)| std::fs::write(jdir.path().join(n), t).unwrap();
        let mut h = History::open(&db).unwrap();
        if in_order {
            write(OLD_JOURNAL);
            write((&new_name, &new_text));
            h.import(jdir.path(), &ImportOptions::default()).unwrap();
        } else {
            write((&new_name, &new_text));
            h.import(jdir.path(), &ImportOptions::default()).unwrap();
            write(OLD_JOURNAL);
            h.import(jdir.path(), &ImportOptions::default()).unwrap();
        }
        assert_eq!(commander_keys(&h), ["F7"], "in order: {in_order}");
        let jumps: Vec<String> = h.visits("F7").unwrap().into_iter().map(|v| v.ts).collect();
        assert_eq!(jumps, ["2016-06-10T14:05:00Z", "2026-09-07T10:05:00Z"]);
        assert_eq!(h.logbook(None, "", 10).unwrap().totals.system_visits, 2);
        let want = snapshot(&h);
        h.rebuild_ledgers().unwrap();
        assert_eq!(snapshot(&h), want);
        h.conn().execute("INSERT INTO pending VALUES ('commanders')", []).unwrap();
        drop(h);
        assert_eq!(snapshot(&History::open(&db).unwrap()), want);
    }
}

/// Two FIDs with the same name stay apart, and the journals from before FID belong to neither.
#[test]
fn two_fids_with_one_name_stay_apart() {
    let dir = tempfile::tempdir().unwrap();
    let write = |(n, t): (&str, &str)| std::fs::write(dir.path().join(n), t).unwrap();
    write(OLD_JOURNAL);
    for (fid, day) in [("F7", 7), ("F8", 8)] {
        let (n, t) = new_journal(fid, "Old Timer", day);
        write((&n, &t));
    }
    let mut h = History::in_memory().unwrap();
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    assert_eq!(commander_keys(&h), ["F7", "F8", "name:Old Timer"]);
    assert_eq!(h.visits("F7").unwrap().len(), 1);
}

/// A continuation part whose first part is missing doesn't take the commander of whatever
/// file sorts before it.
#[test]
fn continuation_part_of_another_session_has_no_commander() {
    let dir = tempfile::tempdir().unwrap();
    let (n, t) = new_journal("F7", "Old Timer", 7);
    std::fs::write(dir.path().join(n), t).unwrap();
    std::fs::write(
        dir.path().join("Journal.2026-09-08T100000.02.log"),
        r#"{"timestamp":"2026-09-08T12:00:00Z","event":"Fileheader","part":2,"gameversion":"4.0.0.1900"}
{"timestamp":"2026-09-08T12:05:00Z","event":"FSDJump","StarSystem":"Alpha Centauri","SystemAddress":1,"StarPos":[3,0,0],"JumpDist":4.4}
"#,
    )
    .unwrap();
    let mut h = History::in_memory().unwrap();
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    assert_eq!(h.visits("F7").unwrap().len(), 1);
    let unowned: i64 = h.conn().query_row("SELECT COUNT(*) FROM events WHERE cmdr IS NULL", [], |r| r.get(0)).unwrap();
    assert_eq!(unowned, 3, "both Fileheaders and the other session's jump");
}

/// SQLite's lower() folds ASCII only; the search folds names and query alike (astromech-hloh).
#[test]
fn logbook_search_folds_non_ascii_capitals() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("Journal.2026-09-09T100000.01.log"),
        r#"{"timestamp":"2026-09-09T10:00:00Z","event":"Commander","FID":"F9","Name":"Unicode"}
{"timestamp":"2026-09-09T10:01:00Z","event":"FSDJump","StarSystem":"Ölberg","SystemAddress":9,"StarPos":[0,0,0],"JumpDist":3.0}
{"timestamp":"2026-09-09T10:02:00Z","event":"Docked","StationName":"ÉCOLE DES ÉTOILES","StationType":"FleetCarrier","StarSystem":"Ölberg","SystemAddress":9,"MarketID":3700000001}
"#,
    )
    .unwrap();
    let mut h = History::in_memory().unwrap();
    h.import(dir.path(), &ImportOptions::default()).unwrap();
    let found = |q: &str| -> (Vec<String>, usize) {
        let b = h.logbook(None, q, 10).unwrap();
        (b.places.into_iter().map(|p| p.station_name.unwrap_or(p.star_system)).collect(), b.timeline.len())
    };
    assert_eq!(found("école"), (vec!["ÉCOLE DES ÉTOILES".to_string()], 1));
    assert_eq!(found("ölb"), (vec!["ÉCOLE DES ÉTOILES".to_string(), "Ölberg".to_string()], 2));
    assert_eq!(found("ÖLBERG").1, 2);
    assert_eq!(found("zzz"), (vec![], 0));
}

/// The window's read-only connection waits out a writer's brief lock instead of failing.
#[test]
fn read_only_open_waits_when_busy() {
    let dir = tempfile::tempdir().unwrap();
    let db = dir.path().join("history.sqlite");
    History::open(&db).unwrap();
    let r = History::open_read_only(&db).unwrap();
    let ms: i64 = r.conn().query_row("PRAGMA busy_timeout", [], |row| row.get(0)).unwrap();
    assert_eq!(ms, 5000);
}
