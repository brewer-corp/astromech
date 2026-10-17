//! Docking callouts. On `DockingGranted` at a Coriolis, Orbis or Ocellus starport, where the
//! pad is in the bay ("Pad at seven, middle."): the HUD shows the number, not where it is. On
//! `DockingDenied`, the reason ("Docking denied. No free pad."); COVAS only says it was denied.
//!
//! **The layout.** Seen from the slot as you fly in, green lights on the right, the bay's wall
//! is twelve sectors read as a clock face. Each sector has pads at the front (near the slot),
//! the middle and the back. Pads 1 to 15 fill the four sectors from six o'clock clockwise to
//! nine, 16 to 30 the four from ten to one, and 31 to 45 the four from two to five, always in
//! the same pattern ([`GROUP`]). The three starport types share it. Positions are player-visible
//! game facts, as the EDMC LandingPad plugin draws them (see NOTICE.md); no code from it.
//! Outposts, carriers and surface ports get no line: their pads are in plain sight.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Depth {
    Front,
    Middle,
    Back,
}

use Depth::{Back, Front, Middle};

/// One group of fifteen pads: (sector from the group's first, depth), in pad order.
pub const GROUP: [(u8, Depth); 15] = [
    (0, Front),
    (0, Front),
    (0, Back),
    (0, Back),
    (1, Front),
    (1, Front),
    (1, Middle),
    (1, Back),
    (2, Front),
    (2, Back),
    (3, Front),
    (3, Front),
    (3, Middle),
    (3, Back),
    (3, Back),
];

/// Station types with the 45-pad starport bay (as `DockingGranted` writes them).
const STARPORTS: &[&str] = &["Coriolis", "Orbis", "Ocellus", "Bernal"];

const HOURS: [&str; 12] =
    ["twelve", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven"];

/// Clock hour (1 to 12) and depth of a starport pad, or None outside 1 to 45.
pub fn position(pad: i64) -> Option<(u8, Depth)> {
    if !(1..=45).contains(&pad) {
        return None;
    }
    let i = (pad - 1) as usize;
    let (sector, depth) = GROUP[i % 15];
    let sector = sector as usize + (i / 15) * 4;
    let hour = (6 + sector - 1) % 12 + 1;
    Some((hour as u8, depth))
}

/// "Pad at seven, middle." for a starport pad.
pub fn pad_line(pad: i64, station_type: &str) -> Option<String> {
    if !STARPORTS.iter().any(|t| t.eq_ignore_ascii_case(station_type)) {
        return None;
    }
    let (hour, depth) = position(pad)?;
    let depth = match depth {
        Front => "front",
        Middle => "middle",
        Back => "back",
    };
    Some(format!("Pad at {}, {depth}.", HOURS[hour as usize % 12]))
}

/// The line for a `DockingDenied` reason (journal manual names, plus the carrier ones).
pub fn denied_line(reason: Option<&str>) -> String {
    let why = match reason.unwrap_or_default() {
        "NoSpace" => "No free pad.",
        "TooLarge" => "Ship too large.",
        "Hostile" => "Station hostile.",
        "Offences" => "Offences here.",
        "Distance" => "Too far away.",
        "ActiveFighter" => "Fighter still out.",
        "NoReason" => "No reason given.",
        "RestrictedAccess" => "Access restricted.",
        "DockOffline" => "Docking offline.",
        "JumpImminent" => "Carrier about to jump.",
        _ => return "Docking denied.".into(),
    };
    format!("Docking denied. {why}")
}

pub struct Docking;

impl Rule for Docking {
    fn id(&self) -> &'static str {
        "docking"
    }
    /// One line per request; the game writes one event per request.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let JournalEvent::Extra(ExtraEvent::Docking { granted, station_type, landing_pad, reason, .. }) = &l.event
        else {
            return None;
        };
        let text = if *granted {
            pad_line((*landing_pad)?, station_type.as_deref()?)?
        } else {
            denied_line(reason.as_deref())
        };
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
}
