//! # astromech-tools
//!
//! Typed tools (ARCHITECTURE.md §5.13 and the T1 table in §9). Each ship toggle knows its
//! Elite action, the Status.json flag that verifies it, its preconditions, and what to say.
//! Planning is pure: given the current flags it returns press / already-there / refused.
//! Execution goes through `astromech-policy`.
//!
//! **Rule:** depends on `astromech-policy`, never on `astromech-input` directly.

use astromech_gamestate::GameState;
use astromech_proto::Tier;
use astromech_status as st;

pub use astromech_policy;

/// Status.json flag bits used here (Frontier's Status file documentation).
pub mod flags {
    pub use astromech_status::{
        FLAG_DOCKED, FLAG_FLIGHT_ASSIST_OFF, FLAG_GEAR, FLAG_HUD_ANALYSIS, FLAG_IN_MAIN_SHIP, FLAG_IN_SRV, FLAG_LANDED,
        FLAG_LIGHTS, FLAG_NIGHT_VISION, FLAG_SHIELDS_UP, FLAG_SILENT_RUNNING, FLAG_SRV_DRIVE_ASSIST,
        FLAG_SRV_HANDBRAKE, FLAG_SUPERCRUISE, GUI_COMMS, GUI_GALAXY, GUI_LEFT, GUI_NONE, GUI_RIGHT, GUI_ROLE,
        GUI_SYSTEM,
    };
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
    FlightAssist,
    SilentRunning,
    HudAnalysis,
    SrvHandbrake,
    SrvDriveAssist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    On,
    Off,
    Toggle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    GalaxyMap,
    SystemMap,
    Close,
    Left,
    Right,
    Comms,
    Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query {
    Fuel,
    Cargo,
    JumpRange,
    Location,
    Jumps,
    Hull,
    Shields,
    Credits,
    Ship,
    Legal,
}

/// One typed tool call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolCall {
    Toggle {
        toggle: Toggle,
        target: Target,
    },
    Screen(Screen),
    ThrottleZero,
    Query(Query),
    /// The nearest raw-material site of a kind (`astromech_galaxy::SITE_KINDS` id): a network
    /// lookup the runner answers, nothing pressed.
    Nearest(&'static str),
}

/// Site kinds a voice query can ask for (`sites.<kind>` commands); the same ids as
/// `astromech_galaxy::SITE_KINDS`.
pub const SITE_KINDS: &[&str] = &["crystalline_shards", "brain_trees", "crashed_ships", "geological", "raw_farms"];

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
    /// When true, Status flag set means the "on" state (false for flight assist: bit 5 = FA off).
    pub flag_set_is_on: bool,
}

impl Toggle {
    pub fn spec(self) -> Spec {
        let s = |name, action, flag, noun, on_word, off_word, flag_set_is_on| Spec {
            name,
            action,
            flag,
            tier: Tier::T1,
            noun,
            on_word,
            off_word,
            flag_set_is_on,
        };
        match self {
            Toggle::LandingGear => s(
                "set_landing_gear",
                "LandingGearToggle",
                flags::FLAG_GEAR,
                "Landing gear",
                "deployed",
                "retracted",
                true,
            ),
            Toggle::Lights => s("set_lights", "ShipSpotLightToggle", flags::FLAG_LIGHTS, "Lights", "on", "off", true),
            Toggle::Hardpoints => s(
                "set_hardpoints",
                "DeployHardpointToggle",
                flags::HARDPOINTS,
                "Hardpoints",
                "deployed",
                "retracted",
                true,
            ),
            Toggle::CargoScoop => {
                s("set_cargo_scoop", "ToggleCargoScoop", flags::CARGO_SCOOP, "Cargo scoop", "open", "closed", true)
            }
            Toggle::NightVision => {
                s("set_night_vision", "NightVisionToggle", flags::FLAG_NIGHT_VISION, "Night vision", "on", "off", true)
            }
            Toggle::FlightAssist => s(
                "set_flight_assist",
                "ToggleFlightAssist",
                flags::FLAG_FLIGHT_ASSIST_OFF,
                "Flight assist",
                "on",
                "off",
                false,
            ),
            Toggle::SilentRunning => s(
                "set_silent_running",
                "ToggleButtonUpInput",
                flags::FLAG_SILENT_RUNNING,
                "Silent running",
                "on",
                "off",
                true,
            ),
            Toggle::HudAnalysis => s(
                "set_hud_mode",
                "PlayerHUDModeToggle",
                flags::FLAG_HUD_ANALYSIS,
                "HUD analysis mode",
                "on",
                "off",
                true,
            ),
            Toggle::SrvHandbrake => {
                s("set_srv_handbrake", "Handbrake", flags::FLAG_SRV_HANDBRAKE, "Handbrake", "on", "off", true)
            }
            Toggle::SrvDriveAssist => s(
                "set_srv_drive_assist",
                "ToggleDriveAssist",
                flags::FLAG_SRV_DRIVE_ASSIST,
                "Drive assist",
                "on",
                "off",
                true,
            ),
        }
    }
    pub fn all() -> [Toggle; 10] {
        [
            Toggle::LandingGear,
            Toggle::Lights,
            Toggle::Hardpoints,
            Toggle::CargoScoop,
            Toggle::NightVision,
            Toggle::FlightAssist,
            Toggle::SilentRunning,
            Toggle::HudAnalysis,
            Toggle::SrvHandbrake,
            Toggle::SrvDriveAssist,
        ]
    }
}

impl Screen {
    pub fn action(self) -> &'static str {
        match self {
            Screen::GalaxyMap => "GalaxyMapOpen",
            Screen::SystemMap => "SystemMapOpen",
            Screen::Close => "UI_Back",
            Screen::Left => "FocusLeftPanel",
            Screen::Right => "FocusRightPanel",
            Screen::Comms => "FocusCommsPanel",
            Screen::Role => "FocusRadarPanel",
        }
    }
    pub fn gui(self) -> u64 {
        match self {
            Screen::GalaxyMap => flags::GUI_GALAXY,
            Screen::SystemMap => flags::GUI_SYSTEM,
            Screen::Close => flags::GUI_NONE,
            Screen::Left => flags::GUI_LEFT,
            Screen::Right => flags::GUI_RIGHT,
            Screen::Comms => flags::GUI_COMMS,
            Screen::Role => flags::GUI_ROLE,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Screen::GalaxyMap => "open_screen.galaxy",
            Screen::SystemMap => "open_screen.system",
            Screen::Close => "open_screen.none",
            Screen::Left => "open_screen.left",
            Screen::Right => "open_screen.right",
            Screen::Comms => "open_screen.comms",
            Screen::Role => "open_screen.role",
        }
    }
    pub fn noun(self) -> &'static str {
        match self {
            Screen::GalaxyMap => "Galaxy map",
            Screen::SystemMap => "System map",
            Screen::Close => "Panel",
            Screen::Left => "Left panel",
            Screen::Right => "Right panel",
            Screen::Comms => "Comms panel",
            Screen::Role => "Role panel",
        }
    }
}

/// Map an `astromech-intent` command id to a tool call.
pub fn from_command(id: &str) -> Option<ToolCall> {
    if id == "throttle.zero" {
        return Some(ToolCall::ThrottleZero);
    }
    if let Some(kind) = id.strip_prefix("sites.") {
        return SITE_KINDS.iter().find(|k| **k == kind).map(|k| ToolCall::Nearest(k));
    }
    if let Some(rest) = id.strip_prefix("status.") {
        let q = match rest {
            "fuel" => Query::Fuel,
            "cargo" => Query::Cargo,
            "jump_range" => Query::JumpRange,
            "location" => Query::Location,
            "jumps" => Query::Jumps,
            "hull" => Query::Hull,
            "shields" => Query::Shields,
            "credits" => Query::Credits,
            "ship" => Query::Ship,
            "legal" => Query::Legal,
            _ => return None,
        };
        return Some(ToolCall::Query(q));
    }
    match id {
        "galaxy_map.open" => return Some(ToolCall::Screen(Screen::GalaxyMap)),
        "system_map.open" => return Some(ToolCall::Screen(Screen::SystemMap)),
        "panel.close" => return Some(ToolCall::Screen(Screen::Close)),
        "panel.left" => return Some(ToolCall::Screen(Screen::Left)),
        "panel.right" => return Some(ToolCall::Screen(Screen::Right)),
        "panel.comms" => return Some(ToolCall::Screen(Screen::Comms)),
        "panel.role" => return Some(ToolCall::Screen(Screen::Role)),
        _ => {}
    }
    let (t, a) = id.split_once('.')?;
    let toggle = match t {
        "landing_gear" => Toggle::LandingGear,
        "lights" => Toggle::Lights,
        "hardpoints" => Toggle::Hardpoints,
        "cargo_scoop" => Toggle::CargoScoop,
        "night_vision" => Toggle::NightVision,
        "flight_assist" => Toggle::FlightAssist,
        "silent_running" => Toggle::SilentRunning,
        "hud" => Toggle::HudAnalysis,
        "srv_handbrake" => Toggle::SrvHandbrake,
        "srv_drive_assist" => Toggle::SrvDriveAssist,
        _ => return None,
    };
    let target = match a {
        "down" | "on" | "deploy" | "open" | "analysis" => Target::On,
        "up" | "off" | "retract" | "close" | "combat" => Target::Off,
        "toggle" => Target::Toggle,
        _ => return None,
    };
    Some(ToolCall::Toggle { toggle, target })
}

/// How to verify a press.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verify {
    Flag { bit: u64, expect_set: bool },
    GuiFocus(u64),
    None,
}

/// What to do, decided from the current Status flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Plan {
    /// Press the action once.
    Press { action: &'static str, verify: Verify, confirm: String, failure: String },
    /// Nothing to press: already in the requested state.
    Already(String),
    /// Not allowed in the current state.
    Refused(String),
    /// Read-only spoken answer (T0).
    Speak(String),
    /// Read-only answer that needs a lookup (T0): the nearest site of this kind.
    Lookup(&'static str),
    /// Ask before pressing (silent running on).
    Confirm { prompt: String, action: &'static str, verify: Verify, confirm: String, failure: String },
}

impl Plan {
    pub fn as_press(&self) -> Option<(&str, Verify, &str)> {
        match self {
            Plan::Press { action, verify, confirm, .. } => Some((*action, *verify, confirm.as_str())),
            _ => None,
        }
    }
}

/// Plan from flags only (toggles). Screens and queries need [`plan_with`].
pub fn plan(call: ToolCall, flags: Option<u64>) -> Plan {
    let mut g = GameState::default();
    if let Some(f) = flags {
        g.status_flags = f;
        g.status_seen = true;
    }
    plan_with(call, &g)
}

/// Plan a tool call against a full snapshot.
pub fn plan_with(call: ToolCall, g: &GameState) -> Plan {
    match call {
        ToolCall::Toggle { toggle, target } => plan_toggle(toggle, target, g),
        ToolCall::Screen(s) => plan_screen(s, g),
        ToolCall::ThrottleZero => plan_throttle(g),
        ToolCall::Query(q) => Plan::Speak(answer(q, g)),
        ToolCall::Nearest(kind) => Plan::Lookup(kind),
    }
}

fn in_ship_or_srv(f: u64) -> bool {
    f & flags::IN_MAIN_SHIP != 0 || f & flags::IN_SRV != 0
}

fn plan_toggle(toggle: Toggle, target: Target, g: &GameState) -> Plan {
    let sp = toggle.spec();
    let Some(f) = g.flags() else {
        return Plan::Refused("Game state not available yet. Is Elite running and are you in your ship?".into());
    };
    let has = |b: u64| f & b != 0;
    let refuse = |why: &str| Plan::Refused(format!("{} not available: {why}.", sp.noun));
    match toggle {
        Toggle::LandingGear | Toggle::Hardpoints | Toggle::CargoScoop => {
            if !has(flags::IN_MAIN_SHIP) {
                return refuse("not in ship");
            }
            if has(flags::FLAG_DOCKED) {
                return refuse("docked");
            }
            if has(flags::FLAG_LANDED) {
                return refuse("landed");
            }
            if has(flags::FLAG_SUPERCRUISE) && toggle != Toggle::CargoScoop {
                return refuse("supercruise");
            }
        }
        Toggle::Lights | Toggle::NightVision | Toggle::HudAnalysis => {
            if !in_ship_or_srv(f) {
                return refuse("not in ship or SRV");
            }
        }
        Toggle::FlightAssist | Toggle::SilentRunning => {
            if !has(flags::IN_MAIN_SHIP) {
                return refuse("not in ship");
            }
        }
        Toggle::SrvHandbrake | Toggle::SrvDriveAssist => {
            if !has(flags::IN_SRV) {
                return refuse("not in SRV");
            }
        }
    }
    let flag_set = has(sp.flag);
    let now_on = if sp.flag_set_is_on { flag_set } else { !flag_set };
    let want_on = match target {
        Target::On => true,
        Target::Off => false,
        Target::Toggle => !now_on,
    };
    let word = |on: bool| if on { sp.on_word } else { sp.off_word };
    if target != Target::Toggle && now_on == want_on {
        let verb = if sp.noun.ends_with('s') && sp.noun != "Night vision" && sp.noun != "HUD analysis mode" {
            "are"
        } else {
            "is"
        };
        return Plan::Already(format!("{} {verb} already {}.", sp.noun, word(want_on)));
    }
    let expect_set = if sp.flag_set_is_on { want_on } else { !want_on };
    let confirm = if toggle == Toggle::HudAnalysis {
        if want_on {
            "HUD analysis mode.".to_string()
        } else {
            "HUD combat mode.".to_string()
        }
    } else {
        format!("{} {}.", sp.noun, word(want_on))
    };
    let press = Plan::Press {
        action: sp.action,
        verify: Verify::Flag { bit: sp.flag, expect_set },
        confirm,
        failure: format!("{} didn't respond.", sp.noun),
    };
    if toggle == Toggle::SilentRunning && want_on {
        return Plan::Confirm {
            prompt: "Silent running produces heat. Confirm?".into(),
            action: sp.action,
            verify: Verify::Flag { bit: sp.flag, expect_set: true },
            confirm: "Silent running on.".into(),
            failure: "Silent running didn't respond.".into(),
        };
    }
    press
}

fn plan_screen(s: Screen, g: &GameState) -> Plan {
    let Some(f) = g.flags() else {
        return Plan::Refused("Game state not available yet. Is Elite running and are you in your ship?".into());
    };
    if !in_ship_or_srv(f) {
        return Plan::Refused(format!("{} not available: not in ship or SRV.", s.noun()));
    }
    let now = g.gui_focus.unwrap_or(0);
    let want = s.gui();
    if now == want {
        return Plan::Already(match s {
            Screen::Close => "No panel is open.".into(),
            _ => format!("{} is already open.", s.noun()),
        });
    }
    let confirm = match s {
        Screen::Close => "Panel closed.".to_string(),
        _ => format!("{} open.", s.noun()),
    };
    Plan::Press {
        action: s.action(),
        verify: Verify::GuiFocus(want),
        confirm,
        failure: format!("{} didn't respond.", s.noun()),
    }
}

fn plan_throttle(g: &GameState) -> Plan {
    let Some(f) = g.flags() else {
        return Plan::Refused("Game state not available yet. Is Elite running and are you in your ship?".into());
    };
    if f & flags::IN_MAIN_SHIP == 0 {
        return Plan::Refused("Throttle to zero not available: not in ship.".into());
    }
    if f & flags::FLAG_SUPERCRUISE != 0 && g.body.as_ref().is_some_and(|b| !b.is_empty()) {
        return Plan::Refused("Throttle to zero not available: near a body in supercruise.".into());
    }
    Plan::Press {
        action: "SetSpeedZero",
        verify: Verify::None,
        confirm: "Throttle zero.".into(),
        failure: "Throttle didn't respond.".into(),
    }
}

pub fn answer(q: Query, g: &GameState) -> String {
    match q {
        Query::Fuel => {
            let Some(main) = g.fuel_main else {
                return "No fuel data yet.".into();
            };
            let mut s = format!("Fuel: {main:.1} tonnes main");
            if let Some(r) = g.fuel_reservoir {
                s += &format!(", {r:.2} tonnes reservoir");
            }
            s.push('.');
            if let Some(cap) = g.fuel_capacity_main {
                if cap > 0.0 {
                    s += &format!(" Tank {:.0} percent.", 100.0 * main / cap);
                }
            }
            s
        }
        Query::Cargo => {
            let used = g.cargo_tonnes.map(|n| n as i64);
            let Some(used) = used else {
                return "No cargo data yet.".into();
            };
            let mut s = match g.cargo_capacity {
                Some(cap) => format!("Cargo: {used} of {cap} tonnes."),
                None => format!("Cargo: {used} tonnes."),
            };
            let top: Vec<String> = g.cargo_items.iter().take(3).map(|(n, c)| format!("{c} {n}")).collect();
            if !top.is_empty() {
                s += &format!(" {}", top.join(", "));
                s.push('.');
            }
            s
        }
        Query::JumpRange => match g.max_jump_range {
            Some(r) => format!("Jump range: {r:.1} light years."),
            None => "No jump range data yet.".into(),
        },
        Query::Location => {
            let Some(sys) = &g.system else {
                return "Location not known yet.".into();
            };
            let mut s = format!("System {sys}.");
            if let Some(b) = &g.body {
                s += &format!(" Body {b}.");
            }
            if let Some(stn) = &g.station {
                s += &format!(" Docked at {stn}.");
            }
            s
        }
        Query::Jumps => {
            if g.route.len() < 2 {
                return "No route plotted.".into();
            }
            let next = &g.route[1];
            let left = g.route.len() - 1;
            format!("Next system {next}. {left} jumps remaining.")
        }
        Query::Hull => match g.hull_health {
            Some(h) => format!("Hull: {:.0} percent.", (h * 100.0).clamp(0.0, 100.0)),
            None => "No hull data yet.".into(),
        },
        Query::Shields => match g.flags() {
            None => "No shield data yet.".into(),
            Some(f) if f & flags::FLAG_SHIELDS_UP != 0 => "Shields are up.".into(),
            Some(_) => "Shields are down.".into(),
        },
        Query::Credits => {
            let mut s = match g.credits {
                Some(c) => format!("Credits: {c}."),
                None => "No credit data yet.".into(),
            };
            if let Some(r) = g.rebuy {
                if s.starts_with("No credit") {
                    s = format!("Rebuy: {r}.");
                } else {
                    s += &format!(" Rebuy: {r}.");
                }
            }
            s
        }
        Query::Ship => {
            let kind = g.ship.clone().unwrap_or_else(|| "unknown ship".into());
            match (&g.ship_name, &g.ship_ident) {
                (Some(n), Some(i)) => format!("{n} ({i}), {kind}."),
                (Some(n), None) => format!("{n}, {kind}."),
                (None, Some(i)) => format!("{kind} ({i})."),
                (None, None) => format!("{kind}."),
            }
        }
        Query::Legal => match &g.legal_state {
            Some(st) => format!("Legal state: {st}."),
            None => "No legal-state data yet.".into(),
        },
    }
}

/// Turn a spoken-confirm plan into a press (after the user said yes).
pub fn authorize_plan(p: Plan) -> Plan {
    match p {
        Plan::Confirm { action, verify, confirm, failure, .. } => Plan::Press { action, verify, confirm, failure },
        other => other,
    }
}

/// Policy tool name for a call (dedupe / logs).
pub fn tool_name(call: ToolCall) -> &'static str {
    match call {
        ToolCall::Toggle { toggle, .. } => toggle.spec().name,
        ToolCall::Screen(s) => s.name(),
        ToolCall::ThrottleZero => "set_speed_zero",
        ToolCall::Query(_) => "get_status",
        ToolCall::Nearest(_) => "find_nearest_site",
    }
}

pub fn action_noun(call: ToolCall) -> &'static str {
    match call {
        ToolCall::Toggle { toggle, .. } => toggle.spec().noun,
        ToolCall::Screen(s) => s.noun(),
        ToolCall::ThrottleZero => "Throttle",
        ToolCall::Query(_) => "Status",
        ToolCall::Nearest(_) => "Site lookup",
    }
}

pub fn elite_action(call: ToolCall) -> Option<&'static str> {
    match call {
        ToolCall::Toggle { toggle, .. } => Some(toggle.spec().action),
        ToolCall::Screen(s) => Some(s.action()),
        ToolCall::ThrottleZero => Some("SetSpeedZero"),
        ToolCall::Query(_) | ToolCall::Nearest(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::flags::*;
    use super::*;

    const FLYING: u64 = IN_MAIN_SHIP;

    fn gear(t: Target) -> ToolCall {
        ToolCall::Toggle { toggle: Toggle::LandingGear, target: t }
    }

    #[test]
    fn commands_map() {
        assert_eq!(from_command("landing_gear.down"), Some(gear(Target::On)));
        assert_eq!(
            from_command("cargo_scoop.close"),
            Some(ToolCall::Toggle { toggle: Toggle::CargoScoop, target: Target::Off })
        );
        assert_eq!(from_command("request_docking"), None);
        assert_eq!(
            from_command("flight_assist.off"),
            Some(ToolCall::Toggle { toggle: Toggle::FlightAssist, target: Target::Off })
        );
        assert_eq!(
            from_command("hud.analysis"),
            Some(ToolCall::Toggle { toggle: Toggle::HudAnalysis, target: Target::On })
        );
        assert_eq!(from_command("galaxy_map.open"), Some(ToolCall::Screen(Screen::GalaxyMap)));
        assert_eq!(from_command("throttle.zero"), Some(ToolCall::ThrottleZero));
        assert_eq!(from_command("status.fuel"), Some(ToolCall::Query(Query::Fuel)));
        assert_eq!(from_command("sites.crystalline_shards"), Some(ToolCall::Nearest("crystalline_shards")));
        assert_eq!(from_command("sites.nowhere"), None);
        assert_eq!(plan_with(ToolCall::Nearest("brain_trees"), &GameState::default()), Plan::Lookup("brain_trees"));
    }

    #[test]
    fn gear_state_rules() {
        assert!(matches!(plan(gear(Target::On), Some(FLYING)), Plan::Press { .. }));
        assert_eq!(
            plan(gear(Target::On), Some(FLYING | FLAG_GEAR)),
            Plan::Already("Landing gear is already deployed.".into())
        );
        assert_eq!(plan(gear(Target::Off), Some(FLYING)), Plan::Already("Landing gear is already retracted.".into()));
        match plan(gear(Target::Toggle), Some(FLYING | FLAG_GEAR)) {
            Plan::Press { verify: Verify::Flag { expect_set: false, .. }, .. } => {}
            p => panic!("{p:?}"),
        }
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
        let lights = ToolCall::Toggle { toggle: Toggle::Lights, target: Target::On };
        assert!(matches!(plan(lights, Some(IN_SRV)), Plan::Press { .. }));
        assert!(matches!(plan(lights, Some(FLYING | FLAG_DOCKED)), Plan::Press { .. }), "lights work docked");
        let hp = ToolCall::Toggle { toggle: Toggle::Hardpoints, target: Target::On };
        assert!(matches!(plan(hp, Some(FLYING | FLAG_SUPERCRUISE)), Plan::Refused(_)));
        assert_eq!(plan(hp, Some(FLYING | HARDPOINTS)), Plan::Already("Hardpoints are already deployed.".into()));
        let scoop = ToolCall::Toggle { toggle: Toggle::CargoScoop, target: Target::Off };
        assert!(matches!(
            plan(scoop, Some(FLYING | FLAG_SUPERCRUISE | CARGO_SCOOP)),
            Plan::Press { verify: Verify::Flag { expect_set: false, .. }, .. }
        ));
        let nv = ToolCall::Toggle { toggle: Toggle::NightVision, target: Target::On };
        assert_eq!(plan(nv, Some(FLYING | FLAG_NIGHT_VISION)), Plan::Already("Night vision is already on.".into()));
    }

    #[test]
    fn flight_assist_flag_is_inverted() {
        let on = ToolCall::Toggle { toggle: Toggle::FlightAssist, target: Target::On };
        // Bit 5 set = FA off, so "on" must clear the bit.
        match plan(on, Some(FLYING | FLAG_FLIGHT_ASSIST_OFF)) {
            Plan::Press { verify: Verify::Flag { expect_set: false, .. }, confirm, .. } => {
                assert_eq!(confirm, "Flight assist on.");
            }
            p => panic!("{p:?}"),
        }
        assert!(matches!(plan(on, Some(FLYING)), Plan::Already(_)));
    }

    #[test]
    fn silent_running_on_asks() {
        let on = ToolCall::Toggle { toggle: Toggle::SilentRunning, target: Target::On };
        assert!(matches!(plan(on, Some(FLYING)), Plan::Confirm { .. }));
        let off = ToolCall::Toggle { toggle: Toggle::SilentRunning, target: Target::Off };
        assert!(matches!(plan(off, Some(FLYING | FLAG_SILENT_RUNNING)), Plan::Press { .. }));
    }

    #[test]
    fn screens_and_throttle() {
        let mut g = GameState { status_flags: FLYING, status_seen: true, gui_focus: Some(0), ..Default::default() };
        match plan_with(ToolCall::Screen(Screen::GalaxyMap), &g) {
            Plan::Press { verify: Verify::GuiFocus(6), .. } => {}
            p => panic!("{p:?}"),
        }
        g.gui_focus = Some(6);
        assert!(matches!(plan_with(ToolCall::Screen(Screen::GalaxyMap), &g), Plan::Already(_)));
        g.body = Some("Earth".into());
        g.supercruise = true;
        g.status_flags |= FLAG_SUPERCRUISE;
        assert!(matches!(plan_with(ToolCall::ThrottleZero, &g), Plan::Refused(m) if m.contains("body")));
        g.body = None;
        assert!(matches!(plan_with(ToolCall::ThrottleZero, &g), Plan::Press { verify: Verify::None, .. }));
    }

    #[test]
    fn status_answers() {
        let mut g = GameState {
            status_flags: FLYING | FLAG_SHIELDS_UP,
            status_seen: true,
            system: Some("Sol".into()),
            fuel_main: Some(16.0),
            fuel_reservoir: Some(0.5),
            fuel_capacity_main: Some(32.0),
            hull_health: Some(0.8),
            credits: Some(1_000_000),
            ship: Some("krait_mkii".into()),
            ship_name: Some("Test".into()),
            legal_state: Some("Clean".into()),
            route: vec!["Sol".into(), "Barnard's Star".into(), "Sirius".into()],
            ..Default::default()
        };
        assert!(answer(Query::Fuel, &g).contains("16.0"));
        assert!(answer(Query::Fuel, &g).contains("50 percent"));
        assert_eq!(answer(Query::Location, &g), "System Sol.");
        assert!(answer(Query::Jumps, &g).contains("Barnard's Star"));
        assert_eq!(answer(Query::Hull, &g), "Hull: 80 percent.");
        assert_eq!(answer(Query::Shields, &g), "Shields are up.");
        g.cargo_tonnes = Some(4.0);
        g.cargo_capacity = Some(96);
        g.cargo_items = vec![("Gold".into(), 3)];
        assert!(answer(Query::Cargo, &g).contains("4 of 96"));
        assert!(answer(Query::Ship, &g).contains("Test"));
        assert!(answer(Query::Legal, &g).contains("Clean"));
        assert!(answer(Query::Credits, &g).contains("1000000"));
    }
}
