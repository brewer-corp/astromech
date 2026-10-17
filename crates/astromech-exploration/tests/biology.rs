//! The system's biology (the biology view over the system fold) from synthetic journal lines: FSS counts, the prediction from the
//! detailed scan, the DSS genus list, a sample pinning the species, and a jump starting again.

use astromech_exploration::biology::ranked;
use astromech_exploration::SystemFold;
use astromech_journal::parse_line;

fn feed(s: &mut SystemFold, lines: &str) {
    for l in lines.lines().map(str::trim).filter(|l| !l.is_empty()) {
        s.apply(&parse_line(l).unwrap());
    }
}

const ARRIVE: &str = r#"
{"timestamp":"2026-10-17T10:00:00Z","event":"FSDJump","StarSystem":"Synthetic Sector AB-C d1","SystemAddress":99,"StarPos":[1,2,3],"JumpDist":20.0}
{"timestamp":"2026-10-17T10:00:05Z","event":"Scan","ScanType":"AutoScan","BodyName":"Synthetic Sector AB-C d1 A","BodyID":1,"SystemAddress":99,"StarSystem":"Synthetic Sector AB-C d1","DistanceFromArrivalLS":0.0,"StarType":"K","Subclass":3}
"#;

/// A high metal content world with thin carbon dioxide at 180 K, 0.30 g, 0.05 atm around a K
/// star: Stratum Tectonicas country.
const SCAN_A2: &str = r#"
{"timestamp":"2026-10-17T10:02:00Z","event":"Scan","ScanType":"Detailed","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"Parents":[{"Star":1}],"SystemAddress":99,"StarSystem":"Synthetic Sector AB-C d1","DistanceFromArrivalLS":512.0,"PlanetClass":"High metal content body","Atmosphere":"thin carbon dioxide atmosphere","AtmosphereType":"CarbonDioxide","Volcanism":"","Landable":true,"SurfaceGravity":2.9393,"SurfaceTemperature":180.0,"SurfacePressure":5061.6}
"#;

const FSS_A2: &str = r#"
{"timestamp":"2026-10-17T10:01:00Z","event":"FSSBodySignals","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"SystemAddress":99,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":2},{"Type":"$SAA_SignalType_Geological;","Type_Localised":"Geological","Count":3}]}
"#;

const DSS_A2: &str = r#"
{"timestamp":"2026-10-17T10:20:00Z","event":"SAASignalsFound","BodyName":"Synthetic Sector AB-C d1 A 2","BodyID":5,"SystemAddress":99,"Signals":[{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":2}],"Genuses":[{"Genus":"$Codex_Ent_Bacterial_Genus_Name;","Genus_Localised":"Bacterium"},{"Genus":"$Codex_Ent_Stratum_Genus_Name;","Genus_Localised":"Stratum"}]}
"#;

const SAMPLE_A2: &str = r#"
{"timestamp":"2026-10-17T10:40:00Z","event":"ScanOrganic","ScanType":"Log","Genus":"$Codex_Ent_Stratum_Genus_Name;","Genus_Localised":"Stratum","Species":"$Codex_Ent_Stratum_07_Name;","Species_Localised":"Stratum Tectonicas","Variant":"$Codex_Ent_Stratum_07_K_Name;","Variant_Localised":"Stratum Tectonicas - Emerald","SystemAddress":99,"Body":5}
"#;

#[test]
fn genus_list_and_prediction_follow_fss_dss_and_samples() {
    let mut s = SystemFold::default();
    feed(&mut s, ARRIVE);
    assert_eq!(s.system(), Some("Synthetic Sector AB-C d1"));
    assert!(ranked(&s).is_empty(), "no biology known yet");

    // FSS first: the count, nothing to predict from yet.
    feed(&mut s, FSS_A2);
    let r = ranked(&s);
    assert_eq!(r.len(), 1);
    assert_eq!((r[0].signals, r[0].mapped, r[0].genera.len()), (Some(2), false, 0));

    // The detailed scan: a prediction, Stratum Tectonicas among it.
    feed(&mut s, SCAN_A2);
    let b = &ranked(&s)[0];
    assert_eq!(b.name, "Synthetic Sector AB-C d1 A 2");
    assert!(b.genera.iter().all(|g| !g.known));
    let stratum = b.genera.iter().find(|g| g.genus == "Stratum").expect("Stratum predicted");
    assert!(stratum.species.iter().any(|c| c.species == "Stratum Tectonicas"));
    assert_eq!(stratum.value.1, 19_010_800);
    assert_eq!(stratum.colony_range_m, Some(500));
    // Two genera: at most the two dearest predicted genera, at least the two cheapest.
    let mut maxes: Vec<i64> = b.genera.iter().map(|g| g.value.1).collect();
    maxes.sort_by(|a, b| b.cmp(a));
    let mut mins: Vec<i64> = b.genera.iter().map(|g| g.value.0).collect();
    mins.sort();
    assert_eq!(b.value, (mins[0] + mins[1], maxes[0] + maxes[1]));
    assert!(b.genera.len() > 2);

    // The DSS names the two genera: only those, both known, Tectonicas still a candidate.
    feed(&mut s, DSS_A2);
    let b = &ranked(&s)[0];
    assert!(b.mapped);
    let names: Vec<&str> = b.genera.iter().map(|g| g.genus.as_str()).collect();
    assert_eq!(names, ["Stratum", "Bacterium"], "most valuable first");
    assert!(b.genera.iter().all(|g| g.known));
    assert!(b.genera[0].species.iter().any(|c| c.species == "Stratum Tectonicas"));
    assert_eq!(b.value, (b.genera[0].value.0 + b.genera[1].value.0, b.genera[0].value.1 + b.genera[1].value.1));

    // Sampling pins the species.
    feed(&mut s, SAMPLE_A2);
    let b = &ranked(&s)[0];
    assert_eq!(b.genera[0].species.len(), 1);
    assert_eq!(b.genera[0].value, (19_010_800, 19_010_800));

    // Another system: start again.
    feed(
        &mut s,
        r#"{"timestamp":"2026-10-17T11:00:00Z","event":"FSDJump","StarSystem":"Elsewhere","SystemAddress":100,"StarPos":[0,0,0],"JumpDist":9.0}"#,
    );
    assert_eq!(s.system(), Some("Elsewhere"));
    assert!(ranked(&s).is_empty());
}

#[test]
fn a_login_here_keeps_what_is_known_and_another_system_starts_afresh() {
    let mut s = SystemFold::default();
    feed(&mut s, ARRIVE);
    feed(&mut s, FSS_A2);
    feed(
        &mut s,
        r#"{"timestamp":"2026-10-17T12:00:00Z","event":"Location","StarSystem":"Synthetic Sector AB-C d1","SystemAddress":99,"StarPos":[1,2,3],"Docked":false}"#,
    );
    assert_eq!(ranked(&s).len(), 1);
    // A line from another system means a jump the journal missed: that system, afresh.
    feed(
        &mut s,
        &FSS_A2.replace("\"SystemAddress\":99", "\"SystemAddress\":7").replace("\"BodyID\":5", "\"BodyID\":6"),
    );
    let r = ranked(&s);
    assert_eq!((r.len(), r[0].body_id), (1, Some(6)));
}

#[test]
fn a_dss_genus_the_conditions_do_not_explain_keeps_its_whole_range() {
    let mut s = SystemFold::default();
    feed(&mut s, ARRIVE);
    feed(&mut s, SCAN_A2);
    // Fonticulua grows on icy bodies, not this one; the DSS says it's there anyway.
    feed(&mut s, &DSS_A2.replace("$Codex_Ent_Bacterial_Genus_Name;", "$Codex_Ent_Fonticulus_Genus_Name;"));
    let b = &ranked(&s)[0];
    let f = b.genera.iter().find(|g| g.genus == "Fonticulua").unwrap();
    assert!(f.species.is_empty());
    assert_eq!(f.value, astromech_exobiology::catalog::genus("Fonticulua").unwrap().value_range());
}
