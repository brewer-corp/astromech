//! Journal replay tests for the high-value discovery callout.

mod common;

use common::Run;

fn jump(name: &str, address: i64) -> String {
    format!(
        r#"{{"timestamp":"2026-10-17T10:00:00Z","event":"FSDJump","StarSystem":"{name}","SystemAddress":{address},"StarPos":[0,0,0],"JumpDist":10.0}}"#
    )
}

fn scan(address: i64, body: &str, scan_type: &str, fields: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-17T10:01:00Z","event":"Scan","ScanType":"{scan_type}","BodyName":"Sys {body}","BodyID":3,"StarSystem":"Sys","SystemAddress":{address},"DistanceFromArrivalLS":120.0,{fields}}}"#
    )
}

const ELW: &str = r#""PlanetClass":"Earthlike body","TerraformState":"Terraformed","MassEM":0.8,"WasDiscovered":false,"WasMapped":false"#;

#[test]
fn an_earth_like_world_gets_one_line_and_all_bodies_found_none() {
    let mut r = Run::new();
    r.spoken(&jump("Sys", 1));
    // Mapped efficiently and first discovered: about 4.4 million.
    assert_eq!(r.spoken(&scan(1, "A 2", "Detailed", ELW)), ["Earth-like world. 4.4 million mapped."]);
    // The same body scanned again (a detailed scan after the automatic one) says nothing.
    assert!(r.spoken(&scan(1, "A 2", "Detailed", ELW)).is_empty());
    let all = r#"{"timestamp":"2026-10-17T10:02:00Z","event":"FSSAllBodiesFound","SystemName":"Sys","SystemAddress":1,"Count":7}"#;
    assert!(r.spoken(all).is_empty());
    let honk = r#"{"timestamp":"2026-10-17T10:00:30Z","event":"FSSDiscoveryScan","Progress":0.2,"BodyCount":7,"NonBodyCount":0,"SystemName":"Sys","SystemAddress":1}"#;
    assert!(r.spoken(honk).is_empty());
}

#[test]
fn terraformables_and_mapping_targets_are_called_out_and_the_rest_are_not() {
    let mut r = Run::new();
    r.spoken(&jump("Sys", 1));
    let tf_hmc = r#""PlanetClass":"High metal content body","TerraformState":"Terraformable","MassEM":0.4,"WasDiscovered":true,"WasMapped":false"#;
    assert_eq!(
        r.spoken(&scan(1, "A 3", "Detailed", tf_hmc)),
        ["Terraformable high metal content world. 2.1 million mapped."]
    );
    let ww = r#""PlanetClass":"Water world","TerraformState":"","MassEM":1.2,"WasDiscovered":true,"WasMapped":true"#;
    assert_eq!(r.spoken(&scan(1, "A 4", "Detailed", ww)), ["Water world. 557 thousand mapped."]);
    let aw = r#""PlanetClass":"Ammonia world","MassEM":0.5,"WasDiscovered":false,"WasMapped":false"#;
    assert_eq!(r.spoken(&scan(1, "A 5", "AutoScan", aw)), ["Ammonia world. 2.3 million mapped."]);
    // Not worth a line: an icy body, a gas giant, a plain high metal content world, a star.
    for (body, fields) in [
        ("B 1", r#""PlanetClass":"Icy body","MassEM":0.1,"Landable":true,"WasDiscovered":false,"WasMapped":false"#),
        ("B 2", r#""PlanetClass":"Sudarsky class I gas giant","MassEM":300.0,"WasDiscovered":false,"WasMapped":false"#),
        ("B 3", r#""PlanetClass":"High metal content body","TerraformState":"","MassEM":2.0,"WasDiscovered":false"#),
        ("B", r#""StarType":"N","StellarMass":1.4,"WasDiscovered":false"#),
    ] {
        assert!(r.spoken(&scan(1, body, "Detailed", fields)).is_empty(), "{body}");
    }
}

#[test]
fn nav_beacon_data_is_not_a_discovery_and_each_visit_starts_afresh() {
    let mut r = Run::new();
    r.spoken(&jump("Sys", 1));
    assert!(r.spoken(&scan(1, "A 2", "NavBeaconDetail", ELW)).is_empty());
    assert_eq!(r.spoken(&scan(1, "A 2", "Detailed", ELW)).len(), 1);
    r.spoken(&jump("Other", 2));
    r.spoken(&jump("Sys", 1));
    assert_eq!(r.spoken(&scan(1, "A 2", "Detailed", ELW)).len(), 1);
}

#[test]
fn it_can_be_turned_off() {
    let off: astromech_announce::Settings = toml::from_str("[discovery]\nenabled = false\n").unwrap();
    let mut r = Run::with(Default::default(), &off);
    r.spoken(&jump("Sys", 1));
    assert!(r.spoken(&scan(1, "A 2", "Detailed", ELW)).is_empty());
}
