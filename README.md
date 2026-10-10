# astromech

A voice-first copilot for **Elite Dangerous** on Windows. It runs fully local: push-to-talk
speech recognition, spoken confirmations, game-state awareness from the journal and
`Status.json`, verified in-game actions, a keybinds manager with a built-in health check, and a
SteamVR overlay. Everything lives in one app.

> **Status: pre-alpha.** This repository is the scaffold. The crates are compiling stubs seeded
> from the spikes, and nothing is usable as a product yet. astromech is an unofficial fan project
> and is not affiliated with Frontier Developments.

## Vision

- **One app, two processes:** a lightweight Rust **core** (tray, voice, overlay, game state,
  local API) that keeps running while you play, and an on-demand **app window** (Tauri v2 +
  Svelte 5) that hosts modules: keybinds first, then ship, engineering and route.
- **Local and safe:** no cloud account needed. Every action goes through one policy layer
  (tiers, confirmation, rate limits, kill switch) and is verified against `Status.json`.
- **Works with any controller:** push-to-talk and confirm/cancel on any HOTAS, pad, keyboard or
  mouse button, through background Raw Input. The S7 spike showed this works while Elite has
  focus.

## Design docs

- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md): architecture, crates, roadmap and spike results (§17)
- [docs/KEYBINDS.md](docs/KEYBINDS.md): the keybinds module and the health check
- [docs/REPORT.md](docs/REPORT.md): the initial research report (existing tools, data sources, voice)
- [docs/diagrams/](docs/diagrams/): Mermaid sources and rendered PNGs
- [spikes/](spikes/): source of the feasibility spikes

## Layout

```
crates/     astromech-{proto,journal,status,gamestate,keybinds,input,voice,policy,tools,overlay,store,core}
app/        Svelte 5 + TypeScript + Vite app shell (module manifests in src/modules/*)
app/src-tauri/  astromech-shell, the Tauri v2 window (its own cargo workspace)
docs/       design documents
spikes/     spike tools (own code only)
scripts/    CI helpers (dependency-rule check)
```

## Building

Requirements: Rust (pinned in `rust-toolchain.toml`), Node 22+ and pnpm 10 for the app.

```sh
cargo build --workspace          # all crates; Windows-only code compiles to stubs elsewhere
cargo test --workspace
python3 scripts/check-deps.py    # dependency rules
cargo deny check                 # licences, advisories, sources

cd app && pnpm install && pnpm build    # frontend
pnpm tauri build                        # app window (Windows; Linux needs webkit2gtk-4.1)
```

The local voice worker is behind a feature, because it downloads prebuilt sherpa-onnx
libraries and makes the binary GPL-3.0 (see NOTICE.md):
`cargo build -p astromech-voice --features sherpa --release`.

Windows binaries can be cross-built from Linux with `cargo xwin build --target x86_64-pc-windows-msvc`.
They use a static CRT (`.cargo/config.toml`), so no DLLs are needed.

## Licence

MIT, Copyright (c) 2026 Sumeet Padavala. See [LICENSE](LICENSE). Binaries built with the
optional `sherpa` voice feature are GPL-3.0 as a whole; see [NOTICE.md](NOTICE.md) and
[THIRD_PARTY.md](THIRD_PARTY.md).
