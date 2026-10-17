//! Route briefing (`route`): when a route is plotted (a new `NavRoute.json`), what the route
//! panel shows only in the galaxy map. One line each, most important first:
//!
//! - **Fuel risk** (high priority): "Fuel too low to reach a scoopable star." when the fuel in
//!   the tank won't reach the first scoopable star (the start can't be scooped), or "Fuel gap:
//!   18 jumps between scoopable stars." for the first stretch a full tank won't cross (ending at
//!   the destination: "Fuel gap: 6 jumps to the destination."). Fuel per hop as in [`fuel`]
//!   (needs a jump to go by; before one, no fuel line).
//! - **Summary**: "Route: 19 jumps, 380 ly."
//! - **White dwarfs and neutron stars** (exclusion zones, and a neutron boost if wanted):
//!   "White dwarf at jump 4." or "3 white dwarfs, first at jump 4."; the same for neutron stars.
//!
//! [`fuel`]: crate::fuel

use crate::fuel::{hop_fuel, hop_ly, scoopable};
use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_gamestate::GameState;
use astromech_journal::RouteHop;
use std::time::Duration;

fn is_scoopable(h: &RouteHop) -> bool {
    h.star_class.as_deref().is_some_and(scoopable)
}

/// The fuel line for a route, or None when the fuel reaches every scoopable star (or can't be
/// estimated).
pub fn fuel_line(hops: &[RouteHop], state: &GameState) -> Option<String> {
    let capacity = state.fuel_capacity_main?;
    let start_scoopable = is_scoopable(hops.first()?);
    let (mut start, mut tank, mut need) =
        (0, if start_scoopable { capacity } else { state.fuel_main.unwrap_or(capacity) }, 0.0);
    for k in 1..hops.len() {
        need += hop_fuel(state, &hops[k - 1], &hops[k])?;
        let last = k == hops.len() - 1;
        if !is_scoopable(&hops[k]) && !last {
            continue;
        }
        if need > tank {
            return Some(gap_line(k - start, start == 0 && !start_scoopable, !is_scoopable(&hops[k])));
        }
        (start, tank, need) = (k, capacity, 0.0);
    }
    None
}

/// `first`: the stretch from the start, on the tank's fuel. `to_end`: it ends at the destination
/// with nothing to scoop.
pub fn gap_line(jumps: usize, first: bool, to_end: bool) -> String {
    match (first, to_end) {
        (true, false) => "Fuel too low to reach a scoopable star.".into(),
        (true, true) => "Fuel too low to reach the destination.".into(),
        (false, false) => format!("Fuel gap: {jumps} jumps between scoopable stars."),
        (false, true) => format!("Fuel gap: {jumps} jumps to the destination."),
    }
}

/// "Route: 19 jumps, 380 ly." (without positions, just the jumps).
pub fn summary_line(hops: &[RouteHop]) -> String {
    let jumps = hops.len().saturating_sub(1);
    let ly: Option<f64> = hops.windows(2).map(|w| hop_ly(&w[0], &w[1])).sum();
    match ly {
        Some(ly) => format!("Route: {jumps} jumps, {ly:.0} ly."),
        None => format!("Route: {jumps} jumps."),
    }
}

/// "White dwarf at jump 4." / "3 white dwarfs, first at jump 4." for the stars `is` matches.
pub fn stars_line(hops: &[RouteHop], is: fn(&str) -> bool, one: &str, many: &str) -> Option<String> {
    let at: Vec<usize> = hops
        .iter()
        .enumerate()
        .skip(1)
        .filter(|(_, h)| h.star_class.as_deref().is_some_and(is))
        .map(|(i, _)| i)
        .collect();
    match at.as_slice() {
        [] => None,
        [k] => Some(format!("{one} at jump {k}.")),
        [k, ..] => Some(format!("{} {many}, first at jump {k}.", at.len())),
    }
}

pub fn white_dwarf(class: &str) -> bool {
    class.starts_with('D')
}

pub fn neutron(class: &str) -> bool {
    class == "N"
}

pub struct Route;

impl Rule for Route {
    fn id(&self) -> &'static str {
        "route"
    }
    /// The engine already ignores the same route written again.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, _: &Input, _: &GameState) -> Option<Callout> {
        None
    }
    fn react_all(&self, input: &Input, state: &GameState) -> Vec<Callout> {
        let Input::Route(r) = input else { return vec![] };
        let hops = &r.hops;
        if hops.len() < 2 {
            return vec![];
        }
        let line = |text: String, priority| Callout { text, priority, channels: Channels::SPEECH | Channels::TOAST };
        let mut out: Vec<Callout> = fuel_line(hops, state).map(|t| line(t, Priority::High)).into_iter().collect();
        out.push(line(summary_line(hops), Priority::Normal));
        out.extend(
            [
                stars_line(hops, white_dwarf, "White dwarf", "white dwarfs"),
                stars_line(hops, neutron, "Neutron star", "neutron stars"),
            ]
            .into_iter()
            .flatten()
            .map(|t| line(t, Priority::Normal)),
        );
        out
    }
}
