//! # astromech-core
//!
//! The long-running core process (ARCHITECTURE.md §18): it will own the tray icon, the journal
//! and Status pipeline, voice, the VR overlay and the local WebSocket API that the app window
//! (`astromech-shell`) connects to. This is the only crate that wires everything together.
//!
//! Pre-alpha: for now it prints what it would start and runs a quick self-check.

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("astromech-core {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    let tools = astromech_tools::mvp();
    println!(
        "astromech-core {} (pre-alpha; protocol v{})",
        env!("CARGO_PKG_VERSION"),
        astromech_proto::PROTOCOL_VERSION
    );
    println!("  tools: {}", tools.names().collect::<Vec<_>>().join(", "));
    println!("  voice: {} inference threads by default", astromech_voice::DEFAULT_THREADS);
    println!("  game window: {}", astromech_policy::TARGET_WINDOW);
    match astromech_keybinds::game::default_bindings_dir() {
        Some(d) => println!("  bindings folder: {}", d.display()),
        None => println!("  bindings folder: not found on this machine"),
    }
    let state = astromech_gamestate::GameState::default();
    let _ = (&state, astromech_store::Store::in_memory().map(|s| s.schema_version()));
    let _ = (
        astromech_journal::parse_line,
        astromech_status::parse_status,
        astromech_input::listen::tracker::Processor::new,
        astromech_overlay::toast,
    );
}
