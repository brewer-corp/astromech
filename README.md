# astromech

A voice-first copilot for **Elite Dangerous** on Windows. It runs fully local: push-to-talk
speech recognition, spoken confirmations, game-state awareness from the journal and
`Status.json`, verified in-game actions, a keybinds manager with a built-in health check, and a
SteamVR overlay. Everything lives in one app.

> **Status: pre-alpha.** The first vertical slice (push-to-talk "gear down", verified against
> Status.json; see [Try the gear-down slice](#try-the-gear-down-slice)) works end to end in
> tests. Most other crates are still stubs seeded from the spikes, and nothing is usable as a
> product yet. astromech is an unofficial fan project
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
- [docs/REPORT.md](docs/REPORT.md): the initial research report (data sources, voice, EDCoPilot/COVAS as of 8 Oct)
- [docs/COMPETITORS.md](docs/COMPETITORS.md): copilots and companions as of 11 Oct, and what to steal
- [docs/diagrams/](docs/diagrams/): Mermaid sources and rendered PNGs
- [spikes/](spikes/): source of the feasibility spikes

## Layout

```
crates/     astromech-{proto,journal,status,gamestate,keybinds,input,voice,policy,tools,overlay,store,materials,core}
app/        Svelte 5 + TypeScript + Vite app shell (module manifests in src/modules/*)
app/src-tauri/  astromech-shell, the Tauri v2 window (its own cargo workspace)
docs/       design documents
spikes/     spike tools (own code only)
scripts/    CI helpers (dependency-rule check); `scripts/ui` drives the Vite shell in headless Chrome
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

## Install (M0 preview)
Download `astromech_<version>_x64-setup.exe` from the latest
[Release build](https://github.com/spadaval/astromech/actions/workflows/release.yml) run (or a
GitHub Release once tagged). It installs per-user to `%LOCALAPPDATA%\astromech`, no admin. The
build is **unsigned**: SmartScreen says "Windows protected your PC" → More info → Run anyway.

astromech then runs in the tray and starts with Windows (untick "Start with Windows" in the tray
menu to stop that). It stays idle until Elite (or its launcher) starts, then loads voice. Your
config is `%LOCALAPPDATA%\astromech\data\astromech.toml` (tray → Open config file); speech
models go in `data\models` (tray → Download speech models). Logs: `data\logs`, or tray → Save
diagnostics. Updates are manual: run a newer installer.

## Try the gear-down slice

A Windows console preview of the first vertical slice: hold push-to-talk, say **"gear down"**,
and astromech presses your landing-gear key, says "Landing gear deployed.", and checks Status.json
to confirm it worked. It also understands lights, hardpoints, cargo scoop, night vision, flight assist,
silent running (asks before turning on), HUD mode, maps and panels, throttle to zero,
and read-only status questions (fuel, cargo, location, …). It only presses keys that
have a **keyboard** binding in your active ship preset.

**Safe conditions for the gear test: in normal space only.** Not docked, not landed, not in
supercruise, not in a fight, ideally hovering or drifting well clear of a station or planet
surface. astromech refuses the gear when docked, landed or in supercruise anyway, but test where
an unexpected gear change can't hurt.

1. **Unzip** `astromech-gear-down-preview.zip` to a folder of its own outside the game's
   folders, e.g. `Desktop\astromech-preview`. It holds `astromech-core.exe`,
   `astromech-voice-worker.exe`, `astromech.sample.toml` and the licence files.
2. **Speech models** (same files as the S9 test):
   - if you still have the S9 folder, copy its `models` folder next to `astromech-core.exe`
     (it holds `sherpa-onnx-nemo-parakeet-tdt-0.6b-v2-int8` and `kokoro-en-v0_19`), or set
     `models_dir` in the config to that folder;
   - otherwise run `astromech-core.exe --download-models` (about 800 MB, SHA-256-checked).
3. **Config:** run `astromech-core.exe --write-config`, then open `astromech.toml`. Keep the
   name exactly `astromech.toml`: if Notepad saves it as `astromech.toml.txt`, astromech
   refuses to start and names the file it found. The console's first lines show the config
   path it used. It also refuses to start without a `ptt` button (except `--listen`,
   `--health`, `--transcribe`, `--download-models`, `--list-mics`, `--eval` and
   `--calibrate-files`).
   - `ptt` is preset to `{ device = "231D3201", key = "Joy_4" }` (the Omni Throttle left-stick
     button 4 that worked as PTT in S9).
   - Set `kill_switch` to a spare button. To see a button's name, run
     `astromech-core.exe --listen 20` and press it; it prints the line to paste.
4. **Keyboard bind:** in Elite, give **Landing Gear** a keyboard key (e.g. as its secondary
   binding), since astromech presses keys, not joystick buttons. Check with
   `astromech-core.exe --health`, which should show `LandingGearToggle  Key_…`. (Lights,
   hardpoints, cargo scoop and night vision need keyboard binds too, otherwise they're refused
   with a hint.) The full report goes to `astromech-data\health.html`.
5. **Dry run first:** start Elite, get into your ship in normal space, then run
   `astromech-core.exe --dry-run` and switch back to the game. Hold PTT, say "gear down",
   release. You should hear "Landing gear deployed." and the console shows the key it *would* have
   sent. Nothing is pressed, so 2 s later the console logs "set_landing_gear: not confirmed
   within 2.0 s (no retry)". That's expected in a dry run. Nothing is spoken for it.
6. **Live:** run `astromech-core.exe` (not as administrator, and the game not as
   administrator either), then switch to Elite in normal space.
   - "gear down": the gear deploys, and you hear "Landing gear deployed.".
   - "gear down" again: "Landing gear is already deployed.", and nothing is pressed.
   - "gear up": the gear retracts.
   - Press the kill switch, then say "gear down": it's refused. Hold the kill switch for 1 s
     to re-arm.
   - In supercruise, "gear down" says "Landing gear not available: you're in supercruise."
   - Mumbled or near-miss phrases get "Did you mean …?" (answer "yes" or "no" on the next PTT).
     Anything else gets "Didn't catch that.".
7. You can also **type** a command into the console and press Enter: `gear down`, `!kill`,
   `!rearm`, `!state`, `!health`, `quit`. While you type, the console has focus, so anything
   that would press a key is refused ("Elite isn't the active window…"). Typed commands are
   useful for checking state answers and refusals, not for pressing keys.

**Each push is logged:** `PTT down (t=…)` and `PTT up (t=…) — held N ms; clip N ms, RMS … dBFS,
loudest frame … dBFS; transcribing`. Taps under 300 ms, near-silent clips (loudest frame below
−45 dBFS) and noise-only transcripts ("Mm-hmm.", "Yeah.") are ignored without a reply. A switch
bounce shows as `PTT bounce ignored`. If real speech gets ignored as too quiet, lower
`min_level_dbfs` in `[voice]` (e.g. to −55). Set `mic` to a device-name substring
if SteamVR stole the Windows default; `--list-mics` lists them. After the gate,
clips are gain-normalised toward −6 dBFS. `save_clips = true` keeps WAVs under
`data/clips` for `astromech-core --eval DIR`. `--calibrate-files silent.wav
gear.wav …` stores a per-device gate. A confirmed "Did you mean …?" is remembered
as a personal alias.

**If PTT does nothing:** start with `astromech-core.exe --debug-input`. Every button press is
logged with its device ID and Elite name (e.g. `input: button Joy_15 on 231D3201 (...) down ->
PTT`), and the startup lines show whether the PTT device is connected. Without the flag, the
first few presses of *other* buttons on the PTT device are still logged (`Joy_14 on 231D3201
pressed; PTT is Joy_15`). A working press logs `PTT down — listening` and `PTT up —
transcribing (N ms audio)`. Note that `--debug-input` also logs keyboard keys.

Without the models (or with `--no-voice`), replies are printed instead of spoken, and only
typed commands work. Everything astromech writes goes to `astromech-data` next to the exe. It
never writes to the game's folders.

## Licence

MIT, Copyright (c) 2026 Sumeet Padavala. See [LICENSE](LICENSE). Binaries built with the
optional `sherpa` voice feature are GPL-3.0 as a whole; see [NOTICE.md](NOTICE.md) and
[THIRD_PARTY.md](THIRD_PARTY.md).
