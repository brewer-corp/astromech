//! High Grade Emissions: G4–G5 manufactured pools from faction allegiance + state.
//! Community tables (Inara / EDCD), not Frontier assets. A signal is spawned by a *minor
//! faction*; a Federation system can still drop Imperial Shielding if an Imperial faction
//! spawned it. System-level prediction is a likely pool, not a guarantee for one contact.

use crate::catalog::{self, Spec};
use astromech_journal::{FactionSnap, JournalEvent, JournalLine};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceNote {
    pub kind: String,
    pub summary: String,
    pub hint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PoolMat {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Here {
    pub system: String,
    pub allegiance: String,
    pub population: i64,
    pub hge_signal: bool,
    pub pool: Vec<PoolMat>,
    pub line: String,
}

struct Rule {
    id: &'static str,
    ids: &'static [&'static str],
    summary: &'static str,
    hint: &'static str,
}

const FED_COMPOSITES: Rule = Rule {
    id: "fed_composites",
    ids: &["fedproprietarycomposites", "fedcorecomposites"],
    summary: "High Grade Emissions spawned by a Federation faction.",
    hint: "Prefer high-population Federation systems; honk or scan the nav beacon. Check Contacts → INFO for Faction / State.",
};

const EMPIRE_SHIELDING: Rule = Rule {
    id: "empire_shielding",
    ids: &["imperialshielding"],
    summary: "High Grade Emissions spawned by an Empire faction.",
    hint:
        "Prefer high-population Empire systems; honk or scan the nav beacon. Check Contacts → INFO for Faction / State.",
};

const PROTO: Rule = Rule {
    id: "proto",
    ids: &["protoheatradiators", "protoradiolicalloys", "protolightalloys"],
    summary: "High Grade Emissions spawned by an Independent or Alliance faction in Boom.",
    hint: "Independent or Alliance systems in Boom. Honk or scan the nav beacon, then check Contacts → INFO.",
};

const MILITARY: Rule = Rule {
    id: "military",
    ids: &["militarygradealloys", "militarysupercapacitors"],
    summary: "High Grade Emissions spawned by a faction in War or Civil War.",
    hint: "Any allegiance. War / Civil War on the spawning faction. Check Contacts → INFO.",
};

const IMPROVISED: Rule = Rule {
    id: "improvised",
    ids: &["improvisedcomponents"],
    summary: "High Grade Emissions spawned by an Independent or Alliance faction in Civil Unrest.",
    hint: "Independent or Alliance systems in Civil Unrest. Check Contacts → INFO.",
};

const PHARMA: Rule = Rule {
    id: "pharma",
    ids: &["pharmaceuticalisolators"],
    summary: "High Grade Emissions spawned by an Independent or Alliance faction in Outbreak.",
    hint: "Independent or Alliance systems in Outbreak, population about a million or more.",
};

const RULES: &[&Rule] = &[&FED_COMPOSITES, &EMPIRE_SHIELDING, &PROTO, &MILITARY, &IMPROVISED, &PHARMA];

const NOT_HGE: &[(&str, &str)] = &[
    ("biotechconductors", "Not at High Grade Emissions. Missions or a manufactured trader."),
    ("exquisitefocuscrystals", "Not at High Grade Emissions. Missions or a manufactured trader."),
];

fn key(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_lowercase()).collect()
}

fn has_state(states: &[String], want: &str) -> bool {
    states.iter().any(|s| s == want)
}

fn faction_states(f: &FactionSnap) -> Vec<String> {
    let mut out: Vec<String> = f.active_states.iter().map(|s| key(s)).filter(|s| s != "none").collect();
    let st = key(&f.state);
    if st != "none" && !st.is_empty() && !out.iter().any(|s| s == &st) {
        out.push(st);
    }
    out
}

fn allegiance_of(s: &str) -> String {
    key(s)
}

/// Which HGE pool a single faction would spawn. War outranks Fed/Empire composites.
fn rule_for_faction(f: &FactionSnap, population: i64) -> Option<&'static Rule> {
    let al = allegiance_of(&f.allegiance);
    let st = faction_states(f);
    if has_state(&st, "war") || has_state(&st, "civilwar") {
        return Some(&MILITARY);
    }
    let indish = al == "independent" || al == "alliance";
    if indish && has_state(&st, "civilunrest") {
        return Some(&IMPROVISED);
    }
    if indish && has_state(&st, "outbreak") && population >= 1_000_000 {
        return Some(&PHARMA);
    }
    if indish && has_state(&st, "boom") {
        return Some(&PROTO);
    }
    if al == "federation" {
        return Some(&FED_COMPOSITES);
    }
    if al == "empire" {
        return Some(&EMPIRE_SHIELDING);
    }
    None
}

pub fn pool_for_factions(factions: &[FactionSnap], population: i64) -> Vec<PoolMat> {
    let mut scored: Vec<(&Rule, f64)> = vec![];
    for f in factions {
        let Some(rule) = rule_for_faction(f, population) else { continue };
        if let Some(existing) = scored.iter_mut().find(|(r, _)| r.id == rule.id) {
            existing.1 = existing.1.max(f.influence);
        } else {
            scored.push((rule, f.influence));
        }
    }
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.0.id.cmp(b.0.id)));
    let mut out = vec![];
    for (rule, _) in scored {
        for id in rule.ids {
            if out.iter().any(|p: &PoolMat| p.id == *id) {
                continue;
            }
            let name = catalog::lookup(id).map(|s| s.name.to_string()).unwrap_or_else(|| (*id).to_string());
            out.push(PoolMat { id: (*id).to_string(), name });
        }
    }
    out
}

pub fn sources_for(spec: &Spec) -> Vec<SourceNote> {
    let mut notes = vec![];
    for (id, summary) in NOT_HGE {
        if spec.id == *id {
            notes.push(SourceNote {
                kind: "not_hge".into(),
                summary: (*summary).into(),
                hint: "Trade up at a manufactured trader, or look at mission rewards.".into(),
            });
            return notes;
        }
    }
    for rule in RULES {
        if rule.ids.contains(&spec.id) {
            notes.push(SourceNote { kind: "hge".into(), summary: rule.summary.into(), hint: rule.hint.into() });
        }
    }
    notes
}

fn format_pop(n: i64) -> String {
    if n >= 1_000_000 {
        format!("{:.0}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.0}k", n as f64 / 1_000.0)
    } else if n > 0 {
        n.to_string()
    } else {
        "uninhabited".into()
    }
}

pub fn here_line(system: &str, allegiance: &str, population: i64, pool: &[PoolMat], hge_signal: bool) -> String {
    let al = if allegiance.is_empty() { "unknown allegiance".into() } else { allegiance.to_string() };
    let mats = if pool.is_empty() {
        "no predicted High Grade pool".into()
    } else {
        let names: Vec<&str> = pool.iter().map(|p| p.name.as_str()).collect();
        format!("HGEs here: {}", names.join(" / "))
    };
    let mut line = format!("{system} · {al} · pop {} · {mats}", format_pop(population));
    if hge_signal {
        line.push_str(" · High Grade Emissions on the scanner");
    }
    line
}

pub fn here_from_lines<'a>(lines: impl IntoIterator<Item = &'a JournalLine>) -> Option<Here> {
    let mut system = String::new();
    let mut allegiance = String::new();
    let mut population = 0i64;
    let mut factions: Vec<FactionSnap> = vec![];
    let mut hge_signal = false;
    let mut seen = false;
    for l in lines {
        match &l.event {
            JournalEvent::Location { star_system, allegiance: al, population: pop, factions: fac, .. }
            | JournalEvent::FsdJump { star_system, allegiance: al, population: pop, factions: fac, .. } => {
                system = star_system.clone();
                allegiance = al.clone();
                population = *pop;
                factions = fac.clone();
                hge_signal = false;
                seen = true;
            }
            JournalEvent::FssSignal { high_grade: true, .. } if seen => {
                hge_signal = true;
            }
            _ => {}
        }
    }
    if !seen {
        return None;
    }
    let pool = pool_for_factions(&factions, population);
    let line = here_line(&system, &allegiance, population, &pool, hge_signal);
    Some(Here { system, allegiance, population, hge_signal, pool, line })
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_journal::parse_line;

    fn fac(al: &str, state: &str, inf: f64) -> FactionSnap {
        FactionSnap {
            name: format!("{al}-{state}"),
            allegiance: al.into(),
            state: state.into(),
            influence: inf,
            active_states: if state.eq_ignore_ascii_case("none") { vec![] } else { vec![state.into()] },
        }
    }

    #[test]
    fn pools_follow_allegiance_and_state() {
        let fed = pool_for_factions(&[fac("Federation", "None", 0.6)], 85_000_000);
        let names: Vec<_> = fed.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(names, vec!["fedproprietarycomposites", "fedcorecomposites"]);

        let emp = pool_for_factions(&[fac("Empire", "Boom", 0.4)], 2_000_000);
        assert_eq!(emp.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), vec!["imperialshielding"]);

        let proto = pool_for_factions(&[fac("Independent", "Boom", 0.5)], 3_000_000);
        assert!(proto.iter().any(|p| p.id == "protoheatradiators"));
        assert!(proto.iter().any(|p| p.id == "protolightalloys"));

        let war = pool_for_factions(&[fac("Federation", "War", 0.7)], 10_000_000);
        assert!(war.iter().any(|p| p.id == "militarygradealloys"));
        assert!(!war.iter().any(|p| p.id == "fedcorecomposites"));

        let outbreak = pool_for_factions(&[fac("Alliance", "Outbreak", 0.3)], 2_000_000);
        assert_eq!(outbreak[0].id, "pharmaceuticalisolators");

        let unrest = pool_for_factions(&[fac("Independent", "CivilUnrest", 0.2)], 500_000);
        assert_eq!(unrest[0].id, "improvisedcomponents");
    }

    #[test]
    fn mixed_system_unions_pools() {
        let pool = pool_for_factions(&[fac("Federation", "None", 0.61), fac("Empire", "Boom", 0.12)], 85_000_000);
        let ids: Vec<_> = pool.iter().map(|p| p.id.as_str()).collect();
        assert!(ids.contains(&"fedcorecomposites"));
        assert!(ids.contains(&"imperialshielding"));
        assert_eq!(ids[0], "fedproprietarycomposites");
    }

    #[test]
    fn jump_and_hge_signal_from_journal() {
        let jump = parse_line(
            r#"{"timestamp":"t","event":"FSDJump","StarSystem":"Shinrarta Dezhra","SystemAllegiance":"Federation","Population":85262351,"Factions":[{"Name":"Pilots Federation","Allegiance":"Federation","FactionState":"None","Influence":0.61}]}"#,
        )
        .unwrap();
        let sig =
            parse_line(r#"{"timestamp":"t2","event":"FSSSignalDiscovered","USSType":"$USS_Type_HighGradeEmissions;"}"#)
                .unwrap();
        let here = here_from_lines([&jump, &sig]).unwrap();
        assert_eq!(here.system, "Shinrarta Dezhra");
        assert!(here.hge_signal);
        assert!(here.line.contains("Core Dynamics"));
        assert!(here.line.contains("Proprietary Composites"));
        assert!(here.line.contains("High Grade Emissions on the scanner"));
        assert!(here.line.contains("85M"));
    }

    #[test]
    fn imperial_shielding_note() {
        let s = catalog::lookup("imperialshielding").unwrap();
        let n = sources_for(s);
        assert_eq!(n[0].kind, "hge");
        assert!(n[0].summary.contains("Empire"));
        let s = catalog::lookup("biotechconductors").unwrap();
        assert_eq!(sources_for(s)[0].kind, "not_hge");
    }
}
