fn main() {
    // The star map preload is built by scripts/gen-starmap.py (release CI) and not committed.
    // Without it, bundle an empty file: the app skips it and fills in from EDSM as you travel.
    let preload = std::path::Path::new("resources/starmap.db");
    if !preload.exists() {
        let _ = std::fs::create_dir_all("resources");
        let _ = std::fs::write(preload, b"");
    }
    tauri_build::build()
}
