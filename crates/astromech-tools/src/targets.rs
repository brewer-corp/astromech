//! Powerplay targets by voice (read-only, tier 0): "where can I undermine near me", "nearest
//! system to reinforce". One command per kind of work, `powerplay.<kind>`, which is also the key
//! the app's Powerplay > Where to work remembers its filters under
//! (`search.filters.powerplay.<kind>`). The search is `Galaxy::powerplay_targets`, the one that
//! page runs, for the power pledged to in the journal; this turns its result into the answer.

use astromech_galaxy::{Target, Targets};

/// How far the voice search looks: the Where to work page's default radius.
pub const WITHIN_LY: f64 = 40.0;

/// Said to a commander who is not pledged to a power.
pub const NOT_PLEDGED: &str = "Not pledged to a power, so there are no targets.";

/// The kind of work behind an `astromech-intent` command id.
pub fn from_command(command: &str) -> Option<Target> {
    match command {
        "powerplay.acquire" => Some(Target::Acquire),
        "powerplay.reinforce" => Some(Target::Reinforce),
        "powerplay.undermine" => Some(Target::Undermine),
        _ => None,
    }
}

/// The command id for `target`, and the key its filters are remembered under.
pub fn lookup(target: Target) -> &'static str {
    match target {
        Target::Acquire => "powerplay.acquire",
        Target::Reinforce => "powerplay.reinforce",
        Target::Undermine => "powerplay.undermine",
    }
}

fn verb(target: Target) -> &'static str {
    match target {
        Target::Acquire => "acquire",
        Target::Reinforce => "reinforce",
        Target::Undermine => "undermine",
    }
}

/// What the voice tester shows instead of running the search.
pub fn describe(target: Target) -> String {
    format!("Would look up the nearest system to {} for your power on Spansh and say it.", verb(target))
}

fn percent(f: f64) -> String {
    format!("{:.0} percent", f * 100.0)
}

/// "7.2 light years", "12 light years", "In this system".
fn distance(ly: f64) -> String {
    if ly < 0.05 {
        "In this system".into()
    } else if ly < 10.0 {
        format!("{ly:.1} light years")
    } else {
        format!("{ly:.0} light years")
    }
}

/// The spoken answer, a short sentence at a time: the nearest target, how far and how many
/// jumps, and its Powerplay state ("Pranav Antal exploited, 8 percent."; an acquisition target
/// gives the pledged power's progress).
pub fn spoken(t: &Targets) -> String {
    let verb = verb(t.target);
    let Some(h) = t.hits.first() else {
        return match &t.warning {
            Some(w) => w.clone(),
            None => format!("No systems to {verb} within {:.0} light years with your filters.", t.within_ly),
        };
    };
    let mut s = format!("Nearest system to {verb}: {}. {}", h.name, distance(h.hop.ly));
    if h.hop.ly >= 0.05 {
        let about = if h.hop.exact { "" } else { "about " };
        s += &format!(", {about}{} jump{}", h.hop.jumps, if h.hop.jumps == 1 { "" } else { "s" });
    }
    s.push('.');
    let state = match (&h.controlling_power, &h.state) {
        (Some(c), Some(st)) if t.target == Target::Undermine => format!("{c} {}", st.to_lowercase()),
        (Some(_), Some(st)) => st.clone(),
        (Some(c), None) => format!("Controlled by {c}"),
        (None, _) => "Unoccupied".into(),
    };
    let progress = if h.controlling_power.is_some() {
        h.control_progress.map(|p| format!(", {}", percent(p)))
    } else {
        h.acquisition.map(|p| format!(", acquisition {}", percent(p)))
    };
    s + &format!(" {state}{}.", progress.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_galaxy::{Here, Hop, TargetHit};

    fn targets(target: Target, hits: Vec<TargetHit>) -> Targets {
        Targets {
            target,
            power: "Li Yong-Rui".into(),
            here: Here { system: "Lembava".into(), id64: None, pos: [0.0; 3] },
            range: 20.0,
            within_ly: WITHIN_LY,
            fetched: String::new(),
            hits,
            warning: None,
        }
    }

    fn hit(ly: f64, jumps: u32, exact: bool, controller: Option<&str>, state: &str, progress: f64) -> TargetHit {
        TargetHit {
            name: "Pikum".into(),
            id64: 1,
            hop: Hop { ly, jumps, exact },
            controlling_power: controller.map(str::to_string),
            state: Some(state.into()),
            control_progress: controller.map(|_| progress),
            reinforcement: None,
            undermining: None,
            acquisition: controller.is_none().then_some(progress),
            others: vec![],
            updated_at: None,
        }
    }

    #[test]
    fn every_kind_has_a_command() {
        for t in [Target::Acquire, Target::Reinforce, Target::Undermine] {
            assert_eq!(from_command(lookup(t)), Some(t));
        }
        assert_eq!(from_command("status.powerplay"), None);
    }

    #[test]
    fn the_answer_says_where_how_far_and_the_state() {
        let t = targets(Target::Undermine, vec![hit(21.78, 2, true, Some("Pranav Antal"), "Exploited", 0.0765)]);
        assert_eq!(
            spoken(&t),
            "Nearest system to undermine: Pikum. 22 light years, 2 jumps. Pranav Antal exploited, 8 percent."
        );
        let t = targets(Target::Reinforce, vec![hit(5.16, 1, false, Some("Li Yong-Rui"), "Stronghold", 0.486)]);
        assert_eq!(
            spoken(&t),
            "Nearest system to reinforce: Pikum. 5.2 light years, about 1 jump. Stronghold, 49 percent."
        );
        let t = targets(Target::Reinforce, vec![hit(0.0, 0, true, Some("Li Yong-Rui"), "Stronghold", 0.25)]);
        assert_eq!(spoken(&t), "Nearest system to reinforce: Pikum. In this system. Stronghold, 25 percent.");
        let t = targets(Target::Acquire, vec![hit(10.27, 1, true, None, "Unoccupied", 0.3169)]);
        assert_eq!(
            spoken(&t),
            "Nearest system to acquire: Pikum. 10 light years, 1 jump. Unoccupied, acquisition 32 percent."
        );
    }

    #[test]
    fn nothing_found_says_so() {
        assert_eq!(
            spoken(&targets(Target::Acquire, vec![])),
            "No systems to acquire within 40 light years with your filters."
        );
    }

    #[test]
    fn the_tester_says_what_it_would_look_up() {
        assert_eq!(
            describe(Target::Undermine),
            "Would look up the nearest system to undermine for your power on Spansh and say it."
        );
    }
}
