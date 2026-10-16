//! Print sample trade searches, a cargo plan and where to buy a colonisation shopping list from
//! the recorded Ardent answers and a fixture hold (no network), for the app's
//! browser preview (`app/src/lib/fixtures/trade.json`):
//! `cargo run -q -p astromech-market --example dump-trade > app/src/lib/fixtures/trade.json`
use astromech_data::{Cache, Client, Clock, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Filters, Galaxy, Here, Store};
use astromech_market::{search, Side};
use std::sync::Arc;

struct Recorded;

impl Transport for Recorded {
    fn send(&self, url: &str, _req: &Request) -> Result<Response, TransportError> {
        let body = if url.contains("/painite/nearby/imports") {
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
    let out = serde_json::json!({ "sell": sell, "buy": buy, "cargo": cargo, "hold": hold.inventory, "list": list });
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
