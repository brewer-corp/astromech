//! Print a sample "nearby Famine systems" result from the Spansh fixture (no network):
//! `cargo run -q -p astromech-galaxy --example dump-nearby`
use astromech_galaxy::{Galaxy, Here, Store};

fn main() -> anyhow::Result<()> {
    let s = Store::in_memory()?;
    let now = chrono::Utc::now().timestamp();
    s.add_coverage(astromech_galaxy::Sphere { center: [0.0; 3], radius: 1.0 }, "sample", "t")?;
    s.cache_put("spansh:famine:0:0:0", now, include_str!("../tests/fixtures/spansh-famine.json"))?;
    let g = Galaxy::with_store(s);
    let here = Here { system: "Sol".into(), id64: Some(10477373803), pos: [0.0; 3] };
    print!("{}", serde_json::to_string_pretty(&g.nearby("famine", &here, 32.5, 8)?)?);
    Ok(())
}
