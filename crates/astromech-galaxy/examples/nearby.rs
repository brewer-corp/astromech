//! Live lookup against a star map: `cargo run -p astromech-galaxy --example nearby -- <galaxy.db> <system> <query> [range]`
//! (system must be in the store; queries: famine, fed_composites, empire_shielding, proto, military, improvised, pharma)
fn main() -> anyhow::Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let [db, system, query, rest @ ..] = a.as_slice() else {
        anyhow::bail!("usage: nearby <galaxy.db> <system> <query> [range]");
    };
    let range: f64 = rest.first().map(|r| r.parse()).transpose()?.unwrap_or(30.0);
    let g = astromech_galaxy::Galaxy::with_store(astromech_galaxy::Store::open(std::path::Path::new(db))?);
    let pos = g.position(system)?.ok_or_else(|| anyhow::anyhow!("{system} is not in the store"))?;
    let here = astromech_galaxy::Here { system: system.clone(), id64: None, pos };
    let t = std::time::Instant::now();
    let n = g.nearby(query, &here, range, 6)?;
    println!(
        "{} near {} at {range} ly ({:?}){}",
        n.query.label,
        n.here.system,
        t.elapsed(),
        n.warning.map(|w| format!(" WARNING {w}")).unwrap_or_default()
    );
    for h in &n.hits {
        let f = h
            .factions
            .first()
            .map(|f| format!("{} ({}, {})", f.name, f.allegiance, f.active_states.join("/")))
            .unwrap_or_default();
        println!(
            "  {:28} {:>3} jumps{} {:>6.1} ly  {f}  [{}]",
            h.name,
            h.hop.jumps,
            if h.hop.exact { " " } else { "~" },
            h.hop.ly,
            h.updated_at.as_deref().unwrap_or("")
        );
    }
    println!("store: {} systems", g.summary()?.0);
    Ok(())
}
