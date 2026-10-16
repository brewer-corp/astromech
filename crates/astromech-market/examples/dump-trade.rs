//! Print sample trade searches from the recorded Ardent answers (no network), for the app's
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
        } else if url.contains("/tritium/nearby/exports") {
            include_str!("../tests/fixtures/ardent-buy-tritium-sol.json")
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
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "sell": sell, "buy": buy }))?);
    Ok(())
}
