//! Being-scanned callout (`Scanned`): the HUD's scan warning is small and easy to miss in a
//! fight, so the scan is spoken. "Cargo scan." and, when stolen goods are aboard
//! (`Cargo.json`), "Cargo scan. Stolen goods aboard." The ship that scanned isn't named:
//! `Scanned` doesn't say who it was.
//!
//! A cargo scan with stolen goods aboard is [`Priority::Urgent`], so it speaks during docking
//! and combat (station security scans on approach); other scans are quiet then. Only a cargo
//! scan finds cargo, so only it gets the suffix. Goods prohibited in the system aren't
//! checked: the journal and its companion files don't list them.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use std::time::Duration;

pub struct Scan;

/// The line for a scan type, given whether stolen goods are aboard.
pub fn line(scan_type: &str, stolen: bool) -> (String, Priority) {
    let what = match scan_type.to_ascii_lowercase().as_str() {
        "cargo" => "Cargo scan.",
        "crime" => "Crime scan.",
        "cabin" => "Cabin scan.",
        "data" => "Data scan.",
        _ => "Scan.",
    };
    if what == "Cargo scan." && stolen {
        (format!("{what} Stolen goods aboard."), Priority::Urgent)
    } else {
        (what.to_string(), Priority::Normal)
    }
}

impl Rule for Scan {
    fn id(&self) -> &'static str {
        "scan"
    }
    /// Several ships often scan one after another; say it once.
    fn cooldown(&self) -> Duration {
        Duration::from_secs(30)
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let JournalEvent::Extra(ExtraEvent::Scanned { scan_type }) = &l.event else { return None };
        let (text, priority) = line(scan_type, state.cargo_stolen > 0);
        Some(Callout { text, priority, channels: Channels::SPEECH | Channels::TOAST })
    }
}
