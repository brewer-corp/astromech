//! Fuel along the route (`NavRoute.json` and each `FSDJump`). The HUD's low-fuel warning and
//! scoop progress aren't repeated; what it doesn't show is how far the next fuel star is.
//!
//! - [`Scoopable`] (`scoopable`): arriving at a star that can't be scooped while following a
//!   route, "Next scoopable star in 3 jumps.", "Next star is scoopable." or "No scoopable stars
//!   left on the route." Nothing at a scoopable star or off the route.
//! - [`FuelRange`] (`fuel_range`): on arrival, when the main tank won't reach the next
//!   scoopable star (or the end of the route if there is none), "Fuel too low to reach a
//!   scoopable star."; at a scoopable star, "Scoop here. Next scoopable star out of reach."
//!   High priority, at most once per 10 minutes.
//!
//! Fuel per jump is estimated from the last jump: the FSD's fuel use grows with distance to its
//! power (2.0 for a size 2 drive up to 2.9 for size 8; 2.45 when the drive isn't known), so a hop
//! of `d` ly costs `used × (d / last distance)^power`. Mass changes along the way are ignored.
//! Scoopable stars are the KGBFOAM classes, giants included ("K_OrangeGiant").

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::{JournalEvent, RouteHop};
use std::time::Duration;

/// Whether a `StarClass` (as `NavRoute.json` writes it) can be fuel-scooped.
pub fn scoopable(class: &str) -> bool {
    matches!(class.split('_').next().unwrap_or(""), "O" | "B" | "A" | "F" | "G" | "K" | "M")
}

/// The FSD's fuel power constant for its size.
fn power(size: Option<u8>) -> f64 {
    match size {
        Some(2) => 2.0,
        Some(3) => 2.15,
        Some(4) => 2.3,
        Some(6) => 2.6,
        Some(7) => 2.75,
        Some(8) => 2.9,
        _ => 2.45,
    }
}

/// Light years between two route hops (None without both positions).
pub fn hop_ly(a: &RouteHop, b: &RouteHop) -> Option<f64> {
    let (a, b) = (a.star_pos?, b.star_pos?);
    Some(((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt())
}

/// Estimated tonnes for the jump from `a` to `b`, from the last jump (None before any jump).
pub fn hop_fuel(state: &GameState, a: &RouteHop, b: &RouteHop) -> Option<f64> {
    let (d0, f0) = state.last_jump?;
    Some(f0 * (hop_ly(a, b)? / d0).powf(power(state.fsd_size)))
}

/// What lies ahead on the route from the current system.
#[derive(Debug, Clone, PartialEq)]
pub struct Ahead {
    /// Whether the current system's star can be scooped (None: class unknown).
    pub here_scoopable: Option<bool>,
    /// Jumps to the next scoopable star after this one; None if there is none.
    pub jumps: Option<usize>,
    /// Estimated tonnes to reach it (or the end of the route); None without a jump to go by.
    pub fuel_needed: Option<f64>,
}

/// None when there is no route, the current system isn't on it, or it is the destination.
pub fn ahead(state: &GameState) -> Option<Ahead> {
    let hops = &state.route_hops;
    let here = state.system.as_deref()?;
    let i = hops.iter().rposition(|h| h.system.eq_ignore_ascii_case(here))?;
    if i + 1 >= hops.len() {
        return None;
    }
    let jumps = hops[i + 1..].iter().position(|h| h.star_class.as_deref().is_some_and(scoopable)).map(|k| k + 1);
    let end = jumps.map_or(hops.len() - 1, |k| i + k);
    let fuel_needed = hops[i..=end].windows(2).map(|w| hop_fuel(state, &w[0], &w[1])).sum();
    Some(Ahead { here_scoopable: hops[i].star_class.as_deref().map(scoopable), jumps, fuel_needed })
}

/// The line for arriving at a star that can't be scooped.
pub fn next_line(jumps: Option<usize>) -> String {
    match jumps {
        Some(1) => "Next star is scoopable.".into(),
        Some(n) => format!("Next scoopable star in {n} jumps."),
        None => "No scoopable stars left on the route.".into(),
    }
}

/// The warning, or None when the main tank reaches the next fuel star.
pub fn range_line(a: &Ahead, fuel_main: f64) -> Option<&'static str> {
    if a.fuel_needed? <= fuel_main {
        return None;
    }
    Some(match (a.here_scoopable, a.jumps) {
        (Some(true), _) => "Scoop here. Next scoopable star out of reach.",
        (_, Some(_)) => "Fuel too low to reach a scoopable star.",
        (_, None) => "Fuel too low to reach the destination.",
    })
}

fn arrival<'a>(input: &Input<'a>) -> bool {
    matches!(input, Input::Journal(l) if matches!(l.event, JournalEvent::FsdJump { from_carrier: false, .. }))
}

pub struct Scoopable;

impl Rule for Scoopable {
    fn id(&self) -> &'static str {
        "scoopable"
    }
    /// One line per arrival.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        if !arrival(input) {
            return None;
        }
        let a = ahead(state)?;
        (a.here_scoopable == Some(false)).then(|| Callout {
            text: next_line(a.jumps),
            priority: Priority::Normal,
            channels: Channels::SPEECH | Channels::TOAST,
        })
    }
}

pub struct FuelRange;

impl Rule for FuelRange {
    fn id(&self) -> &'static str {
        "fuel_range"
    }
    /// Still short at the next jumps (the fuel balance doesn't change by jumping); say it again
    /// only after a while.
    fn cooldown(&self) -> Duration {
        Duration::from_secs(600)
    }
    fn react(&self, input: &Input, state: &GameState) -> Option<Callout> {
        if !arrival(input) {
            return None;
        }
        let text = range_line(&ahead(state)?, state.fuel_main?)?;
        Some(Callout { text: text.into(), priority: Priority::High, channels: Channels::SPEECH | Channels::TOAST })
    }
}
