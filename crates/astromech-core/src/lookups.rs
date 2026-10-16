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
use astromech_tools::lookup;

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

/// The filters a voice lookup uses: the app's remembered ones for that lookup, the pad
/// following the current ship unless the commander set it.
pub fn filters_for(store: Option<&Store>, want: &Want, game: &GameState) -> Filters {
    let pad = game.ship.as_deref().and_then(Pad::for_ship);
    store.and_then(|s| filters::load(s, want.lookup(), pad).ok()).unwrap_or_default().for_ship(pad)
}

/// The spoken answer to `want` near the commander's current system.
pub fn answer(client: &Client, filters: &Filters, want: &Want, game: &GameState, now_s: i64) -> String {
    let Some(system) = game.system.as_deref() else {
        return "I don't know which system you're in yet.".into();
    };
    let found = stations::find(client, want, Reference::System(system.to_string()), filters, 1, now_s)
        .map_err(|e| e.to_string());
    lookup::spoken(want, system, &found, now_s)
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
    fn spoken_answer_uses_the_ships_pad_and_saved_filters() {
        let rec = Arc::new(Recorded { page: "refuel", bodies: Mutex::new(vec![]) });
        let client = Client::with_transport(Cache::in_memory().unwrap(), rec.clone()).limiter(Limiter::none());
        let store = Store::in_memory().unwrap();
        let want = Want::Service { service: Service::Refuel };
        filters::save(&store, want.lookup(), &Filters { exclude_odyssey: true, ..Filters::default() }).unwrap();
        let game = GameState { system: Some("Sol".into()), ship: Some("Anaconda".into()), ..Default::default() };
        let f = filters_for(Some(&store), &want, &game);
        assert_eq!((f.pad, f.exclude_odyssey), (Some(Pad::L), true));
        assert_eq!(
            answer(&client, &f, &want, &game, NOW),
            "Nearest refuel: Ehrlich City in Sol, in this system, 228 light seconds from the star."
        );
        let body = rec.bodies.lock().unwrap().last().unwrap().clone();
        assert_eq!(body["reference_system"], "Sol");
        assert_eq!(body["filters"]["has_large_pad"]["value"], true, "an Anaconda needs a large pad");

        let lost = GameState::default();
        assert_eq!(answer(&client, &f, &want, &lost, NOW), "I don't know which system you're in yet.");
    }
}
