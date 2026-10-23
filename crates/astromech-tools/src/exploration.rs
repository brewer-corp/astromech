//! Exploration questions by voice (read-only, tier 0): "what is this system worth", "is this
//! system worth mapping", "which organics haven't I scanned here", "how much is my unsold data
//! worth". The core folds the current system the way the app's Exploration pages do
//! (`astromech_exploration::system` over the newest journals, with earlier visits from the
//! history; `astromech_exploration::biology` over that fold) and reads the unsold data from the
//! history (`History::unsold`); this turns them into short spoken answers.
//!
//! Values are never shown by the game until the data is sold, and neither is the system's
//! biology across bodies, so these say both; scan progress is said only while the FSS has not
//! found everything.

use astromech_announce::discovery::credits as short_credits;
use astromech_announce::session_summary::Unreadable;
use astromech_exploration::biology::BodyBio;
use astromech_exploration::system::{BodyView, SystemView};
use astromech_exploration::Unsold;

/// An exploration question the grammar recognises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ask {
    /// "What is this system worth?"
    SystemValue,
    /// "Is this system worth mapping?"
    WorthMapping,
    /// "Which organics haven't I scanned here?"
    Biology,
    /// "How much is my unsold data worth?"
    Unsold,
}

/// The exploration question behind an `astromech-intent` command id.
pub fn from_command(command: &str) -> Option<Ask> {
    match command {
        "exploration.system_value" => Some(Ask::SystemValue),
        "exploration.worth_mapping" => Some(Ask::WorthMapping),
        "exploration.biology" => Some(Ask::Biology),
        "exploration.unsold" => Some(Ask::Unsold),
        _ => None,
    }
}

/// What the voice tester shows instead of reading the journals.
pub fn describe(ask: Ask) -> String {
    match ask {
        Ask::SystemValue => "Reads this system's scans from your journals and says what they are worth.",
        Ask::WorthMapping => "Reads this system's scans from your journals and says which bodies are worth mapping.",
        Ask::Biology => "Reads this system's biology from your journals and says what is left to sample.",
        Ask::Unsold => "Reads your journal history and says what your unsold data is worth.",
    }
    .into()
}

/// Said before the journal has named the current system.
pub const NO_SYSTEM: &str = "The current system is not known yet. It comes from the next jump or login.";

/// Said in a system with nothing scanned.
pub const NOTHING_SCANNED: &str = "Nothing scanned in this system yet.";

/// Said when there is no journal history to read the unsold data from.
pub const NO_HISTORY: &str = "No journal history yet.";

/// Said when the history exists but could not be read.
pub const CANT_READ: &str = "Journal history is unreadable right now.";

/// The most bodies an answer names; the rest are counted.
const MAX_BODIES: usize = 3;

/// The most genera named for one body; the rest are counted.
const MAX_GENERA: usize = 4;

/// "1.2 billion credits", "34 million credits", "578 thousand credits", "900 credits": an
/// expedition's unsold data runs into billions.
pub fn credits(c: i64) -> String {
    let trim = |v: f64| format!("{v:.1}").trim_end_matches(".0").to_string();
    if c >= 999_950_000 {
        format!("{} billion credits", trim(c as f64 / 1e9))
    } else if c >= 999_500 {
        format!("{} million credits", trim(c as f64 / 1e6))
    } else if c >= 10_000 {
        format!("{:.0} thousand credits", c as f64 / 1e3)
    } else if c == 1 {
        "1 credit".into()
    } else {
        format!("{c} credits")
    }
}

/// A body's line: "A 1: Earth-like world, 4.4 million mapped." A name that is not short (a body
/// named apart from its system's catalogue name) is a sentence of its own, as the copy guide
/// asks: "Col 285 Sector AB-C d14-123 A 1. Earth-like world, 4.4 million mapped."
fn body_line(name: &str, rest: &str) -> String {
    if name.split_whitespace().count() > 3 {
        let mut c = rest.chars();
        let rest = c.next().map(|f| f.to_uppercase().chain(c).collect::<String>()).unwrap_or_default();
        format!("{name}. {rest}.")
    } else {
        format!("{name}: {rest}.")
    }
}

/// The sentences joined into one answer.
fn join(lines: Vec<String>) -> String {
    lines.join(" ")
}

/// "5 of 12 bodies scanned." while the FSS has not found everything the honk counted.
fn progress(v: &SystemView) -> Option<String> {
    let total = v.body_count.filter(|_| !v.all_found)?;
    let scanned = v.bodies.len() as i64;
    (scanned < total).then(|| format!("{scanned} of {total} bodies scanned."))
}

/// Why there is nothing to say about the system, if there is nothing.
fn empty(v: &SystemView) -> Option<String> {
    if v.system.is_none() && v.bodies.is_empty() {
        return Some(NO_SYSTEM.into());
    }
    v.bodies.is_empty().then(|| NOTHING_SCANNED.into())
}

/// "Worth 1.2 million credits now. 4.5 million credits mapped." and the scan progress.
pub fn system_value(v: &SystemView) -> String {
    if let Some(s) = empty(v) {
        return s;
    }
    let mut lines = vec![format!("Worth {} now.", credits(v.value))];
    if v.value_if_mapped > v.value {
        lines.push(format!("{} with the best mapped.", credits(v.value_if_mapped)));
    } else {
        lines.push("Nothing left worth mapping.".into());
    }
    lines.extend(progress(v));
    join(lines)
}

/// "A 1: Earth-like world, 4.4 million mapped."
fn body_to_map(b: &BodyView) -> String {
    body_line(&b.short_name, &format!("{}, {} mapped", b.class_name, short_credits(b.value_mapped)))
}

/// The planets worth mapping, most valuable first, with what mapping them adds, and the scan
/// progress.
pub fn worth_mapping(v: &SystemView) -> String {
    if let Some(s) = empty(v) {
        return s;
    }
    let mut map: Vec<&BodyView> = v.bodies.iter().filter(|b| b.worth_mapping).collect();
    map.sort_by(|a, b| b.value_mapped.cmp(&a.value_mapped).then(a.name.cmp(&b.name)));
    let mut lines = vec![];
    if map.is_empty() {
        let so_far = if progress(v).is_some() { " so far" } else { "" };
        lines.push(format!("Nothing scanned{so_far} is worth mapping."));
    } else {
        let n = if map.len() == 1 { "1 body".to_string() } else { format!("{} bodies", map.len()) };
        lines.push(format!("{n} worth mapping, {} more.", credits(v.value_if_mapped - v.value)));
        lines.extend(map.iter().take(MAX_BODIES).map(|b| body_to_map(b)));
        if map.len() > MAX_BODIES {
            lines.push(format!("{} more on the System page.", map.len() - MAX_BODIES));
        }
    }
    lines.extend(progress(v));
    join(lines)
}

fn genera(n: usize) -> String {
    if n == 1 {
        "1 genus".into()
    } else {
        format!("{n} genera")
    }
}

/// What is left to sample on a body, and how many genera that is: the genera the DSS (or a
/// sample) found that are not analysed yet; before the DSS, the signals not accounted for by an
/// analysed genus. None when nothing is left.
fn left_on(b: &BodyBio, system: Option<&str>) -> Option<(usize, String)> {
    let short = astromech_announce::biology::short_name(&b.name, system);
    let analysed = b.genera.iter().filter(|g| g.known && g.analysed).count();
    if !b.mapped {
        let left = (b.signals.unwrap_or(0) as usize).saturating_sub(analysed);
        return (left > 0).then(|| (left, body_line(short, &format!("{}, not mapped", genera(left)))));
    }
    let left: Vec<&str> = b.genera.iter().filter(|g| g.known && !g.analysed).map(|g| g.genus.as_str()).collect();
    if left.is_empty() {
        return None;
    }
    let mut names = left.iter().take(MAX_GENERA).copied().collect::<Vec<_>>().join(", ");
    if left.len() > MAX_GENERA {
        names += &format!(", and {} more", left.len() - MAX_GENERA);
    }
    Some((left.len(), body_line(short, &names)))
}

/// The biology left to sample: how many genera on how many bodies, then the bodies (most
/// valuable first) with the genera left on each, and the scan progress. `bodies` is
/// `astromech_exploration::biology::ranked` over the system's fold.
pub fn biology(bodies: &[BodyBio], v: &SystemView) -> String {
    if bodies.is_empty() {
        if let Some(s) = empty(v) {
            return s;
        }
    }
    let mut lines = vec![];
    let left: Vec<(usize, String)> = bodies.iter().filter_map(|b| left_on(b, v.system.as_deref())).collect();
    if bodies.is_empty() {
        let so_far = if progress(v).is_some() { " yet" } else { "" };
        lines.push(format!("No biology found in this system{so_far}."));
    } else if left.is_empty() {
        lines.push("All biology found here is sampled.".into());
    } else {
        let total: usize = left.iter().map(|(n, _)| n).sum();
        let on = if left.len() == 1 { "1 body".to_string() } else { format!("{} bodies", left.len()) };
        lines.push(format!("{} left to sample on {on}.", genera(total)));
        lines.extend(left.iter().take(MAX_BODIES).map(|(_, l)| l.clone()));
        if left.len() > MAX_BODIES {
            lines.push(format!("{} more on the Biology page.", left.len() - MAX_BODIES));
        }
    }
    lines.extend(progress(v));
    join(lines)
}

/// The unsold data (`Ok(None)`: no history yet; `Err`: it could not be read): the total, then
/// cartographic and organic apart when there are both, and samples of unknown value.
pub fn unsold(u: Result<Option<&Unsold>, Unreadable>) -> String {
    let u = match u {
        Err(Unreadable) => return CANT_READ.into(),
        Ok(None) => return NO_HISTORY.into(),
        Ok(Some(u)) => u,
    };
    if u.bodies == 0 && u.samples == 0 {
        return "No unsold data.".into();
    }
    let mut lines = vec![format!("Unsold data worth {}.", credits(u.total))];
    if u.cartographic > 0 && u.organic > 0 {
        lines.push(format!("Cartographic {}.", credits(u.cartographic)));
        lines.push(format!("Organic {}.", credits(u.organic)));
    }
    match u.unknown_samples {
        0 => {}
        1 => lines.push("1 sample of unknown value.".into()),
        n => lines.push(format!("{n} samples of unknown value.")),
    }
    join(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands() {
        assert_eq!(from_command("exploration.system_value"), Some(Ask::SystemValue));
        assert_eq!(from_command("exploration.worth_mapping"), Some(Ask::WorthMapping));
        assert_eq!(from_command("exploration.biology"), Some(Ask::Biology));
        assert_eq!(from_command("exploration.unsold"), Some(Ask::Unsold));
        assert_eq!(from_command("status.location"), None);
    }

    #[test]
    fn unsold_says_the_total_then_the_parts() {
        let u = Unsold {
            cartographic: 30_120_000,
            organic: 4_100_000,
            total: 34_220_000,
            bodies: 40,
            samples: 3,
            unknown_samples: 1,
            ..Unsold::default()
        };
        assert_eq!(
            unsold(Ok(Some(&u))),
            "Unsold data worth 34.2 million credits. Cartographic 30.1 million credits. Organic 4.1 million credits. \
             1 sample of unknown value."
        );
        let carto = Unsold { cartographic: 578_000, total: 578_000, bodies: 4, ..Unsold::default() };
        assert_eq!(unsold(Ok(Some(&carto))), "Unsold data worth 578 thousand credits.");
        assert_eq!(unsold(Ok(Some(&Unsold::default()))), "No unsold data.");
        let expedition = Unsold { organic: 1_460_000_000, total: 1_460_000_000, samples: 90, ..Unsold::default() };
        assert_eq!(unsold(Ok(Some(&expedition))), "Unsold data worth 1.5 billion credits.");
        assert_eq!(unsold(Ok(None)), NO_HISTORY);
        assert_eq!(unsold(Err(Unreadable)), CANT_READ);
    }

    #[test]
    fn an_empty_system() {
        let none = SystemView::default();
        assert_eq!(system_value(&none), NO_SYSTEM);
        assert_eq!(biology(&[], &none), NO_SYSTEM);
        let honked = SystemView { system: Some("Sol".into()), body_count: Some(12), ..SystemView::default() };
        assert_eq!(worth_mapping(&honked), NOTHING_SCANNED);
        assert_eq!(biology(&[], &honked), NOTHING_SCANNED);
    }
}
