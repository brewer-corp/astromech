//! Print sample trade searches (near Sol and anywhere), a cargo plan and where to buy a colonisation shopping list from
//! the recorded Ardent answers and a fixture hold (no network), for the app's
//! browser preview (`app/src/lib/fixtures/trade.json`):
//! `cargo run -q -p astromech-market --example dump-trade > app/src/lib/fixtures/trade.json`
use astromech_data::{Cache, Client, Clock, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Here, Store};
use astromech_market::{anywhere, search, Side};
use std::sync::Arc;

struct Recorded;

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let no_carriers = req.query.contains(&("fleetCarriers".into(), "false".into()));
        let body = if url.ends_with("/v2/commodity/name/painite/imports") {
            include_str!("../tests/fixtures/ardent-galaxy-sell-painite.json")
        } else if url.ends_with("/v2/commodity/name/tritium/exports") && no_carriers {
            include_str!("../tests/fixtures/ardent-galaxy-buy-tritium-nocarriers.json")
        } else if url.ends_with("/v2/commodity/name/tritium/exports") {
            include_str!("../tests/fixtures/ardent-galaxy-buy-tritium.json")
        } else if url.contains("/painite/nearby/imports") {
            include_str!("../tests/fixtures/ardent-sell-painite-sol.json")
        } else if url.contains("/platinum/nearby/imports") {
            include_str!("../tests/fixtures/ardent-sell-platinum-sol.json")
        } else if url.contains("/lowtemperaturediamond/nearby/imports") {
            include_str!("../tests/fixtures/ardent-sell-lowtemperaturediamond-sol.json")
        } else if url.contains("/tritium/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-tritium-sol.json")
        } else if url.contains("/steel/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-steel-sol.json")
        } else if url.contains("/ceramiccomposites/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-ceramiccomposites-sol.json")
        } else if url.contains("/cmmcomposite/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-cmmcomposite-sol.json")
        } else if url.contains("/insulatingmembrane/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-insulatingmembrane-sol.json")
        } else {
            "[]"
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

/// When the answers were recorded.
const EVENING: i64 = 1_792_184_400; // 2026-10-16T21:00:00Z

struct Evening;

impl Clock for Evening {
    fn now_s(&self) -> i64 {
        EVENING
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_transport(Cache::in_memory()?, Arc::new(Recorded))
        .clock(Arc::new(Evening))
        .limiter(Limiter::none());
    let g = Galaxy::with_remote(Store::in_memory()?, client);
    let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    let f = Filters { max_distance_ly: Some(40), ..Filters::default() };
    let sell = search(&g, &here, 20.0, Side::Sell, &["painite"], &f, 12, EVENING)?;
    let buy = search(&g, &here, 20.0, Side::Buy, &["tritium"], &f, 12, EVENING)?;
    let hold = astromech_journal::parse_cargo(include_bytes!("../tests/fixtures/cargo-mined.json")).unwrap_or_default();
    let cargo = astromech_market::cargo::plan_search(&g, &here, 20.0, &hold.inventory, &f, 8, EVENING)?;
    // The shopping list of the preview's construction site (app/src/lib/fixtures/colonisation.json).
    let need = |symbol: &str, name: &str, tonnes| astromech_market::list::Need {
        symbol: symbol.into(),
        name: name.into(),
        tonnes,
    };
    let needs = [
        need("steel", "Steel", 4_912),
        need("cmmcomposite", "CMM Composite", 3_820),
        need("ceramiccomposites", "Ceramic Composites", 512),
        need("insulatingmembrane", "Insulating Membrane", 310),
    ];
    let f20 = Filters { max_distance_ly: Some(20), ..Filters::default() };
    let list = astromech_market::list::plan_search(&g, &here, 20.0, &needs, &f20, 12, EVENING)?;
    // Anywhere: the preview filters one answer itself, so the buy side holds the carriers'
    // answer and the stations-only one together (Ardent's cheapest 100 tritium are all carriers).
    let any = Filters::default();
    let any_sell = anywhere(g.remote(), &here, 20.0, Side::Sell, "painite", &any, EVENING)?;
    let mut any_buy = anywhere(g.remote(), &here, 20.0, Side::Buy, "tritium", &any, EVENING)?;
    let stations = Filters { exclude_carriers: true, ..any };
    let more = anywhere(g.remote(), &here, 20.0, Side::Buy, "tritium", &stations, EVENING)?;
    any_buy.results[0].offers.extend(more.results.into_iter().flat_map(|p| p.offers));
    let out = serde_json::json!({
        "sell": sell, "buy": buy, "cargo": cargo, "hold": hold.inventory, "list": list,
        "anywhere": { "sell": any_sell, "buy": any_buy },
    });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
