fn main() {
    print!("{}", serde_json::to_string_pretty(&astromech_materials::sample_inventory()).unwrap());
}
