//! The sampling tracker: counts, positions and clearance from synthetic journal and Status.json.

use astromech_exobiology::sampling::Sampling;
use astromech_journal::parse_line;
use astromech_status::parse_status;

fn organic(s: &mut Sampling, kind: &str) {
    let l = format!(
        r#"{{"timestamp":"2026-10-17T10:00:00Z","event":"ScanOrganic","ScanType":"{kind}","Genus":"$Codex_Ent_Bacterial_Genus_Name;","Genus_Localised":"Bacterium","Species":"$Codex_Ent_Bacterial_04_Name;","Species_Localised":"Bacterium Vesicula","SystemAddress":99,"Body":5}}"#
    );
    s.apply(&parse_line(&l).unwrap().event);
}

fn at(s: &mut Sampling, lat: f64, lon: f64) {
    let j = format!(r#"{{"Flags":2097152,"Latitude":{lat},"Longitude":{lon},"PlanetRadius":2000000.0}}"#);
    s.apply_status(&parse_status(j.as_bytes()).unwrap());
}

#[test]
fn counts_and_clearance() {
    let mut s = Sampling::default();
    at(&mut s, 0.0, 0.0);
    organic(&mut s, "Log");
    let p = s.current.as_ref().unwrap();
    assert_eq!((p.genus.as_str(), p.count, p.colony_range_m, p.complete), ("Bacterium", 1, Some(500), false));
    // 0.01° east on a 2000 km body is 349 m; the first sample is due west.
    at(&mut s, 0.0, 0.01);
    let c = s.clearance().unwrap();
    assert_eq!((c.next, c.far_enough), (2, false));
    assert!((c.nearest_m - 349.07).abs() < 0.01 && (c.bearing_deg - 270.0).abs() < 1e-6, "{c:?}");
    at(&mut s, 0.0, 0.015);
    assert!(s.clearance().unwrap().far_enough);
    organic(&mut s, "Sample");
    organic(&mut s, "Sample");
    assert_eq!(s.current.as_ref().unwrap().count, 3);
    assert_eq!(s.clearance(), None, "no next sample");
    organic(&mut s, "Analyse");
    assert!(s.current.as_ref().unwrap().complete);
}

#[test]
fn a_sample_without_its_log_and_leaving_the_surface() {
    let mut s = Sampling::default();
    at(&mut s, 1.0, 1.0);
    organic(&mut s, "Sample");
    assert_eq!(s.current.as_ref().unwrap().count, 2);
    assert_eq!(s.clearance(), None, "the first sample's place is unknown");
    s.apply_status(&parse_status(br#"{"Flags":16777216}"#).unwrap());
    assert_eq!(s.position, None);
    s.apply(&parse_line(r#"{"timestamp":"2026-10-17T11:00:00Z","event":"FSDJump","StarSystem":"X","SystemAddress":1,"StarPos":[0,0,0],"JumpDist":5.0}"#).unwrap().event);
    assert_eq!(s.current, None);
}
