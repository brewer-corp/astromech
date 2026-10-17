//! Faction-state lookups by voice (read-only, tier 0): "nearest famine system", "where is
//! there a boom near me". One command per `astromech_galaxy::QUERIES` lookup, `faction.<id>`,
//! which is also the key the app remembers that lookup's filters under
//! (`search.filters.faction.<id>`). The search is `Galaxy::nearby`, the one the app's Nearby
//! lookups run; this turns its result into the spoken answer.

use astromech_galaxy::{Nearby, Query, HGE_POPULATION};

const PREFIX: &str = "faction.";

/// The lookup behind an `astromech-intent` command id.
pub fn from_command(command: &str) -> Option<&'static Query> {
    astromech_galaxy::query(command.strip_prefix(PREFIX)?)
}

/// The command id for `q`, and the key its filters are remembered under.
pub fn lookup(q: &Query) -> String {
    format!("{PREFIX}{}", q.id)
}

/// What the lookup finds, as said: "famine system".
pub fn noun(q: &Query) -> &'static str {
    match q.id {
        "famine" => "famine system",
        "fed_composites" => "Federation system",
        "empire_shielding" => "Empire system",
        "proto" => "boom system",
        "military" => "war system",
        "improvised" => "civil unrest system",
        "pharma" => "outbreak system",
        _ => q.label,
    }
}

/// The lookups ranked by population (High Grade Emissions) answer with the best system, the
/// others with the nearest.
fn lead(q: &Query) -> &'static str {
    if q.by_population {
        "Best"
    } else {
        "Nearest"
    }
}

/// What the voice tester shows instead of running the lookup.
pub fn describe(q: &Query) -> String {
    format!("Would look up the {} {} on Spansh and say it.", lead(q).to_lowercase(), noun(q))
}

/// "7.2 light years", "12 light years", "in this system".
fn distance(ly: f64) -> String {
    if ly < 0.05 {
        "in this system".into()
    } else if ly < 10.0 {
        format!("{ly:.1} light years")
    } else {
        format!("{ly:.0} light years")
    }
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

/// " Reported 3 days ago." (or " Seen on your visit today." when the states are the
/// commander's own); "" when the age is unknown.
fn age(updated_at: Option<&str>, visit: bool, now_s: i64) -> String {
    let Some(t) = updated_at.and_then(|u| chrono::DateTime::parse_from_rfc3339(u).ok()) else {
        return String::new();
    };
    let when = match (now_s - t.timestamp()).div_euclid(86_400) {
        ..=0 => "today".to_string(),
        1 => "yesterday".to_string(),
        d => format!("{d} days ago"),
    };
    let how = if visit { "Seen on your visit" } else { "Reported" };
    format!(" {how} {when}.")
}

/// The spoken answer, a short sentence at a time: the first hit (the nearest, or the best for High Grade Emissions), how
/// far and how many jumps, and how old its faction states are.
pub fn spoken(n: &Nearby, now_s: i64) -> String {
    let noun = noun(&n.query);
    let Some(h) = n.hits.first() else {
        return match &n.warning {
            Some(w) => w.clone(),
            None => format!("No {noun}s near {} with your filters.", n.here.system),
        };
    };
    let mut s = format!("{} {noun}: {}. {}", lead(&n.query), h.name, capitalise(&distance(h.hop.ly)));
    if h.hop.ly >= 0.05 {
        let about = if h.hop.exact { "" } else { "about " };
        s += &format!(", {about}{} jump{}", h.hop.jumps, if h.hop.jumps == 1 { "" } else { "s" });
    }
    s.push('.');
    if n.query.by_population && h.population >= HGE_POPULATION {
        s += " Over a billion people.";
    }
    s + &age(h.updated_at.as_deref(), h.seen_on_visit, now_s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_galaxy::{Here, Hop, NearbyHit, QUERIES};

    const NOW: i64 = 1_792_152_000; // 2026-10-16T12:00:00Z

    fn nearby(id: &str, hits: Vec<NearbyHit>) -> Nearby {
        Nearby {
            query: astromech_galaxy::query(id).unwrap().clone(),
            here: Here { system: "Sol".into(), id64: None, pos: [0.0; 3] },
            range: 20.0,
            fetched: "2026-10-16T12:00:00Z".into(),
            hits,
            warning: None,
        }
    }

    fn hit(ly: f64, jumps: u32, exact: bool, population: i64, updated_at: &str) -> NearbyHit {
        NearbyHit {
            name: "Ross 154".into(),
            id64: 1,
            hop: Hop { ly, jumps, exact },
            population,
            updated_at: Some(updated_at.into()),
            factions: vec![],
            seen_on_visit: false,
            score: None,
        }
    }

    #[test]
    fn every_lookup_has_a_command_and_a_spoken_noun() {
        for q in QUERIES {
            assert_eq!(from_command(&lookup(q)), Some(q));
            assert_ne!(noun(q), q.label, "{} has no spoken noun", q.id);
        }
        assert_eq!(from_command("faction.nonsense"), None);
        assert_eq!(from_command("sites.famine"), None);
    }

    #[test]
    fn the_answer_says_where_how_far_and_how_old() {
        let n = nearby("famine", vec![hit(9.689, 1, true, 2_006_272, "2026-10-13T16:32:04Z")]);
        assert_eq!(spoken(&n, NOW), "Nearest famine system: Ross 154. 9.7 light years, 1 jump. Reported 2 days ago.");
        let n = nearby("famine", vec![hit(31.4, 3, false, 0, "2026-10-16T01:00:00Z")]);
        assert_eq!(spoken(&n, NOW), "Nearest famine system: Ross 154. 31 light years, about 3 jumps. Reported today.");
        let mut visited = hit(0.0, 0, true, 0, "2026-10-15T11:00:00Z");
        visited.seen_on_visit = true;
        let n = nearby("famine", vec![visited]);
        assert_eq!(spoken(&n, NOW), "Nearest famine system: Ross 154. In this system. Seen on your visit yesterday.");
    }

    #[test]
    fn the_emissions_lookups_say_the_best_system_and_a_billion_people() {
        let n = nearby("military", vec![hit(11.94, 2, true, 5_000_152_143, "2026-10-13T17:55:17Z")]);
        assert_eq!(
            spoken(&n, NOW),
            "Best war system: Ross 154. 12 light years, 2 jumps. Over a billion people. Reported 2 days ago."
        );
        let n = nearby("proto", vec![hit(4.0, 1, true, 20_000, "2026-10-16T11:00:00Z")]);
        assert_eq!(spoken(&n, NOW), "Best boom system: Ross 154. 4.0 light years, 1 jump. Reported today.");
    }

    #[test]
    fn nothing_found_says_so() {
        assert_eq!(spoken(&nearby("pharma", vec![]), NOW), "No outbreak systems near Sol with your filters.");
        let mut n = nearby("pharma", vec![]);
        n.warning = Some("Spansh changed its search.".into());
        assert_eq!(spoken(&n, NOW), "Spansh changed its search.");
    }

    #[test]
    fn the_tester_says_what_it_would_look_up() {
        assert_eq!(
            describe(astromech_galaxy::query("famine").unwrap()),
            "Would look up the nearest famine system on Spansh and say it."
        );
        assert_eq!(
            describe(astromech_galaxy::query("proto").unwrap()),
            "Would look up the best boom system on Spansh and say it."
        );
    }
}
