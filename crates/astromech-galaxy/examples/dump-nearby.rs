//! Print a sample "nearby systems" result from the Spansh fixture (no network), for the app's
//! browser preview: `cargo run -q -p astromech-galaxy --example dump-nearby [lookup]` (default
//! `famine`; `fed_composites` gives the High Grade Emissions sample with scores).
use astromech_galaxy::{Galaxy, Here, Store};

fn main() -> anyhow::Result<()> {
    let q = std::env::args().nth(1).unwrap_or_else(|| "famine".into());
    let s = Store::in_memory()?;
    let now = chrono::Utc::now().timestamp();
    s.add_coverage(astromech_galaxy::Sphere { center: [0.0; 3], radius: 1.0 }, "sample", "t")?;
    s.cache_put(&format!("spansh:{q}:0:0:0"), now, include_str!("../tests/fixtures/spansh-famine.json"))?;
    s.cache_put(&format!("spansh:{q}:pop:0:0:0"), now, r#"{"count": 0, "from": 0, "results": []}"#)?;
    let g = Galaxy::with_remote(s, astromech_data::Client::new(astromech_data::Cache::in_memory()?));
    let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    print!("{}", serde_json::to_string_pretty(&g.nearby(&q, &here, 32.5, 8, &astromech_galaxy::Filters::default())?)?);
    Ok(())
}
