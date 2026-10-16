//! Print sample raw-material site results from Sol, one per kind, from the recorded Spansh
//! searches (no network), for the app's browser preview (`app/src/lib/fixtures/sites.json`):
//! `cargo run -q -p astromech-galaxy --example dump-sites`
use astromech_data::{Body, Cache, Client, Limiter, Request, Response, Transport, TransportError};
use astromech_galaxy::{Galaxy, Here, Store, SITE_KINDS};
use std::sync::Arc;

struct Recorded;

impl Transport for Recorded {
    fn send(&self, url: &str, req: &Request) -> Result<Response, TransportError> {
        let b = match &req.body {
            Body::Json(v) => v.to_string(),
            _ => String::new(),
        };
        let body = if !url.contains("spansh") {
            "[]"
        } else if b.contains("Crystalline Shards") {
            include_str!("../tests/fixtures/spansh-sites-crystalline_shards.json")
        } else if b.contains("Brain Tree") {
            include_str!("../tests/fixtures/spansh-sites-brain_trees.json")
        } else if b.contains("Crashed Ship") {
            include_str!("../tests/fixtures/spansh-sites-crashed_ships.json")
        } else {
            include_str!("../tests/fixtures/spansh-sites-geological.json")
        };
        Ok(Response { status: 200, body: body.into() })
    }
}

fn main() -> anyhow::Result<()> {
    let client = Client::with_transport(Cache::in_memory()?, Arc::new(Recorded)).limiter(Limiter::none());
    let g = Galaxy::with_remote(Store::in_memory()?, client);
    let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    let mut out = serde_json::Map::new();
    for k in SITE_KINDS {
        let s = g.sites(k.id, &here, 32.5, 5, &astromech_galaxy::Filters::default())?;
        out.insert(k.id.into(), serde_json::to_value(s)?);
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}
