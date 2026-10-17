//! High-value discovery callout (`discovery`): on a `Scan` of a planet worth flying to, its class
//! and what it is worth mapped: "Earth-like world. 4.4 million mapped." The game shows the class
//! in the FSS but never the value, and a find among dozens of bodies is easy to miss.
//!
//! Called out: Earth-like, water and ammonia worlds, any terraformable planet, and any other
//! planet worth mapping (at least [`WORTH_MAPPING`] mapped efficiently). The value is the
//! efficient mapping value with the first-discovery bonus when nobody had discovered it
//! (`astromech_exploration::value`). Once per body per system visit, so the automatic scan and
//! a later detailed scan of the same body give one line. Nav beacon scans (the system's data
//! from its beacon, not a discovery) and stars stay quiet, and so does `FSSAllBodiesFound`,
//! which the FSS shows itself.

use crate::{Callout, Channels, Input, Priority, Rule};
use astromech_exploration::system::class_name;
use astromech_exploration::value::{estimate, terraformable, WORTH_MAPPING};
use astromech_gamestate::GameState;
use astromech_journal::{ExtraEvent, JournalEvent, ScanBody};
use std::sync::Mutex;
use std::time::Duration;

/// Always worth hearing about, whatever their value.
const PRIZES: &[&str] = &["Earthlike body", "Water world", "Ammonia world"];

/// "4.4 million", "720 thousand".
pub fn credits(v: i64) -> String {
    let tenths = (v as f64 / 100_000.0).round() / 10.0;
    if tenths >= 1.0 {
        let m = format!("{tenths:.1}");
        format!("{} million", m.trim_end_matches(".0"))
    } else {
        format!("{} thousand", (v as f64 / 1000.0).round() as i64)
    }
}

/// The line for a scanned body, or `None` when it isn't a find worth calling out.
pub fn line(b: &ScanBody) -> Option<String> {
    if b.star_type.is_some() || b.scan_type.eq_ignore_ascii_case("NavBeaconDetail") {
        return None;
    }
    let class = b.planet_class.as_deref().filter(|c| !c.is_empty())?;
    let (_, mapped) = estimate(b, None)?;
    let prize = PRIZES.contains(&class);
    // Gas giants are never terraformable in practice; an Earth-like world always is.
    let tf = terraformable(b) && class != "Earthlike body" && !class.contains("giant");
    if !(prize || tf || mapped >= WORTH_MAPPING) {
        return None;
    }
    let name = class_name(class, None);
    let name = if tf { format!("Terraformable {}", name.to_lowercase()) } else { name };
    Some(format!("{name}. {} mapped.", credits(mapped)))
}

/// The bodies called out in one system.
#[derive(Default)]
struct Said {
    system: Option<i64>,
    bodies: Vec<String>,
}

#[derive(Default)]
pub struct Discovery {
    said: Mutex<Said>,
}

impl Rule for Discovery {
    fn id(&self) -> &'static str {
        "discovery"
    }
    /// Once per body is the rule's own; several finds in one honk each get their line.
    fn cooldown(&self) -> Duration {
        Duration::ZERO
    }
    fn react(&self, input: &Input, _state: &GameState) -> Option<Callout> {
        let Input::Journal(l) = input else { return None };
        let mut said = self.said.lock().unwrap();
        let b = match &l.event {
            JournalEvent::FsdJump { .. } => {
                *said = Said::default();
                return None;
            }
            JournalEvent::Extra(ExtraEvent::Scan(b)) => b,
            _ => return None,
        };
        let text = line(b)?;
        if b.system_address.is_some() && said.system != b.system_address {
            *said = Said { system: b.system_address, bodies: vec![] };
        }
        if said.bodies.contains(&b.body_name) {
            return None;
        }
        said.bodies.push(b.body_name.clone());
        Some(Callout { text, priority: Priority::Normal, channels: Channels::SPEECH | Channels::TOAST })
    }
}
