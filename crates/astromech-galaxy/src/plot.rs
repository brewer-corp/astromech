//! Plotting a route for the route follower (docs/PLOTTING.md). [`plot_route`] plots a neutron
//! route on this PC (`crate::neutron_plot`) when the neutron star file is there; otherwise, and
//! for exact routes, Spansh's neutron and exact plotters do it ([`plot`], a job polled until done
//! through astromech-data). The jump range and drive come from the current ship's loadout
//! (astromech-ship), and the result is a list of stops the core's route follower takes as it is
//! (`route_set`).
//!
//! A neutron route's stops are the neutron stars to boost from and the destination; the jumps
//! between them are left to the galaxy map. An exact route has every jump as a stop, and says
//! where to refuel.
//!
//! What each plotter says about a stop and the leg to it ([`Stop`]), and nothing more:
//!
//! - Neutron routes, on this PC or by Spansh: each leg's jump count and distance, and whether a
//!   stop is a neutron star. The systems jumped through on a leg are not listed (`hops` is
//!   absent), and no star class is known.
//! - Exact routes (Spansh): every jump is a stop, so each leg is the one jump to its stop, listed
//!   in `hops` with Spansh's scoopable and neutron flags. Spansh gives no star class, and flags
//!   both ends of a route not scoopable whatever their stars, so the ends have no scoopable flag.
//!
//! A plotted route keeps what it was plotted with ([`Settings`]) so it can be plotted again from
//! where the commander is (`route_replot`).

use astromech_data::spansh::{ExactRoute, NeutronRoute, Polling};
use astromech_data::Client;
use serde::{Deserialize, Serialize};

/// Which Spansh plotter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
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

/// Who works a neutron route out: this PC, from the neutron star file, or Spansh. Exact routes are
/// always Spansh's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
#[serde(rename_all = "lowercase")]
pub enum Engine {
    Local,
    Spansh,
}

/// Neutron routes: how far Spansh may stray to use neutron stars (its default).
pub const DEFAULT_EFFICIENCY: u8 = 60;

fn default_efficiency() -> u8 {
    DEFAULT_EFFICIENCY
}

fn yes() -> bool {
    true
}

/// A route to plot from the commander's system, as the window's top bar asks for it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "PlotAsk")]
pub struct Ask {
    pub plotter: Plotter,
    pub to: String,
    /// Neutron routes, 1–100.
    #[serde(default = "default_efficiency")]
    pub efficiency: u8,
    /// Exact routes: fill up at every scoopable star, not only when the tank needs it.
    #[serde(default = "yes")]
    pub refuel_every_scoopable: bool,
    /// Neutron routes: who works it out. None: this PC when the neutron star file is there,
    /// else Spansh. `Spansh` always asks Spansh; `Local` is as None (with no file, Spansh plots
    /// it and the reply says why).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub engine: Option<Engine>,
    /// Neutron routes: plot for this jump range (ly) instead of the current ship's, as a replot
    /// does with the range the route was plotted at. Exact routes take the ship's drive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub range: Option<f64>,
}

/// What a route was plotted with, kept with the route so `route_replot` can plot it again from
/// where the commander is now.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "PlotSettings")]
pub struct Settings {
    /// The destination as asked for.
    pub to: String,
    pub plotter: Plotter,
    /// Who worked it out (a neutron route Spansh plotted because the file couldn't be read is
    /// `spansh`).
    pub engine: Engine,
    pub efficiency: u8,
    pub refuel_every_scoopable: bool,
    /// The jump range it was plotted with, ly.
    pub range: f64,
}

impl Settings {
    /// The ask that plots the route again with these settings.
    pub fn ask(&self) -> Ask {
        Ask {
            plotter: self.plotter,
            to: self.to.clone(),
            efficiency: self.efficiency,
            refuel_every_scoopable: self.refuel_every_scoopable,
            engine: Some(self.engine),
            range: Some(self.range),
        }
    }
}

/// The ship a route is plotted for.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "PlotterShip")]
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

/// A system on a route as a source describes it: a stop, or a jump of a leg. Each field is
/// present only when the source says it; nothing is guessed from another.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ts_rs::TS)]
#[ts(rename = "PlotHop", optional_fields)]
pub struct Hop {
    pub system: String,
    /// `StarClass` as `NavRoute.json` writes it ("K", "DA", "N", "M_RedGiant").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub star_class: Option<String>,
    /// The plotter's flag that its star can be fuel-scooped (Spansh's exact plotter).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scoopable: Option<bool>,
    /// The plotter's flag that its star is a neutron star.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neutron: Option<bool>,
}

/// A stop on a plotted route (the core's `Waypoint`, which reads the same fields).
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
#[ts(rename = "PlotStop")]
pub struct Stop {
    pub system: String,
    /// "neutron boost", "fuel", or both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub label: Option<String>,
    /// Jumps from the stop before (0 for the first): the route strip's "N jumps left".
    pub jumps: u32,
    /// Light years from the stop before (0 for the first).
    pub ly: f64,
    /// Whether the stop is a neutron star, where the plotter says (not for the start of a Spansh
    /// route, which Spansh doesn't describe).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub neutron: Option<bool>,
    /// Exact routes: whether Spansh says its star can be scooped.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub scoopable: Option<bool>,
    /// The leg's jumps, the last being this stop, where the plotter lists them (exact routes:
    /// the one jump). Absent on a neutron route, whose legs have only a count.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub hops: Option<Vec<Hop>>,
}

/// Labels: what to do at a stop besides plotting on. "Next waypoint. X. For fuel."
pub const NEUTRON: &str = "neutron boost";
pub const FUEL: &str = "fuel";

fn stop(system: &str, jumps: u32, ly: f64, neutron: bool, refuel: bool) -> Stop {
    let label = match (neutron, refuel) {
        (true, true) => Some(format!("{NEUTRON} and {FUEL}")),
        (true, false) => Some(NEUTRON.to_string()),
        (false, true) => Some(FUEL.to_string()),
        (false, false) => None,
    };
    Stop { system: system.to_string(), label, jumps, ly, neutron: None, scoopable: None, hops: None }
}

/// A plotted route: what the core's `route_set` takes (`name`, `source`, `waypoints`), and its
/// totals for the page.
#[derive(Debug, Clone, PartialEq, Serialize, ts_rs::TS)]
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
    /// Worked out on this PC by the local neutron plotter (`crate::neutron_plot`), not by Spansh.
    pub local: bool,
    /// Local routes: whether the search proved no route has fewer jumps; false for a long route
    /// it found near the fewest ([`crate::neutron_plot::Route::fewest`]). `None` from Spansh.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub fewest: Option<bool>,
    /// Why Spansh plotted a neutron route although the neutron star file is there: it can't be
    /// read.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub fallback: Option<String>,
    /// What it was plotted with, for `route_replot`; the core keeps it with the route.
    pub plot: Settings,
}

impl Plotted {
    /// Record what `ask` plotted this with (the range is the one plotted with, the engine the one
    /// that worked it out).
    fn asked(mut self, ask: &Ask) -> Self {
        self.plot = Settings {
            to: ask.to.trim().to_string(),
            plotter: ask.plotter,
            engine: if self.local { Engine::Local } else { Engine::Spansh },
            efficiency: ask.efficiency.clamp(1, 100),
            refuel_every_scoopable: ask.refuel_every_scoopable,
            range: self.range,
        };
        self
    }
}

/// Settings for a route before [`Plotted::asked`] fills them in.
pub(crate) fn unset(plotter: Plotter, range: f64) -> Settings {
    Settings {
        to: String::new(),
        plotter,
        engine: Engine::Spansh,
        efficiency: DEFAULT_EFFICIENCY,
        refuel_every_scoopable: true,
        range,
    }
}

/// Plots `ask` for `ship` (blocks: run it off the UI thread). A neutron route is plotted on this
/// PC when the neutron star file in `neutron` is there and readable, else by Spansh, and a file
/// that can't be read is said in [`Plotted::fallback`]. Exact routes always come from Spansh.
/// The route starts at `here`, the commander's system. The local plotter finds the ends with
/// `locate` ([`crate::Galaxy::locate`]: the local star map, then Spansh's system names), so it
/// needs the network only for a system the star map lacks. Errors are sentences for the page.
pub fn plot_route(
    client: &Client,
    neutron: &crate::neutron::Kept,
    locate: impl Fn(&str) -> anyhow::Result<crate::Here>,
    ask: &Ask,
    here: Option<&crate::Here>,
    ship: &Ship,
    polling: Polling,
) -> Result<Plotted, String> {
    let here = here.ok_or(NOT_HERE)?;
    if ask.to.trim().is_empty() {
        return Err("Enter where the route ends.".into());
    }
    let mut ship = ship.clone();
    if let (Plotter::Neutron, Some(range)) = (ask.plotter, ask.range) {
        ship.range = range;
    }
    let ship = &ship;
    if ask.plotter == Plotter::Exact || ask.engine == Some(Engine::Spansh) {
        return plot(client, &here.system, ask, ship, polling);
    }
    let file = match neutron.get(&**client.clock_in_use()) {
        Ok(Some(file)) => file,
        Ok(None) => return plot(client, &here.system, ask, ship, polling),
        Err(e) => {
            let mut p = plot(client, &here.system, ask, ship, polling)?;
            p.fallback = Some(format!("{e:#}"));
            return Ok(p);
        }
    };
    let to = locate(ask.to.trim()).map_err(|e| format!("{e:#}"))?;
    crate::neutron_plot::plot(&file, here, &to, ship).map(|p| p.asked(ask))
}

/// Routes start where the commander is; before the journal says where that is, there is no start.
pub const NOT_HERE: &str = "astromech doesn't know your current system yet. Log in to Elite, then plot again.";

/// Plot `ask` from `from` for `ship` with Spansh and wait for it (blocks: run it off the UI
/// thread; gives up after `polling.timeout`). Errors are sentences for the page.
pub fn plot(client: &Client, from: &str, ask: &Ask, ship: &Ship, polling: Polling) -> Result<Plotted, String> {
    let (from, to) = (from.trim().to_string(), ask.to.trim().to_string());
    if from.is_empty() {
        return Err(NOT_HERE.into());
    }
    if to.is_empty() {
        return Err("Enter where the route ends.".into());
    }
    spansh(client, ask, from, to, ship, polling).map(|p| p.asked(ask)).map_err(|e| e.to_string())
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
            let waypoints: Vec<Stop> = plan
                .system_jumps
                .iter()
                .enumerate()
                .map(|(i, j)| Stop {
                    neutron: (i > 0).then_some(j.neutron_star),
                    ..stop(&j.system, j.jumps, j.distance_jumped, j.neutron_star, false)
                })
                .collect();
            Ok(Plotted {
                name,
                source: plotter.source(),
                neutrons: plan.system_jumps.iter().filter(|j| j.neutron_star).count(),
                refuels: 0,
                waypoints,
                // Spansh's `total_jumps` counts legs; each waypoint carries its leg's jumps.
                jumps: plan.system_jumps.iter().map(|j| j.jumps).sum(),
                ly: plan.distance,
                range: r.range,
                local: false,
                fewest: None,
                fallback: None,
                plot: unset(plotter, r.range),
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
                // Every stop is one jump on from the one before; the first is where it starts.
                waypoints: plan
                    .jumps
                    .iter()
                    .enumerate()
                    .map(|(i, j)| {
                        let s = stop(&j.name, u32::from(i > 0), j.distance, j.has_neutron, j.must_refuel);
                        if i == 0 {
                            return s;
                        }
                        // Spansh flags both ends not scoopable, whatever the star (Sol, a G star,
                        // and Maia, a B star, on the recorded route), so the destination's flag
                        // isn't kept.
                        let last = i + 1 == plan.jumps.len();
                        let hop = Hop {
                            system: j.name.clone(),
                            star_class: None,
                            scoopable: (!last).then_some(j.is_scoopable),
                            neutron: Some(j.has_neutron),
                        };
                        Stop { neutron: hop.neutron, scoopable: hop.scoopable, hops: Some(vec![hop]), ..s }
                    })
                    .collect(),
                jumps: plan.jumps.len().saturating_sub(1) as u32,
                ly: plan.jumps.iter().map(|j| j.distance).sum(),
                neutrons: plan.jumps.iter().filter(|j| j.has_neutron).count(),
                refuels: plan.jumps.iter().filter(|j| j.must_refuel).count(),
                range: ship.range,
                local: false,
                fewest: None,
                fallback: None,
                plot: unset(plotter, ship.range),
            })
        }
    }
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
