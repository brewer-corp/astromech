//! Print the best match for each argument: `cargo run -p astromech-intent --example rank -- "gear down"`.
fn main() {
    let g = astromech_intent::Grammar::mvp(&["Sol", "Colonia", "Shinrarta Dezhra", "Achenar"]);
    for t in std::env::args().skip(1) {
        let r = g.rank(&t);
        let b = g.best(&t);
        let next = r.iter().find(|m| m.command != b.command);
        println!(
            "{t:?}: {:?} {:?} {:.3} (next: {:?} {:.3})",
            b.decision,
            b.phrase,
            b.score,
            next.map(|m| m.phrase.as_str()),
            next.map(|m| m.score).unwrap_or(0.0)
        );
    }
}
