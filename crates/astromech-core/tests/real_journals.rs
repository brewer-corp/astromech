//! Replays a real journal folder through the core and every module (astromech-yin0), and prints
//! a summary per module: what it read, what it made of it, how long it took, and anything that
//! looks wrong. It skips unless `ASTROMECH_REAL_JOURNALS` names a folder, so CI never runs it.
//!
//! ```sh
//! ASTROMECH_REAL_JOURNALS=~/edjournal cargo test -p astromech-core --release \
//!     --test real_journals -- --nocapture
//! ```
//!
//! A real folder is personal data (commander name, FID, history). The harness copies it to a
//! temporary folder and reads only that copy. Nothing goes over the network: the EDDN and EDSM
//! parts build their messages into in-memory queues with no delivery worker, and no HTTP
//! transport is created. The summary prints counts and checks, not journal contents. With
//! `ASTROMECH_REAL_OUT` set to a folder (outside the repository), each module's full result is
//! also written there as JSON for a closer look.
//!
//! The test fails if any module panics or a check fails; the summary says which.

use astromech_announce::Engine;
use astromech_core::callouts::Callouts;
use astromech_core::hub::StateHub;
use astromech_eddn::edsm::{Credentials, Discards, Switch, Uploader};
use astromech_eddn::{Feed, Outbox, Outcome, Sender, Software};
use astromech_gamestate::GameState;
use astromech_journal::{JournalEvent, JournalLine};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// One module's part of the summary.
struct Section {
    name: &'static str,
    elapsed: Duration,
    lines: Vec<String>,
    problems: Vec<String>,
}

#[derive(Default)]
struct Out {
    lines: Vec<String>,
    problems: Vec<String>,
}

impl Out {
    fn say(&mut self, s: impl Into<String>) {
        self.lines.push(s.into());
    }
    fn problem(&mut self, s: impl Into<String>) {
        self.problems.push(s.into());
    }
    fn check(&mut self, ok: bool, what: impl Into<String>) {
        if !ok {
            self.problem(what);
        }
    }
}

struct Harness {
    dir: PathBuf,
    scratch: PathBuf,
    dump: Option<PathBuf>,
}

/// Run one module's part, catching a panic so the others still run.
fn run(sections: &mut Vec<Section>, name: &'static str, f: impl FnOnce(&mut Out)) {
    let mut out = Out::default();
    let t = Instant::now();
    let r = catch_unwind(AssertUnwindSafe(|| f(&mut out)));
    let elapsed = t.elapsed();
    if let Err(p) = r {
        let msg = p
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default();
        out.problem(format!("PANIC: {msg}"));
    }
    sections.push(Section { name, elapsed, lines: out.lines, problems: out.problems });
}

impl Harness {
    /// Write `v` to the dump folder, if there is one.
    fn dump(&self, name: &str, v: &impl serde::Serialize) {
        if let Some(d) = &self.dump {
            let _ = std::fs::write(d.join(format!("{name}.json")), serde_json::to_vec_pretty(v).unwrap_or_default());
        }
    }
}

/// Copy the journals and companion files, so nothing can write next to the originals.
fn copy_folder(from: &Path, to: &Path) -> usize {
    let mut n = 0;
    for e in std::fs::read_dir(from).unwrap().flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let wanted = (name.starts_with("Journal.") && name.ends_with(".log")) || name.ends_with(".json");
        if wanted && e.file_type().is_ok_and(|t| t.is_file()) {
            let dest = to.join(&name);
            std::fs::copy(e.path(), &dest).unwrap();
            // Keep the modification time: the pipeline picks the journal the game wrote last.
            let modified = e.metadata().unwrap().modified().unwrap();
            std::fs::File::options().write(true).open(&dest).unwrap().set_modified(modified).unwrap();
            n += 1;
        }
    }
    n
}

fn unix(ts: &str) -> Option<i64> {
    chrono::DateTime::parse_from_rfc3339(ts).ok().map(|t| t.timestamp())
}

fn all_lines(dir: &Path) -> Vec<JournalLine> {
    astromech_journal::journal_logs(dir).iter().flat_map(|p| astromech_journal::parse_file(p)).collect()
}

/// Every commander name and every FID in the folder, to check what EDDN would be sent: the name
/// goes only in the header's `uploaderID` (EDDN asks for it there), the FID nowhere.
struct Identities {
    names: BTreeSet<String>,
    fids: BTreeSet<String>,
}

fn identities(lines: &[JournalLine]) -> Identities {
    let mut ids = Identities { names: BTreeSet::new(), fids: BTreeSet::new() };
    for l in lines.iter().filter(|l| l.name == "LoadGame" || l.name == "Commander") {
        let v: Value = serde_json::from_str(&l.raw_json).unwrap_or_default();
        let s = |k: &str| v.get(k).and_then(Value::as_str).filter(|s| s.len() >= 3).map(str::to_string);
        ids.names.extend(s("Commander").or(s("Name")));
        ids.fids.extend(s("FID"));
    }
    ids
}

fn fmt_counts(m: &BTreeMap<String, usize>) -> String {
    let mut v: Vec<_> = m.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    v.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ")
}

fn gunzip(b: &[u8]) -> Value {
    serde_json::from_reader(flate2::read::GzDecoder::new(b)).unwrap_or_default()
}

/// `https://eddn.edcd.io/schemas/journal/1` → `journal-v1.0.json`.
fn schema_file(schema_ref: &str) -> String {
    let parts: Vec<&str> = schema_ref.trim_end_matches("/test").rsplitn(3, '/').collect();
    format!("{}-v{}.0.json", parts.get(1).unwrap_or(&""), parts.first().unwrap_or(&""))
}

fn schema_errors(cache: &mut BTreeMap<String, Option<jsonschema::Validator>>, file: &str, v: &Value) -> Vec<String> {
    let validator = cache.entry(file.to_string()).or_insert_with(|| {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../astromech-eddn/tests/fixtures/schemas").join(file);
        let schema: Value = serde_json::from_slice(&std::fs::read(path).ok()?).ok()?;
        jsonschema::draft4::options().should_validate_formats(true).build(&schema).ok()
    });
    match validator {
        Some(val) => val.iter_errors(v).map(|e| format!("{}: {e}", e.instance_path())).collect(),
        None => vec![format!("no schema file {file}")],
    }
}

/// The game state after every line of every journal, oldest first, then the companion files.
fn full_state(dir: &Path, lines: &[JournalLine]) -> GameState {
    let mut g = GameState::default();
    for l in lines {
        g.apply_line(l);
    }
    if let Some(s) = astromech_status::read_status(&dir.join("Status.json")) {
        g.apply_status(&s);
    }
    if let Some(c) = std::fs::read(dir.join("Cargo.json")).ok().and_then(|b| astromech_journal::parse_cargo(&b)) {
        g.apply_cargo(&c);
    }
    g
}

#[test]
fn real_journals() {
    let Some(src) = std::env::var_os("ASTROMECH_REAL_JOURNALS").map(PathBuf::from) else {
        eprintln!("real_journals: skipped (set ASTROMECH_REAL_JOURNALS to a journal folder)");
        return;
    };
    if !src.is_dir() {
        eprintln!("real_journals: skipped ({} is not a folder)", src.display());
        return;
    }
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("journals");
    let scratch = tmp.path().join("scratch");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::create_dir_all(&scratch).unwrap();
    let copied = copy_folder(&src, &dir);
    let dump = std::env::var_os("ASTROMECH_REAL_OUT").map(PathBuf::from);
    if let Some(d) = &dump {
        std::fs::create_dir_all(d).unwrap();
    }
    let h = Harness { dir: dir.clone(), scratch, dump };
    let mut sections = vec![];

    let t = Instant::now();
    let lines = all_lines(&dir);
    let parse_time = t.elapsed();
    let ids = identities(&lines);

    run(&mut sections, "journal parse", |o| parse_section(&dir, copied, parse_time, o));
    let full = full_state(&dir, &lines);
    run(&mut sections, "history import, logbook, session", |o| history_section(&h, o));
    run(&mut sections, "game state (every journal)", |o| state_section(&h, &full, &lines, o));
    run(&mut sections, "pipeline catch-up (core startup)", |o| catch_up_section(&h, &full, o));
    run(&mut sections, "callouts in replay (must stay silent)", |o| callouts_replay_section(&lines, o));
    run(&mut sections, "callouts if every line were live (counts only)", |o| callouts_live_section(&h, &lines, o));
    run(&mut sections, "materials ledger", |o| materials_section(&h, &lines, o));
    run(&mut sections, "fleet, ships, engineering", |o| fleet_section(&h, &full, &lines, o));
    run(&mut sections, "Odyssey micro-resources", |o| odyssey_section(&h, o));
    run(&mut sections, "colonisation", |o| colonisation_section(&h, o));
    run(&mut sections, "Powerplay", |o| powerplay_section(&h, &lines, o));
    run(&mut sections, "exploration, exobiology", |o| exploration_section(&h, &full, o));
    run(&mut sections, "galaxy visits, station, market", |o| galaxy_section(&h, &full, o));
    run(&mut sections, "missions", |o| missions_section(&full, &lines, o));
    run(&mut sections, "EDDN dry run (nothing sent)", |o| eddn_section(&dir, &lines, &ids, o));
    run(&mut sections, "EDSM dry run (nothing sent)", |o| edsm_section(&lines, o));

    let mut failed = 0;
    println!("\n==== real journals: {} files copied, {} lines ====", copied, lines.len());
    for s in &sections {
        println!("\n## {} ({:.2} s)", s.name, s.elapsed.as_secs_f64());
        for l in &s.lines {
            println!("  {l}");
        }
        for p in &s.problems {
            println!("  PROBLEM: {p}");
        }
        failed += s.problems.len();
    }
    println!("\n==== {failed} problem(s) ====");
    assert_eq!(failed, 0, "see the PROBLEM lines above");
}

fn parse_section(dir: &Path, copied: usize, parse_time: Duration, o: &mut Out) {
    let mut total = 0usize;
    let mut malformed = 0usize;
    let mut typed: BTreeMap<String, usize> = BTreeMap::new();
    let mut other: BTreeMap<String, usize> = BTreeMap::new();
    let mut conditional: BTreeMap<String, usize> = BTreeMap::new();
    let mut files = 0;
    for p in astromech_journal::journal_logs(dir) {
        files += 1;
        let text = std::fs::read_to_string(&p).unwrap_or_default();
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
            total += 1;
            match astromech_journal::parse_line(line) {
                Ok(l) => match l.event {
                    // Typed only in one form: a purchase with a trade-in, a contribution of materials.
                    JournalEvent::Other { event, raw }
                        if (event == "ShipyardBuy" && raw.get("SellShipID").is_none())
                            || (event == "EngineerContribution"
                                && !raw
                                    .get("Type")
                                    .and_then(Value::as_str)
                                    .is_some_and(|t| t.eq_ignore_ascii_case("materials"))) =>
                    {
                        *conditional.entry(event).or_default() += 1
                    }
                    JournalEvent::Other { event, .. } => *other.entry(event).or_default() += 1,
                    _ => *typed.entry(l.name).or_default() += 1,
                },
                Err(_) => malformed += 1,
            }
        }
    }
    o.say(format!(
        "{files} journals ({copied} files copied), {total} lines, parsed in {:.2} s",
        parse_time.as_secs_f64()
    ));
    o.say(format!(
        "{} event types typed ({} lines), {} kept as raw JSON ({} lines), {malformed} malformed",
        typed.len(),
        typed.values().sum::<usize>(),
        other.len(),
        other.values().sum::<usize>()
    ));
    let fell_back: BTreeMap<String, usize> =
        other.iter().filter(|(k, _)| typed.contains_key(*k)).map(|(k, v)| (k.clone(), *v)).collect();
    if !fell_back.is_empty() {
        o.problem(format!("typed events that fell back to raw JSON (a field missing?): {}", fmt_counts(&fell_back)));
    }
    o.say(format!("raw JSON: {}", fmt_counts(&other)));
    o.say(format!("raw JSON by design (no trade-in, not materials): {}", fmt_counts(&conditional)));
    o.check(malformed == 0, format!("{malformed} malformed lines"));
}

fn history_section(h: &Harness, o: &mut Out) {
    use astromech_history::{History, ImportOptions};
    let db = h.scratch.join("history.sqlite");
    let mut hist = History::open(&db).unwrap();
    let t = Instant::now();
    let r = hist.import(&h.dir, &ImportOptions::default()).unwrap();
    let first = t.elapsed();
    o.say(format!(
        "full import: {} files, {} lines, {} events, {} malformed, {:.1} MB in {:.2} s",
        r.files,
        r.lines,
        r.events,
        r.malformed,
        r.bytes as f64 / 1e6,
        first.as_secs_f64()
    ));
    o.check(r.malformed == 0, format!("{} malformed lines skipped", r.malformed));
    o.check(r.shrunk.is_empty(), format!("files shrank: {}", r.shrunk.len()));
    o.check(r.events == r.lines, format!("{} lines but {} events stored", r.lines, r.events));
    let t = Instant::now();
    let again = hist.import(&h.dir, &ImportOptions::default()).unwrap();
    o.say(format!("second import: {} new events in {:.3} s", again.events, t.elapsed().as_secs_f64()));
    o.check(again.events == 0, "a second import stored events again");
    let t = Instant::now();
    hist.rebuild_ledgers().unwrap();
    o.say(format!("ledger rebuild: {:.2} s", t.elapsed().as_secs_f64()));
    let size = std::fs::metadata(&db).map(|m| m.len()).unwrap_or(0);
    o.say(format!("history.sqlite: {:.1} MB", size as f64 / 1e6));

    let t = Instant::now();
    let book = hist.logbook(None, "", 50).unwrap();
    o.say(format!(
        "logbook ({:.3} s): {} commander(s), {} systems / {} system visits, {} stations / {} dockings",
        t.elapsed().as_secs_f64(),
        book.commanders.len(),
        book.totals.systems,
        book.totals.system_visits,
        book.totals.stations,
        book.totals.station_visits
    ));
    o.say(format!("legacy commanders: {}", book.commanders.iter().filter(|c| c.legacy).count()));
    h.dump("logbook", &book);
    let cmdr = book.commander.as_ref().map(|c| c.cmdr.clone()).unwrap_or_default();
    let ships = hist.ships(&cmdr).unwrap();
    o.say(format!(
        "ships ledger: {} rows, {} sold ({})",
        ships.len(),
        ships.iter().filter(|s| s.sold.is_some()).count(),
        ships
            .iter()
            .map(|s| format!("#{} {}{}", s.ship_id, s.ship_type, if s.sold.is_some() { " sold" } else { "" }))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    h.dump("ships", &ships);
    // Every ship the fleet has now is in the ledger, under its type, and not sold.
    let fleet = astromech_engineering::fleet::from_journal_dir(&h.dir);
    for f in &fleet.ships {
        match ships.iter().find(|s| s.ship_id == f.id) {
            None => o.problem(format!("fleet ship #{} {} is not in the ships ledger", f.id, f.ship_type)),
            Some(s) if s.sold.is_some() || s.ship_type != f.ship_type => o.problem(format!(
                "fleet ship #{} {} is in the ships ledger as {}{}",
                f.id,
                f.ship_type,
                s.ship_type,
                if s.sold.is_some() { ", sold" } else { "" }
            )),
            Some(_) => {}
        }
    }
    // Every ship type the commander flew (a Loadout) has a row.
    let flown: BTreeSet<String> = hist
        .conn()
        .prepare("SELECT DISTINCT lower(json_extract(json, '$.Ship')) FROM events WHERE event = 'Loadout'")
        .unwrap()
        .query_map([], |r| r.get::<_, Option<String>>(0))
        .unwrap()
        .filter_map(|r| r.ok().flatten())
        .collect();
    let have: BTreeSet<String> = ships.iter().map(|s| s.ship_type.clone()).collect();
    let lost: Vec<_> = flown.difference(&have).cloned().collect();
    o.check(lost.is_empty(), format!("ship types flown but not in the ships ledger: {}", lost.join(", ")));

    let t = Instant::now();
    let s = hist.session(None, None).unwrap();
    o.say(format!("latest session ({:.3} s):", t.elapsed().as_secs_f64()));
    if let Some(s) = &s {
        o.say(format!(
            "  {} s long, {} cr total ({:?}/h), trade profit {}, {} jumps / {:.0} ly, {} bounties, data sold {}",
            s.duration_secs,
            s.total,
            s.per_hour,
            s.trade_profit,
            s.jumps,
            s.distance_ly,
            s.bounties_awarded,
            s.data_sold
        ));
        o.say(format!(
            "  earnings: {}",
            s.earnings.iter().map(|e| format!("{} {}x {}", e.kind, e.count, e.amount)).collect::<Vec<_>>().join(", ")
        ));
        o.check(s.duration_secs >= 0, "negative session length");
        o.check(s.total >= 0, "negative session earnings");
    } else {
        o.problem("no session found");
    }
    h.dump("session", &s);
    let t = Instant::now();
    let fold = astromech_exploration::system::fold_from_journal_dir(&h.dir);
    let live_bodies = fold.view().bodies.len();
    let view = hist.system_view(fold);
    o.say(format!(
        "current system with earlier visits ({:.3} s): {} bodies ({} from this visit), honk count {:?}",
        t.elapsed().as_secs_f64(),
        view.bodies.len(),
        live_bodies,
        view.body_count
    ));
    let t = Instant::now();
    let unsold = hist.unsold(None).unwrap();
    o.say(format!("unsold data ({:.3} s): {}", t.elapsed().as_secs_f64(), unsold.is_some()));
}

fn state_section(h: &Harness, g: &GameState, lines: &[JournalLine], o: &mut Out) {
    o.say(format!(
        "commander {}, ship {:?} (id {:?}), docked {}, system set {}, station set {}",
        if g.commander.is_some() { "set" } else { "MISSING" },
        g.ship,
        g.ship_id,
        g.docked,
        g.system.is_some(),
        g.station.is_some()
    ));
    o.say(format!(
        "credits {:?}, rebuy {:?}, hull {:?}, jump range {:?}, fuel {:?}/{:?}, cargo {:?}/{:?}",
        g.credits,
        g.rebuy,
        g.hull_health,
        g.max_jump_range,
        g.fuel_main,
        g.fuel_capacity_main,
        g.cargo_tonnes,
        g.cargo_capacity
    ));
    o.say(format!(
        "materials {} kinds, missions {}, route {} hops, legal {:?}",
        g.materials.counts.len(),
        g.missions.len(),
        g.route.len(),
        g.legal_state
    ));
    h.dump("gamestate", g);
    // Against the journal itself.
    if lines.iter().any(|l| l.name == "LoadGame") {
        o.check(g.commander.is_some(), "no commander after a LoadGame");
    }
    let status: Option<Value> =
        std::fs::read(h.dir.join("Status.json")).ok().and_then(|b| serde_json::from_slice(&b).ok());
    if let Some(bal) = status.as_ref().and_then(|s| s.get("Balance")).and_then(Value::as_i64) {
        o.say(format!("Status.json Balance {bal}"));
        o.check(g.credits == Some(bal), format!("credits {:?} but Status.json says {bal}", g.credits));
    }
    if let Some(lo) = lines.iter().rev().find(|l| l.name == "Loadout") {
        let v: Value = serde_json::from_str(&lo.raw_json).unwrap();
        let ship = v.get("Ship").and_then(Value::as_str).map(str::to_ascii_lowercase);
        o.check(
            g.ship.as_deref().map(str::to_ascii_lowercase) == ship,
            format!("ship {:?} but the last Loadout is {ship:?}", g.ship),
        );
        let id = v.get("ShipID").and_then(Value::as_i64);
        o.check(g.ship_id == id, format!("ship id {:?} but the last Loadout says {id:?}", g.ship_id));
    }
    // The last position event names the system.
    if let Some(p) =
        lines.iter().rev().find(|l| matches!(l.event, JournalEvent::Location { .. } | JournalEvent::FsdJump { .. }))
    {
        let sys = match &p.event {
            JournalEvent::Location { star_system, .. } | JournalEvent::FsdJump { star_system, .. } => star_system,
            _ => unreachable!(),
        };
        o.check(g.system.as_deref() == Some(sys.as_str()), "system is not the last Location/FSDJump's");
    }
}

fn catch_up_section(h: &Harness, full: &GameState, o: &mut Out) {
    let hub = Arc::new(StateHub::new());
    let fired = Arc::new(AtomicUsize::new(0));
    let fired2 = fired.clone();
    let shared = astromech_announce::Shared::default();
    let engine = Engine::new(astromech_announce::builtin(shared), &Default::default());
    let callouts = Callouts {
        engine,
        deliver: Box::new(move |r: Vec<astromech_announce::Reaction>| {
            fired2.fetch_add(r.len(), Ordering::SeqCst);
        }),
    };
    let eddn_db = h.scratch.join("catchup-eddn.sqlite");
    let edsm_db = h.scratch.join("catchup-edsm.sqlite");
    let soft = Software { name: "astromech".into(), version: "test".into() };
    // No delivery worker: these feeds can only queue.
    let feed = Feed::new(Sender::new(Outbox::open(&eddn_db).unwrap(), soft, true));
    let switch = Arc::new(Switch::default());
    if let Some(c) = &full.commander {
        switch.set(Some(Credentials { commander: c.clone(), api_key: "none".into() }));
    }
    let edsm = Uploader::new(Outbox::open(&edsm_db).unwrap().with_label("edsm"), switch, Arc::new(Discards::default()));
    let logs = Arc::new(Mutex::new(Vec::<String>::new()));
    let logs2 = logs.clone();
    let store = h.scratch.join("astromech.sqlite");
    let t = Instant::now();
    let p = astromech_core::pipeline::start_with_eddn(
        &h.dir,
        hub.clone(),
        Some(&store),
        move |m| logs2.lock().unwrap().push(m.to_string()),
        Some(feed),
        Some(edsm),
        Some(callouts),
    )
    .unwrap();
    let took = t.elapsed();
    let stats = p.stats.lock().unwrap().clone();
    drop(p);
    let g = hub.snapshot();
    o.say(format!(
        "catch-up {:.3} s: {} journal lines, {} bad, {} companion files read, {} unused, {} Status.json reads",
        took.as_secs_f64(),
        stats.journal_lines,
        stats.bad_lines,
        stats.companions_read,
        stats.companions_unused,
        stats.status_reads
    ));
    o.check(stats.bad_lines == 0, format!("{} bad lines in the catch-up", stats.bad_lines));
    o.say(format!("pipeline log: {} lines", logs.lock().unwrap().len()));
    for l in logs.lock().unwrap().iter().take(20) {
        o.say(format!("  log: {}", l.chars().take(160).collect::<String>()));
    }
    let n = fired.load(Ordering::SeqCst);
    o.check(n == 0, format!("{n} callouts fired during the catch-up"));
    let eddn_q = Outbox::open(&eddn_db).unwrap().pending().unwrap();
    let edsm_q = Outbox::open(&edsm_db).unwrap().pending().unwrap();
    o.say(format!("queued during catch-up: EDDN {eddn_q}, EDSM {edsm_q}"));
    o.check(eddn_q == 0 && edsm_q == 0, "the catch-up queued messages to send");
    // The newest journal alone should leave the same ship, place and credits as all of them.
    for (what, a, b) in [
        ("ship", format!("{:?}", g.ship), format!("{:?}", full.ship)),
        ("ship id", format!("{:?}", g.ship_id), format!("{:?}", full.ship_id)),
        ("system", format!("{:?}", g.system), format!("{:?}", full.system)),
        ("station", format!("{:?}", g.station), format!("{:?}", full.station)),
        ("docked", format!("{:?}", g.docked), format!("{:?}", full.docked)),
        ("credits", format!("{:?}", g.credits), format!("{:?}", full.credits)),
        ("commander", format!("{:?}", g.commander.is_some()), format!("{:?}", full.commander.is_some())),
        ("materials kinds", format!("{}", g.materials.counts.len()), format!("{}", full.materials.counts.len())),
        ("missions", format!("{}", g.missions.len()), format!("{}", full.missions.len())),
        ("powerplay", format!("{:?}", g.powerplay.power), format!("{:?}", full.powerplay.power)),
    ] {
        let same = a == b;
        let shown = if what == "system" || what == "station" || what == "commander" {
            if same {
                "same".to_string()
            } else {
                "differs".to_string()
            }
        } else {
            format!("{a} vs {b}")
        };
        o.say(format!("catch-up vs every journal, {what}: {shown}"));
        if !same {
            o.problem(format!("catch-up {what} differs from the full replay"));
        }
    }
}

fn callouts_replay_section(lines: &[JournalLine], o: &mut Out) {
    let mut engine = Engine::new(astromech_announce::builtin(Default::default()), &Default::default());
    engine.set_live(false);
    let mut g = GameState::default();
    let now = Instant::now();
    let mut fired = 0;
    for l in lines {
        g.apply_line(l);
        fired += engine.journal(l, &g, now).len();
    }
    o.say(format!("{} lines replayed, {fired} callouts", lines.len()));
    o.check(fired == 0, format!("{fired} callouts fired in replay mode"));
}

fn callouts_live_section(h: &Harness, lines: &[JournalLine], o: &mut Out) {
    // Not a check: what the rules would say if each line arrived live, in game time, to spot
    // rules that panic on real data or would talk far too much.
    let mut engine = Engine::new(astromech_announce::builtin(Default::default()), &Default::default());
    engine.set_live(true);
    let mut g = GameState::default();
    let base = Instant::now();
    let first = lines.iter().find_map(|l| unix(&l.timestamp)).unwrap_or(0);
    let mut by_rule: BTreeMap<String, usize> = BTreeMap::new();
    let mut samples: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut last_tick = 0;
    for l in lines {
        g.apply_line(l);
        let ts = unix(&l.timestamp).unwrap_or(first);
        let now = base + Duration::from_secs((ts - first).max(0) as u64);
        let mut r = engine.journal(l, &g, now);
        if ts != last_tick {
            last_tick = ts;
            r.extend(engine.tick(ts, &g, now));
        }
        for x in r {
            *by_rule.entry(x.rule.to_string()).or_default() += 1;
            let s = samples.entry(x.rule.to_string()).or_default();
            if s.len() < 3 {
                // Only the shape of the line: digits and capitalised words (names) blanked.
                let shape: String = x
                    .text
                    .split(' ')
                    .map(|w| {
                        if w.chars().next().is_some_and(|c| c.is_ascii_digit()) {
                            "#"
                        } else if w.chars().next().is_some_and(char::is_uppercase) {
                            "X"
                        } else {
                            w
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ");
                s.insert(shape);
            }
        }
    }
    o.say(format!("by rule: {}", fmt_counts(&by_rule)));
    for (rule, s) in &samples {
        o.say(format!("  {rule}: {}", s.iter().cloned().collect::<Vec<_>>().join(" | ")));
    }
    h.dump("callouts_live", &by_rule);
}

fn materials_section(h: &Harness, lines: &[JournalLine], o: &mut Out) {
    let inv = astromech_materials::view::from_journal_dir(&h.dir);
    let cells: Vec<_> = inv.kinds.iter().flat_map(|k| k.groups.iter().flat_map(|g| g.cells.iter())).collect();
    let held: i64 = inv.kinds.iter().map(|k| k.held).sum();
    o.say(format!(
        "view: snapshot {}, {} kinds, {} trader materials ({} held, {} units), {} others held (Guardian, Thargoid, unknown)",
        if inv.snapshot_ts.is_some() { "found" } else { "MISSING" },
        inv.kinds.len(),
        cells.len(),
        cells.iter().filter(|c| c.count > 0).count(),
        held,
        inv.other.len()
    ));
    for k in &inv.kinds {
        o.say(format!("  {}: {} of {}", k.id, k.held, k.capacity));
    }
    let unknown: Vec<String> = inv.other.iter().filter(|c| c.group == "Unknown").map(|c| c.id.clone()).collect();
    if !unknown.is_empty() {
        o.problem(format!("materials not in the catalogue: {}", unknown.join(", ")));
    }
    for c in cells.iter().filter(|c| c.count > c.capacity && c.capacity > 0) {
        o.problem(format!("{} holds {} over its cap {}", c.id, c.count, c.capacity));
    }
    for c in cells.iter().filter(|c| c.count < 0) {
        o.problem(format!("{} is negative ({})", c.id, c.count));
    }
    h.dump("materials", &inv);
    // The self-check of §18.5: the deltas since one `Materials` snapshot should give the next.
    let mut ledger = astromech_materials::Ledger::default();
    let mut snapshots = 0;
    let mut drift_total = 0;
    let mut drifts: BTreeMap<String, i64> = BTreeMap::new();
    for l in lines {
        if let JournalEvent::Materials { .. } = &l.event {
            if ledger.snapshot_ts.is_some() {
                let mut fresh = astromech_materials::Ledger::default();
                fresh.apply(&l.timestamp, &l.event);
                let keys: BTreeSet<&String> = ledger.counts.keys().chain(fresh.counts.keys()).collect();
                for k in keys {
                    let (a, b) =
                        (ledger.counts.get(k).copied().unwrap_or(0), fresh.counts.get(k).copied().unwrap_or(0));
                    if a != b {
                        drift_total += 1;
                        *drifts.entry(k.clone()).or_default() += a - b;
                    }
                }
            }
            snapshots += 1;
        }
        ledger.apply(&l.timestamp, &l.event);
    }
    o.say(format!("drift check: {snapshots} snapshots, {drift_total} material counts off at the next login"));
    if drift_total > 0 {
        let mut v: Vec<_> = drifts.into_iter().collect();
        v.sort_by_key(|(_, d)| -d.abs());
        o.problem(format!(
            "ledger drift (ledger minus next snapshot, summed): {}",
            v.iter().take(25).map(|(k, d)| format!("{k} {d:+}")).collect::<Vec<_>>().join(", ")
        ));
    }
}

fn fleet_section(h: &Harness, full: &GameState, lines: &[JournalLine], o: &mut Out) {
    let fleet = astromech_engineering::fleet::from_journal_dir(&h.dir);
    let current: Vec<_> = fleet.ships.iter().filter(|s| s.current).collect();
    o.say(format!(
        "fleet: {} ships ({} current, {} in transit), {} stored modules",
        fleet.ships.len(),
        current.len(),
        fleet.ships.iter().filter(|s| s.in_transit).count(),
        fleet.stored_modules.len()
    ));
    for s in &fleet.ships {
        o.say(format!(
            "  #{} {} {}{} modules {} engineered {} value {:?} system {}",
            s.id,
            s.ship_type,
            if s.current { "(current) " } else { "" },
            if s.in_transit { "(in transit) " } else { "" },
            s.modules.len(),
            s.modules.iter().filter(|m| m.engineering.is_some()).count(),
            s.value,
            if s.system.is_some() { "known" } else { "unknown" }
        ));
    }
    o.check(current.len() == 1, format!("{} current ships", current.len()));
    if let Some(c) = current.first() {
        o.check(
            Some(c.id) == full.ship_id,
            format!("current ship #{} but the game state flies {:?}", c.id, full.ship_id),
        );
    }
    let unnamed: Vec<_> = fleet
        .ships
        .iter()
        .flat_map(|s| s.modules.iter())
        .filter(|m| m.name.is_empty() || m.name.eq_ignore_ascii_case(&m.item))
        .map(|m| m.item.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if !unnamed.is_empty() {
        o.problem(format!("fitted modules with no display name: {}", unnamed.join(", ")));
    }
    let unnamed_stored: BTreeSet<_> = fleet
        .stored_modules
        .iter()
        .filter(|m| m.name.is_empty() || m.name.eq_ignore_ascii_case(&m.item))
        .map(|m| m.item.clone())
        .collect();
    if !unnamed_stored.is_empty() {
        o.problem(format!(
            "stored modules with no display name: {}",
            unnamed_stored.into_iter().collect::<Vec<_>>().join(", ")
        ));
    }
    // Ships the journal says exist: the last StoredShips plus the ship flown then.
    if let Some(i) = lines.iter().rposition(|l| l.name == "StoredShips") {
        let v: Value = serde_json::from_str(&lines[i].raw_json).unwrap();
        let mut ids: BTreeSet<i64> = ["ShipsHere", "ShipsRemote"]
            .iter()
            .flat_map(|k| v.get(*k).and_then(Value::as_array).cloned().unwrap_or_default())
            .filter_map(|s| s.get("ShipID").and_then(Value::as_i64))
            .collect();
        let flown = lines[..i].iter().rev().find_map(|l| match &l.event {
            JournalEvent::Loadout { ship_id, .. } => *ship_id,
            _ => None,
        });
        ids.extend(flown);
        // Later buys, sells and swaps change the list; only compare when none came after.
        let later_changes = lines[i..].iter().any(|l| {
            matches!(
                l.name.as_str(),
                "ShipyardBuy" | "ShipyardSell" | "SellShipOnRebuy" | "ShipyardNew" | "ShipRedeemed"
            )
        });
        let fleet_ids: BTreeSet<i64> = fleet.ships.iter().map(|s| s.id).collect();
        o.say(format!(
            "last StoredShips + flown: {} ships; fleet: {} ships{}",
            ids.len(),
            fleet_ids.len(),
            if later_changes { " (fleet changed since)" } else { "" }
        ));
        if !later_changes {
            o.check(ids == fleet_ids, format!("fleet ids {fleet_ids:?} but the journal lists {ids:?}"));
        }
    }
    h.dump("fleet", &fleet);
    let heads = astromech_ship::headlines(&fleet);
    o.say(format!("ship headlines: {}", heads.len()));
    let engineers = astromech_engineering::engineers::from_journal_dir(&h.dir);
    let known: Vec<_> = engineers.iter().filter(|e| e.progress != "Unknown" && !e.progress.is_empty()).collect();
    o.say(format!(
        "engineers: {} in the catalogue, {} with progress ({})",
        engineers.len(),
        known.len(),
        known
            .iter()
            .map(|e| format!("{}:{}{}", e.id, e.progress, e.rank.map(|r| format!(" G{r}")).unwrap_or_default()))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    // Engineers the journal names that the catalogue doesn't know.
    let cat: BTreeSet<i64> = engineers.iter().map(|e| e.id).collect();
    let mut missing = BTreeSet::new();
    for l in lines.iter().filter(|l| l.name == "EngineerProgress") {
        let v: Value = serde_json::from_str(&l.raw_json).unwrap();
        for e in v.get("Engineers").and_then(Value::as_array).cloned().unwrap_or_default() {
            if let Some(id) = e.get("EngineerID").and_then(Value::as_i64) {
                if !cat.contains(&id) {
                    missing.insert(format!("{id} {}", e.get("Engineer").and_then(Value::as_str).unwrap_or("")));
                }
            }
        }
    }
    o.say(format!("engineers in the journal but not the ship-engineer catalogue: {}", missing.len()));
    h.dump("engineers", &engineers);
}

fn odyssey_section(h: &Harness, o: &mut Out) {
    let hold = astromech_odyssey::inventory::Holdings::from_journal_dir(&h.dir);
    let log_lines = Mutex::new(Vec::<String>::new());
    let log = |m: &str| log_lines.lock().unwrap().push(m.to_string());
    let state = astromech_odyssey::journal_state(&h.dir, &log);
    let view = astromech_odyssey::view(&hold, &state, "journal");
    let items = astromech_odyssey::inventory::item_lines(&hold);
    o.say(format!(
        "locker {}, backpack {}: {} item lines ({} units in the locker), {} unknown items",
        if hold.locker_ts.is_some() { "found" } else { "MISSING" },
        if hold.backpack_ts.is_some() { "found" } else { "missing" },
        items.len(),
        items.iter().map(|i| i.locker).sum::<i64>(),
        hold.unknown.len()
    ));
    o.say(format!(
        "owned gear {}, engineers {}, log lines {}",
        view.owned.len(),
        view.engineers.len(),
        log_lines.lock().unwrap().len()
    ));
    for l in log_lines.lock().unwrap().iter().take(10) {
        o.say(format!("  log: {l}"));
    }
    if !hold.unknown.is_empty() {
        o.problem(format!("unknown micro-resources: {}", hold.unknown.keys().cloned().collect::<Vec<_>>().join(", ")));
    }
    for i in items.iter().filter(|i| i.locker < 0 || i.backpack < 0) {
        o.problem(format!("{} negative", i.id));
    }
    h.dump("odyssey", &view);
}

fn colonisation_section(h: &Harness, o: &mut Out) {
    let projects = astromech_colonisation::from_journal_dir(&h.dir);
    o.say(format!(
        "{} projects ({} complete, {} failed)",
        projects.len(),
        projects.iter().filter(|p| p.project.complete).count(),
        projects.iter().filter(|p| p.project.failed).count()
    ));
    h.dump("colonisation", &projects);
}

fn powerplay_section(h: &Harness, lines: &[JournalLine], o: &mut Out) {
    let j = astromech_powerplay::from_journal_dir(&h.dir);
    let ov = j.overview();
    o.say(format!(
        "power {}, rank {:?}, merits {:?}, session merits {}, week merits {}, here {}, system power {}",
        if ov.standing.power.is_some() { "pledged" } else { "none" },
        ov.standing.rank,
        ov.standing.merits,
        ov.standing.session_merits,
        ov.standing.week_merits,
        ov.here.is_some(),
        ov.system.is_some()
    ));
    // The journal's own word. `TotalMerits - MeritsGained` is the total before each award and
    // the login `Powerplay` gives the total outright, so an award the game wrote but didn't
    // count shows up as a later total that is short of the running one.
    let cycle = astromech_powerplay::rules().cycle_start(chrono::Utc::now()).timestamp();
    let (mut total, mut week, mut uncounted, mut last_rank) = (None::<i64>, 0i64, 0usize, None::<i64>);
    for l in lines.iter().filter(|l| l.name.starts_with("Powerplay")) {
        let v: Value = serde_json::from_str(&l.raw_json).unwrap();
        let int = |k: &str| v.get(k).and_then(Value::as_i64);
        let in_cycle = unix(&l.timestamp).is_some_and(|t| t >= cycle);
        let before = match l.name.as_str() {
            "Powerplay" => int("Merits"),
            "PowerplayMerits" => int("TotalMerits").zip(int("MeritsGained")).map(|(t, g)| t - g),
            _ => None,
        };
        if let (Some(run), Some(b)) = (total, before) {
            if run > b {
                uncounted += 1;
                if in_cycle {
                    week -= run - b;
                }
            }
        }
        match l.name.as_str() {
            "Powerplay" => {
                total = int("Merits");
                last_rank = int("Rank").or(last_rank);
            }
            "PowerplayMerits" => {
                total = int("TotalMerits");
                if in_cycle {
                    week += int("MeritsGained").unwrap_or(0);
                }
            }
            "PowerplayRank" => last_rank = int("Rank").or(last_rank),
            "PowerplayJoin" | "PowerplayDefect" => total = Some(0),
            _ => {}
        }
    }
    o.say(format!("by the journal: merits {total:?}, rank {last_rank:?}, {week} this cycle, {uncounted} award(s) the game did not count"));
    if total.is_some() {
        o.check(ov.standing.power.is_some(), "Powerplay events but no power");
        o.check(ov.standing.merits == total, format!("merits {:?} but the journal says {total:?}", ov.standing.merits));
        o.check(
            ov.standing.rank == last_rank,
            format!("rank {:?} but the journal says {last_rank:?}", ov.standing.rank),
        );
        o.check(
            ov.standing.week_merits == week,
            format!("week merits {} but the journal says {week}", ov.standing.week_merits),
        );
    }
    h.dump("powerplay", &ov);
}

fn exploration_section(h: &Harness, full: &GameState, o: &mut Out) {
    let view = astromech_exploration::system::from_journal_dir(&h.dir);
    o.say(format!(
        "current system view: {} bodies (honk count {:?}, all found {}), value {} / {} if mapped",
        view.bodies.len(),
        view.body_count,
        view.all_found,
        view.value,
        view.value_if_mapped
    ));
    o.check(view.system == full.system, "exploration's system is not the game state's");
    if let Some(n) = view.body_count {
        o.check(view.bodies.len() as i64 <= n + 20, format!("{} bodies but the honk counted {n}", view.bodies.len()));
    }
    o.check(view.value >= 0 && view.value <= view.value_if_mapped.max(view.value), "value above value if mapped");
    h.dump("exploration", &view);
    let bio = serde_json::to_value(&full.exobiology).unwrap_or_default();
    o.say(format!("exobiology state: {}", bio.to_string().chars().take(200).collect::<String>()));
}

fn galaxy_section(h: &Harness, full: &GameState, o: &mut Out) {
    let visits = astromech_galaxy::visits::visits_from_journal_dir(&h.dir, astromech_galaxy::visits::RECENT_JOURNALS);
    let here = astromech_galaxy::here_from_journal_dir(&h.dir);
    o.say(format!("visits (newest {} journals): {}", astromech_galaxy::visits::RECENT_JOURNALS, visits.len()));
    o.check(here.as_ref().map(|x| &x.system) == full.system.as_ref(), "galaxy 'here' is not the game state's system");
    let st = astromech_station::state_from_journal_dir(&h.dir);
    o.check(st.station == full.station, "station page's station is not the game state's");
    o.check(st.ship_id == full.ship_id, "station page's ship is not the game state's");
    let market = astromech_market::here::from_journal_dir(&h.dir);
    o.say(format!(
        "market here: {}",
        market.as_ref().map(|m| format!("{} commodities", m.items.len())).unwrap_or("none".into())
    ));
    let cargo = astromech_market::cargo::from_journal_dir(&h.dir);
    o.say(format!("cargo: {} lines, {} t", cargo.len(), cargo.iter().map(|c| c.count).sum::<i64>()));
    if let Some(t) = full.cargo_tonnes {
        o.check(cargo.iter().map(|c| c.count).sum::<i64>() as f64 == t, "cargo lines don't add up to the hold");
    }
}

fn missions_section(full: &GameState, lines: &[JournalLine], o: &mut Out) {
    // The login `Missions` event lists what is active; later accept/complete/fail/abandon change it.
    let mut active: BTreeSet<i64> = BTreeSet::new();
    for l in lines {
        let v: Value = match l.name.as_str() {
            "Missions" | "MissionAccepted" | "MissionCompleted" | "MissionFailed" | "MissionAbandoned" => {
                serde_json::from_str(&l.raw_json).unwrap()
            }
            _ => continue,
        };
        let id = v.get("MissionID").and_then(Value::as_i64);
        match l.name.as_str() {
            "Missions" => {
                active = v
                    .get("Active")
                    .and_then(Value::as_array)
                    .cloned()
                    .unwrap_or_default()
                    .iter()
                    .filter_map(|m| m.get("MissionID").and_then(Value::as_i64))
                    .collect();
            }
            "MissionAccepted" => {
                active.extend(id);
            }
            _ => {
                if let Some(id) = id {
                    active.remove(&id);
                }
            }
        }
    }
    let ours: BTreeSet<i64> = full.missions.iter().map(|m| m.id).collect();
    o.say(format!("active by the journal: {}; game state: {}", active.len(), ours.len()));
    o.check(
        active == ours,
        format!(
            "missions differ: journal-only {:?}, state-only {:?}",
            active.difference(&ours).count(),
            ours.difference(&active).count()
        ),
    );
    let no_expiry = full.missions.iter().filter(|m| m.expires.is_none()).count();
    o.say(format!("missions without an expiry: {no_expiry}"));
}

fn eddn_section(dir: &Path, lines: &[JournalLine], ids: &Identities, o: &mut Out) {
    let soft = Software { name: "astromech".into(), version: "test".into() };
    // No delivery worker and no transport: messages are only queued, in memory. Live schemas
    // (not `/test`), so the messages are what would really go.
    let mut feed = Feed::new(Sender::new(Outbox::in_memory().unwrap(), soft, false));
    let mut last_ts: BTreeMap<String, i64> = BTreeMap::new();
    // The queue keeps at most 2,000 messages: take them off as they come.
    let mut queued = vec![];
    let drain = |feed: &Feed, queued: &mut Vec<astromech_eddn::outbox::Queued>| {
        let out = feed.sender().outbox();
        for q in out.due(i64::MAX, true, usize::MAX).unwrap() {
            out.take(q.id).unwrap();
            queued.push(q);
        }
    };
    for l in lines {
        let v: Value = serde_json::from_str(&l.raw_json).unwrap();
        // Each line as if it had just been written, so the age guard lets it through.
        let now = unix(&l.timestamp).unwrap_or(0);
        feed.journal(&v, true, now);
        last_ts.insert(l.name.clone(), now);
        drain(&feed, &mut queued);
    }
    // Only the newest companion files are on disk: hand each to the last event of its kind.
    let mut companions = 0;
    for (event, file) in [
        ("Market", "Market.json"),
        ("Outfitting", "Outfitting.json"),
        ("Shipyard", "Shipyard.json"),
        ("NavRoute", "NavRoute.json"),
    ] {
        if let (true, Some(v)) = (
            feed.awaiting(event),
            std::fs::read(dir.join(file)).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok()),
        ) {
            feed.companion_file(event, &v, last_ts[event]);
            companions += 1;
        }
    }
    feed.flush(last_ts.values().copied().max().unwrap_or(0));
    drain(&feed, &mut queued);
    let outbox = feed.sender().outbox();
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    let mut invalid: BTreeMap<String, usize> = BTreeMap::new();
    let mut first_errors: BTreeMap<String, String> = BTreeMap::new();
    let mut leaks = 0;
    let mut validators = BTreeMap::new();
    for q in &queued {
        let up = gunzip(&q.body);
        let schema = schema_file(&q.schema_ref);
        *by.entry(schema.clone()).or_default() += 1;
        let errors = schema_errors(&mut validators, &schema, &up);
        if !errors.is_empty() {
            *invalid.entry(schema.clone()).or_default() += 1;
            first_errors.entry(schema.clone()).or_insert_with(|| errors[0].chars().take(200).collect());
        }
        let (message, all) = (up["message"].to_string(), up.to_string());
        if ids.names.iter().any(|n| message.contains(n.as_str())) || ids.fids.iter().any(|f| all.contains(f.as_str())) {
            leaks += 1;
        }
    }
    o.say(format!("{} messages would be sent ({companions} from companion files): {}", queued.len(), fmt_counts(&by)));
    let dropped = outbox.recent_of(&[Outcome::Dropped], 1000).unwrap();
    let mut why: BTreeMap<String, usize> = BTreeMap::new();
    for d in &dropped {
        // Reasons can quote a name; keep the part before any colon.
        let reason: String = d.detail.split(':').next().unwrap_or("").chars().take(80).collect();
        *why.entry(format!("{} ({reason})", schema_file(&d.schema_ref))).or_default() += 1;
    }
    o.say(format!("{} skipped or dropped: {}", dropped.len(), fmt_counts(&why)));
    for (schema, n) in &invalid {
        o.problem(format!("{n} {schema} messages break the schema, e.g. {}", first_errors[schema]));
    }
    o.check(leaks == 0, format!("{leaks} messages carry the commander name outside the header, or the FID"));
}

fn edsm_section(lines: &[JournalLine], o: &mut Out) {
    let commander = lines.iter().rev().find_map(|l| match &l.event {
        JournalEvent::LoadGame { commander, .. } => Some(commander.clone()),
        _ => None,
    });
    let switch = Arc::new(Switch::default());
    switch.set(commander.map(|c| Credentials { commander: c, api_key: "none".into() }));
    let mut up = Uploader::new(Outbox::in_memory().unwrap().with_label("edsm"), switch, Arc::new(Discards::default()));
    let mut queued = vec![];
    for l in lines {
        let v: Value = serde_json::from_str(&l.raw_json).unwrap();
        up.journal(&v, true, unix(&l.timestamp).unwrap_or(0));
        // The queue keeps at most 2,000 events: take them off as they come.
        for q in up.outbox().due(i64::MAX, true, usize::MAX).unwrap() {
            up.outbox().take(q.id).unwrap();
            queued.push(q);
        }
    }
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for q in &queued {
        *by.entry(q.schema_ref.clone()).or_default() += 1;
    }
    let held = up.outbox().recent_of(&[Outcome::Dropped], 50).unwrap();
    o.say(format!("{} events would be uploaded: {}", queued.len(), fmt_counts(&by)));
    o.say(format!(
        "held back: {}",
        held.iter().map(|e| e.detail.clone()).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>().join("; ")
    ));
}
