//! Plotting a route with Spansh's neutron and exact plotters for the route follower
//! (docs/PLOTTING.md): the jump range and drive come from the current ship's loadout
//! (astromech-ship), the route from Spansh (astromech-data, a job polled until done), and the
//! result is a list of stops the core's route follower takes as it is (`route_set`).
//!
//! A neutron route's stops are the neutron stars to boost from and the destination; the jumps
//! between them are left to the galaxy map. An exact route has every jump as a stop, and says
//! where to refuel.

use astromech_data::spansh::{ExactRoute, NeutronRoute, Polling};
use astromech_data::Client;
use serde::{Deserialize, Serialize};

/// Which Spansh plotter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Plotter {
    /// Boosts from neutron stars on the way (`/api/route`).
    Neutron,
    /// Every jump, with refuel stops, for this ship's drive and tank (`/api/generic/route`).
    Exact,
}

impl Plotter {
    /// The route's `source` on the Route page.
    pub fn source(self) -> &'static str {
        match self {
            Plotter::Neutron => "neutron plotter",
            Plotter::Exact => "exact plotter",
        }
    }
}

/// Neutron routes: how far Spansh may stray to use neutron stars (its default).
pub const DEFAULT_EFFICIENCY: u8 = 60;

fn default_efficiency() -> u8 {
    DEFAULT_EFFICIENCY
}

fn yes() -> bool {
    true
}

/// A route to plot, as the Route page asks for it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ask {
    pub plotter: Plotter,
    pub from: String,
    pub to: String,
    /// Neutron routes, 1–100.
    #[serde(default = "default_efficiency")]
    pub efficiency: u8,
    /// Exact routes: fill up at every scoopable star, not only when the tank needs it.
    #[serde(default = "yes")]
    pub refuel_every_scoopable: bool,
}

/// The ship a route is plotted for.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Ship {
    /// "Asp Explorer", or its name if it has one.
    pub name: String,
    pub fsd: astromech_ship::Fsd,
    /// Tonnes aboard now.
    pub cargo: f64,
    /// One jump with a full tank and this cargo, ly.
    pub range: f64,
    /// Modules the calculations left out: the range may be off.
    pub unresolved: Vec<String>,
}

impl Ship {
    /// The fleet's current ship, from its last `Loadout`, carrying `cargo` tonnes.
    pub fn current(fleet: &astromech_engineering::Fleet, cargo: f64) -> Result<Ship, String> {
        let s = fleet.ships.iter().find(|s| s.current).ok_or("No ship in the journal yet. Board one in game.")?;
        if s.loadout.is_empty() {
            return Err(format!("No loadout for your {} in the journal yet. Fly it once.", s.type_name));
        }
        let stats = astromech_ship::stats(&s.ship_type, &s.loadout).map_err(|e| match e {
            astromech_ship::Error::UnknownShip(_) => {
                format!("astromech can't work out the jump range of a {}.", s.type_name)
            }
            e => e.to_string(),
        })?;
        let fsd = stats.fsd.ok_or_else(|| format!("Your {} has no powered frame shift drive.", s.type_name))?;
        let cargo = cargo.max(0.0);
        Ok(Ship {
            name: s.name.clone().filter(|n| !n.trim().is_empty()).unwrap_or_else(|| s.type_name.clone()),
            range: fsd.range(cargo),
            fsd,
            cargo,
            unresolved: stats.unresolved,
        })
    }
}

/// A stop on a plotted route (the core's `Waypoint`).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Stop {
    pub system: String,
    /// "neutron boost", "fuel", or both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// Labels: what to do at a stop besides plotting on. "Next waypoint. X. For fuel."
pub const NEUTRON: &str = "neutron boost";
pub const FUEL: &str = "fuel";

fn stop(system: &str, neutron: bool, refuel: bool) -> Stop {
    let label = match (neutron, refuel) {
        (true, true) => Some(format!("{NEUTRON} and {FUEL}")),
        (true, false) => Some(NEUTRON.to_string()),
        (false, true) => Some(FUEL.to_string()),
        (false, false) => None,
    };
    Stop { system: system.to_string(), label }
}

/// A plotted route: what the core's `route_set` takes (`name`, `source`, `waypoints`), and its
/// totals for the page.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Plotted {
    pub name: String,
    pub source: &'static str,
    pub waypoints: Vec<Stop>,
    /// Jumps in all (a neutron route has more jumps than stops).
    pub jumps: u32,
    pub ly: f64,
    pub neutrons: usize,
    pub refuels: usize,
    /// The range it was plotted with, ly.
    pub range: f64,
}

/// Plot `ask` for `ship` and wait for Spansh (blocks: run it off the UI thread; gives up after
/// `polling.timeout`). Errors are sentences for the page.
pub fn plot(client: &Client, ask: &Ask, ship: &Ship, polling: Polling) -> Result<Plotted, String> {
    let (from, to) = (ask.from.trim().to_string(), ask.to.trim().to_string());
    if from.is_empty() || to.is_empty() {
        return Err("Enter where the route starts and where it ends.".into());
    }
    spansh(client, ask, from, to, ship, polling).map_err(|e| e.to_string())
}

fn spansh(
    client: &Client,
    ask: &Ask,
    from: String,
    to: String,
    ship: &Ship,
    polling: Polling,
) -> astromech_data::Result<Plotted> {
    let plotter = ask.plotter;
    let name = format!("{from} to {to}");
    match plotter {
        Plotter::Neutron => {
            let r = NeutronRoute {
                from,
                to,
                range: round2(ship.range),
                efficiency: ask.efficiency.clamp(1, 100),
                supercharge_multiplier: ship.fsd.supercharge_multiplier,
            };
            let plan = client.spansh_route_wait(&r, polling)?;
            let waypoints: Vec<Stop> =
                plan.system_jumps.iter().map(|j| stop(&j.system, j.neutron_star, false)).collect();
            Ok(Plotted {
                name,
                source: plotter.source(),
                neutrons: plan.system_jumps.iter().filter(|j| j.neutron_star).count(),
                refuels: 0,
                waypoints,
                jumps: plan.total_jumps,
                ly: plan.distance,
                range: r.range,
            })
        }
        Plotter::Exact => {
            let f = &ship.fsd;
            let r = ExactRoute {
                from,
                to,
                optimal_mass: f.optimal_mass,
                max_fuel_per_jump: f.max_fuel_per_jump,
                fuel_multiplier: f.fuel_multiplier,
                fuel_power: f.fuel_power,
                base_mass: round2(f.base_mass),
                tank_size: f.tank_size,
                internal_tank_size: f.reserve_size,
                range_boost: f.range_boost,
                supercharge_multiplier: f.supercharge_multiplier,
                cargo: ship.cargo.round(),
                refuel_every_scoopable: ask.refuel_every_scoopable,
            };
            let plan = client.spansh_exact_route_wait(&r, polling)?;
            Ok(Plotted {
                name,
                source: plotter.source(),
                waypoints: plan.jumps.iter().map(|j| stop(&j.name, j.has_neutron, j.must_refuel)).collect(),
                jumps: plan.jumps.len().saturating_sub(1) as u32,
                ly: plan.jumps.iter().map(|j| j.distance).sum(),
                neutrons: plan.jumps.iter().filter(|j| j.has_neutron).count(),
                refuels: plan.jumps.iter().filter(|j| j.must_refuel).count(),
                range: ship.range,
            })
        }
    }
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
