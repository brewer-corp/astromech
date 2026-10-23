//! Exploration questions by voice, end to end (astromech-m3xd): phrase → grammar → the current
//! system folded from a synthetic journal folder the way the Exploration pages fold it
//! (`astromech_exploration::system`, earlier visits from the history), its biology
//! (`astromech_exploration::biology`), and the unsold data from the history imported from the
//! same folder (`History::unsold`) → the spoken answer. Nothing is pressed and no test touches
//! the network.

use astromech_core::actions::{Outcome, Runner};
use astromech_core::assistant::{Assistant, Handled};
use astromech_core::health::BindsState;
use astromech_core::hub::StateHub;
use astromech_core::speech::RecordingSpeaker;
use astromech_core::tester::{Source, Would};
use astromech_data::{Cache, Client, Request, Response, Transport, TransportError};
use astromech_history::{History, ImportOptions};
use astromech_input::exec::keys::ScanKey;
use astromech_input::exec::platform::{GameIntegrity, IntegrityReport, Platform, GAME_WINDOW_TITLE};
use astromech_intent::Decision;
use astromech_policy::{Config as PolicyConfig, Policy};
use astromech_proto::Origin;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// Test System (address 1001), honked (7 bodies): a K star, an Earth-like world and a
/// terraformable high metal content world not mapped (worth mapping), a small icy body (not
/// worth it), A 4 mapped with three genera from the DSS (Bacterium analysed, Stratum one sample
/// in, Fonticulua not started), and A 5 with two biological signals, not mapped. Six bodies
/// scanned. An earlier system's scans and a sample were sold.
const JOURNAL: &str = r#"{"timestamp":"2026-10-20T09:00:00Z","event":"Fileheader","part":1,"gameversion":"4.2.0.0","build":"r1"}
{"timestamp":"2026-10-20T09:00:01Z","event":"LoadGame","FID":"F1","Commander":"Test","Ship":"krait_mkii"}
{"timestamp":"2026-10-20T09:05:00Z","event":"FSDJump","StarSystem":"Old System","SystemAddress":900,"StarPos":[1,0,0],"Population":0,"JumpDist":9.5}
{"timestamp":"2026-10-20T09:06:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Old System 1","BodyID":1,"StarSystem":"Old System","SystemAddress":900,"PlanetClass":"Icy body","MassEM":0.5,"WasDiscovered":true,"WasMapped":true}
{"timestamp":"2026-10-20T09:30:00Z","event":"SellExplorationData","Systems":["Old System"],"Discovered":[],"BaseValue":10,"Bonus":0,"TotalEarnings":10}
{"timestamp":"2026-10-20T10:00:00Z","event":"FSDJump","StarSystem":"Test System","SystemAddress":1001,"StarPos":[0,0,0],"Population":0,"JumpDist":9.5}
{"timestamp":"2026-10-20T10:00:05Z","event":"Scan","ScanType":"AutoScan","BodyName":"Test System A","BodyID":1,"StarSystem":"Test System","SystemAddress":1001,"StarType":"K","Subclass":3,"StellarMass":0.8,"WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:01:00Z","event":"FSSDiscoveryScan","Progress":0.4,"BodyCount":7,"NonBodyCount":2,"SystemName":"Test System","SystemAddress":1001}
{"timestamp":"2026-10-20T10:02:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Test System A 1","BodyID":2,"Parents":[{"Star":1}],"StarSystem":"Test System","SystemAddress":1001,"DistanceFromArrivalLS":410.0,"PlanetClass":"Earthlike body","TerraformState":"Terraformed","MassEM":0.9,"Landable":false,"WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:03:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Test System A 2","BodyID":3,"Parents":[{"Star":1}],"StarSystem":"Test System","SystemAddress":1001,"DistanceFromArrivalLS":820.0,"PlanetClass":"High metal content body","TerraformState":"Terraformable","MassEM":0.4,"Landable":false,"WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:04:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Test System A 3","BodyID":4,"Parents":[{"Star":1}],"StarSystem":"Test System","SystemAddress":1001,"PlanetClass":"Icy body","MassEM":0.01,"Landable":false,"WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:05:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Test System A 4","BodyID":5,"Parents":[{"Star":1}],"StarSystem":"Test System","SystemAddress":1001,"PlanetClass":"Rocky body","MassEM":0.05,"Landable":true,"SurfaceGravity":2.5,"SurfaceTemperature":170.0,"AtmosphereType":"CarbonDioxide","Atmosphere":"thin carbon dioxide atmosphere","WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:06:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Test System A 5","BodyID":6,"Parents":[{"Star":1}],"StarSystem":"Test System","SystemAddress":1001,"PlanetClass":"Rocky body","MassEM":0.04,"Landable":true,"SurfaceGravity":2.0,"SurfaceTemperature":160.0,"AtmosphereType":"CarbonDioxide","Atmosphere":"thin carbon dioxide atmosphere","WasDiscovered":true,"WasMapped":false}
{"timestamp":"2026-10-20T10:07:00Z","event":"FSSBodySignals","BodyName":"Test System A 5","BodyID":6,"SystemAddress":1001,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":2}]}
{"timestamp":"2026-10-20T10:20:00Z","event":"SAAScanComplete","BodyName":"Test System A 4","BodyID":5,"SystemAddress":1001,"ProbesUsed":3,"EfficiencyTarget":4}
{"timestamp":"2026-10-20T10:20:01Z","event":"SAASignalsFound","BodyName":"Test System A 4","BodyID":5,"SystemAddress":1001,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":3}],"Genuses":[{"Genus":"$Codex_Ent_Bacterial_Genus_Name;","Genus_Localised":"Bacterium"},{"Genus":"$Codex_Ent_Stratum_Genus_Name;","Genus_Localised":"Stratum"},{"Genus":"$Codex_Ent_Fonticulus_Genus_Name;","Genus_Localised":"Fonticulua"}]}
{"timestamp":"2026-10-20T10:40:00Z","event":"ScanOrganic","ScanType":"Log","Genus":"$Codex_Ent_Bacterial_Genus_Name;","Species":"$Codex_Ent_Bacterial_04_Name;","Species_Localised":"Bacterium Acies","SystemAddress":1001,"Body":5}
{"timestamp":"2026-10-20T10:50:00Z","event":"ScanOrganic","ScanType":"Analyse","Genus":"$Codex_Ent_Bacterial_Genus_Name;","Species":"$Codex_Ent_Bacterial_04_Name;","Species_Localised":"Bacterium Acies","SystemAddress":1001,"Body":5}
{"timestamp":"2026-10-20T11:00:00Z","event":"ScanOrganic","ScanType":"Log","Genus":"$Codex_Ent_Stratum_Genus_Name;","Species":"$Codex_Ent_Stratum_04_Name;","Species_Localised":"Stratum Araneamus","SystemAddress":1001,"Body":5}
"#;

const LOG: &str = "Journal.2026-10-20T090000.01.log";

struct NoKeys;

/// Fails the test on any request: the answers are all local.
struct Offline;

impl Transport for Offline {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        panic!("an exploration question went to the network: {url}")
    }
}

impl Platform for NoKeys {
    fn foreground_title(&self) -> Option<String> {
        Some(GAME_WINDOW_TITLE.into())
    }
    fn send_key(&self, k: ScanKey, _up: bool) -> Result<(), String> {
        panic!("an exploration question pressed {k:?}")
    }
    fn integrity(&self) -> IntegrityReport {
        IntegrityReport { own: Some(0x2000), game: GameIntegrity::Level(0x2000) }
    }
}

struct Rig {
    assistant: Assistant,
    speaker: Arc<RecordingSpeaker>,
    journals: tempfile::TempDir,
    data: tempfile::TempDir,
}

/// The history the core keeps, imported from `journals`.
fn import(journals: &Path, db: &Path) {
    History::open(db).unwrap().import(journals, &ImportOptions::default()).unwrap();
}

/// The journal folder above, with the history imported from it when `history`.
fn rig(journal: &str, history: bool) -> Rig {
    let journals = tempfile::tempdir().unwrap();
    std::fs::write(journals.path().join(LOG), journal).unwrap();
    let data = tempfile::tempdir().unwrap();
    let db = data.path().join("history.sqlite");
    if history {
        import(journals.path(), &db);
    }
    let speaker = Arc::new(RecordingSpeaker::default());
    let runner = Arc::new(Runner {
        policy: Policy::new(Arc::new(NoKeys), PolicyConfig::default()),
        hub: Arc::new(StateHub::new()),
        binds: Arc::new(RwLock::new(BindsState::default())),
        speaker: speaker.clone(),
        verify_timeout: Duration::from_millis(200),
        log: Arc::new(|_: &str| {}),
        galaxy: None,
        remote: Arc::new(Client::with_transport(Cache::in_memory().unwrap(), Arc::new(Offline))),
    });
    let assistant = Assistant::new(runner).with_journals(journals.path().to_path_buf()).with_history(db);
    Rig { assistant, speaker, journals, data }
}

fn answered(h: Handled) -> (&'static str, String) {
    match h {
        Handled::Ran { command, outcome: Outcome::LookingUp(h) } => (command, h.join().unwrap()),
        h => panic!("expected an answer, got {h:?}"),
    }
}

/// What the System page shows for the folder: the same fold, with the history.
fn page_view(r: &Rig) -> astromech_exploration::SystemView {
    let live = astromech_exploration::system::fold_from_journal_dir(r.journals.path());
    History::open_read_only(&r.data.path().join("history.sqlite")).unwrap().system_view(live)
}

#[test]
fn exploration_phrases_resolve_to_their_commands() {
    let g = astromech_core::assistant::grammar();
    for (phrase, command) in [
        // The five the toolbox validation asked on 22 Oct.
        ("is this system worth mapping", "exploration.worth_mapping"),
        ("which organics haven't I scanned here", "exploration.biology"),
        ("what is this system worth", "exploration.system_value"),
        ("how much is my unsold data worth", "exploration.unsold"),
        ("what biology is in this system", "exploration.biology"),
        ("Which bodies are worth mapping?", "exploration.worth_mapping"),
        ("What's left to sample?", "exploration.biology"),
        ("Unsold data.", "exploration.unsold"),
        // Neighbours keep theirs, with the commodity and service slots in the grammar.
        ("what is left to deliver", "colonisation.remaining"),
        ("where can I sell gold", "trade.sell"),
        ("what system", "status.location"),
        ("system map", "system_map.open"),
        ("nearest vista genomics", "find.service"),
        ("nearest universal cartographics", "find.service"),
    ] {
        let m = g.best(phrase);
        assert_eq!((m.command, m.decision), (command, Decision::Execute), "{phrase:?} -> {m:?}");
    }
}

#[test]
fn the_system_value_now_and_with_the_best_mapped() {
    let r = rig(JOURNAL, true);
    let (command, said) = answered(r.assistant.handle("What is this system worth?", Origin::Voice));
    assert_eq!(command, "exploration.system_value");
    assert_eq!(
        said,
        "Worth 451 thousand credits now. 5.8 million credits with the best mapped. 6 of 7 bodies scanned."
    );
    // The System page's totals for the same folder.
    let v = page_view(&r);
    let credits = astromech_announce::session_summary::credits;
    assert_eq!(
        (credits(v.value), credits(v.value_if_mapped)),
        ("451 thousand credits".into(), "5.8 million credits".into())
    );
    assert_eq!(r.speaker.take(), vec!["Looking that up.".to_string(), said]);
}

#[test]
fn the_bodies_worth_mapping_best_first() {
    let r = rig(JOURNAL, true);
    let (command, said) = answered(r.assistant.handle("Is this system worth mapping?", Origin::Voice));
    assert_eq!(command, "exploration.worth_mapping");
    assert_eq!(
        said,
        "2 bodies worth mapping, 5.4 million credits more. A 1: Earth-like world, 3.7 million mapped. \
         A 2: High metal content world, 2.1 million mapped. 6 of 7 bodies scanned."
    );
    let v = page_view(&r);
    let worth: Vec<&str> = v.bodies.iter().filter(|b| b.worth_mapping).map(|b| b.short_name.as_str()).collect();
    assert_eq!(worth, ["A 1", "A 2"], "the page marks the same ones");
}

#[test]
fn the_biology_left_to_sample() {
    let r = rig(JOURNAL, true);
    let (command, said) = answered(r.assistant.handle("Which organics haven't I scanned here?", Origin::Voice));
    assert_eq!(command, "exploration.biology");
    // Bacterium is analysed; Stratum has one sample of three, so it is still to do.
    assert_eq!(
        said,
        "4 genera left to sample on 2 bodies. A 4: Fonticulua, Stratum. A 5: 2 genera, not mapped. 6 of 7 bodies scanned."
    );
    let (_, again) = answered(r.assistant.handle("what biology is in this system", Origin::Voice));
    assert_eq!(again, said);
}

#[test]
fn every_genus_analysed_says_so() {
    let analyse = |t: &str, genus: &str, species: &str, body: i64| {
        format!(
            r#"{{"timestamp":"{t}","event":"ScanOrganic","ScanType":"Analyse","Genus":"$Codex_Ent_{genus}_Genus_Name;","Species":"$Codex_Ent_{species}_Name;","SystemAddress":1001,"Body":{body}}}"#
        )
    };
    let all_found = r#"{"timestamp":"2026-10-20T13:00:00Z","event":"FSSAllBodiesFound","SystemName":"Test System","SystemAddress":1001,"Count":7}"#;
    let journal = [
        JOURNAL.trim_end().replace(r#""Count":2}]}"#, r#""Count":1}]}"#),
        analyse("2026-10-20T11:10:00Z", "Stratum", "Stratum_04", 5),
        analyse("2026-10-20T11:20:00Z", "Fonticulus", "Fonticulus_02", 5),
        analyse("2026-10-20T12:00:00Z", "Bacterial", "Bacterial_04", 6),
        all_found.into(),
    ]
    .join("\n")
        + "\n";
    let r = rig(&journal, true);
    let (_, said) = answered(r.assistant.handle("what's left to sample", Origin::Voice));
    assert_eq!(said, "All biology found here is sampled.");
}

#[test]
fn unsold_data_from_the_history() {
    let r = rig(JOURNAL, true);
    let (command, said) = answered(r.assistant.handle("How much is my unsold data worth?", Origin::Voice));
    assert_eq!(command, "exploration.unsold");
    // Test System's six scans and one Bacterium Acies (1 million); Old System was sold.
    let u = History::open_read_only(&r.data.path().join("history.sqlite")).unwrap().unsold(None).unwrap().unwrap();
    assert_eq!((u.unsold.bodies, u.unsold.samples, u.unsold.organic), (6, 1, 1_000_000));
    assert_eq!(
        said,
        "Unsold data worth 1.5 million credits. Cartographic 451 thousand credits. Organic 1 million credits."
    );
}

#[test]
fn without_a_history() {
    let r = rig(JOURNAL, false);
    let (_, said) = answered(r.assistant.handle("unsold data", Origin::Voice));
    assert_eq!(said, "No journal history yet.");
    // The system answers come from the journals alone.
    let (_, said) = answered(r.assistant.handle("is this system worth mapping", Origin::Voice));
    assert!(said.starts_with("2 bodies worth mapping"), "{said}");
}

#[test]
fn a_return_visit_answers_from_what_was_scanned_before() {
    let journals = tempfile::tempdir().unwrap();
    // The first visit's journal, then two newer ones: a jump elsewhere and back to Test System,
    // where only the arrival star is scanned again. The System page folds the two newest.
    std::fs::write(journals.path().join(LOG), JOURNAL).unwrap();
    std::fs::write(
        journals.path().join("Journal.2026-10-21T090000.01.log"),
        r#"{"timestamp":"2026-10-21T09:00:00Z","event":"LoadGame","FID":"F1","Commander":"Test","Ship":"krait_mkii"}
{"timestamp":"2026-10-21T09:05:00Z","event":"FSDJump","StarSystem":"Elsewhere","SystemAddress":77,"StarPos":[5,0,0],"Population":0,"JumpDist":9.5}
"#,
    )
    .unwrap();
    std::fs::write(
        journals.path().join("Journal.2026-10-22T090000.01.log"),
        r#"{"timestamp":"2026-10-22T09:00:00Z","event":"LoadGame","FID":"F1","Commander":"Test","Ship":"krait_mkii"}
{"timestamp":"2026-10-22T09:05:00Z","event":"FSDJump","StarSystem":"Test System","SystemAddress":1001,"StarPos":[0,0,0],"Population":0,"JumpDist":9.5}
{"timestamp":"2026-10-22T09:05:05Z","event":"Scan","ScanType":"AutoScan","BodyName":"Test System A","BodyID":1,"StarSystem":"Test System","SystemAddress":1001,"StarType":"K","Subclass":3,"StellarMass":0.8,"WasDiscovered":true,"WasMapped":false}
"#,
    )
    .unwrap();
    let mut r = rig(JOURNAL, false);
    let db = r.data.path().join("history.sqlite");
    import(journals.path(), &db);
    r.assistant = r.assistant.with_journals(journals.path().to_path_buf()).with_history(db);
    let (_, said) = answered(r.assistant.handle("which organics haven't I scanned here", Origin::Voice));
    // From the history: only completed samples are kept, so Stratum's one sample is not.
    assert_eq!(
        said,
        "4 genera left to sample on 2 bodies. A 4: Fonticulua, Stratum. A 5: 2 genera, not mapped. 6 of 7 bodies scanned."
    );
}

#[test]
fn the_tester_shows_the_question_without_answering_it() {
    let r = rig(JOURNAL, true);
    for (phrase, message) in [
        ("what is this system worth", "Reads this system's scans from your journals and says what they are worth."),
        (
            "is this system worth mapping",
            "Reads this system's scans from your journals and says which bodies are worth mapping.",
        ),
        (
            "which organics haven't I scanned here",
            "Reads this system's biology from your journals and says what is left to sample.",
        ),
        ("how much is my unsold data worth", "Reads your journal history and says what your unsold data is worth."),
    ] {
        let d = r.assistant.dry_run(phrase, Source::Typed);
        assert_eq!((d.would, d.message.as_str()), (Would::Speak, message), "{phrase}");
    }
    assert!(r.speaker.take().is_empty(), "the tester says nothing");
}
