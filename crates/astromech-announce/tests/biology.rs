//! The system biology summary replayed: known biology on arrival from a recorded Spansh answer
//! (at once, or late on the clock tick), the summary after the FSS, and a body's line after
//! the DSS.

mod common;

use astromech_announce::biology::{self, KnownBiology};
use astromech_announce::Shared;
use astromech_exobiology::known::KnownBody;
use astromech_status::parse_status;
use common::Run;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const LUHMAN: i64 = 22960358574928;

/// The recorded Spansh answer for Luhman 16 (astromech-data's fixture), as the known bodies.
fn recorded() -> Vec<KnownBody> {
    let v: serde_json::Value =
        serde_json::from_str(include_str!("../../astromech-data/tests/fixtures/spansh-bodies-biology.json")).unwrap();
    v["results"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| {
            let bio = b["signals"].as_array().unwrap().iter().find(|s| s["name"] == "Biological");
            KnownBody::new(
                b["name"].as_str().unwrap(),
                b["body_id"].as_i64(),
                bio.and_then(|s| s["count"].as_u64()).unwrap_or(0) as u32,
                b["genuses"].as_array().unwrap().iter().map(|g| g["name"].as_str().unwrap()),
                b["landmarks"].as_array().unwrap().iter().map(|l| l["subtype"].as_str().unwrap()),
            )
        })
        .collect()
}

/// Answers for Luhman 16 once `ready`; nothing known elsewhere.
struct Recorded {
    ready: AtomicBool,
}

impl KnownBiology for Recorded {
    fn known(&self, system_address: i64, _: Option<[f64; 3]>) -> Option<Vec<KnownBody>> {
        if !self.ready.load(Ordering::SeqCst) {
            return None;
        }
        Some(if system_address == LUHMAN { recorded() } else { vec![] })
    }
}

fn run(ready: bool) -> (Run, Arc<Recorded>) {
    let shared = Shared::default();
    let r = Arc::new(Recorded { ready: AtomicBool::new(ready) });
    shared.known_bio.set(r.clone());
    (Run::with(shared, &Default::default()), r)
}

fn jump(ts: &str, system: &str, address: i64) -> String {
    format!(
        r#"{{"timestamp":"{ts}","event":"FSDJump","StarSystem":"{system}","SystemAddress":{address},"StarPos":[6.3125,0.59375,1.71875],"JumpDist":8.2}}"#
    )
}

fn epoch(ts: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(ts).unwrap().timestamp()
}

fn tick(r: &mut Run, ts: &str) -> Vec<String> {
    let rs = r.engine.tick(epoch(ts), &r.state, Instant::now() + Duration::from_secs(600));
    rs.into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
}

/// Status.json with the GUI focus `gui` (9: the FSS scanner).
fn gui(r: &mut Run, gui: u64) -> Vec<String> {
    let s = parse_status(format!(r#"{{"Flags":16777216,"GuiFocus":{gui}}}"#).as_bytes()).unwrap();
    r.state.apply_status(&s);
    let rs = r.engine.status(&s, &r.state, Instant::now() + Duration::from_secs(600));
    rs.into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
}

#[test]
fn arrival_with_known_biology_says_it_before_any_scan() {
    let (mut r, _) = run(true);
    let said = r.spoken(&jump("2026-10-17T10:00:00Z", "Luhman 16", LUHMAN));
    // B 3: four sampled species, the dearest Fungoida Bullarum; B 4 is worth less.
    assert_eq!(said, ["Known biology on 2 bodies.", "B 3: Fungoida Bullarum, 3.7 M."]);
    // A system nobody reported biology in: nothing.
    assert!(r.spoken(&jump("2026-10-17T10:05:00Z", "Elsewhere", 5)).is_empty());
}

#[test]
fn a_late_answer_is_said_on_the_tick_within_a_minute_and_once() {
    let (mut r, recorded) = run(false);
    assert!(r.spoken(&jump("2026-10-17T10:00:00Z", "Luhman 16", LUHMAN)).is_empty());
    assert!(tick(&mut r, "2026-10-17T10:00:01Z").is_empty(), "still looking it up");
    recorded.ready.store(true, Ordering::SeqCst);
    assert_eq!(tick(&mut r, "2026-10-17T10:00:03Z"), ["Known biology on 2 bodies.", "B 3: Fungoida Bullarum, 3.7 M."]);
    assert!(tick(&mut r, "2026-10-17T10:00:04Z").is_empty());

    // Too late: over a minute after the jump.
    let (mut r, recorded) = run(false);
    r.spoken(&jump("2026-10-17T10:00:00Z", "Luhman 16", LUHMAN));
    recorded.ready.store(true, Ordering::SeqCst);
    assert!(tick(&mut r, "2026-10-17T10:01:01Z").is_empty());

    // The FSS got there first: the journal's own results stand.
    let (mut r, recorded) = run(false);
    r.spoken(&jump("2026-10-17T10:00:00Z", "Luhman 16", LUHMAN));
    r.spoken(
        &FSS_B3
            .replace("Synthetic Sector AB-C d1", "Luhman 16")
            .replace("\"SystemAddress\":99", &format!("\"SystemAddress\":{LUHMAN}")),
    );
    recorded.ready.store(true, Ordering::SeqCst);
    assert!(tick(&mut r, "2026-10-17T10:00:10Z").is_empty());
}

const SYSTEM: &str = r#"
{"timestamp":"2026-10-17T10:00:00Z","event":"FSDJump","StarSystem":"Synthetic Sector AB-C d1","SystemAddress":99,"StarPos":[1,2,3],"JumpDist":20.0}
{"timestamp":"2026-10-17T10:00:05Z","event":"Scan","ScanType":"AutoScan","BodyName":"Synthetic Sector AB-C d1 A","BodyID":1,"SystemAddress":99,"DistanceFromArrivalLS":0.0,"StarType":"K","Subclass":3,"WasDiscovered":true}
"#;

/// A high metal content world with thin carbon dioxide, 180 K, 0.30 g, 0.05 atm, round a K star,
/// and an icy body with thin argon (Bacterium Vesicula and Fonticulua Campestris country).
const SCANS: &str = r#"
{"timestamp":"2026-10-17T10:01:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"Parents":[{"Star":1}],"SystemAddress":99,"DistanceFromArrivalLS":512.0,"PlanetClass":"High metal content body","Atmosphere":"thin carbon dioxide atmosphere","AtmosphereType":"CarbonDioxide","Volcanism":"","Landable":true,"SurfaceGravity":2.9393,"SurfaceTemperature":180.0,"SurfacePressure":5061.6}
{"timestamp":"2026-10-17T10:01:10Z","event":"Scan","ScanType":"Detailed","BodyName":"Synthetic Sector AB-C d1 B 3","BodyID":7,"Parents":[{"Star":1}],"SystemAddress":99,"DistanceFromArrivalLS":1612.0,"PlanetClass":"Icy body","Atmosphere":"thin argon atmosphere","AtmosphereType":"Argon","Volcanism":"","Landable":true,"SurfaceGravity":2.53,"SurfaceTemperature":79.0,"SurfacePressure":1928.0}
"#;

const FSS_A2: &str = r#"{"timestamp":"2026-10-17T10:02:00Z","event":"FSSBodySignals","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"SystemAddress":99,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":1}]}"#;
const FSS_B3: &str = r#"{"timestamp":"2026-10-17T10:02:10Z","event":"FSSBodySignals","BodyName":"Synthetic Sector AB-C d1 B 3","BodyID":7,"SystemAddress":99,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":2}]}"#;
const DSS_A2: &str = r#"{"timestamp":"2026-10-17T10:20:00Z","event":"SAASignalsFound","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"SystemAddress":99,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":1}],"Genuses":[{"Genus":"$Codex_Ent_Bacterial_Genus_Name;","Genus_Localised":"Bacterium"}]}"#;

#[test]
fn the_summary_follows_the_fss_and_the_dss() {
    let (mut r, _) = run(true);
    assert!(r.spoken(SYSTEM).is_empty(), "nothing known about this system");
    r.spoken(SCANS);
    // In the FSS: the counts arrive, nothing is said per body.
    gui(&mut r, 9);
    assert!(r.spoken(FSS_A2).is_empty());
    // Leaving the FSS: the system's total and its best body. A 2's conditions single out no
    // species among its dearest genus, so it's "up to" the genus's top value.
    let first = gui(&mut r, 0);
    assert_eq!(first.len(), 2, "{first:?}");
    assert_eq!(first[0], "Biology on 1 body.");
    let top = r.state.exobiology.system.ranked()[0].best().unwrap();
    assert_eq!(first[1], biology::body_line("A 2", &top.0, top.1, top.2));
    assert!(first[1].starts_with("A 2: ") && first[1].contains("up to"), "{first:?}");
    // Back in and out with nothing new: silent.
    gui(&mut r, 9);
    assert!(gui(&mut r, 0).is_empty());
    // Another body found: the summary again.
    gui(&mut r, 9);
    r.spoken(FSS_B3);
    let second = gui(&mut r, 0);
    assert_eq!(second[0], "Biology on 2 bodies.");
    assert_ne!(second, first);

    // The DSS on A 2: one genus, Bacterium; its line, from the real genus list.
    let dss = r.spoken(DSS_A2);
    let a2 = r.state.exobiology.system.ranked().into_iter().find(|b| b.body_id == 5).unwrap();
    assert!(a2.mapped && a2.genera.len() == 1 && a2.genera[0].genus == "Bacterium");
    let best = a2.best().unwrap();
    assert_eq!(dss, [biology::body_line("A 2", &best.0, best.1, best.2)]);
    assert!(dss[0].starts_with("A 2: Bacterium"), "{dss:?}");
    // The same DSS result again says nothing.
    assert!(r.spoken(DSS_A2).is_empty());
}

#[test]
fn values_and_lines_read_well() {
    assert_eq!(biology::millions(19_010_800), "19 M");
    assert_eq!(biology::millions(3_703_200), "3.7 M");
    assert_eq!(biology::millions(1_000_000), "1 M");
    assert_eq!(biology::millions(9_960_000), "10 M");
    assert_eq!(biology::count_line(1, false), "Biology on 1 body.");
    assert_eq!(biology::count_line(3, true), "Known biology on 3 bodies.");
    assert_eq!(biology::body_line("A 2", "Stratum", 19_010_800, false), "A 2: Stratum, up to 19 M.");
    assert_eq!(biology::body_line("A 2", "Stratum Tectonicas", 19_010_800, true), "A 2: Stratum Tectonicas, 19 M.");
    // A long name drops "up to" to keep to eight words.
    assert_eq!(biology::body_line("AB 1 c a", "Stratum", 19_010_800, false), "AB 1 c a: Stratum, 19 M.");
    assert_eq!(biology::short_name("Luhman 16 B 3", Some("Luhman 16")), "B 3");
    assert_eq!(biology::short_name("Hollow", Some("Luhman 16")), "Hollow");
}

#[test]
fn can_be_turned_off() {
    let shared = Shared::default();
    shared.known_bio.set(Arc::new(Recorded { ready: AtomicBool::new(true) }));
    let off: astromech_announce::Settings = toml::from_str("[biology]\nenabled = false\n").unwrap();
    let mut r = Run::with(shared, &off);
    assert!(r.spoken(&jump("2026-10-17T10:00:00Z", "Luhman 16", LUHMAN)).is_empty());
}
