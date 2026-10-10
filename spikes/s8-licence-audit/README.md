# S8: licence and build audit (8 Oct 2026)
- `crate-src/`: sherpa-onnx and sherpa-onnx-sys 1.13.8 crate sources (see `sherpa-onnx-sys-1.13.8/build.rs`).
- `prebuilt/`: the v1.13.8 Windows archives the crate downloads (static and shared). The static archive is kept as the tarball only (it extracts to about 1.1 GB). Inspect with llvm-nm (`rustup component add llvm-tools`).
- `skeleton/`: an empty workspace that depends on the planned crates, plus `deny.toml` (the revised allowlist), `about.toml` and the generated `THIRD-PARTY.html`. Run `cargo deny check` and `cargo deny --features ui-slint check licenses`.
Findings: ARCHITECTURE.md §17.3 and §16.
