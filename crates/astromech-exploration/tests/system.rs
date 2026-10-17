//! Journal replay of a whole system into the live system view. Synthetic journal lines in the
//! game's format.

use astromech_exploration::system::{SystemFold, SystemView};
use astromech_journal::parse_line;

fn replay(lines: &[String]) -> SystemView {
    let mut f = SystemFold::default();
    for l in lines {
        f.apply(&parse_line(l).unwrap());
    }
    f.view()
}

fn jump(name: &str, address: i64, population: i64) -> String {
    format!(
        r#"{{"timestamp":"2026-10-17T10:00:00Z","event":"FSDJump","StarSystem":"{name}","SystemAddress":{address},"StarPos":[0,0,0],"Population":{population},"JumpDist":20.1}}"#
    )
}

fn scan(address: i64, system: &str, body: &str, id: i64, fields: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-17T10:01:00Z","event":"Scan","ScanType":"Detailed","BodyName":"{system} {body}","BodyID":{id},"StarSystem":"{system}","SystemAddress":{address},"DistanceFromArrivalLS":{},{fields}}}"#,
        id * 100
    )
}

/// An unexplored system: the arrival star, an Earth-like world nobody discovered (mapped
/// efficiently), a terraformable high metal content world, a landable icy moon with biology,
/// a ringed gas giant, a belt cluster, and the honk saying there are five bodies.
pub fn unexplored() -> Vec<String> {
    let (a, s) = (7_000_001, "Synuefe AB-C d1-2");
    vec![
        jump(s, a, 0),
        format!(
            r#"{{"timestamp":"2026-10-17T10:00:05Z","event":"Scan","ScanType":"AutoScan","BodyName":"{s} A","BodyID":1,"StarSystem":"{s}","SystemAddress":{a},"DistanceFromArrivalLS":0.0,"StarType":"K","Subclass":3,"StellarMass":0.7,"Rings":[{{"Name":"{s} A Belt","RingClass":"eRingClass_Rocky"}}],"WasDiscovered":false,"WasMapped":false}}"#
        ),
        format!(
            r#"{{"timestamp":"2026-10-17T10:00:10Z","event":"FSSDiscoveryScan","Progress":0.4,"BodyCount":5,"NonBodyCount":2,"SystemName":"{s}","SystemAddress":{a}}}"#
        ),
        scan(
            a,
            s,
            "A 2",
            2,
            r#""PlanetClass":"Earthlike body","TerraformState":"Terraformed","MassEM":0.8,"SurfaceGravity":8.9,"Landable":false,"WasDiscovered":false,"WasMapped":false"#,
        ),
        scan(
            a,
            s,
            "A 3",
            3,
            r#""PlanetClass":"High metal content body","TerraformState":"Terraformable","MassEM":0.4,"SurfaceGravity":6.2,"Landable":false,"WasDiscovered":false,"WasMapped":false"#,
        ),
        format!(
            r#"{{"timestamp":"2026-10-17T10:01:30Z","event":"FSSBodySignals","BodyName":"{s} A 4 a","BodyID":5,"SystemAddress":{a},"Signals":[{{"Type":"$SAA_SignalType_Biological;","Type_Localised":"Biological","Count":2}},{{"Type":"$SAA_SignalType_Geological;","Type_Localised":"Geological","Count":1}}]}}"#
        ),
        scan(
            a,
            s,
            "A 4 a",
            5,
            r#""PlanetClass":"Icy body","TerraformState":"","MassEM":0.02,"SurfaceGravity":1.47,"Landable":true,"WasDiscovered":false,"WasMapped":false"#,
        ),
        scan(
            a,
            s,
            "A 4",
            4,
            r#""PlanetClass":"Sudarsky class I gas giant","MassEM":120.0,"Rings":[{"Name":"A 4 A Ring","RingClass":"eRingClass_Icy"}],"WasDiscovered":false,"WasMapped":false"#,
        ),
        scan(a, s, "A Belt Cluster 1", 6, r#""WasDiscovered":false"#),
        format!(
            r#"{{"timestamp":"2026-10-17T10:05:00Z","event":"SAAScanComplete","BodyName":"{s} A 2","BodyID":2,"SystemAddress":{a},"ProbesUsed":5,"EfficiencyTarget":6}}"#
        ),
        format!(
            r#"{{"timestamp":"2026-10-17T10:06:00Z","event":"FSSAllBodiesFound","SystemName":"{s}","SystemAddress":{a},"Count":5}}"#
        ),
    ]
}

#[test]
fn a_replayed_system_shows_every_body_with_its_value() {
    let v = replay(&unexplored());
    assert_eq!(v.system.as_deref(), Some("Synuefe AB-C d1-2"));
    assert_eq!(v.body_count, Some(5));
    assert!(v.all_found);
    // Every body the honk counted, in body order, and not the belt cluster.
    let names: Vec<&str> = v.bodies.iter().map(|b| b.short_name.as_str()).collect();
    assert_eq!(names, ["A", "A 2", "A 3", "A 4", "A 4 a"]);
    assert_eq!(v.bodies.len() as i64, v.body_count.unwrap());

    let star = &v.bodies[0];
    assert!(star.star && star.first_discovery && !star.worth_mapping);
    assert_eq!(star.class_name, "K3 star");
    // The journal lists the star's asteroid belt under Rings: a belt, not a ring.
    assert!(star.belts && !star.rings);
    assert_eq!(star.value, astromech_exploration::value::star_value("K", Some(0.7), true));

    let elw = &v.bodies[1];
    assert_eq!(elw.class_name, "Earth-like world");
    assert!(elw.terraformable && elw.mapped && elw.mapped_efficiently && !elw.worth_mapping);
    // Mapped efficiently: the value now is the mapped value.
    assert_eq!(elw.value, elw.value_mapped);
    assert!(elw.value > 3_000_000, "{}", elw.value);
    assert!((elw.gravity_g.unwrap() - 0.9076).abs() < 1e-3);

    let hmc = &v.bodies[2];
    assert!(hmc.terraformable && !hmc.mapped && hmc.worth_mapping);
    assert!(hmc.value_mapped > hmc.value * 5);

    let gg = &v.bodies[3];
    assert!(gg.rings && !gg.belts && !gg.landable && !gg.worth_mapping);

    let moon = &v.bodies[4];
    assert!(moon.landable && !moon.worth_mapping);
    assert_eq!((moon.bio_signals, moon.geo_signals), (2, 1));

    // The system's value now, and with the high metal content world mapped too.
    let now: i64 = v.bodies.iter().map(|b| b.value).sum();
    assert_eq!(v.value, now);
    assert_eq!(v.value_if_mapped, now - hmc.value + hmc.value_mapped);
}

#[test]
fn a_jump_starts_the_next_system_and_a_relog_keeps_this_one() {
    let mut lines = unexplored();
    lines.push(
        r#"{"timestamp":"2026-10-17T11:00:00Z","event":"Location","StarSystem":"Synuefe AB-C d1-2","SystemAddress":7000001,"StarPos":[0,0,0],"Population":0,"Docked":false}"#
            .into(),
    );
    assert_eq!(replay(&lines).bodies.len(), 5);
    lines.push(jump("Next", 7_000_002, 0));
    let v = replay(&lines);
    assert_eq!(v.system.as_deref(), Some("Next"));
    assert!(v.bodies.is_empty() && v.body_count.is_none() && !v.all_found);
}

#[test]
fn a_journal_that_starts_mid_system_still_shows_its_scans() {
    let lines: Vec<String> = unexplored().into_iter().skip(1).collect();
    let v = replay(&lines);
    assert_eq!(v.system.as_deref(), Some("Synuefe AB-C d1-2"));
    assert_eq!(v.bodies.len(), 5);
}
