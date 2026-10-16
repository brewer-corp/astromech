//! Print the Odyssey page's sample: the view of the synthetic fixtures in `tests/fixtures`, and
//! the shopping list for a sample set of picks (the page's preview outside the app).
//! `cargo run -q -p astromech-odyssey --example dump-odyssey-fixture > app/src/lib/fixtures/odyssey.json`
use astromech_odyssey::{sample, shopping, view, Pick};

fn main() {
    let (h, p) = sample();
    let picks = vec![
        Pick::Upgrade { gear: "maverick".into(), from: 1, to: 3 },
        Pick::Mod { id: "nightvision".into(), maker: None },
        Pick::Upgrade { gear: "karmap15".into(), from: 1, to: 2 },
    ];
    let list = shopping(&picks, &h, &p);
    let out = serde_json::json!({
        "view": view(&h, &p, "Sample ShipLocker"),
        "picks": picks,
        "shopping": list,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
