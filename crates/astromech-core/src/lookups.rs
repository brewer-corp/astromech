//! Voice station lookups ("where's the nearest Interstellar Factors?"): the grammar's lookup
//! slots, and the spoken answer from Spansh near the commander's system, with the same
//! remembered filters the app's lookup uses (the pad following the current ship).

use astromech_data::spansh::Reference;
use astromech_data::Client;
use astromech_galaxy::stations::{self, Want};
use astromech_galaxy::{filters, Filters, Pad};
use astromech_gamestate::GameState;
use astromech_intent::Grammar;
use astromech_store::Store;
use astromech_tools::{lookup, targets};

/// The command grammar with the lookups' slot values: market commodities (trade), station
/// services, modules and ships.
pub fn grammar(systems: &[&str]) -> Grammar {
    let services = lookup::service_values();
    let modifier: astromech_intent::FreeWord = astromech_galaxy::outfitting::is_modifier;
    Grammar::mvp(systems)
        .with_slot("commodity", astromech_market::catalog::spoken_names(), None)
        .with_slot("service", services.iter().map(|(said, id)| (*said, id.as_str())), None)
        .with_slot("module", lookup::module_values(), Some(modifier))
        .with_slot("ship", lookup::ship_values(), None)
}

/// The filters a voice lookup uses: the app's remembered ones for `lookup` (`search.filters.
/// <lookup>`), the pad following the current ship unless the commander set it. Shared by the
/// station and trade lookups.
pub fn filters_for(store: Option<&Store>, lookup: &str, game: &GameState) -> Filters {
    let pad = game.ship.as_deref().and_then(Pad::for_ship);
    store.and_then(|s| filters::load(s, lookup, pad).ok()).unwrap_or_default().for_ship(pad)
}

/// The spoken answer to "where can I undermine near me" and the other Powerplay target
/// questions: the Where to work search (`Galaxy::powerplay_targets`) for the power pledged to in
/// the journal, within [`targets::WITHIN_LY`] of the journal's position, with jumps at the
/// ship's range (20 ly when unknown).
pub fn targets_answer(
    galaxy: &astromech_galaxy::Galaxy,
    filters: &Filters,
    target: astromech_galaxy::Target,
    game: &GameState,
) -> String {
    let Some(power) = game.powerplay.power.as_deref() else {
        return targets::NOT_PLEDGED.into();
    };
    let (Some(system), Some(pos)) = (game.system.clone(), game.star_pos) else {
        return lookup::NO_POSITION.into();
    };
    let range = game.max_jump_range.filter(|r| *r > 1.0).unwrap_or(20.0);
    let here = astromech_galaxy::Here { system, id64: None, pos };
    match galaxy.powerplay_targets(target, power, &here, range, targets::WITHIN_LY, 1, filters) {
        Ok(t) => targets::spoken(&t),
        Err(e) => format!("Could not look that up. {e}"),
    }
}

/// Said when Spansh does not know the commander's system and there are no coordinates to search from.
pub const UNKNOWN_SYSTEM: &str = "Spansh does not know this system yet.";

/// The spoken answer to `want` near the commander's current system, searched from its
/// position (the journal's `StarPos`) when known, so a system Spansh hasn't heard of works too.
pub fn answer(client: &Client, filters: &Filters, want: &Want, game: &GameState, now_s: i64) -> String {
    let Some(system) = game.system.as_deref() else {
        return lookup::NO_POSITION.into();
    };
    let reference = match game.star_pos {
        Some(pos) => Reference::Coords(pos),
        None => Reference::System(system.to_string()),
    };
    let by_name = matches!(reference, Reference::System(_));
    let found = stations::find(client, want, reference, filters, 1, now_s).map_err(|e| match e {
        // Spansh's answer to a reference system it doesn't know.
        astromech_data::Error::Rejected { .. } if by_name => UNKNOWN_SYSTEM.into(),
        e => e.to_string(),
    });
    lookup::spoken(want, system, &found, now_s)
}

/// The spoken answer to a faction-state lookup (`q`, from `astromech_galaxy::QUERIES`) near
/// the commander: the app's Nearby search (`Galaxy::nearby`) from the journal's position, with
/// jumps at the ship's range (20 ly when unknown).
pub fn faction_answer(
    galaxy: &astromech_galaxy::Galaxy,
    filters: &Filters,
    q: &astromech_galaxy::Query,
    game: &GameState,
    journals: Option<&std::path::Path>,
    now_s: i64,
) -> String {
    let (Some(system), Some(pos)) = (game.system.clone(), game.star_pos) else {
        return lookup::NO_POSITION.into();
    };
    if let Some(dir) = journals {
        // As the app does before a search. Best effort: without them the answer is Spansh's.
        let _ = galaxy.record_journal_visits(dir);
    }
    let range = game.max_jump_range.filter(|r| *r > 1.0).unwrap_or(20.0);
    let here = astromech_galaxy::Here { system, id64: None, pos };
    match galaxy.nearby(q.id, &here, range, 1, filters) {
        Ok(n) => astromech_tools::faction::spoken(&n, now_s),
        Err(e) => format!("Could not look that up. {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use astromech_data::{Cache, Limiter, Request, Response, Transport, TransportError};
    use astromech_galaxy::stations::Service;
    use astromech_intent::Decision;
    use std::sync::{Arc, Mutex};

    const SERVICES: &str = include_str!("../../astromech-galaxy/tests/fixtures/spansh-services.json");
    const NOW: i64 = 1_792_152_000; // 2026-10-16T12:00:00Z

    /// Answers with one recorded page and keeps the request bodies.
    struct Recorded {
        page: &'static str,
        bodies: Mutex<Vec<serde_json::Value>>,
    }

    impl Transport for Recorded {
        fn send(&self, _url: &str, req: &Request) -> Result<Response, TransportError> {
            if let astromech_data::Body::Json(b) = &req.body {
                self.bodies.lock().unwrap().push(b.clone());
            }
            let all: serde_json::Value = serde_json::from_str(SERVICES).unwrap();
            Ok(Response { status: 200, body: all[self.page].to_string() })
        }
    }

    fn heard(g: &Grammar, phrase: &str) -> (Option<Want>, Decision) {
        let m = g.best(phrase);
        (lookup::want(m.command, m.slot.as_deref(), m.slot_text.as_deref()), m.decision)
    }

    #[test]
    fn service_phrases_resolve_to_lookups() {
        let g = grammar(&["Sol"]);
        let svc = |s| Some(Want::Service { service: s });
        for (phrase, want) in [
            ("Nearest interstellar factors", svc(Service::InterstellarFactors)),
            ("Where's the nearest Interstellar Factors?", svc(Service::InterstellarFactors)),
            ("where is the nearest raw material trader", svc(Service::RawTrader)),
            ("nearest encoded trader", svc(Service::EncodedTrader)),
            ("Find the nearest manufactured trader.", svc(Service::ManufacturedTrader)),
            ("nearest guardian tech broker", svc(Service::GuardianBroker)),
            ("nearest human technology broker", svc(Service::HumanBroker)),
            ("nearest universal cartographics", svc(Service::UniversalCartographics)),
            ("Nearest Vista Genomics.", svc(Service::VistaGenomics)),
            ("nearest pioneer supplies", svc(Service::PioneerSupplies)),
            ("where's the nearest bartender", svc(Service::Bartender)),
            ("nearest frontline solutions", svc(Service::FrontlineSolutions)),
            ("nearest apex interstellar", svc(Service::Apex)),
            ("nearest black market", svc(Service::BlackMarket)),
            ("nearest search and rescue", svc(Service::SearchAndRescue)),
            ("find repairs", svc(Service::Repair)),
            ("nearest rearm", svc(Service::Rearm)),
            ("nearest refuel", svc(Service::Refuel)),
            ("nearest material trader", svc(Service::MaterialTrader)),
        ] {
            assert_eq!(heard(&g, phrase), (want, Decision::Execute), "{phrase:?}");
        }
        // a near miss asks first
        assert_eq!(heard(&g, "nearest interstellar factor").1, Decision::Ask);
        // and the ship commands still work
        assert_eq!(g.best("gear down").command, "landing_gear.down");
        assert_eq!(g.best("how much fuel").command, "status.fuel");
        // and the Powerplay questions, with the lookup slots in the grammar
        for (t, c) in [
            ("what's my powerplay rank", "status.powerplay"),
            ("Power play rank", "status.powerplay"),
            ("How many merits?", "status.powerplay"),
            ("Who controls this system?", "status.system_power"),
            ("system powerplay", "status.system_power"),
            ("What earns merits here?", "status.merits_here"),
            ("how much did I make this session", "session.summary"),
            ("session summary", "session.summary"),
            ("How much have I earned?", "session.summary"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
    }

    #[test]
    fn powerplay_target_phrases_resolve_to_the_galaxy_search() {
        use astromech_galaxy::Target;
        let g = grammar(&["Sol"]);
        for (phrase, target) in [
            ("Where can I undermine near me?", Target::Undermine),
            ("nearest system to undermine", Target::Undermine),
            ("where can I undermine", Target::Undermine),
            ("nearest undermining target", Target::Undermine),
            ("Nearest system to reinforce.", Target::Reinforce),
            ("where can I reinforce near me", Target::Reinforce),
            ("nearest reinforcement target", Target::Reinforce),
            ("where can I acquire near me", Target::Acquire),
            ("nearest system to acquire", Target::Acquire),
            ("nearest acquisition target", Target::Acquire),
        ] {
            let m = g.best(phrase);
            let t = astromech_tools::targets::from_command(m.command);
            assert_eq!((t, m.decision), (Some(target), Decision::Execute), "{phrase:?} -> {m:?}");
            // the command is the key the app remembers the search's filters under
            assert_eq!(astromech_tools::targets::lookup(target), m.command);
        }
        // the Powerplay status questions keep their phrases
        for (t, c) in [
            ("what's my powerplay rank", "status.powerplay"),
            ("who controls this system", "status.system_power"),
            ("what earns merits here", "status.merits_here"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
    }

    #[test]
    fn module_phrases_resolve_to_lookups() {
        let g = grammar(&["Sol"]);
        let label = |phrase: &str| {
            let (w, d) = heard(&g, phrase);
            (w.map(|w| w.label()), d)
        };
        for (phrase, want) in [
            ("nearest 6A frame shift drive", "6A Frame Shift Drive"),
            ("Where can I buy a 6A FSD?", "6A Frame Shift Drive"),
            ("nearest 5 A FSD", "5A Frame Shift Drive"),
            ("where can I get a 3C gimballed multi-cannon", "3C Multi-Cannon (gimballed)"),
            ("nearest 2E fixed multicannon", "2E Multi-Cannon (fixed)"),
            ("nearest class 5 fuel scoop", "class 5 Fuel Scoop"),
            ("find a detailed surface scanner", "Detailed Surface Scanner"),
            ("where can I buy an advanced docking computer", "Advanced Docking Computer"),
            ("nearest 4A shield generator", "4A Shield Generator"),
            ("buy a guardian FSD booster", "Guardian FSD Booster"),
            ("nearest A rated FSD", "A-rated Frame Shift Drive"),
            ("nearest A-rated frame shift drive", "A-rated Frame Shift Drive"),
            ("where can I buy a class five A FSD", "5A Frame Shift Drive"),
            ("nearest military grade composite", "Military Grade Composite"),
            ("where can I buy mirrored surface composite", "Mirrored Surface Composite"),
            ("nearest enduring feedback rail gun", "Enduring Feedback Rail Gun"),
            ("where can I buy a caustic sink launcher", "Caustic Sink Launcher"),
            ("nearest long range detailed surface scanner", "Long Range Detailed Surface Scanner"),
        ] {
            assert_eq!(label(phrase), (Some(want.to_string()), Decision::Execute), "{phrase:?}");
        }
        // the same request as typed in the app
        let (w, _) = heard(&g, "nearest 6A frame shift drive");
        let typed = Want::Module(astromech_galaxy::outfitting::ModuleWant {
            name: "Frame Shift Drive".into(),
            class: Some(6),
            rating: Some("A".into()),
            mount: None,
        });
        assert_eq!(w, Some(typed));
        // services still win their phrases
        assert_eq!(heard(&g, "nearest interstellar factors").0.map(|w| w.lookup()), Some("stations.service"));
    }

    #[test]
    fn ship_phrases_resolve_to_lookups() {
        let g = grammar(&["Sol"]);
        let ship = |s: &str| Some(Want::Ship { ship: s.into() });
        for (phrase, want) in [
            ("Where can I buy a Krait?", ship("Krait MkII")),
            ("where can I buy a Krait MkII", ship("Krait MkII")),
            ("nearest Anaconda", ship("Anaconda")),
            ("who sells the Python Mk II", ship("Python MkII")),
            ("where can I get a Type-9", ship("Type-9 Heavy")),
            ("nearest Imperial Cutter", ship("Imperial Cutter")),
            ("buy a Mandalay", ship("Mandalay")),
            ("nearest Cobra Mark Three", ship("Cobra MkIII")),
            ("where can I buy a Viper Mk 4", ship("Viper MkIV")),
            ("where's the nearest Fer-de-Lance", ship("Fer-de-Lance")),
        ] {
            assert_eq!(heard(&g, phrase), (want, Decision::Execute), "{phrase:?}");
        }
        // modules and services keep their phrases
        assert_eq!(heard(&g, "where can I buy a 6A FSD").0.map(|w| w.lookup()), Some("stations.module"));
        assert_eq!(heard(&g, "nearest black market").0.map(|w| w.lookup()), Some("stations.service"));
    }

    #[test]
    fn buy_goes_to_the_commodity_module_or_ship_the_slot_names() {
        let g = grammar(&["Sol"]);
        for (phrase, command, slot) in [
            ("buy gold", "trade.buy", "Gold"),
            ("where can I buy tritium", "trade.buy", "Tritium"),
            ("buy a Krait", "find.ship", "Krait MkII"),
            ("where can I buy an Anaconda", "find.ship", "Anaconda"),
            ("buy a 5A FSD", "find.module", "Frame Shift Drive"),
            ("where can I buy a fuel scoop", "find.module", "Fuel Scoop"),
            ("where can I sell painite", "trade.sell", "Painite"),
        ] {
            let m = g.best(phrase);
            assert_eq!(
                (m.command, m.slot.as_deref(), m.decision),
                (command, Some(slot), Decision::Execute),
                "{phrase:?}"
            );
        }
    }

    #[test]
    fn faction_state_phrases_resolve_to_the_galaxy_lookups() {
        let g = grammar(&["Sol"]);
        for (phrase, id) in [
            ("Nearest famine system.", "famine"),
            ("nearest famine", "famine"),
            ("Where is there a famine near me?", "famine"),
            ("find a famine system", "famine"),
            ("nearest boom system", "proto"),
            ("Where is there a boom near me?", "proto"),
            ("nearest boom", "proto"),
            ("nearest war system", "military"),
            ("where is there a war near me", "military"),
            ("nearest civil war", "military"),
            ("nearest civil unrest system", "improvised"),
            ("where is there civil unrest near me", "improvised"),
            ("nearest outbreak system", "pharma"),
            ("where is there an outbreak near me", "pharma"),
            ("nearest federation system", "fed_composites"),
            ("nearest federal system", "fed_composites"),
            ("nearest empire system", "empire_shielding"),
            ("nearest imperial system", "empire_shielding"),
            ("nearest federation", "fed_composites"),
            ("nearest federal", "fed_composites"),
            ("nearest empire", "empire_shielding"),
            ("nearest imperial", "empire_shielding"),
        ] {
            let m = g.best(phrase);
            let q = astromech_tools::faction::from_command(m.command).map(|q| q.id);
            assert_eq!((q, m.decision), (Some(id), Decision::Execute), "{phrase:?} -> {m:?}");
        }
        // every lookup the app offers has phrases, under the key its filters are remembered by
        for q in astromech_galaxy::QUERIES {
            let command = Grammar::intern(&format!("faction.{}", q.id)).expect("no phrases");
            assert_eq!(astromech_tools::faction::from_command(command), Some(q));
            assert_eq!(astromech_tools::faction::lookup(q), command);
        }
        // the neighbours keep theirs
        for (t, c) in [
            ("nearest raw farm", "sites.raw_farms"),
            ("nearest refuel", "find.service"),
            ("nearest crashed ship", "sites.crashed_ships"),
            ("nearest Anaconda", "find.ship"),
        ] {
            let m = g.best(t);
            assert_eq!((m.command, m.decision), (c, Decision::Execute), "{t:?} -> {m:?}");
        }
    }

    #[test]
    fn spoken_answer_uses_the_ships_pad_and_saved_filters() {
        let rec = Arc::new(Recorded { page: "refuel", bodies: Mutex::new(vec![]) });
        let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
        let store = Store::in_memory().unwrap();
        let want = Want::Service { service: Service::Refuel };
        filters::save(&store, want.lookup(), &Filters { exclude_odyssey: true, ..Filters::default() }).unwrap();
        let game = GameState { system: Some("Sol".into()), ship: Some("Anaconda".into()), ..Default::default() };
        let f = filters_for(Some(&store), want.lookup(), &game);
        assert_eq!((f.pad, f.exclude_odyssey), (Some(Pad::L), true));
        assert_eq!(
            answer(&client, &f, &want, &game, NOW),
            "Nearest refuel: Ehrlich City in Sol, in this system, 228 light seconds from the star."
        );
        let body = rec.bodies.lock().unwrap().last().unwrap().clone();
        assert_eq!(body["reference_system"], "Sol");
        assert_eq!(body["filters"]["has_large_pad"]["value"], true, "an Anaconda needs a large pad");

        let lost = GameState::default();
        assert_eq!(answer(&client, &f, &want, &lost, NOW), lookup::NO_POSITION);
    }

    /// Spansh's answer to a search from a system it doesn't know (HTTP 400), or the refuel
    /// page for a search from coordinates.
    struct Unknown {
        bodies: Mutex<Vec<serde_json::Value>>,
    }

    impl Transport for Unknown {
        fn send(&self, _url: &str, req: &Request) -> Result<Response, TransportError> {
            let astromech_data::Body::Json(b) = &req.body else { panic!("expected JSON") };
            self.bodies.lock().unwrap().push(b.clone());
            if b.get("reference_system").is_some() {
                return Ok(Response { status: 400, body: r#"{"error":"Invalid request"}"#.into() });
            }
            let all: serde_json::Value = serde_json::from_str(SERVICES).unwrap();
            Ok(Response { status: 200, body: all["refuel"].to_string() })
        }
    }

    #[test]
    fn a_system_spansh_doesnt_know_is_searched_from_its_position_or_said_plainly() {
        let rec = Arc::new(Unknown { bodies: Mutex::new(vec![]) });
        let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
        let want = Want::Service { service: Service::Refuel };
        let f = Filters::default();
        // Just discovered: the journal's StarPos is known, Spansh hasn't heard of the name.
        let here = GameState {
            system: Some("Oochoss XY-Z d1-23".into()),
            star_pos: Some([0.0, 0.0, 0.0]),
            ..Default::default()
        };
        let said = answer(&client, &f, &want, &here, NOW);
        assert!(said.starts_with("Nearest refuel: "), "{said}");
        let body = rec.bodies.lock().unwrap().last().unwrap().clone();
        assert_eq!(body["reference_coords"], serde_json::json!({ "x": 0.0, "y": 0.0, "z": 0.0 }));
        // No position yet: a plain answer, not Spansh's "Invalid request".
        let nameless = GameState { star_pos: None, ..here };
        assert_eq!(answer(&client, &f, &want, &nameless, NOW), UNKNOWN_SYSTEM);
    }

    /// One "position not known" line for every position-based lookup (astromech-ps6t).
    #[test]
    fn the_faction_and_powerplay_lookups_share_one_no_position_line() {
        let client = Arc::new(Client::with_transport(
            Cache::in_memory().unwrap(),
            Arc::new(Recorded { page: "refuel", bodies: Mutex::new(vec![]) }),
        ));
        let galaxy = astromech_galaxy::Galaxy::with_remote(astromech_galaxy::Store::in_memory().unwrap(), client);
        let mut game = GameState::default();
        game.powerplay.power = Some("Archon Delaine".into());
        let q = &astromech_galaxy::QUERIES[0];
        assert_eq!(faction_answer(&galaxy, &Filters::default(), q, &game, None, NOW), lookup::NO_POSITION);
        let t = astromech_galaxy::Target::Undermine;
        assert_eq!(targets_answer(&galaxy, &Filters::default(), t, &game), lookup::NO_POSITION);
    }
}
