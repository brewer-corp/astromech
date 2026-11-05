//! The mining hotspot lookup by voice (read-only, tier 0): "nearest painite hotspot". The
//! command is `mining.hotspots` with the grammar's `{mineral}` slot (an ore's symbol, from
//! `astromech_market::mining::spoken_ores`). The search is the one the Mining hotspots finder in
//! Places runs (`astromech_market::mining::hotspots_search`), nearest first and without the
//! finder's choices; this turns its answer into the spoken one.

use astromech_market::mining::{Hotspots, Want};

/// The command id.
pub const COMMAND: &str = "mining.hotspots";

/// Said when the phrase names no ore the lookup knows.
pub const WHICH_MINERAL: &str = "Which mineral? Say, for example, nearest painite hotspot.";

/// Whether `command` is the hotspot lookup.
pub fn is_hotspots(command: &str) -> bool {
    command == COMMAND
}

/// What the matched lookup asks for: any ring with a hotspot of the ore in `slot` (a symbol).
pub fn want(slot: Option<&str>) -> Option<Want> {
    let ore = slot?;
    astromech_market::mining::spoken_ores().iter().any(|(_, s)| *s == ore).then(|| Want::ore(ore))
}

/// Values for the grammar's `{mineral}` slot: (a way to say it, the ore's symbol).
pub fn mineral_values() -> Vec<(&'static str, &'static str)> {
    astromech_market::mining::spoken_ores()
}

/// What the voice tester shows instead of running the lookup.
pub fn describe(want: Option<&Want>) -> String {
    match want {
        Some(w) => format!("Would look up the nearest {} hotspot on Spansh and say it.", w.name()),
        None => WHICH_MINERAL.into(),
    }
}

/// "7.2 light years", "12 light years".
fn distance(ly: f64) -> String {
    if ly < 10.0 {
        format!("{ly:.1} light years")
    } else {
        format!("{ly:.0} light years")
    }
}

fn plural(n: u32, word: &str) -> String {
    format!("{n} {word}{}", if n == 1 { "" } else { "s" })
}

/// The spoken answer, a short sentence at a time: the nearest ring (its name a sentence of its
/// own, as procedural names run long), how far and how many jumps, and its hotspots of the ore
/// and reserves. `named`: the search is from a system the commander
/// named, not the current one, so a ring there is not "in this system".
pub fn spoken(h: &Hotspots, named: bool) -> String {
    let Some(r) = h.rings.first() else {
        return format!("No {} hotspots known nearby.", h.name);
    };
    let mut s = format!("Nearest {} hotspot. {}.", h.name, r.ring);
    if r.hop.ly >= 0.05 {
        let about = if r.hop.exact { "" } else { "about " };
        s += &format!(" {}, {about}{}.", capitalise(&distance(r.hop.ly)), plural(r.hop.jumps, "jump"));
    } else if !named {
        s += " In this system.";
    }
    s += &format!(" {}", plural(r.count, "hotspot"));
    match &r.reserve {
        Some(level) => s + &format!(", {} reserves.", level.to_lowercase()),
        None => s + ".",
    }
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::spansh::Signal;
    use astromech_galaxy::{Here, Hop};
    use astromech_market::mining::Hotspot;

    fn found(rings: Vec<Hotspot>) -> Hotspots {
        Hotspots {
            want: Want::ore("painite"),
            name: "Painite".into(),
            here: Here { system: "Sol".into(), id64: None, pos: [0.0; 3] },
            range: 20.0,
            radius_ly: None,
            rings,
            fetched_at: 0,
        }
    }

    fn ring(ly: f64, jumps: u32, exact: bool, count: u32, reserve: Option<&str>) -> Hotspot {
        Hotspot {
            system: "Barnard's Star".into(),
            system_pos: [0.0; 3],
            body: "Barnard's Star 5".into(),
            ring: "Barnard's Star 5 A Ring".into(),
            ring_type: "Metal Rich".into(),
            count,
            also: vec![Signal { name: "Platinum".into(), count: 1 }],
            reserve: reserve.map(String::from),
            hop: Hop { ly, jumps, exact },
            arrival_ls: Some(3272.4),
            updated_at: None,
        }
    }

    #[test]
    fn the_answer_says_the_ring_how_far_and_its_hotspots() {
        assert_eq!(
            spoken(&found(vec![ring(5.95, 1, true, 2, Some("Depleted"))]), false),
            "Nearest Painite hotspot. Barnard's Star 5 A Ring. 6.0 light years, 1 jump. 2 hotspots, depleted reserves."
        );
        assert_eq!(
            spoken(&found(vec![ring(31.4, 3, false, 1, None)]), false),
            "Nearest Painite hotspot. Barnard's Star 5 A Ring. 31 light years, about 3 jumps. 1 hotspot."
        );
        assert_eq!(
            spoken(&found(vec![ring(0.0, 0, true, 3, Some("Pristine"))]), false),
            "Nearest Painite hotspot. Barnard's Star 5 A Ring. In this system. 3 hotspots, pristine reserves."
        );
        // searched from a named system: the ring names it, so not "this system"
        assert_eq!(
            spoken(&found(vec![ring(0.0, 0, true, 3, Some("Pristine"))]), true),
            "Nearest Painite hotspot. Barnard's Star 5 A Ring. 3 hotspots, pristine reserves."
        );
    }

    #[test]
    fn nothing_found_says_so() {
        assert_eq!(spoken(&found(vec![]), false), "No Painite hotspots known nearby.");
    }

    #[test]
    fn the_mineral_slot_is_the_ore_list_with_its_other_names() {
        let v = mineral_values();
        for o in astromech_journal::ore::VALUE_ORDER {
            assert!(v.iter().any(|(_, s)| *s == o.symbol), "{} has no spoken name", o.symbol);
        }
        assert!(v.contains(&("LTDs", "lowtemperaturediamond")) && v.contains(&("Void Opal", "opal")));
        assert_eq!(want(Some("painite")), Some(Want::ore("painite")));
        assert_eq!(want(Some("gold")), None, "not a hotspot ore");
        assert_eq!(want(None), None);
        assert!(is_hotspots("mining.hotspots") && !is_hotspots("trade.sell"));
        assert_eq!(
            describe(want(Some("opal")).as_ref()),
            "Would look up the nearest Void Opal hotspot on Spansh and say it."
        );
        assert_eq!(describe(None), WHICH_MINERAL);
    }
}
