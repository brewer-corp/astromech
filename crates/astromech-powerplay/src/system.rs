//! The current system's Powerplay state from the arrival event (`FSDJump`, `CarrierJump`,
//! `Location`): who controls it, the state (exploited, fortified, stronghold, or unoccupied
//! with acquisition progress), progress through the state, reinforcement against undermining
//! this cycle, and the other powers active there. [`SystemPower::spoken`] is the arrival line.
//!
//! The game shows this only in the system and galaxy maps, not on the HUD, so it is fit to
//! speak on arrival (strategy: voice only for what the HUD doesn't show prominently).

use astromech_journal::{JournalEvent, PowerProgress, SystemPowerplay};
use serde::{Deserialize, Serialize};

/// What the system is for the commander's power.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Work {
    /// Controlled by my power.
    Reinforce,
    /// Controlled by another power.
    Undermine,
    /// Unoccupied and my power is acquiring it.
    Acquire,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PowerShare {
    pub power: String,
    pub progress: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SystemPower {
    pub system: String,
    pub controlling_power: Option<String>,
    /// As the journal writes it: `Exploited`, `Fortified`, `Stronghold`, `Unoccupied`, ...
    pub state: Option<String>,
    /// Progress through the state's band, a fraction (may be below 0 or above 1).
    pub control_progress: Option<f64>,
    pub reinforcement: Option<i64>,
    pub undermining: Option<i64>,
    /// Powers other than the controller active here.
    pub contesting: Vec<String>,
    /// Unoccupied systems: acquisition progress, highest first.
    pub acquisition: Vec<PowerShare>,
}

fn pct(f: f64) -> String {
    format!("{:.0} percent", f * 100.0)
}

impl SystemPower {
    /// The Powerplay state after a system event; `None` for other events and for systems with
    /// no Powerplay activity.
    pub fn from_event(ev: &JournalEvent) -> Option<Option<SystemPower>> {
        let (system, p) = match ev {
            JournalEvent::FsdJump { star_system, powerplay, .. }
            | JournalEvent::Location { star_system, powerplay, .. } => (star_system, powerplay),
            _ => return None,
        };
        Some(p.as_deref().and_then(|p| SystemPower::new(system, p)))
    }

    pub fn new(system: &str, p: &SystemPowerplay) -> Option<SystemPower> {
        let mut acquisition: Vec<PowerShare> = p
            .conflict
            .iter()
            .map(|PowerProgress { power, progress }| PowerShare { power: power.clone(), progress: *progress })
            .collect();
        acquisition.sort_by(|a, b| b.progress.total_cmp(&a.progress));
        let contesting: Vec<String> =
            p.powers.iter().filter(|x| Some(*x) != p.controlling_power.as_ref()).cloned().collect();
        if p.controlling_power.is_none() && contesting.is_empty() && acquisition.is_empty() {
            return None;
        }
        Some(SystemPower {
            system: system.to_string(),
            controlling_power: p.controlling_power.clone(),
            state: p.state.clone(),
            control_progress: p.control_progress,
            reinforcement: p.reinforcement,
            undermining: p.undermining,
            contesting,
            acquisition,
        })
    }

    /// What the system offers `pledged`, if anything.
    pub fn work_for(&self, pledged: Option<&str>) -> Option<Work> {
        let me = pledged?;
        match &self.controlling_power {
            Some(c) if c == me => Some(Work::Reinforce),
            Some(_) => Some(Work::Undermine),
            None => (self.acquisition.iter().any(|a| a.power == me) || self.contesting.iter().any(|p| p == me))
                .then_some(Work::Acquire),
        }
    }

    /// One line for the screen, e.g. "Li Yong-Rui stronghold, 25% · reinforcement 0, undermining 166".
    pub fn summary(&self) -> String {
        let mut parts = vec![];
        match (&self.controlling_power, &self.state) {
            (Some(c), Some(s)) => parts.push(format!("{c} {}", s.to_lowercase())),
            (Some(c), None) => parts.push(c.clone()),
            (None, Some(s)) => parts.push(s.clone()),
            (None, None) => parts.push("Unoccupied".into()),
        }
        if self.controlling_power.is_some() {
            if let Some(p) = self.control_progress {
                parts[0] += &format!(", {:.0}%", p * 100.0);
            }
            if let (Some(r), Some(u)) = (self.reinforcement, self.undermining) {
                parts.push(format!("reinforcement {r}, undermining {u}"));
            }
            if !self.contesting.is_empty() {
                parts.push(format!("also {}", self.contesting.join(", ")));
            }
        } else if !self.acquisition.is_empty() {
            let a: Vec<String> =
                self.acquisition.iter().map(|a| format!("{} {:.0}%", a.power, a.progress * 100.0)).collect();
            parts.push(format!("acquisition: {}", a.join(", ")));
        }
        parts.join(" · ")
    }

    /// The arrival line, in the COVAS register: short, numbers rounded, and what it means for
    /// the commander's power last.
    pub fn spoken(&self, pledged: Option<&str>) -> String {
        let mut s = match (&self.controlling_power, &self.state) {
            (Some(c), Some(st)) => format!("{c} {}", st.to_lowercase()),
            (Some(c), None) => format!("Controlled by {c}"),
            // No controller: the state as the journal names it (`Unoccupied`, `Contested`).
            (None, Some(st)) => st.clone(),
            (None, None) => "Unoccupied".to_string(),
        };
        if self.controlling_power.is_some() {
            if let Some(p) = self.control_progress {
                s += &format!(", {}", pct(p));
            }
            s.push('.');
            if let (Some(r), Some(u)) = (self.reinforcement, self.undermining) {
                s += &format!(" Reinforcement {r}, undermining {u}.");
            }
            if !self.contesting.is_empty() {
                s += &format!(" Also active: {}.", self.contesting.join(", "));
            }
        } else {
            s.push('.');
            if !self.acquisition.is_empty() {
                let a: Vec<String> =
                    self.acquisition.iter().map(|a| format!("{} {}", a.power, pct(a.progress))).collect();
                s += &format!(" Acquisition: {}.", a.join(", "));
            }
        }
        match self.work_for(pledged) {
            Some(Work::Reinforce) => s += " Your power: reinforce here.",
            Some(Work::Undermine) => s += " Undermining target.",
            Some(Work::Acquire) => s += " Acquisition target.",
            None => {}
        }
        s
    }
}
