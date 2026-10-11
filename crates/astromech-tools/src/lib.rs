//! # astromech-tools
//!
//! Typed tools (ARCHITECTURE.md §5.13 and the T1 table in §9). Each ship toggle knows its
//! Elite action, the Status.json flag that verifies it, its preconditions, and what to say.
//! Planning is pure: given the current flags it returns press / already-there / refused.
//! Execution goes through `astromech-policy`.
//!
//! **Rule:** depends on `astromech-policy`, never on `astromech-input` directly.

use astromech_proto::Tier;
use astromech_status as st;

pub use astromech_policy;

/// Status.json flag bits used here (Frontier's Status file documentation).
pub mod flags {
    pub use astromech_status::{FLAG_DOCKED, FLAG_GEAR, FLAG_LANDED, FLAG_LIGHTS, FLAG_NIGHT_VISION, FLAG_SUPERCRUISE};
    pub const HARDPOINTS: u64 = 1 << 6;
    pub const CARGO_SCOOP: u64 = 1 << 9;
    pub const IN_MAIN_SHIP: u64 = super::st::FLAG_IN_MAIN_SHIP;
    pub const IN_SRV: u64 = super::st::FLAG_IN_SRV;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toggle {
    LandingGear,
    Lights,
    Hardpoints,
    CargoScoop,
    NightVision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    On,
    Off,
    Toggle,
}

/// One typed tool call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToolCall {
    pub toggle: Toggle,
    pub target: Target,
}

pub struct Spec {
    /// Stable tool name (policy dedupe key and logs).
    pub name: &'static str,
    /// Elite binding action pressed.
    pub action: &'static str,
    pub flag: u64,
    pub tier: Tier,
    /// Human name used in speech.
    pub noun: &'static str,
    pub on_word: &'static str,
    pub off_word: &'static str,
}

impl Toggle {
    pub fn spec(self) -> Spec {
        let s = |name, action, flag, noun, on_word, off_word| Spec {
            name,
            action,
            flag,
            tier: Tier::T1,
            noun,
            on_word,
            off_word,
        };
        match self {
            Toggle::LandingGear => {
                s("set_landing_gear", "LandingGearToggle", flags::FLAG_GEAR, "Landing gear", "deployed", "retracted")
            }
            Toggle::Lights => s("set_lights", "ShipSpotLightToggle", flags::FLAG_LIGHTS, "Lights", "on", "off"),
            Toggle::Hardpoints => {
                s("set_hardpoints", "DeployHardpointToggle", flags::HARDPOINTS, "Hardpoints", "deployed", "retracted")
            }
            Toggle::CargoScoop => {
                s("set_cargo_scoop", "ToggleCargoScoop", flags::CARGO_SCOOP, "Cargo scoop", "open", "closed")
            }
            Toggle::NightVision => {
                s("set_night_vision", "NightVisionToggle", flags::FLAG_NIGHT_VISION, "Night vision", "on", "off")
            }
        }
    }
    pub fn all() -> [Toggle; 5] {
        [Toggle::LandingGear, Toggle::Lights, Toggle::Hardpoints, Toggle::CargoScoop, Toggle::NightVision]
    }
}

/// Map an `astromech-intent` command id to a tool call.
pub fn from_command(id: &str) -> Option<ToolCall> {
    let (t, a) = id.split_once('.')?;
    let toggle = match t {
        "landing_gear" => Toggle::LandingGear,
        "lights" => Toggle::Lights,
        "hardpoints" => Toggle::Hardpoints,
        "cargo_scoop" => Toggle::CargoScoop,
        "night_vision" => Toggle::NightVision,
        _ => return None,
    };
    let target = match a {
        "down" | "on" | "deploy" | "open" => Target::On,
        "up" | "off" | "retract" | "close" => Target::Off,
        "toggle" => Target::Toggle,
        _ => return None,
    };
    Some(ToolCall { toggle, target })
}

/// What to do, decided from the current Status flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Press the action once; verification waits for `flag` to equal `expect_set`.
    Press { action: &'static str, flag: u64, expect_set: bool, confirm: String, failure: String },
    /// Nothing to press: already in the requested state.
    Already(String),
    /// Not allowed in the current state.
    Refused(String),
}

/// Plan a tool call. `flags` is `None` when Status.json has no Flags (main menu, game not
/// running), which refuses everything.
pub fn plan(call: ToolCall, flags: Option<u64>) -> Plan {
    let sp = call.toggle.spec();
    let Some(f) = flags else {
        return Plan::Refused("I can't see the game state yet. Is Elite running and are you in your ship?".into());
    };
    let has = |b: u64| f & b != 0;
    let in_ship = has(flags::IN_MAIN_SHIP);
    let in_srv = has(flags::IN_SRV);
    let refuse = |why: &str| Plan::Refused(format!("{} not available: {why}.", sp.noun));
    match call.toggle {
        Toggle::LandingGear | Toggle::Hardpoints | Toggle::CargoScoop => {
            if !in_ship {
                return refuse("you're not in your ship");
            }
            if has(flags::FLAG_DOCKED) {
                return refuse("you're docked");
            }
            if has(flags::FLAG_LANDED) {
                return refuse("you're landed");
            }
            // S5 rules: the game ignores the landing gear in supercruise; hardpoints too.
            if has(flags::FLAG_SUPERCRUISE) && call.toggle != Toggle::CargoScoop {
                return refuse("you're in supercruise");
            }
        }
        Toggle::Lights | Toggle::NightVision => {
            if !(in_ship || in_srv) {
                return refuse("you're not in your ship or SRV");
            }
        }
    }
    let now = has(sp.flag);
    let want = match call.target {
        Target::On => true,
        Target::Off => false,
        Target::Toggle => !now,
    };
    let word = |on: bool| if on { sp.on_word } else { sp.off_word };
    if call.target != Target::Toggle && now == want {
        let verb = if sp.noun.ends_with('s') && sp.noun != "Night vision" { "are" } else { "is" };
        return Plan::Already(format!("{} {verb} already {}.", sp.noun, word(want)));
    }
    Plan::Press {
        action: sp.action,
        flag: sp.flag,
        expect_set: want,
        confirm: format!("{} {}.", sp.noun, word(want)),
        failure: format!("{} didn't respond.", sp.noun),
    }
}

#[cfg(test)]
mod tests {
    use super::flags::*;
    use super::*;

    const FLYING: u64 = IN_MAIN_SHIP;

    fn gear(t: Target) -> ToolCall {
        ToolCall { toggle: Toggle::LandingGear, target: t }
    }

    #[test]
    fn commands_map() {
        assert_eq!(from_command("landing_gear.down"), Some(gear(Target::On)));
        assert_eq!(from_command("cargo_scoop.close").unwrap().target, Target::Off);
        assert_eq!(from_command("request_docking"), None);
    }

    #[test]
    fn gear_state_rules() {
        assert!(matches!(plan(gear(Target::On), Some(FLYING)), Plan::Press { expect_set: true, .. }));
        assert_eq!(
            plan(gear(Target::On), Some(FLYING | FLAG_GEAR)),
            Plan::Already("Landing gear is already deployed.".into())
        );
        assert_eq!(plan(gear(Target::Off), Some(FLYING)), Plan::Already("Landing gear is already retracted.".into()));
        assert!(matches!(plan(gear(Target::Toggle), Some(FLYING | FLAG_GEAR)), Plan::Press { expect_set: false, .. }));
        for (f, why) in [(FLAG_DOCKED, "docked"), (FLAG_LANDED, "landed"), (FLAG_SUPERCRUISE, "supercruise")] {
            match plan(gear(Target::On), Some(FLYING | f)) {
                Plan::Refused(m) => assert!(m.contains(why), "{m}"),
                p => panic!("{p:?}"),
            }
        }
        assert!(matches!(plan(gear(Target::On), Some(IN_SRV)), Plan::Refused(_)));
        assert!(matches!(plan(gear(Target::On), None), Plan::Refused(_)));
    }

    #[test]
    fn other_toggles() {
        let lights = ToolCall { toggle: Toggle::Lights, target: Target::On };
        assert!(matches!(plan(lights, Some(IN_SRV)), Plan::Press { .. }));
        assert!(matches!(plan(lights, Some(FLYING | FLAG_DOCKED)), Plan::Press { .. }), "lights work docked");
        let hp = ToolCall { toggle: Toggle::Hardpoints, target: Target::On };
        assert!(matches!(plan(hp, Some(FLYING | FLAG_SUPERCRUISE)), Plan::Refused(_)));
        assert_eq!(plan(hp, Some(FLYING | HARDPOINTS)), Plan::Already("Hardpoints are already deployed.".into()));
        let scoop = ToolCall { toggle: Toggle::CargoScoop, target: Target::Off };
        assert!(matches!(
            plan(scoop, Some(FLYING | FLAG_SUPERCRUISE | CARGO_SCOOP)),
            Plan::Press { expect_set: false, .. }
        ));
        let nv = ToolCall { toggle: Toggle::NightVision, target: Target::On };
        assert_eq!(plan(nv, Some(FLYING | FLAG_NIGHT_VISION)), Plan::Already("Night vision is already on.".into()));
    }
}
