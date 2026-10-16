//! Print the app window's sample engineer unlock tree, from the fixture `EngineerProgress`:
//! `cargo run -q -p astromech-engineering --example dump-unlocks`
use astromech_engineering::{engineers, unlocks};

fn main() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/unlocks");
    let t = unlocks::tree(&engineers::builder_from_journal_dir(&dir));
    println!("{}", serde_json::to_string_pretty(&t).unwrap());
}
