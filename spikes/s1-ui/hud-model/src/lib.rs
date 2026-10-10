//! Shared fake "ship state" for the S1 UI spike.
//!
//! Everything is a pure function of time `t` (seconds since start), so every
//! UI library renders the *same* frame for the same `t`, which keeps the
//! screenshots and offscreen timings comparable. Nothing here touches the game.

pub const ACCENT: [u8; 3] = [0xFF, 0x8C, 0x1A]; // Elite orange
pub const ACCENT_DIM: [u8; 3] = [0x8A, 0x4A, 0x0C];
pub const BG: [u8; 3] = [0x07, 0x09, 0x0D];
pub const PANEL: [u8; 3] = [0x0E, 0x12, 0x19];
pub const PANEL_HI: [u8; 3] = [0x16, 0x1C, 0x26];
pub const LINE: [u8; 3] = [0x26, 0x2F, 0x3D];
pub const TEXT: [u8; 3] = [0xE6, 0xE9, 0xEE];
pub const TEXT_DIM: [u8; 3] = [0x8A, 0x94, 0xA6];
pub const CYAN: [u8; 3] = [0x3D, 0xD6, 0xF5];
pub const GREEN: [u8; 3] = [0x4A, 0xE0, 0x8A];
pub const RED: [u8; 3] = [0xFF, 0x4D, 0x4D];

pub const OFFSCREEN_W: u32 = 1024;
pub const OFFSCREEN_H: u32 = 768;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Toggle {
    pub on: bool,
    /// The copilot pressed the key less than ~1 s ago and Status.json hasn't
    /// confirmed yet (S5: ~1 s rewrite cycle). Shown as "VERIFYING".
    pub pending: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FsdState {
    Ready,
    Charging,
    Jumping,
    Cooldown,
    MassLocked,
}

impl FsdState {
    pub fn label(self) -> &'static str {
        match self {
            FsdState::Ready => "READY",
            FsdState::Charging => "CHARGING",
            FsdState::Jumping => "HYPERSPACE",
            FsdState::Cooldown => "COOLDOWN",
            FsdState::MassLocked => "MASS LOCKED",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ptt {
    Idle,
    Listening,
    Thinking,
}

impl Ptt {
    pub fn label(self) -> &'static str {
        match self {
            Ptt::Idle => "PUSH TO TALK",
            Ptt::Listening => "LISTENING",
            Ptt::Thinking => "PROCESSING",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speaker {
    Cmdr,
    Copilot,
    System,
}

impl Speaker {
    pub fn label(self) -> &'static str {
        match self {
            Speaker::Cmdr => "CMDR",
            Speaker::Copilot => "HELM",
            Speaker::System => "SYS",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LogEntry {
    pub time: String,
    pub speaker: Speaker,
    pub text: &'static str,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Snapshot {
    pub commander: &'static str,
    pub ship_type: &'static str,
    pub ship_name: &'static str,
    pub ship_ident: &'static str,
    pub system: &'static str,
    pub body: &'static str,
    pub clock: String,
    pub gear: Toggle,
    pub lights: Toggle,
    pub hardpoints: Toggle,
    pub cargo_scoop: Toggle,
    pub night_vision: Toggle,
    pub flight_assist: Toggle,
    pub fsd: FsdState,
    /// 0..1, meaningful for Charging / Cooldown.
    pub fsd_progress: f32,
    pub fuel_main: f32,
    pub fuel_capacity: f32,
    pub fuel_reservoir: f32,
    pub shields: f32,
    pub hull: f32,
    pub heat: f32,
    pub ptt: Ptt,
    /// 0..1 mic level for the PTT meter.
    pub mic_level: f32,
    /// 0..1 phase of a 1.2 s pulse, for "listening" glow animations.
    pub pulse: f32,
    pub log: Vec<LogEntry>,
}

const LINES: &[(Speaker, &str)] = &[
    (Speaker::Cmdr, "Gear down."),
    (Speaker::Copilot, "Landing gear deploying."),
    (Speaker::System, "Status: gear deployed (verified 0.62 s)."),
    (Speaker::Cmdr, "How much fuel do we have?"),
    (Speaker::Copilot, "Main tank at 71 percent. Enough for about nine jumps at current mass."),
    (Speaker::Cmdr, "Lights."),
    (Speaker::Copilot, "Ship lights on."),
    (Speaker::System, "Docking request granted: pad 23."),
    (Speaker::Cmdr, "Plot a route to Colonia."),
    (Speaker::Copilot, "That's 22,000 light years. Say confirm or press your confirm button to start plotting."),
    (Speaker::System, "Awaiting confirmation (T2 action)."),
    (Speaker::Cmdr, "Cancel."),
    (Speaker::Copilot, "Plotting cancelled. Nothing was pressed."),
    (Speaker::Cmdr, "Hardpoints."),
    (Speaker::Copilot, "Hardpoints deploying. Mass lock in effect."),
    (Speaker::System, "Status: hardpoints deployed (verified 0.31 s)."),
    (Speaker::Cmdr, "Retract hardpoints and open the cargo scoop."),
    (Speaker::Copilot, "Hardpoints retracting. Cargo scoop opening."),
    (Speaker::System, "Warning: heat at 82 percent while fuel scooping."),
    (Speaker::Copilot, "Heat is climbing. I'd throttle back a little."),
    (Speaker::Cmdr, "What system are we in?"),
    (Speaker::Copilot, "Shinrarta Dezhra. You're docked at Jameson Memorial."),
];

fn frac(x: f64) -> f64 {
    x - x.floor()
}

fn toggle(t: f64, period: f64, offset: f64) -> Toggle {
    let phase = (t + offset) / period;
    let on = (phase.floor() as i64) % 2 == 1;
    let since_flip = frac(phase) * period;
    Toggle { on, pending: since_flip < 0.9 && t > 1.0 }
}

fn hms(t: f64) -> String {
    // Fixed base clock 20:14:00 so frames are deterministic.
    let s = (20 * 3600 + 14 * 60) as u64 + t.max(0.0) as u64;
    format!("{:02}:{:02}:{:02}", (s / 3600) % 24, (s / 60) % 60, s % 60)
}

/// Deterministic snapshot at time `t` seconds.
pub fn snapshot(t: f64) -> Snapshot {
    // FSD: 14 s cycle: ready 4 s, charging 4 s, hyperspace 2 s, cooldown 4 s.
    let c = frac(t / 14.0) * 14.0;
    let (fsd, fsd_progress) = if c < 4.0 {
        (FsdState::Ready, 0.0)
    } else if c < 8.0 {
        (FsdState::Charging, ((c - 4.0) / 4.0) as f32)
    } else if c < 10.0 {
        (FsdState::Jumping, 1.0)
    } else {
        (FsdState::Cooldown, ((c - 10.0) / 4.0) as f32)
    };
    let hard = toggle(t, 11.0, 3.0);
    let fsd = if hard.on && fsd == FsdState::Ready { FsdState::MassLocked } else { fsd };

    // PTT: 8 s cycle: listening 2.2 s, thinking 0.8 s, idle.
    let p = frac(t / 8.0) * 8.0;
    let ptt = if p < 2.2 {
        Ptt::Listening
    } else if p < 3.0 {
        Ptt::Thinking
    } else {
        Ptt::Idle
    };
    let mic_level = if ptt == Ptt::Listening {
        (0.35 + 0.3 * (t * 9.0).sin().abs() + 0.25 * (t * 23.0).sin().abs()).min(1.0) as f32
    } else {
        0.0
    };

    let fuel_cap = 32.0f32;
    let fuel_main = fuel_cap * (0.25 + 0.7 * (0.5 + 0.5 * (t / 30.0).cos())) as f32;

    // Conversation: one new line every 2.5 s, starting with 6 lines.
    let n = 6 + (t / 2.5).floor() as usize;
    let log = (0..n)
        .map(|i| {
            let (speaker, text) = LINES[i % LINES.len()];
            LogEntry { time: hms(i as f64 * 2.5 - 15.0 + 15.0), speaker, text }
        })
        .collect();

    Snapshot {
        commander: "CMDR SUMEET",
        ship_type: "PYTHON MK II",
        ship_name: "Kestrel's Due",
        ship_ident: "SP-07",
        system: "Shinrarta Dezhra",
        body: "Jameson Memorial",
        clock: hms(t),
        gear: toggle(t, 7.0, 0.0),
        lights: toggle(t, 5.0, 2.0),
        hardpoints: hard,
        cargo_scoop: toggle(t, 13.0, 5.0),
        night_vision: toggle(t, 17.0, 1.0),
        flight_assist: Toggle { on: true, pending: false },
        fsd,
        fsd_progress,
        fuel_main,
        fuel_capacity: fuel_cap,
        fuel_reservoir: 0.42 + 0.05 * (t / 3.0).sin() as f32,
        shields: (0.78 + 0.22 * (t / 4.0).sin()).clamp(0.0, 1.0) as f32,
        hull: 0.87,
        heat: (0.35 + 0.3 * (t / 6.0).sin().abs()) as f32,
        ptt,
        mic_level,
        pulse: frac(t / 1.2) as f32,
        log,
    }
}

/// Simple stats helper shared by all offscreen benchmarks.
pub fn stats_ms(samples: &mut [f64]) -> String {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = samples.len();
    let mean = samples.iter().sum::<f64>() / n as f64;
    let pct = |p: f64| samples[((n as f64 - 1.0) * p).round() as usize];
    format!(
        "n={n} mean={mean:.2}ms median={:.2}ms p95={:.2}ms min={:.2}ms max={:.2}ms",
        pct(0.5),
        pct(0.95),
        samples[0],
        samples[n - 1]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        assert_eq!(snapshot(12.3), snapshot(12.3));
        assert!(snapshot(100.0).log.len() > 40);
    }
}
