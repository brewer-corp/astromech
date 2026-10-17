//! Journal replay tests for the FSS signal callouts. The signal lines follow real journals
//! (see `astromech_announce::signals` for sources).

mod common;

use common::Run;

fn jump(name: &str, address: i64) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"FSDJump","StarSystem":"{name}","SystemAddress":{address},"StarPos":[0,0,0],"JumpDist":10.0}}"#
    )
}

fn signal(address: i64, name: &str, ty: &str, extra: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:01:00Z","event":"FSSSignalDiscovered","SystemAddress":{address},"SignalName":"{name}","SignalType":"{ty}"{extra}}}"#
    )
}

#[test]
fn each_signal_type_gets_its_line_once_per_system() {
    let mut r = Run::new();
    r.spoken(&jump("Sys", 1));
    let honk = [
        signal(1, "$Warzone_PointRace_High:#index=1;", "Combat", ""),
        signal(1, "$Warzone_PointRace_High:#index=2;", "Combat", ""),
        signal(1, "$Warzone_PointRace_Low:#index=3;", "Combat", ""),
        signal(1, "$Warzone_TG_VeryHigh:#index=1;", "Combat", ""),
        signal(1, "$Warzone_Powerplay_Med:#index=1;", "Combat", ""),
        signal(1, "$USS_NonHumanSignal;", "USS", r#","USSType":"$USS_Type_NonHuman;""#),
        signal(1, "$USS_PowerEmissions;", "USS", r#","USSType":"$USS_Type_PowerEmissions;""#),
        signal(1, "Hera Titan", "Titan", ""),
        signal(1, "CEN-770 Lowell-class Researcher", "Megaship", ""),
        signal(1, "Another Megaship", "Megaship", ""),
        // Stations, carriers and everything else stay quiet.
        signal(1, "Morrison Prospect", "Outpost", ""),
        signal(1, "Carrier", "FleetCarrier", r#","IsStation":true"#),
        signal(1, "$MULTIPLAYER_SCENARIO42_TITLE;", "NavBeacon", ""),
    ]
    .join("\n");
    let expected = [
        "Conflict zone, high intensity.",
        "Conflict zone, low intensity.",
        "Thargoid conflict zone, very high intensity.",
        "Powerplay conflict zone, medium intensity.",
        "Thargoid signal.",
        "Powerplay signal.",
        "Thargoid Titan in system.",
        "Megaship in system.",
    ];
    assert_eq!(r.spoken(&honk), expected);
    // A second honk in the same system says nothing new.
    assert!(r.spoken(&honk).is_empty());
    // Another system starts afresh.
    r.spoken(&jump("Other", 2));
    assert_eq!(r.spoken(&signal(2, "Ship", "Megaship", "")), ["Megaship in system."]);
}

#[test]
fn coming_back_to_a_system_says_it_again() {
    let mut r = Run::new();
    r.spoken(&jump("A", 1));
    assert_eq!(r.spoken(&signal(1, "Ship", "Megaship", "")), ["Megaship in system."]);
    r.spoken(&jump("B", 2));
    r.spoken(&jump("A", 1));
    assert_eq!(r.spoken(&signal(1, "Ship", "Megaship", "")), ["Megaship in system."]);
}
