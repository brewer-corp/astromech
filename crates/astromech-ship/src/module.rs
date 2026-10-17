//! One fitted module and its engineered stats: a port of Coriolis's `Module.js` value rules and
//! of how a journal `Engineering` block becomes modifications (`JournalUtils.js`,
//! `BlueprintFunctions.js`, and the build-code round trip in `Ship.js`), from its `Modifiers`
//! when the journal wrote them, else from the blueprint, grade and roll.
//!
//! Coriolis keeps each modification as a scaled integer: hundredths of a percent for percentage
//! stats (`0.1234` → `1234`) and hundredths for numeric ones. Missing values follow JavaScript:
//! `None` is `null` or `undefined`, and arithmetic treats it as zero where Coriolis's code does.

use crate::data::ModuleDef;
use astromech_engineering::data as eng;
use astromech_journal::{EngModifier, ModEngineering};
use std::collections::{BTreeMap, HashMap};

const RESISTANCES: [&str; 4] = ["explres", "kinres", "thermres", "causres"];

/// The stat each journal `Modifiers` label sets, by label in lowercase without
/// `OutfittingFieldType_` or `PerSecond` (how Coriolis matches them). From coriolis-data
/// `modifications/modifierActions.json`, whose `OutfittingFieldType_*` entries each set one stat
/// to the modifier's change, plus two labels the game writes that Coriolis doesn't match:
/// `MaxFuelPerJump` and `CausticResistance`.
const LABELS: &[(&str, &str)] = &[
    ("ammoclipsize", "clip"),
    ("ammomaximum", "ammo"),
    ("armourpenetration", "piercing"),
    ("boottime", "boot"),
    ("brokenregenrate", "brokenregen"),
    ("cargocapacity", "cargo"),
    ("causticresistance", "causres"),
    ("damage", "damage"),
    ("damagefalloffrange", "fallofffromrange"),
    ("defencemodifierhealthaddition", "hullreinforcement"),
    ("defencemodifierhealthmultiplier", "hullboost"),
    ("defencemodifiershieldmultiplier", "shieldboost"),
    ("distributordraw", "distdraw"),
    ("dss_ratemult", "scanrate"),
    ("energyperregen", "distdraw"),
    ("engineheatrate", "thermload"),
    ("engineoptimalmass", "optmass"),
    ("engineoptperformance", "optmul"),
    ("enginescapacity", "engcap"),
    ("enginesrecharge", "engrate"),
    ("explosiveresistance", "explres"),
    ("fsdheatrate", "thermload"),
    ("fsdinterdictorfacinglimit", "facinglimit"),
    ("fsdinterdictorrange", "ranget"),
    ("fsdmaxfuelperjump", "maxfuel"),
    ("fsdoptimalmass", "optmass"),
    ("heatefficiency", "eff"),
    ("integrity", "integrity"),
    ("jitter", "jitter"),
    ("kineticresistance", "kinres"),
    ("mass", "mass"),
    ("maxangle", "angle"),
    ("maxfuelperjump", "maxfuel"),
    ("maximumrange", "range"),
    ("powercapacity", "pgen"),
    ("powerdraw", "power"),
    ("range", "range"),
    ("rateoffire", "rof"),
    ("regenrate", "regen"),
    ("reloadtime", "reload"),
    ("scannerrange", "range"),
    ("scannertimetoscan", "scantime"),
    ("sensortargetscanangle", "angle"),
    ("shieldbankduration", "duration"),
    ("shieldbankheat", "thermload"),
    ("shieldbankreinforcement", "shieldreinforcement"),
    ("shieldbankspinup", "spinup"),
    ("shieldgenoptimalmass", "optmass"),
    ("shieldgenstrength", "optmul"),
    ("shotspeed", "shotspeed"),
    ("systemscapacity", "syscap"),
    ("systemsrecharge", "sysrate"),
    ("thermalload", "thermload"),
    ("thermicresistance", "thermres"),
    ("weapon_burst_rof", "burstrof"),
    ("weapon_burst_size", "burst"),
    ("weapon_clip_size_override", "clip"),
    ("weaponscapacity", "wepcap"),
    ("weaponsrecharge", "weprate"),
];

/// What a journal `Modifiers` label means for the stats.
#[derive(Debug, PartialEq)]
enum Label {
    Stat(&'static str),
    /// Worked out from other stats. `DamagePerSecond` is damage times rate of fire, both of which
    /// have their own entries (Coriolis reads it as damage, counting a rate-of-fire change twice).
    Derived,
    Unknown,
}

fn label(label: &str) -> Label {
    let key = label.to_ascii_lowercase().replace("outfittingfieldtype_", "");
    if key == "damagepersecond" {
        return Label::Derived;
    }
    let key = key.replace("persecond", "");
    LABELS.iter().find(|(l, _)| *l == key).map_or(Label::Unknown, |(_, s)| Label::Stat(s))
}

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
    /// A grade the blueprint doesn't have: (blueprint, grade).
    Grade(String, u8),
    Experimental(String),
    /// A journal `Modifiers` label not in [`LABELS`].
    Modifier(String),
}

impl Module {
    pub fn new(def: &'static ModuleDef) -> Self {
        Module { def, mods: HashMap::new(), special: None, effects: None }
    }

    /// Applies a journal `Engineering` block the way Coriolis imports it (`_addModifications`):
    /// with `Modifiers`, each stat's journal value, less the experimental's share, which the
    /// experimental puts back; without, every blueprint feature at the roll given by `quality`,
    /// the experimental's effects on top. Returns what was left out of the stats.
    pub fn engineer(&mut self, symbol: &str, e: &ModEngineering) -> Vec<Unapplied> {
        let mut missing = vec![];
        let unknown_experimental = self.experimental(symbol, e);
        if e.modifiers.is_empty() {
            missing.extend(self.blueprint(symbol, e));
            missing.extend(unknown_experimental.map(Unapplied::Experimental));
        } else {
            // The journal values already hold the blueprint and the experimental, known or not.
            for m in &e.modifiers {
                match label(&m.label) {
                    Label::Stat(stat) => self.set_from_journal(stat, m),
                    Label::Derived => {}
                    Label::Unknown => missing.push(Unapplied::Modifier(m.label.clone())),
                }
            }
        }
        self.round_trip();
        missing
    }

    /// Every blueprint feature at the roll given by `quality` (`setQualityCB`).
    fn blueprint(&mut self, symbol: &str, e: &ModEngineering) -> Option<Unapplied> {
        // Multi-cannons write their overcharged blueprint as Weapon_Overcharged (getBlueprint).
        let bp_name = if e.blueprint.eq_ignore_ascii_case("weapon_overcharged")
            && symbol.to_ascii_lowercase().contains("multicannon")
        {
            "MC_Overcharged"
        } else {
            &e.blueprint
        };
        let Some((_, bp)) = eng::blueprint(bp_name) else { return Some(Unapplied::Blueprint(e.blueprint.clone())) };
        let Some(grade) = bp.grade(e.level) else { return Some(Unapplied::Grade(e.blueprint.clone(), e.level)) };
        for (stat, range) in &grade.features {
            self.set_quality(stat, *range, e.quality);
        }
        None
    }

    /// Takes on the experimental's effects; returns its id when they aren't known.
    fn experimental(&mut self, symbol: &str, e: &ModEngineering) -> Option<String> {
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
                return Some(sp);
            }
        }
        None
    }

    /// One journal modifier as the change to `stat` (`_addModifications`), then
    /// `setModValue(stat, value, true)`: the experimental's part of the change comes off, as
    /// [`Module::mod_value`] adds it back.
    fn set_from_journal(&mut self, stat: &str, m: &EngModifier) {
        let (v, o) = (m.value, m.original_value);
        let mut value = (v / o * 100.0 - 100.0) * 100.0;
        if value == f64::INFINITY {
            value = v * 100.0;
        }
        if m.label.contains("DamageFalloffRange") {
            value = (v / self.base("range").unwrap_or(f64::NAN) - 1.0) * 100.0;
        }
        if m.label.contains("Resistance") {
            value = v * 100.0 - o * 100.0;
        }
        if m.label.contains("ShieldMultiplier") || m.label.contains("DefenceModifierHealthMultiplier") {
            value = ((100.0 + v) / (100.0 + o) * 100.0 - 100.0) * 100.0;
        }
        let action = self.effects.and_then(|e| e.get(stat)).copied();
        let value = match (action, eng::modifier(stat)) {
            (Some(a), Some(md)) => {
                if RESISTANCES.contains(&stat) {
                    let base = self.base(stat);
                    let res = base.unwrap_or(0.0) + value / 10000.0;
                    let x = a / 100.0;
                    Some(((x - res) / (x - 1.0) - base.unwrap_or(f64::NAN)) * 10000.0)
                } else if md.method == "additive" {
                    // Coriolis subtracts `a` unscaled here (only Jitter, which no stat uses).
                    Some(value - a * 100.0)
                } else if md.method == "overwrite" {
                    None
                } else {
                    Some(((value / 10000.0 + 1.0) / (1.0 + a) - 1.0) * 10000.0)
                }
            }
            _ => Some(value),
        };
        // A value the build code can't hold (no original value, say) is lost in Coriolis's round
        // trip, like a zero.
        match value.filter(|v| *v != 0.0 && v.is_finite()) {
            Some(v) => self.mods.insert(stat.to_string(), v),
            None => self.mods.remove(stat),
        };
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
    ///
    /// Departs from Coriolis for charged weapons (decision astromech-bnnh): the cycle also counts
    /// the last burst's charge, which Coriolis leaves out, so sustained never exceeds burst (a
    /// one-round rail gun cycles in charge plus reload, not reload alone). Weapons without charge
    /// give Coriolis's numbers, except that a cycle of no time at all (a one-round clip with no
    /// reload: the seismic charge launchers) gets 1, where Coriolis divides by zero.
    ///
    /// The cycle leaves out the last shot's own fire interval, as Coriolis does, so a build that
    /// engineers the reload below the interval would come out above burst: the factor is clamped
    /// at 1 (sustained never exceeds burst).
    pub fn sustained_factor(&self) -> f64 {
        let Some(mut clip) = self.clip() else { return 1.0 };
        // An auto-loader gives one reload for every two shots fired.
        if self.is_special("special_auto_loader") {
            clip += clip - 1.0;
        }
        let burst = or(self.get("burst"), 1.0);
        let rof = self.rof();
        let overhead = (burst - 1.0) / or(self.get("burstrof"), 1.0);
        let charge = or(self.get("charge"), 0.0);
        let cycle = (clip - burst) / rof + overhead + charge + self.get("reload").unwrap_or(0.0);
        if cycle <= 0.0 {
            return 1.0;
        }
        ((clip / cycle) / rof).min(1.0)
    }

    pub fn sustained_dps(&self) -> f64 {
        self.dps() * self.sustained_factor()
    }

    /// Distributor draw per second over a reload cycle (`timeToDrainWep`).
    ///
    /// Charged weapons use the same cycle as [`Module::sustained_factor`] (decision
    /// astromech-bnnh): the draw per shot over charge plus reload, so a rail gun's draw and its
    /// damage describe one cycle, and an engineered reload below the fire interval is clamped the
    /// same way. Every other weapon keeps Coriolis's `timeToDrainWep` cycle, the clip's shots at
    /// the burst rate plus the reload, on purpose: Coriolis's own damage cycle leaves the last
    /// shot's interval out and its draw cycle counts it, and the Coriolis numbers are what users
    /// cross-check. That cycle is never shorter than the burst rate, so it never exceeds it.
    pub fn sustained_eps(&self) -> f64 {
        if or(self.get("charge"), 0.0) > 0.0 {
            return self.eps() * self.sustained_factor();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn engineered(item: &str, blueprint: &str, level: u8, quality: f64) -> (Module, Vec<Unapplied>) {
        let mut m = Module::new(crate::data::module(item).unwrap());
        let e = ModEngineering {
            blueprint: blueprint.into(),
            level,
            quality,
            experimental: None,
            experimental_localised: None,
            engineer: None,
            modifiers: vec![],
        };
        let missing = m.engineer(item, &e);
        (m, missing)
    }

    #[test]
    fn sustained_never_exceeds_burst_for_any_catalogue_module() {
        let mut checked = 0;
        for (item, def) in &crate::data::catalog().modules {
            let m = Module::new(def);
            let dps = m.dps();
            if m.clip().is_none() || dps == 0.0 || dps.is_nan() {
                continue;
            }
            let f = m.sustained_factor();
            assert!(f.is_finite() && f <= 1.0 + 1e-9, "{item}: sustained factor {f}");
            checked += 1;
        }
        assert!(checked > 50, "only {checked} weapons with a clip");
    }

    #[test]
    fn rail_gun_sustained_counts_its_charge() {
        // Medium fixed rail gun: 41.5 damage, 0.83 s interval + 1.2 s charge = 2.03 s a shot,
        // clip 1, reload 1 s. Burst 41.5 / 2.03 = 20.44 DPS. Sustained cycles in the 1.2 s
        // charge plus the 1 s reload (Coriolis: the 1 s reload alone, 41.5 DPS), 41.5 / 2.2 = 18.86.
        let m = Module::new(crate::data::module("hpt_railgun_fixed_medium").unwrap());
        assert_eq!(m.clip(), Some(1.0));
        assert!((m.dps() - 41.5 / 2.03).abs() < 1e-9);
        assert!((m.sustained_dps() - 41.5 / 2.2).abs() < 1e-9);
        assert!((m.sustained_factor() - 2.03 / 2.2).abs() < 1e-9);
    }

    #[test]
    fn rail_gun_sustained_draw_uses_the_same_cycle_as_sustained_damage() {
        // Medium fixed rail gun: a shot every 2.03 s (0.83 s interval, 1.2 s charge), clip 1,
        // reload 1 s, so a cycle of 2.2 s: the distributor draw per shot over 2.2 s, as the damage.
        let m = Module::new(crate::data::module("hpt_railgun_fixed_medium").unwrap());
        let draw = m.get("distdraw").unwrap();
        assert!(draw > 0.0);
        assert!((m.eps() - draw / 2.03).abs() < 1e-9);
        assert!((m.sustained_eps() - draw / 2.2).abs() < 1e-9);
        assert!((m.sustained_eps() / m.eps() - m.sustained_factor()).abs() < 1e-9);
    }

    #[test]
    fn an_engineered_reload_below_the_fire_interval_never_shows_sustained_above_burst() {
        // The same rail gun with the reload engineered down to 0.1 s: a cycle of charge plus
        // reload, 1.3 s, is shorter than the 2.03 s a shot takes, which would read as 156 % of burst.
        let (m, missing) = journal("hpt_railgun_fixed_medium", "Weapon_Rapid", None, &[("ReloadTime", 0.1, 1.0)]);
        assert!(missing.is_empty(), "{missing:?}");
        assert!((m.get("reload").unwrap() - 0.1).abs() < 1e-9);
        assert_eq!(m.sustained_factor(), 1.0);
        assert_eq!(m.sustained_dps(), m.dps());
        assert_eq!(m.sustained_eps(), m.eps());
    }

    #[test]
    fn a_weapon_without_charge_with_a_tiny_reload_keeps_sustained_draw_and_damage_within_burst() {
        let (m, missing) = journal("hpt_multicannon_fixed_medium", "Weapon_Rapid", None, &[("ReloadTime", 0.01, 5.0)]);
        assert!(missing.is_empty(), "{missing:?}");
        assert!(m.get("reload").unwrap() < 0.02);
        assert!(m.sustained_dps() <= m.dps() + 1e-9);
        assert!(m.sustained_eps() <= m.eps() + 1e-9);
    }

    #[test]
    fn grade_5_at_roll_0_is_the_grade_4_maximum() {
        // Each grade's range starts where the one below ends, for every ranged stat.
        let fsd = "int_hyperdrive_size5_class5";
        let (g5, _) = engineered(fsd, "FSD_LongRange", 5, 0.0);
        let (g4, _) = engineered(fsd, "FSD_LongRange", 4, 1.0);
        assert_eq!(g5.get("optmass"), g4.get("optmass"));
        assert!(g5.get("optmass") > engineered(fsd, "FSD_LongRange", 4, 0.5).0.get("optmass"));
        let pd = "int_powerdistributor_size7_class5";
        let (g5, _) = engineered(pd, "PowerDistributor_PriorityWeapons", 5, 0.0);
        let (g4, _) = engineered(pd, "PowerDistributor_PriorityWeapons", 4, 1.0);
        assert_eq!(g5.get("wepcap"), g4.get("wepcap"));
        assert_eq!(g5.get("weprate"), g4.get("weprate"));
        // The fixed drawbacks still follow the grade.
        assert!(g5.get("engcap") < g4.get("engcap"));
    }

    #[test]
    fn unknown_grade_is_reported() {
        let (m, missing) = engineered("int_hyperdrive_size5_class5", "FSD_LongRange", 0, 1.0);
        assert_eq!(missing, [Unapplied::Grade("FSD_LongRange".into(), 0)]);
        assert_eq!(m.get("optmass"), m.base("optmass"));
    }

    fn journal(
        item: &str,
        blueprint: &str,
        experimental: Option<&str>,
        mods: &[(&str, f64, f64)],
    ) -> (Module, Vec<Unapplied>) {
        let mut m = Module::new(crate::data::module(item).unwrap());
        let e = ModEngineering {
            blueprint: blueprint.into(),
            level: 5,
            quality: 0.0,
            experimental: experimental.map(Into::into),
            experimental_localised: None,
            engineer: None,
            modifiers: mods
                .iter()
                .map(|(l, v, o)| EngModifier { label: l.to_string(), value: *v, original_value: *o })
                .collect(),
        };
        let missing = m.engineer(item, &e);
        (m, missing)
    }

    #[test]
    fn modifiers_set_the_stats_and_unknown_labels_are_reported() {
        let fsd = "int_hyperdrive_size5_class5";
        // Quality 0 would be +45 %; the journal says +53.1 %. Labels match without
        // `OutfittingFieldType_`, in any case; the blueprint isn't needed.
        let (m, missing) = journal(
            fsd,
            "FSD_Legacy",
            None,
            &[("FSDOptimalMass", 1607.55, 1050.0), ("outfittingfieldtype_Mass", 26.0, 20.0), ("Shininess", 2.0, 1.0)],
        );
        assert_eq!(missing, [Unapplied::Modifier("Shininess".into())]);
        assert!((m.get("optmass").unwrap() - 1607.55).abs() < 1e-9);
        assert!((m.get("mass").unwrap() - 26.0).abs() < 1e-9);
        assert_eq!(m.get("integrity"), m.base("integrity"));
        // The experimental's part comes off the stored change and back on in the stat.
        let (m, missing) = journal(
            fsd,
            "FSD_LongRange",
            Some("special_fsd_heavy"),
            &[("FSDOptimalMass", 1050.0 * 1.5 * 1.04, 1050.0)],
        );
        assert!(missing.is_empty());
        assert_eq!(m.mods.get("optmass"), Some(&5000.0));
        assert!((m.get("optmass").unwrap() - 1638.0).abs() < 1e-9);
        // Resistances are percentage points; damage per second is derived, not a stat.
        let (m, missing) = journal(
            "int_hullreinforcement_size4_class2",
            "HullReinforcement_Kinetic",
            None,
            &[("KineticResistance", 12.5, 2.0), ("CausticResistance", 1.0, 0.0), ("DamagePerSecond", 9.0, 3.0)],
        );
        assert!(missing.is_empty());
        assert!((m.get("kinres").unwrap() - 0.125).abs() < 1e-9);
        assert!((m.get("causres").unwrap() - 0.01).abs() < 1e-9);
        assert_eq!(m.get("damage"), None);
    }
}
