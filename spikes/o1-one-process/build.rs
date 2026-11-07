fn main() {
    // Tauri is a Windows-only dependency (Cargo.toml), so there is nothing to build elsewhere.
    if std::env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }
    // The window icon resource comes from the app's own icons; the spike has none of its own.
    let attrs = tauri_build::Attributes::new().windows_attributes(
        tauri_build::WindowsAttributes::new().window_icon_path("../../app/src-tauri/icons/icon.ico"),
    );
    tauri_build::try_build(attrs).expect("tauri-build");
}
