# Third-party software

astromech depends on open-source Rust crates and npm packages, each under its own licence.

- **Rust:** the allowed licences are enforced in CI by `cargo-deny` (`deny.toml`, from the S8
  audit): MIT, MIT-0, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, 0BSD, BSL-1.0, CC0-1.0,
  Unlicense, Unicode-3.0, MPL-2.0 and CDLA-Permissive-2.0, plus OFL-1.1/Ubuntu-font-1.0 for
  egui's bundled fonts (`epaint_default_fonts`). Release builds will include a full notices file
  generated with `cargo-about`.
- **GPL-3.0 caveat:** the optional `sherpa` feature links sherpa-onnx 1.x prebuilt libraries,
  which contain espeak-ng (GPL-3.0). See `NOTICE.md`.
- **Frontend (app/):** Svelte (MIT), Vite (MIT), Tauri (MIT OR Apache-2.0) and their
  dependencies; see `app/pnpm-lock.yaml`.
- **UI checks (scripts/browser):** puppeteer-core (Apache-2.0); talks to a local Google Chrome
  / Chromium, not shipped in the product.
- **Speech models** (downloaded at runtime, not distributed here): Parakeet TDT 0.6B v2
  (CC-BY-4.0), Kokoro-82M (Apache-2.0), espeak-ng-data (GPL-3.0).
