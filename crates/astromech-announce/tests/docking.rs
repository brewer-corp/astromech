//! Journal replay tests for the docking callouts: pad position on a grant, the reason on a
//! denial.

mod common;

use common::Run;

fn granted(pad: i64, station_type: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"DockingGranted","LandingPad":{pad},"MarketID":128000000,"StationName":"Jameson Memorial","StationType":"{station_type}"}}"#
    )
}

/// Ends an approach, so the next grant is not "during docking" (the engine's quiet).
const CANCELLED: &str = r#"{"timestamp":"2026-10-16T20:00:05Z","event":"DockingCancelled","MarketID":128000000,"StationName":"Jameson Memorial","StationType":"Coriolis"}"#;

/// Spoken lines for a grant, then the approach ends.
fn grant(r: &mut Run, pad: i64, station_type: &str) -> Vec<String> {
    let said = r.spoken(&granted(pad, station_type));
    assert!(r.spoken(CANCELLED).is_empty());
    said
}

fn denied(reason: &str) -> String {
    format!(
        r#"{{"timestamp":"2026-10-16T20:00:00Z","event":"DockingDenied","Reason":"{reason}","MarketID":128000000,"StationName":"Jameson Memorial","StationType":"Coriolis"}}"#
    )
}

#[test]
fn a_coriolis_pad_number_gives_its_position() {
    let mut r = Run::new();
    // Looking in through the slot, green lights on the right: pads 1 to 15 run clockwise from
    // six, front to back in each sector; 16 to 30 from ten; 31 to 45 from two.
    for (pad, line) in [
        (1, "Pad at six, front."),
        (2, "Pad at six, front."),
        (3, "Pad at six, back."),
        (7, "Pad at seven, middle."),
        (8, "Pad at seven, back."),
        (9, "Pad at eight, front."),
        (10, "Pad at eight, back."),
        (13, "Pad at nine, middle."),
        (15, "Pad at nine, back."),
        (16, "Pad at ten, front."),
        (22, "Pad at eleven, middle."),
        (25, "Pad at twelve, back."),
        (30, "Pad at one, back."),
        (31, "Pad at two, front."),
        (37, "Pad at three, middle."),
        (39, "Pad at four, front."),
        (41, "Pad at five, front."),
        (45, "Pad at five, back."),
    ] {
        assert_eq!(grant(&mut r, pad, "Coriolis"), [line], "pad {pad}");
    }
}

#[test]
fn orbis_and_ocellus_share_the_layout_others_get_nothing() {
    let mut r = Run::new();
    assert_eq!(grant(&mut r, 7, "Orbis"), ["Pad at seven, middle."]);
    assert_eq!(grant(&mut r, 7, "Ocellus"), ["Pad at seven, middle."]);
    // Outposts, carriers and surface ports: the pad is in plain sight, nothing to say.
    for t in ["Outpost", "FleetCarrier", "CraterOutpost", "OnFootSettlement"] {
        assert!(grant(&mut r, 3, t).is_empty(), "{t}");
    }
    // A number outside the layout, or no station type: nothing rather than a wrong position.
    assert!(grant(&mut r, 46, "Coriolis").is_empty());
    let untyped = granted(7, "Coriolis").replace(r#","StationType":"Coriolis""#, "");
    assert!(r.spoken(&untyped).is_empty());
}

#[test]
fn each_denial_reason_gets_its_line() {
    let mut r = Run::new();
    for (reason, line) in [
        ("NoSpace", "Docking denied. No free pad."),
        ("TooLarge", "Docking denied. Ship too large."),
        ("Hostile", "Docking denied. Station hostile."),
        ("Offences", "Docking denied. Offences here."),
        ("Distance", "Docking denied. Too far away."),
        ("ActiveFighter", "Docking denied. Fighter still out."),
        ("NoReason", "Docking denied. No reason given."),
        ("RestrictedAccess", "Docking denied. Access restricted."),
        ("DockOffline", "Docking denied. Docking offline."),
        ("JumpImminent", "Docking denied. Carrier about to jump."),
        ("SomethingNew", "Docking denied."),
    ] {
        assert_eq!(r.spoken(&denied(reason)), [line], "{reason}");
    }
}

#[test]
fn docking_callouts_are_silent_during_the_catch_up() {
    let mut r = Run::new();
    r.engine.set_live(false);
    assert!(r.spoken(&[granted(7, "Coriolis"), denied("NoSpace")].join("\n")).is_empty());
}
