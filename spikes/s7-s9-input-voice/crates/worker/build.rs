fn main() {
    // The static sherpa-onnx/onnxruntime libraries reference PDBs from their CI machine;
    // the linker warns once per object (LNK4099). Harmless; silence it.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bins=/IGNORE:4099");
    }
}
