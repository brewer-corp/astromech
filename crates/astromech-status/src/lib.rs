//! # astromech-status
//!
//! `Status.json` (rewritten by the game several times a second) and its flag bits. Seeded from
//! spike S5, which measured the press → Status latency used for action verification
//! (ARCHITECTURE.md §17).

mod status_file;
pub use status_file::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_flags_and_rejects_partial_writes() {
        let s = parse_status(br#"{ "timestamp":"x", "event":"Status", "Flags":16842765, "GuiFocus":0 }"#).unwrap();
        assert_ne!(s.flags & FLAG_GEAR, 0);
        assert_ne!(s.flags & FLAG_DOCKED, 0);
        assert_eq!(s.gui_focus, Some(0));
        let rich = parse_status(
            br#"{ "Flags":16, "GuiFocus":0, "Fuel":{"FuelMain":32.0,"FuelReservoir":0.63}, "Cargo":4, "LegalState":"Clean", "BodyName":"Earth" }"#,
        )
        .unwrap();
        assert_eq!(rich.fuel_main, Some(32.0));
        assert_eq!(rich.legal_state.as_deref(), Some("Clean"));
        assert_eq!(rich.body_name.as_deref(), Some("Earth"));
        assert!(parse_status(br#"{ "timestamp":"x", "event":"Sta"#).is_none());
        assert!(parse_status(br#"{ "timestamp":"x", "event":"Status" }"#).is_none(), "main menu: no Flags");
    }
}
