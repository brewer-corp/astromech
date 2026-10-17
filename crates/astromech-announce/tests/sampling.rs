//! The sampling helper replayed: samples and Status.json positions on a 1000 km body; the cue
//! comes at the genus's colony range from every earlier sample, once per sample.

mod common;

use astromech_status::parse_status;
use common::Run;
use std::time::Duration;

/// Metres to degrees of latitude on a 1000 km body.
fn deg(m: f64) -> f64 {
    (m / 1_000_000.0).to_degrees()
}

/// Status.json at `north_m` metres north and `east_m` east of 10° N, 20° E (on the ground).
fn at(run: &mut Run, north_m: f64, east_m: f64) -> Vec<String> {
    let lon = 20.0 + deg(east_m) / 10f64.to_radians().cos();
    let json = format!(
        r#"{{"timestamp":"2026-10-17T10:00:00Z","event":"Status","Flags":2097152,"Flags2":16,"Latitude":{},"Longitude":{lon},"Altitude":0,"Heading":0,"BodyName":"Synth 1 a","PlanetRadius":1000000.0}}"#,
        10.0 + deg(north_m)
    );
    let s = parse_status(json.as_bytes()).unwrap();
    run.state.apply_status(&s);
    let now = std::time::Instant::now() + Duration::from_secs(1);
    run.engine.status(&s, &run.state, now).into_iter().filter(|r| r.channels.speech).map(|r| r.text).collect()
}

fn organic(kind: &str, genus: &str, species: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-17T10:00:00Z","event":"ScanOrganic","ScanType":"{kind}","Genus":"$Codex_Ent_{genus}_Genus_Name;","Genus_Localised":"{genus}","Species":"$Codex_Ent_{genus}_01_Name;","Species_Localised":"{species}","SystemAddress":99,"Body":5}}"#
    )
}

#[test]
fn cues_at_the_colony_range_from_every_earlier_sample() {
    let mut run = Run::new();
    // Stratum: 500 m.
    assert!(at(&mut run, 0.0, 0.0).is_empty());
    assert!(run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas")).is_empty(), "the sampler shows the log");
    assert!(at(&mut run, 200.0, 0.0).is_empty());
    assert!(at(&mut run, 495.0, 0.0).is_empty(), "just short of 500 m");
    assert_eq!(at(&mut run, 505.0, 0.0), ["Far enough for sample 2."]);
    // Once per sample: back inside and out again says nothing.
    assert!(at(&mut run, 300.0, 0.0).is_empty());
    assert!(at(&mut run, 600.0, 0.0).is_empty());

    // Sample 2 at 600 m north. Sample 3 must be 500 m from both: 600 m east of the first
    // sample is only about 600 m from the first, 850 m from the second.
    assert!(run.spoken(&organic("Sample", "Stratum", "Stratum Tectonicas")).is_empty());
    assert!(at(&mut run, 600.0, 300.0).is_empty(), "300 m from sample 2");
    assert!(at(&mut run, 100.0, 300.0).is_empty(), "far from sample 2, 316 m from sample 1");
    assert_eq!(at(&mut run, 0.0, 520.0), ["Far enough for sample 3."]);

    // The third sample and the analysis: nothing more to cue.
    run.spoken(&organic("Sample", "Stratum", "Stratum Tectonicas"));
    run.spoken(&organic("Analyse", "Stratum", "Stratum Tectonicas"));
    assert!(at(&mut run, 2000.0, 0.0).is_empty());
}

#[test]
fn each_genus_has_its_own_range_and_a_new_species_starts_over() {
    let mut run = Run::new();
    // Fumerola: 100 m; Electricae: 1000 m.
    at(&mut run, 0.0, 0.0);
    run.spoken(&organic("Log", "Fumerolas", "Fumerola Nitris"));
    assert_eq!(at(&mut run, 101.0, 0.0), ["Far enough for sample 2."]);
    // Logging another species abandons the first.
    run.spoken(&organic("Log", "Electricae", "Electricae Radialem"));
    assert!(at(&mut run, 101.0 + 990.0, 0.0).is_empty());
    assert_eq!(at(&mut run, 101.0 + 1001.0, 0.0), ["Far enough for sample 2."]);
}

#[test]
fn no_position_no_cue() {
    let mut run = Run::new();
    // Logged with no Status.json position yet: the sample's place is unknown, so no cue.
    run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas"));
    assert!(at(&mut run, 5000.0, 0.0).is_empty());
}

#[test]
fn can_be_turned_off() {
    let off: astromech_announce::Settings = toml::from_str("[sampling]\nenabled = false\n").unwrap();
    let mut run = Run::with(Default::default(), &off);
    at(&mut run, 0.0, 0.0);
    run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas"));
    assert!(at(&mut run, 600.0, 0.0).is_empty());
}

#[test]
fn a_species_logged_again_on_the_same_body_is_cued_again() {
    let mut run = Run::new();
    at(&mut run, 0.0, 0.0);
    run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas"));
    assert_eq!(at(&mut run, 505.0, 0.0), ["Far enough for sample 2."]);
    // The sample abandoned: the same species logged afresh, here, on the same body.
    run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas"));
    assert!(at(&mut run, 700.0, 0.0).is_empty(), "200 m from the new first sample");
    assert_eq!(at(&mut run, 1010.0, 0.0), ["Far enough for sample 2."]);
}

#[test]
fn a_cue_due_before_a_restart_is_not_said_again_after_it() {
    let mut run = Run::new();
    // The catch-up after a restart: the log, then Status.json already far enough. Silent.
    run.engine.set_live(false);
    at(&mut run, 0.0, 0.0);
    run.spoken(&organic("Log", "Stratum", "Stratum Tectonicas"));
    assert!(at(&mut run, 505.0, 0.0).is_empty());
    // Live again: still the same sample, already past the range, so nothing more.
    run.engine.set_live(true);
    assert!(at(&mut run, 520.0, 0.0).is_empty());
    // The next sample is cued as usual.
    run.spoken(&organic("Sample", "Stratum", "Stratum Tectonicas"));
    assert_eq!(at(&mut run, 1030.0, 0.0), ["Far enough for sample 3."]);
}
