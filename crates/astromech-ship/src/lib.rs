//! # astromech-ship
//!
//! Ship stats from a hull, its fitted modules and their engineering: mass, jump range, fuel and
//! cargo, power, shields, armour, weapons, distributor and speed. Pure calculations, no I/O.
//!
//! The maths is a port of Coriolis (github.com/EDCD/coriolis, MIT): `Ship.js`, `Module.js` and
//! `Calculations.js`, with the base stats from EDCD/coriolis-data (`data/catalog/ships.json`) and
//! the blueprint effects in `astromech-engineering`'s catalog. Values match what Coriolis shows
//! for the same build, including its quirks (noted where they matter).
//!
//! Engineering comes from the journal's `Modifiers` when the module has them, as Coriolis
//! imports a `Loadout`: they are the module's real values, roll by roll, including modules
//! engineered before 3.0 (which show `Quality` 0). Without them (a planned module, or a craft
//! that didn't report them) it comes from the blueprint, grade, roll (`Quality`) and
//! experimental. Pre-engineered modules (SCO V1 drives, Expanded Capacity racks) get only their
//! journal engineering, not their factory blueprints; their `Modifiers` carry the real values
//! whatever `Quality` the journal writes for them.

pub mod data;
mod module;

use astromech_engineering::{planned_modules, Target};
use astromech_journal::ModuleSnap;
use module::{Module, Unapplied};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("unknown ship {0:?}")]
    UnknownShip(String),
    #[error("no loadout for this ship in the journal")]
    NoLoadout,
}

/// A ship's stats as fitted and once its build plan is done.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comparison {
    pub current: Side,
    pub planned: Side,
}

/// One build's stats, with the figures the Ships page shows that are worked out from them.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Side {
    #[serde(flatten)]
    pub stats: ShipStats,
    /// Seconds of fire with four pips to weapons until the weapon capacitor is empty.
    pub time_to_drain: Option<f64>,
}

impl From<ShipStats> for Side {
    fn from(stats: ShipStats) -> Self {
        Side { time_to_drain: stats.time_to_drain(4.0), stats }
    }
}

/// What the ship detail gets: both builds' stats, or the hull the calculations don't have
/// (no stats at all; the page shows the ship as incomplete).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Compared {
    Stats(Box<Comparison>),
    UnknownHull { unknown_hull: String },
}

/// Current stats of a fleet ship and its stats with `targets` done
/// ([`astromech_engineering::planned_modules`]).
pub fn compare(ship: &astromech_engineering::Ship, targets: &[Target]) -> Result<Compared, Error> {
    if ship.loadout.is_empty() {
        return Err(Error::NoLoadout);
    }
    let both = || -> Result<Comparison, Error> {
        Ok(Comparison {
            current: stats(&ship.ship_type, &ship.loadout)?.into(),
            planned: stats(&ship.ship_type, &planned_modules(&ship.loadout, targets))?.into(),
        })
    };
    match both() {
        Ok(c) => Ok(Compared::Stats(Box::new(c))),
        Err(Error::UnknownShip(s)) => Ok(Compared::UnknownHull { unknown_hull: s }),
        Err(e) => Err(e),
    }
}

/// What a ship card shows, from the current modules.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Headline {
    /// Maximum jump range, ly.
    pub jump: Option<f64>,
    /// Shield strength, MJ.
    pub shield: Option<f64>,
    pub dps: Option<f64>,
    /// What the calculations left out; `Hull: <ship>` when they don't know the hull at all.
    pub unresolved: Vec<String>,
}

/// Card stats for every fleet ship with a known loadout, by ship id.
pub fn headlines(fleet: &astromech_engineering::Fleet) -> BTreeMap<i64, Headline> {
    fleet
        .ships
        .iter()
        .filter(|s| !s.loadout.is_empty())
        .map(|s| {
            let h = match stats(&s.ship_type, &s.loadout) {
                Ok(st) => Headline {
                    jump: st.jump.as_ref().map(|j| j.max),
                    shield: st.shield.as_ref().map(|sh| sh.strength),
                    dps: Some(st.weapons.dps),
                    unresolved: st.unresolved,
                },
                Err(e) => Headline {
                    jump: None,
                    shield: None,
                    dps: None,
                    unresolved: vec![match e {
                        Error::UnknownShip(t) => format!("Hull: {t}"),
                        e => e.to_string(),
                    }],
                },
            };
            (s.id, h)
        })
        .collect()
}

/// Everything the Ships page shows for one build.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ShipStats {
    /// Hull alone, t.
    pub hull_mass: f64,
    /// Hull, modules and a full tank, t.
    pub unladen_mass: f64,
    /// Unladen plus full cargo, t.
    pub laden_mass: f64,
    pub fuel_capacity: f64,
    pub cargo_capacity: f64,
    pub passenger_capacity: f64,
    /// `None` when the frame shift drive is missing or unpowered (Coriolis shows 0).
    pub jump: Option<JumpRange>,
    /// The drive and masses route plotters work from; `None` when `jump` is.
    #[serde(skip)]
    pub fsd: Option<Fsd>,
    pub power: Power,
    /// `None` without a shield generator; strength 0 when it is switched off.
    pub shield: Option<Shield>,
    pub armour: Armour,
    pub weapons: Weapons,
    pub distributor: Option<Distributor>,
    /// Top speed with four pips to engines, m/s; `None` when the ship can't thrust.
    pub speed: Option<f64>,
    /// Boost speed, m/s; `None` when it can't boost (the engine capacitor is too small).
    pub boost: Option<f64>,
    /// Modules, blueprints, experimentals or journal modifiers not in the tables, left out of the
    /// stats.
    pub unresolved: Vec<String>,
}

/// Single-jump and total ranges, light years. "Max" is with only one jump's fuel aboard and no
/// cargo; "unladen" and "laden" are with a full tank. All include a Guardian FSD booster.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct JumpRange {
    pub max: f64,
    pub unladen: f64,
    pub laden: f64,
    /// Range on a full tank, jumping until empty.
    pub total_unladen: f64,
    pub total_laden: f64,
}

/// What a route plotter needs to know about the ship's jumps: the frame shift drive, engineering
/// included, and the masses it moves (Spansh's exact plotter takes these numbers).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Fsd {
    /// Optimal mass, t.
    pub optimal_mass: f64,
    /// The most fuel one jump can use, t.
    pub max_fuel_per_jump: f64,
    /// The drive's base fuel multiplier and power (engineering doesn't change them).
    pub fuel_multiplier: f64,
    pub fuel_power: f64,
    /// Hull and modules with the fuel reservoir, without main-tank fuel or cargo, t.
    pub base_mass: f64,
    /// Main tank, t.
    pub tank_size: f64,
    /// Fuel reservoir, t.
    pub reserve_size: f64,
    /// Guardian FSD booster, ly.
    pub range_boost: f64,
}

impl Fsd {
    /// One jump's range with a full tank and `cargo` tonnes aboard, ly (`JumpRange::laden`
    /// with a full hold).
    pub fn range(&self, cargo: f64) -> f64 {
        let mass = self.base_mass + self.tank_size + cargo.max(0.0);
        (self.max_fuel_per_jump / self.fuel_multiplier).powf(1.0 / self.fuel_power) * self.optimal_mass / mass
            + self.range_boost
    }
}

/// Power plant output against module draw, MW.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Power {
    pub generation: f64,
    /// Hardpoints retracted.
    pub retracted: f64,
    /// Hardpoints deployed.
    pub deployed: f64,
}

/// Shield strength (MJ) and resistances (fractions; negative is a weakness), boosters included,
/// with no pips to systems.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Shield {
    pub strength: f64,
    pub explosive: f64,
    pub kinetic: f64,
    pub thermal: f64,
}

/// Hull integrity and resistances (fractions), reinforcement packages included.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Armour {
    pub integrity: f64,
    pub explosive: f64,
    pub kinetic: f64,
    pub thermal: f64,
    pub caustic: f64,
    /// Module reinforcement integrity and the share of damage it absorbs.
    pub module_armour: f64,
    pub module_protection: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Weapons {
    /// Damage per second while firing.
    pub dps: f64,
    /// Damage per second counting reloads (inside every weapon's falloff range).
    pub sustained_dps: f64,
    /// Weapon capacitor draw while firing, MW.
    pub eps: f64,
    /// Weapon capacitor draw counting reloads, MW.
    pub sustained_eps: f64,
}

/// Capacity (MJ) and full recharge rate (MW, at four pips) of each capacitor.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Distributor {
    pub weapons: Capacitor,
    pub systems: Capacitor,
    pub engines: Capacitor,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Capacitor {
    pub capacity: f64,
    pub recharge: f64,
}

impl Capacitor {
    /// Recharge rate with `pips` (0–4, halves allowed) to this capacitor, MW.
    pub fn recharge_at(&self, pips: f64) -> f64 {
        self.recharge * (pips / 4.0).powf(1.1)
    }
}

impl ShipStats {
    /// Seconds of continuous fire until the weapon capacitor is empty, with `wep_pips` to weapons;
    /// `None` when the recharge keeps up.
    pub fn time_to_drain(&self, wep_pips: f64) -> Option<f64> {
        let d = self.distributor.as_ref()?;
        let drain = self.weapons.sustained_eps - d.weapons.recharge_at(wep_pips);
        (drain > 0.0).then(|| d.weapons.capacity / drain)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Standard(Standard),
    /// Weapon hardpoint (`weapon`) or utility mount.
    Hardpoint {
        weapon: bool,
    },
    Internal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Standard {
    PowerPlant,
    Thrusters,
    Fsd,
    LifeSupport,
    Distributor,
    Sensors,
    FuelTank,
}

#[derive(Debug, Clone)]
struct Slot {
    kind: Kind,
    module: Module,
    enabled: bool,
    priority: usize,
}

fn kind(slot: &str) -> Option<Kind> {
    use Standard::*;
    let s = slot.to_ascii_lowercase();
    Some(match s.as_str() {
        "powerplant" => Kind::Standard(PowerPlant),
        "mainengines" => Kind::Standard(Thrusters),
        "frameshiftdrive" => Kind::Standard(Fsd),
        "lifesupport" => Kind::Standard(LifeSupport),
        "powerdistributor" => Kind::Standard(Distributor),
        "radar" => Kind::Standard(Sensors),
        "fueltank" => Kind::Standard(FuelTank),
        _ if s.contains("hardpoint") => Kind::Hardpoint { weapon: !s.starts_with("tiny") },
        _ if s == "planetaryapproachsuite"
            || (s.starts_with("slot") && s.contains("_size"))
            || ["military", "cargo", "limpetcontroller", "fighterbay"]
                .iter()
                .any(|p| s.strip_prefix(p).is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))) =>
        {
            Kind::Internal
        }
        _ => return None,
    })
}

/// A ship assembled from a loadout, with the power bands worked out.
struct Ship {
    def: &'static data::ShipDef,
    bulkheads: Option<Module>,
    slots: Vec<Slot>,
    power_available: f64,
    /// Running total of retracted draw up to each priority band (`priorityBands[i].retractedSum`).
    retracted_sum: [f64; 5],
}

impl Ship {
    fn standard(&self, which: Standard) -> Option<&Slot> {
        self.slots.iter().find(|s| s.kind == Kind::Standard(which))
    }

    fn module(&self, which: Standard) -> Option<&Module> {
        self.standard(which).map(|s| &s.module)
    }

    fn internal(&self) -> impl Iterator<Item = &Slot> {
        self.slots.iter().filter(|s| s.kind == Kind::Internal)
    }

    fn hardpoints(&self) -> impl Iterator<Item = &Slot> {
        self.slots.iter().filter(|s| matches!(s.kind, Kind::Hardpoint { .. }))
    }

    /// Enabled modules in the ship's weapon hardpoints (not utility mounts).
    fn weapons(&self) -> impl Iterator<Item = &Module> {
        self.slots.iter().filter(|s| s.enabled && s.kind == Kind::Hardpoint { weapon: true }).map(|s| &s.module)
    }

    /// `getSlotStatus` with hardpoints retracted: is the module powered?
    fn powered(&self, slot: &Slot) -> bool {
        slot.enabled
            && !(matches!(slot.kind, Kind::Hardpoint { .. }) && !slot.module.def.passive)
            && self.retracted_sum[slot.priority] < self.power_available
    }

    fn mass_sum(&self, stat: &str, each: impl Fn(f64) -> f64) -> f64 {
        self.slots.iter().map(|s| s.module.get(stat).map(&each).unwrap_or(0.0)).sum()
    }

    /// Hull and modules, no fuel or cargo (Coriolis `dryMass`).
    fn dry_mass(&self) -> f64 {
        self.def.hull_mass
            + self.bulkheads.as_ref().and_then(|b| b.get("mass")).unwrap_or(0.0)
            + self.mass_sum("mass", |m| m)
    }

    fn jump_boost(&self) -> f64 {
        self.internal()
            .filter(|s| s.module.def.grp == "gfsb" && self.powered(s))
            .map(|s| s.module.get("jumpboost").unwrap_or(0.0))
            .sum()
    }

    /// `Calc.jumpRange`: `fuel` defaults to the most one jump can use.
    fn jump_range(&self, mass: f64, fsd: &Module, fuel: Option<f64>) -> f64 {
        let max_fuel = fsd.get("maxfuel").unwrap_or(0.0);
        let fuel = fuel.unwrap_or(max_fuel).min(max_fuel);
        let fuelmul = fsd.base("fuelmul").unwrap_or(f64::NAN);
        let fuelpower = fsd.base("fuelpower").unwrap_or(f64::NAN);
        let opt = fsd.get("optmass").unwrap_or(0.0);
        (fuel / fuelmul).powf(1.0 / fuelpower) * opt / (mass + self.def.reserve_fuel) + self.jump_boost()
    }

    /// `Calc.totalJumpRange`: jump until the tank is empty, getting lighter each time.
    fn total_jump_range(&self, mut mass: f64, fsd: &Module, fuel: f64) -> f64 {
        let max_fuel = fsd.get("maxfuel").unwrap_or(0.0);
        let (mut left, mut total) = (fuel, 0.0);
        while left > 0.0 && max_fuel > 0.0 {
            let used = left.min(max_fuel);
            total += self.jump_range(mass, fsd, Some(used));
            mass -= used;
            left -= used;
        }
        total
    }
}

/// `Calc.shieldStrength` / the thruster speed curve: a multiplier from where `mass` sits
/// between the module's minimum, optimal and maximum mass.
fn mass_curve(min_mass: f64, opt_mass: f64, max_mass: f64, min_mul: f64, opt_mul: f64, max_mul: f64, mass: f64) -> f64 {
    let xnorm = ((max_mass - mass) / (max_mass - min_mass)).min(1.0);
    let exponent = ((opt_mul - min_mul) / (max_mul - min_mul)).ln()
        / ((max_mass - opt_mass) / (max_mass - min_mass)).min(1.0).ln();
    min_mul + xnorm.powf(exponent) * (max_mul - min_mul)
}

fn curve(m: &Module, mass: f64, kind: Option<&str>) -> f64 {
    mass_curve(
        m.mass_with_related("minmass").unwrap_or(f64::NAN),
        m.get("optmass").unwrap_or(f64::NAN),
        m.mass_with_related("maxmass").unwrap_or(f64::NAN),
        m.mul("min", kind),
        m.mul("opt", kind),
        m.mul("max", kind),
        mass,
    )
}

fn diminishing(min: f64, max: f64, now: f64) -> f64 {
    min + (max - min) * (now / max)
}

/// `Calc.diminishingReturnsShields`, on damage multipliers.
fn diminishing_shields(shield_mult: f64, combined: f64) -> f64 {
    let max = shield_mult * 0.7;
    if combined < max {
        diminishing(max / 2.0, max, combined)
    } else {
        combined
    }
}

/// `Calc.diminishingReturnsArmour`, on damage multipliers.
fn diminishing_armour(mults: &[f64]) -> f64 {
    let max = mults.iter().copied().fold(0.7, f64::min);
    let combined: f64 = mults.iter().product();
    let diminished = diminishing(0.35, max, combined);
    if diminished < 0.7 {
        diminished
    } else {
        combined
    }
}

/// Builds the ship from a `Loadout`'s ship symbol and modules and works out its stats.
pub fn stats(ship: &str, modules: &[ModuleSnap]) -> Result<ShipStats, Error> {
    let def = data::ship(ship).ok_or_else(|| Error::UnknownShip(ship.to_string()))?;
    let mut unresolved = vec![];
    let mut bulkheads = None;
    let mut slots = vec![];
    // The cargo hatch draws 0.6 MW; on at priority 0 unless the loadout says otherwise.
    let mut hatch = (true, 0usize);
    for m in modules {
        let s = m.slot.to_ascii_lowercase();
        if s == "cargohatch" {
            hatch = (m.on, m.priority as usize);
            continue;
        }
        let k = if s == "armour" { None } else { kind(&m.slot) };
        if s != "armour" && k.is_none() {
            continue; // cosmetics and anything else Coriolis doesn't model
        }
        let Some(mdef) = data::module(&m.item) else {
            unresolved.push(format!("{}: {}", m.slot, m.item));
            continue;
        };
        let mut module = Module::new(mdef);
        if let Some(e) = &m.engineering {
            for u in module.engineer(&m.item, e) {
                unresolved.push(match u {
                    Unapplied::Blueprint(b) => format!("{}: blueprint {b}", m.slot),
                    Unapplied::Grade(b, g) => format!("{}: blueprint {b} grade {g}", m.slot),
                    Unapplied::Experimental(x) => format!("{}: experimental {x}", m.slot),
                    Unapplied::Modifier(l) => format!("{}: modifier {l}", m.slot),
                });
            }
        }
        match k {
            None => bulkheads = Some(module),
            Some(kind) => {
                // The fuel tank is always on, at priority 0 (JournalUtils).
                let fuel_tank = kind == Kind::Standard(Standard::FuelTank);
                slots.push(Slot {
                    kind,
                    module,
                    enabled: fuel_tank || m.on,
                    priority: if fuel_tank { 0 } else { (m.priority as usize).min(4) },
                })
            }
        }
    }

    // Power bands (updatePowerUsed): non-passive hardpoints only draw when deployed.
    let mut retracted = [0.0; 5];
    let mut deployed = [0.0; 5];
    if hatch.0 {
        retracted[hatch.1.min(4)] += 0.6;
    }
    for s in slots.iter().filter(|s| s.enabled) {
        let p = s.module.get("power").unwrap_or(0.0);
        if matches!(s.kind, Kind::Hardpoint { .. }) && !s.module.def.passive {
            deployed[s.priority] += p;
        } else {
            retracted[s.priority] += p;
        }
    }
    let mut retracted_sum = [0.0; 5];
    let (mut r, mut d) = (0.0, 0.0);
    for i in 0..5 {
        r += retracted[i];
        d += deployed[i] + retracted[i];
        retracted_sum[i] = r;
    }

    let mut ship = Ship { def, bulkheads, slots, power_available: 0.0, retracted_sum };
    ship.power_available = ship.module(Standard::PowerPlant).and_then(|m| m.get("pgen")).unwrap_or(0.0);

    // Mass and capacity (recalculateMass); cargo racks hold whole tonnes.
    let dry = ship.dry_mass();
    let fuel = ship.mass_sum("fuel", |f| f);
    let cargo = ship.mass_sum("cargo", f64::floor);
    let passengers = ship.mass_sum("passengers", |p| p);
    let unladen = dry + fuel;

    let jump = ship.standard(Standard::Fsd).filter(|s| ship.powered(s)).map(|s| {
        let fsd = &s.module;
        let max_fuel = fsd.get("maxfuel").unwrap_or(0.0);
        JumpRange {
            max: ship.jump_range(unladen - fuel + max_fuel, fsd, Some(max_fuel)),
            unladen: ship.jump_range(unladen, fsd, Some(fuel)),
            laden: ship.jump_range(unladen + cargo, fsd, Some(fuel)),
            total_unladen: ship.total_jump_range(unladen, fsd, fuel),
            total_laden: ship.total_jump_range(unladen + cargo, fsd, fuel),
        }
    });
    let fsd = jump.as_ref().and_then(|_| ship.standard(Standard::Fsd)).map(|s| Fsd {
        optimal_mass: s.module.get("optmass").unwrap_or(0.0),
        max_fuel_per_jump: s.module.get("maxfuel").unwrap_or(0.0),
        fuel_multiplier: s.module.base("fuelmul").unwrap_or(f64::NAN),
        fuel_power: s.module.base("fuelpower").unwrap_or(f64::NAN),
        base_mass: dry + def.reserve_fuel,
        tank_size: fuel,
        reserve_size: def.reserve_fuel,
        range_boost: ship.jump_boost(),
    });

    let power = Power { generation: ship.power_available, retracted: r, deployed: d };
    let shield = shield(&ship);
    let armour = armour(&ship);
    let weapons = weapons(&ship);
    let distributor = ship.module(Standard::Distributor).map(|pd| {
        let cap =
            |c: &str, r: &str| Capacitor { capacity: pd.get(c).unwrap_or(0.0), recharge: pd.get(r).unwrap_or(0.0) };
        Distributor {
            weapons: cap("wepcap", "weprate"),
            systems: cap("syscap", "sysrate"),
            engines: cap("engcap", "engrate"),
        }
    });

    // Speed (calcSpeed with four pips to engines, a full tank, the reservoir and no cargo).
    let thrusters = ship.standard(Standard::Thrusters);
    let can_thrust = thrusters
        .is_some_and(|t| ship.powered(t) && dry + fuel < t.module.mass_with_related("maxmass").unwrap_or(f64::NAN));
    let (speed, boost) = match thrusters.filter(|_| can_thrust) {
        Some(t) => {
            let mul = curve(&t.module, dry + fuel + def.reserve_fuel, Some("speed"));
            let can_boost = distributor.as_ref().is_some_and(|d| d.engines.capacity >= def.boost_energy);
            (Some(mul * def.speed), can_boost.then_some(mul * def.boost))
        }
        None => (None, None),
    };

    Ok(ShipStats {
        hull_mass: def.hull_mass,
        unladen_mass: unladen,
        laden_mass: unladen + cargo,
        fuel_capacity: fuel,
        cargo_capacity: cargo,
        passenger_capacity: passengers,
        jump,
        fsd,
        power,
        shield,
        armour,
        weapons,
        distributor,
        speed,
        boost,
        unresolved,
    })
}

/// `Calc.shieldMetrics` with no pips to systems.
fn shield(ship: &Ship) -> Option<Shield> {
    let slot = ship.internal().find(|s| matches!(s.module.def.grp.as_str(), "sg" | "bsg" | "psg"))?;
    if !slot.enabled {
        return Some(Shield { strength: 0.0, explosive: 0.0, kinetic: 0.0, thermal: 0.0 });
    }
    let sg = &slot.module;
    let (mut boost, mut expl, mut kin, mut therm) = (0.0, 1.0, 1.0, 1.0);
    for b in ship.hardpoints().filter(|s| s.enabled && s.module.def.grp == "sb").map(|s| &s.module) {
        boost += b.get("shieldboost").unwrap_or(0.0);
        expl *= 1.0 - b.get("explres").unwrap_or(0.0);
        kin *= 1.0 - b.get("kinres").unwrap_or(0.0);
        therm *= 1.0 - b.get("thermres").unwrap_or(0.0);
    }
    let addition: f64 = ship
        .internal()
        .filter(|s| s.enabled && s.module.def.grp == "gsrp")
        .map(|s| s.module.get("shieldaddition").unwrap_or(0.0))
        .sum();
    let generator = ship.def.base_shield * curve(sg, ship.def.hull_mass, None);
    let strength = generator + generator * boost + addition;
    let res = |stat: &str, boosters: f64| {
        let sg_dmg = 1.0 - sg.get(stat).unwrap_or(0.0);
        1.0 - diminishing_shields(sg_dmg, sg_dmg * boosters)
    };
    // Coriolis leaves the resistances empty when the generator gives no strength.
    if generator == 0.0 || generator.is_nan() {
        return Some(Shield { strength: 0.0, explosive: 0.0, kinetic: 0.0, thermal: 0.0 });
    }
    Some(Shield {
        strength,
        explosive: res("explres", expl),
        kinetic: res("kinres", kin),
        thermal: res("thermres", therm),
    })
}

/// `Calc.armourMetrics`.
fn armour(ship: &Ship) -> Armour {
    let base = ship.def.base_armour;
    let bh = ship.bulkheads.as_ref();
    let bh_get = |stat: &str| bh.and_then(|b| b.get(stat)).unwrap_or(0.0);
    let mut reinforcement = 0.0;
    let (mut module_armour, mut module_protection) = (0.0, 1.0);
    let mut dmg: [Vec<f64>; 4] = Default::default();
    const RES: [&str; 4] = ["explres", "kinres", "thermres", "causres"];
    for (i, r) in RES.iter().enumerate() {
        dmg[i].push(1.0 - bh_get(r));
    }
    for m in ship.internal().filter(|s| s.enabled).map(|s| &s.module) {
        match m.def.grp.as_str() {
            "hr" | "ghrp" | "mahr" => {
                reinforcement += m.get("hullreinforcement").unwrap_or(0.0);
                // An HRP's hull boost applies to the ship's base armour.
                reinforcement += base * m.mod_value("hullboost").unwrap_or(0.0) / 10000.0;
                for (i, r) in RES.iter().enumerate() {
                    dmg[i].push(1.0 - m.get(r).unwrap_or(0.0));
                }
            }
            "mrp" | "gmrp" => {
                module_armour += m.get("integrity").unwrap_or(0.0);
                module_protection *= 1.0 - m.get("protection").unwrap_or(0.0);
            }
            _ => {}
        }
    }
    let res = |i: usize| 1.0 - diminishing_armour(&dmg[i]);
    Armour {
        integrity: base + base * bh_get("hullboost") + reinforcement,
        explosive: res(0),
        kinetic: res(1),
        thermal: res(2),
        caustic: res(3),
        module_armour,
        module_protection: 1.0 - module_protection,
    }
}

/// `recalculateDps` / `recalculateEps`, the Offence panel's sustained total, and the draw
/// `timeToDrainWep` uses.
fn weapons(ship: &Ship) -> Weapons {
    let (mut dps, mut sdps, mut eps, mut seps) = (0.0, 0.0, 0.0, 0.0);
    for w in ship.weapons() {
        let d = w.dps();
        if d != 0.0 && !d.is_nan() {
            dps += d;
            seps += w.sustained_eps();
        }
        let e = w.eps();
        if e != 0.0 && !e.is_nan() {
            eps += e;
        }
        let s = w.sustained_dps();
        if s != 0.0 && !s.is_nan() {
            sdps += s;
        }
    }
    Weapons { dps, sustained_dps: sdps, eps, sustained_eps: seps }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_journal::ModEngineering;

    fn snap(slot: &str, item: &str) -> ModuleSnap {
        ModuleSnap { slot: slot.into(), item: item.into(), on: true, priority: 0, engineering: None }
    }

    #[test]
    fn unknown_ship_is_an_error() {
        assert_eq!(stats("not_a_ship", &[]), Err(Error::UnknownShip("not_a_ship".into())));
    }

    #[test]
    fn unknown_modules_and_blueprints_are_reported_not_counted() {
        let mut fsd = snap("FrameShiftDrive", "int_hyperdrive_size5_class5");
        fsd.engineering = Some(ModEngineering {
            blueprint: "FSD_Imaginary".into(),
            level: 5,
            quality: 1.0,
            experimental: Some("special_imaginary".into()),
            experimental_localised: None,
            engineer: None,
            modifiers: vec![],
        });
        let mut pp = snap("PowerPlant", "int_powerplant_size7_class5");
        pp.engineering = Some(ModEngineering {
            blueprint: "PowerPlant_Boosted".into(),
            level: 0,
            quality: 1.0,
            experimental: None,
            experimental_localised: None,
            engineer: None,
            modifiers: vec![],
        });
        let modules = [
            pp,
            fsd,
            snap("Slot01_Size6", "int_not_a_module"),
            snap("PaintJob", "paintjob_krait_mkii_militaire_earth_yellow"),
        ];
        let s = stats("krait_mkii", &modules).unwrap();
        assert_eq!(
            s.unresolved,
            [
                "PowerPlant: blueprint PowerPlant_Boosted grade 0",
                "FrameShiftDrive: blueprint FSD_Imaginary",
                "FrameShiftDrive: experimental special_imaginary",
                "Slot01_Size6: int_not_a_module"
            ]
        );
        // 320 t hull + 40 t power plant + 20 t drive; the unknown module and the paint add nothing.
        assert_eq!(s.unladen_mass, 380.0);
        assert!(s.jump.is_some());
    }

    #[test]
    fn unpowered_drive_has_no_range() {
        let mut fsd = snap("FrameShiftDrive", "int_hyperdrive_size5_class5");
        fsd.on = false;
        let s = stats("krait_mkii", &[snap("PowerPlant", "int_powerplant_size7_class5"), fsd]).unwrap();
        assert_eq!(s.jump, None);
    }

    #[test]
    fn capacitor_recharge_follows_the_pip_curve() {
        let c = Capacitor { capacity: 50.0, recharge: 8.0 };
        assert_eq!(c.recharge_at(4.0), 8.0);
        assert!((c.recharge_at(2.0) - 8.0 * 0.5f64.powf(1.1)).abs() < 1e-12);
        assert_eq!(c.recharge_at(0.0), 0.0);
    }

    fn fleet(loadouts: &[&str]) -> astromech_engineering::Fleet {
        let mut b = astromech_engineering::FleetBuilder::default();
        for (i, l) in loadouts.iter().enumerate() {
            let line = format!(
                r#"{{"timestamp":"2026-10-16T12:00:0{i}Z","event":"Loadout","Ship":"{l}","ShipID":{i},"Modules":[{{"Slot":"PowerPlant","Item":"Int_Powerplant_Size7_Class5"}},{{"Slot":"FrameShiftDrive","Item":"Int_Hyperdrive_Size5_Class5"}}]}}"#
            );
            let l = astromech_journal::parse_line(&line).unwrap();
            b.apply(&l.timestamp, &l.event);
        }
        b.build("test")
    }

    #[test]
    fn unknown_hull_is_incomplete_not_an_error() {
        let f = fleet(&["krait_mkii", "mediumtransport01"]);
        let h = headlines(&f);
        assert_eq!(h.len(), 2);
        let krait = &h[&0];
        assert!(krait.unresolved.is_empty() && krait.jump.is_some() && krait.shield.is_none());
        assert_eq!(krait.dps, Some(0.0));
        let unknown = &h[&1];
        assert_eq!((unknown.jump, unknown.shield, unknown.dps), (None, None, None));
        assert_eq!(unknown.unresolved, ["Hull: mediumtransport01"]);
        let ship = f.ships.iter().find(|s| s.id == 1).unwrap();
        assert_eq!(compare(ship, &[]), Ok(Compared::UnknownHull { unknown_hull: "mediumtransport01".into() }));
        assert_eq!(
            serde_json::to_value(compare(ship, &[]).unwrap()).unwrap(),
            serde_json::json!({ "unknown_hull": "mediumtransport01" })
        );
    }
}
