//! Print the app window's sample fleet and plan: `cargo run -q -p astromech-engineering --example dump-fixture`
fn main() {
    let v = serde_json::json!({
        "fleet": astromech_engineering::sample_fleet(),
        "plan_input": astromech_engineering::sample_plan_input(),
        "plan": astromech_engineering::sample_plan(),
    });
    print!("{}", serde_json::to_string_pretty(&v).unwrap());
}
