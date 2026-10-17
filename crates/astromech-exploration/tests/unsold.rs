//! Journal replay of exploration and its sale into the unsold-data total.

use astromech_exploration::unsold::{from_json_lines, RISKY};
use astromech_exploration::value::{planet_value, star_value, Terms};

fn scan(ts: &str, system: &str, address: i64, body: &str, fields: &str) -> String {
    format!(
        r#"{{"timestamp":"{ts}","event":"Scan","ScanType":"Detailed","BodyName":"{system} {body}","BodyID":2,"StarSystem":"{system}","SystemAddress":{address},{fields}}}"#
    )
}

const ICY: &str = r#""PlanetClass":"Icy body","MassEM":0.5,"WasDiscovered":true,"WasMapped":true"#;
const STAR: &str = r#""StarType":"G","StellarMass":1.0,"WasDiscovered":true"#;
const WW: &str = r#""PlanetClass":"Water world","MassEM":1.0,"WasDiscovered":true,"WasMapped":true"#;

fn organic(ts: &str, species: &str) -> String {
    format!(
        r#"{{"timestamp":"{ts}","event":"ScanOrganic","ScanType":"Analyse","Genus":"$Codex_Ent_Stratum_Genus_Name;","Species":"{species}","Variant":"x","SystemAddress":1,"Body":2}}"#
    )
}

fn known() -> Terms {
    Terms { first_discovery: false, first_mapped: false, mapping: None, odyssey: true }
}

#[test]
fn scans_add_up_and_a_sale_takes_off_what_it_sold() {
    let icy = planet_value("Icy body", false, Some(0.5), known());
    let star = star_value("G", Some(1.0), false);
    let ww_mapped = planet_value("Water world", false, Some(1.0), known().mapped(true));
    let mut lines = vec![
        scan("2026-10-17T10:00:00Z", "Alpha", 1, "A", STAR),
        scan("2026-10-17T10:01:00Z", "Alpha", 1, "A 1", ICY),
        // Scanned twice (automatic, then detailed): counted once.
        scan("2026-10-17T10:01:30Z", "Alpha", 1, "A 1", ICY),
        scan("2026-10-17T11:00:00Z", "Beta", 2, "B 1", WW),
        r#"{"timestamp":"2026-10-17T11:05:00Z","event":"SAAScanComplete","BodyName":"Beta B 1","BodyID":2,"SystemAddress":2,"ProbesUsed":4,"EfficiencyTarget":5}"#.into(),
        organic("2026-10-17T11:30:00Z", "$Codex_Ent_Stratum_07_Name;"),
        organic("2026-10-17T11:40:00Z", "$Codex_Ent_Bacterial_01_Name;"),
    ];
    let u = from_json_lines(&lines);
    assert_eq!((u.systems, u.bodies, u.samples), (2, 3, 2));
    assert_eq!(u.cartographic, star + icy + ww_mapped);
    assert_eq!(u.organic, 19_010_800 + 1_000_000);
    assert_eq!(u.total, u.cartographic + u.organic);
    assert_eq!(u.since.as_deref(), Some("2026-10-17T10:00:00Z"));
    assert!(!u.risky);

    // Selling Alpha only.
    lines.push(r#"{"timestamp":"2026-10-17T12:00:00Z","event":"SellExplorationData","Systems":["Alpha"],"Discovered":[],"BaseValue":1,"Bonus":0,"TotalEarnings":1}"#.into());
    let u = from_json_lines(&lines);
    assert_eq!((u.systems, u.bodies, u.cartographic), (1, 1, ww_mapped));
    assert_eq!(u.since.as_deref(), Some("2026-10-17T11:00:00Z"));

    // Selling the rest at once, and the organics.
    lines.push(r#"{"timestamp":"2026-10-17T12:01:00Z","event":"MultiSellExplorationData","Discovered":[{"SystemName":"Beta","NumBodies":1}],"BaseValue":1,"Bonus":0,"TotalEarnings":1}"#.into());
    lines.push(r#"{"timestamp":"2026-10-17T12:02:00Z","event":"SellOrganicData","MarketID":1,"BioData":[{"Genus":"$Codex_Ent_Stratum_Genus_Name;","Species":"$Codex_Ent_Stratum_07_Name;","Value":19010800,"Bonus":0},{"Genus":"$Codex_Ent_Bacterial_Genus_Name;","Species":"$Codex_Ent_Bacterial_01_Name;","Value":1000000,"Bonus":0}]}"#.into());
    let u = from_json_lines(&lines);
    assert_eq!(u.total, 0);
    assert_eq!((u.systems, u.bodies, u.samples), (0, 0, 0));
    assert_eq!(u.since, None);

    // Scanning on after the sale starts the next total.
    lines.push(scan("2026-10-17T13:00:00Z", "Gamma", 3, "C 1", ICY));
    assert_eq!(from_json_lines(&lines).cartographic, icy);
}

#[test]
fn dying_loses_everything_and_a_big_haul_is_risky() {
    let mut lines: Vec<String> = (0..12)
        .map(|i| {
            format!(
                r#"{{"timestamp":"2026-10-17T10:{i:02}:00Z","event":"ScanOrganic","ScanType":"Analyse","Genus":"$Codex_Ent_Fonticulus_Genus_Name;","Species":"$Codex_Ent_Fonticulus_05_Name;","SystemAddress":1,"Body":2}}"#
            )
        })
        .collect();
    let u = from_json_lines(&lines);
    // The wiki's value for Fonticulua Fluctus (data/catalog/exobiology.json lists the 20 M others give).
    assert_eq!(u.organic, 12 * 16_777_215);
    assert!(u.risky && u.total >= RISKY);
    // Logged samples that are not complete (Log, Sample) are not data yet.
    lines.push(r#"{"timestamp":"2026-10-17T10:30:00Z","event":"ScanOrganic","ScanType":"Log","Genus":"g","Species":"$Codex_Ent_Fonticulus_05_Name;","SystemAddress":1,"Body":2}"#.into());
    assert_eq!(from_json_lines(&lines).samples, 12);
    lines.push(r#"{"timestamp":"2026-10-17T10:31:00Z","event":"Died"}"#.into());
    assert_eq!(from_json_lines(&lines).total, 0);
}

#[test]
fn an_unknown_species_counts_apart() {
    let u = from_json_lines(&[organic("2026-10-17T10:00:00Z", "$Codex_Ent_Brand_New_Name;")]);
    assert_eq!((u.samples, u.unknown_samples, u.organic), (1, 1, 0));
}
