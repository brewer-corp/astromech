fn main() {
    // The star map preload is published separately (.github/workflows/starmap.yml) and downloaded
    // by release builds; it is not committed.
    // Without it, bundle an empty file: the app skips it and fills in from EDSM as you travel.
    let preload = std::path::Path::new("resources/starmap.db");
    if !preload.exists() {
        let _ = std::fs::create_dir_all("resources");
        let _ = std::fs::write(preload, b"");
    }
    tauri_build::build()
}
