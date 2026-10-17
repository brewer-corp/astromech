//! Jump counts on a star map: `cargo run --release -p astromech-galaxy --example hop -- <galaxy.db> <range> <from> <to>...`
fn main() -> anyhow::Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let [db, range, from, to @ ..] = a.as_slice() else {
        anyhow::bail!("usage: hop <galaxy.db> <range> <from> <to>...");
    };
    let g = astromech_galaxy::Galaxy::with_remote(
        astromech_galaxy::Store::open(std::path::Path::new(db))?,
        astromech_data::Client::new(astromech_data::Cache::in_memory()?),
    );
    let t = std::time::Instant::now();
    let m = g.map()?;
    println!("{} stars loaded in {:?}", m.len(), t.elapsed());
    let range: f64 = range.parse()?;
    let p = |n: &str| g.position(n).ok().flatten().ok_or_else(|| anyhow::anyhow!("unknown system {n}"));
    let a = p(from)?;
    for n in to {
        let t = std::time::Instant::now();
        let h = m.hop(a, p(n)?, range);
        println!(
            "{from} -> {n}: {:.1} ly, {} jumps ({}), estimate {} [{:?}]",
            h.ly,
            h.jumps,
            if h.exact { "map" } else { "estimate" },
            astromech_galaxy::estimate(h.ly, range),
            t.elapsed()
        );
    }
    Ok(())
}
