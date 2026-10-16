//! One fitted module and its engineered stats: a port of Coriolis's `Module.js` value rules and
//! of how a journal `Engineering` block becomes modifications (`JournalUtils.js`,
//! `BlueprintFunctions.js`, and the build-code round trip in `Ship.js`).
//!
//! Coriolis keeps each modification as a scaled integer: hundredths of a percent for percentage
//! stats (`0.1234` → `1234`) and hundredths for numeric ones. Missing values follow JavaScript:
//! `None` is `null` or `undefined`, and arithmetic treats it as zero where Coriolis's code does.

use crate::data::ModuleDef;
use astromech_engineering::data as eng;
use astromech_journal::ModEngineering;
use std::collections::{BTreeMap, HashMap};

const RESISTANCES: [&str; 4] = ["explres", "kinres", "thermres", "causres"];

/// JavaScript's `Math.round`: halves round up.
pub(crate) fn js_round(x: f64) -> f64 {
    (x + 0.5).floor()
}

/// JavaScript truthiness of a number: `null`, `0` and `NaN` are false.
fn truthy(x: Option<f64>) -> Option<f64> {
    x.filter(|v| *v != 0.0 && !v.is_nan())
}

fn or(x: Option<f64>, default: f64) -> f64 {
    truthy(x).unwrap_or(default)
}

#[derive(Debug, Clone)]
pub(crate) struct Module {
    pub def: &'static ModuleDef,
    /// Stored modifications, by stat (Coriolis `mods`).
    mods: HashMap<String, f64>,
    /// Experimental id (Coriolis `blueprint.special.edname`).
    special: Option<String>,
    effects: Option<&'static BTreeMap<String, f64>>,
}

/// Why a module's engineering could not be applied.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Unapplied {
    Blueprint(String),
    Experimental(String),
}

impl Module {
    pub fn new(def: &'static ModuleDef) -> Self {
        Module { def, mods: HashMap::new(), special: None, effects: None }
    }

    /// Applies a journal `Engineering` block the way Coriolis imports it when the event carries no
    /// `Modifiers`: every blueprint feature at the roll given by `quality`, the experimental's
    /// effects on top. Returns what could not be found.
    pub fn engineer(&mut self, symbol: &str, e: &ModEngineering) -> Vec<Unapplied> {
        let mut missing = vec![];
        // Multi-cannons write their overcharged blueprint as Weapon_Overcharged (getBlueprint).
        let bp_name = if e.blueprint.eq_ignore_ascii_case("weapon_overcharged")
            && symbol.to_ascii_lowercase().contains("multicannon")
        {
            "MC_Overcharged"
        } else {
            &e.blueprint
        };
        match eng::blueprint(bp_name) {
            Some((_, bp)) => {
                if let Some(grade) = bp.grade(e.level) {
                    for (stat, range) in &grade.features {
                        self.set_quality(stat, *range, e.quality);
                    }
                }
            }
            None => missing.push(Unapplied::Blueprint(e.blueprint.clone())),
        }
        if let Some(sp) = &e.experimental {
            let sp = sp.to_ascii_lowercase();
            // _addModifications: plasma slug has a variant per weapon.
            let sp = if sp == "special_plasma_slug" {
                if symbol.to_ascii_lowercase().contains("plasmaaccelerator") {
                    "special_plasma_slug_pa".to_string()
                } else {
                    "special_plasma_slug_cooled".to_string()
                }
            } else {
                sp
            };
            let effects = eng::effects(&sp);
            if effects.is_some() || eng::experimental(&sp).is_some() {
                self.effects = effects;
                self.special = Some(sp);
            } else {
                missing.push(Unapplied::Experimental(sp));
            }
        }
        self.round_trip();
        missing
    }

    /// `setQualityCB` then `setModValue(name, value, false)`.
    fn set_quality(&mut self, stat: &str, [lo, hi]: [f64; 2], quality: f64) {
        let Some(m) = eng::modifier(stat) else { return };
        let drawback = lo < 0.0 || (lo == 0.0 && hi < 0.0);
        let value = match (m.higherbetter, drawback) {
            (true, true) | (false, false) => hi + (lo - hi) * quality,
            (true, false) | (false, true) => lo + (hi - lo) * quality,
        };
        let mut value = value * scale(&m.kind);
        // Resistance modifiers scale with the base value.
        if RESISTANCES.contains(&stat) {
            value *= 1.0 - self.base(stat).unwrap_or(0.0);
        }
        if value == 0.0 {
            self.mods.remove(stat);
        } else {
            self.mods.insert(stat.to_string(), value);
        }
    }

    /// Coriolis stores the build as a code and rebuilds the ship from it before showing any
    /// stat: only the group's valid modifications survive, each rounded to an integer.
    fn round_trip(&mut self) {
        let valid = eng::group(&self.def.grp).map(|g| g.modifications.as_slice()).unwrap_or(&[]);
        let mods = std::mem::take(&mut self.mods);
        self.mods = mods
            .into_iter()
            .filter(|(k, v)| valid.iter().any(|s| s == k) && *v != 0.0)
            .map(|(k, v)| (k, js_round(v)))
            .collect();
    }

    /// The unmodified stat (`this[name]`); a rate of fire comes from the fire interval.
    pub fn base(&self, stat: &str) -> Option<f64> {
        if stat == "rof" {
            if let Some(r) = self.def.stats.get("rof") {
                return Some(*r);
            }
            return self.def.stats.get("fireint").map(|i| 1.0 / i);
        }
        self.def.stats.get(stat).copied()
    }

    /// `getModValue(name)`: the stored modification plus the experimental's effect, rounded.
    pub fn mod_value(&self, stat: &str) -> Option<f64> {
        let mut result = truthy(self.mods.get(stat).copied());
        if let Some(action) = self.effects.and_then(|e| e.get(stat)) {
            if let Some(m) = eng::modifier(stat) {
                let mult = if m.kind == "percentage" { 10000.0 } else { 100.0 };
                let r = result.unwrap_or(0.0);
                result = Some(if RESISTANCES.contains(&stat) {
                    let base = self.def.stats.get(stat).copied().unwrap_or(f64::NAN);
                    r + action * (1.0 - (base + r / mult)) * 100.0
                } else if m.method == "additive" {
                    r + action * 100.0
                } else if m.method == "overwrite" {
                    *action
                } else {
                    ((1.0 + r / mult) * (1.0 + action) - 1.0) * mult
                });
            }
        }
        result.map(js_round)
    }

    /// `get(name)`: the engineered stat, `None` when the module has none.
    pub fn get(&self, stat: &str) -> Option<f64> {
        let base = self.base(stat);
        let Some(m) = eng::modifier(stat) else { return base.filter(|v| !v.is_nan()) };
        let mut mv = self.mod_value(stat).unwrap_or(0.0) / scale(&m.kind);
        let mut result = base;
        if mv != 0.0 && !mv.is_nan() {
            if truthy(result).is_none() && m.method == "additive" {
                result = Some(0.0);
            }
            result = match result {
                Some(r) => Some(if m.method == "additive" {
                    r + mv
                } else if m.method == "overwrite" {
                    mv
                } else if stat == "shieldboost" || stat == "hullboost" {
                    (1.0 + r) * (1.0 + mv) - 1.0
                } else {
                    // Rate of fire modifiers apply to the fire interval.
                    if stat == "rof" {
                        mv = 1.0 / (1.0 + mv) - 1.0;
                    }
                    r * (1.0 + mv)
                }),
                None if stat == "burst" || stat == "burstrof" => Some(mv),
                None => None,
            };
        }
        result.filter(|v| !v.is_nan())
    }

    /// `getMinMass` / `getMaxMass`: without its own modification, the optimal-mass modification
    /// carries over (for shield generators: always to the minimum, to the maximum only if
    /// positive).
    pub fn mass_with_related(&self, stat: &str) -> Option<f64> {
        let base = self.base(stat)?;
        let mut m = self.mod_value(stat).filter(|v| !v.is_nan());
        if m.is_none() {
            let opt = self.mod_value("optmass");
            m = if matches!(self.def.grp.as_str(), "sg" | "bsg" | "psg") {
                opt.filter(|o| stat == "minmass" || *o > 0.0)
            } else {
                opt
            };
        }
        Some(match m {
            Some(m) if !m.is_nan() => base * (1.0 + m / 10000.0),
            _ => base,
        })
    }

    /// `getMinMul` / `getOptMul` / `getMaxMul`, `which` being `min`, `opt` or `max` and `kind`
    /// `speed` or none.
    pub fn mul(&self, which: &str, kind: Option<&str>) -> f64 {
        let typed = kind.and_then(|k| truthy(self.base(&format!("{which}mul{k}"))));
        let result = typed.or_else(|| truthy(self.base(&format!("{which}mul")))).unwrap_or(0.0);
        if result == 0.0 {
            return 0.0;
        }
        match truthy(self.mod_value("optmul").map(|m| m / 10000.0)) {
            Some(mult) => result * (1.0 + mult),
            None => result,
        }
    }

    pub fn is_special(&self, id: &str) -> bool {
        self.special.as_deref() == Some(id)
    }

    /// `getRoF`: shots per second, counting bursts and charge time.
    pub fn rof(&self) -> f64 {
        let burst = or(self.get("burst"), 1.0);
        let burst_rof = or(self.get("burstrof"), 1.0);
        let int_rof = self.get("rof").unwrap_or(0.0);
        let charge = or(self.get("charge"), 0.0);
        burst / ((burst - 1.0) / burst_rof + 1.0 / int_rof + charge)
    }

    /// `getClip`: always rounded up.
    pub fn clip(&self) -> Option<f64> {
        truthy(self.get("clip")).map(f64::ceil)
    }

    pub fn dps(&self) -> f64 {
        let damage = self.get("damage").unwrap_or(0.0);
        let per_shot = or(self.base("roundspershot"), 1.0);
        damage * per_shot * or(Some(self.rof()), 1.0)
    }

    pub fn eps(&self) -> f64 {
        self.get("distdraw").unwrap_or(0.0) * or(Some(self.rof()), 1.0)
    }

    /// `getSustainedFactor`: sustained over burst rate of fire, counting reloads.
    pub fn sustained_factor(&self) -> f64 {
        let Some(mut clip) = self.clip() else { return 1.0 };
        // An auto-loader gives one reload for every two shots fired.
        if self.is_special("special_auto_loader") {
            clip += clip - 1.0;
        }
        let burst = or(self.get("burst"), 1.0);
        let rof = self.rof();
        let overhead = (burst - 1.0) / or(self.get("burstrof"), 1.0);
        let srof = clip / ((clip - burst) / rof + overhead + self.get("reload").unwrap_or(0.0));
        srof / rof
    }

    pub fn sustained_dps(&self) -> f64 {
        self.dps() * self.sustained_factor()
    }

    /// Distributor draw per second over a reload cycle (`timeToDrainWep`).
    pub fn sustained_eps(&self) -> f64 {
        match self.clip() {
            Some(clip) => {
                let rof = self.rof();
                (clip * self.eps() / rof) / (clip / rof + self.get("reload").unwrap_or(0.0))
            }
            None => self.eps(),
        }
    }
}

fn scale(kind: &str) -> f64 {
    match kind {
        "percentage" => 10000.0,
        "numeric" => 100.0,
        _ => 1.0,
    }
}
