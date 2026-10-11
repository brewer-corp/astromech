# astromech v1 design

Status: **proposal for review** (branch `design/v1`). Date: 10 Oct 2026. Inputs: the gear-down
CLI preview (PR #1, merged as 93687e5), Sumeet's two real-game rounds, `ARCHITECTURE.md` v0.13
and `KEYBINDS.md` v0.7. This document decides what v1 *is*; the architecture doc stays the
reference for how the pieces work inside.

**In one paragraph:** v1 is a Windows tray app that wakes up when Elite starts. It has two
headline features: a **keybinds manager** that shows what every button on every device does,
finds and fixes problems, and tests buttons live; and an **always-on push-to-talk voice
assistant** for hands-on-stick ship toggles and read-only status questions. Everything is
local. It installs per-user without admin, updates itself from GitHub Releases, never updates
while the game is running, and looks like it belongs in the cockpit.

---

## 1. v1 scope

### 1.1 Principles
- **Small and solid beats broad.** Two features done well: keybinds and voice. Everything else
  is either infrastructure that makes those two pleasant, or it waits.
- **Read-only by default.** The only things v1 *writes* are a copy of a `.binds` file (always
  backed up first) and keypresses that the user asked for by voice.
- **Hands on sticks.** Anything needed during flight works by voice or by a bound button. The
  window is for setup and for sorting out binds.

### 1.2 In scope (v1)

**Keybinds (the biggest value).** Configuring sticks in the game's menu is miserable: one
long list per context, no picture of the device, no search by button, no warnings.

| Feature | What it does | Seed / source |
|---|---|---|
| **Profiles** | Lists every preset in the Bindings folder plus the active one per context (`StartPreset*.start`), with health badges. Duplicate `PresetName`s and file-name mismatches are flagged. | `astromech-keybinds::active`, k0-binds-doctor |
| **Health check** | Runs on app start, device plug/unplug, Bindings folder changes, before launch and on import. Checks: missing devices / PID drift (wrong-firmware warning with reflash steps), buttons above Joy_32, axes the device doesn't have, keyboard-only presets, actions that voice needs but that have no keyboard key. Each issue has a plain-English fix. | k0 + `astromech-core::health` |
| **Controller view** | A generic stick/throttle/HOTAS drawing (ours, not vendor art) per connected device, with leader lines to the action bound on each button and axis. One tab per device; his VKB pair side by side. Context switch: ship / SRV / on foot / UI. | new; layouts from a small device-layout JSON (§3.9) |
| **Ship view** | The same data grouped by what a pilot thinks about: flight, weapons, utilities, landing, UI. | new |
| **Search** | Free-text over action names and Elite's UI labels, with filters (context, bound/unbound, conflicts, "no keyboard key"). **Press a button to find it**: the reverse lookup "what does this button do?". | new |
| **Key tester** | Live strip that shows each press with its device, Elite name (`Joy_10`, `Joy_POV1Up`, `Joy_RZAxis`) and the bound actions. Read-only Raw Input, proven in S7 not to disturb the game. Bounce detection is shown. | s7-input-listener → `astromech-input::listen` |
| **Small edits, on a copy** | Exactly one class of edit in v1: **add a keyboard key to an action's free slot** (the fix voice needs), and the manual "replace device ID in a copy" from KEYBINDS §7.3. Always: back up, write a copy, tell the user to reselect the preset. | KEYBINDS §7 |
| **Backup / restore** | Automatic snapshot of the Bindings folder before any edit and when files change; restore any snapshot. | new, small |

**Voice (the second value).**
- PTT voice commands with the §5.12 matcher, did-you-mean, policy, kill switch and silent
  Status.json verification exactly as in the preview, plus the VR accuracy work in §2.
- **Current set:** landing gear up/down/toggle, lights on/off, hardpoints deploy/retract,
  cargo scoop open/close, night vision on/off, yes/no. Known-but-unavailable intents answer "I
  can't do that yet".
- **Recommended additions (tier 0/1, all toggles with a Status.json flag to check against):**

| Command | Tier | Verification signal | Why it's safe |
|---|---|---|---|
| Flight assist on/off | T1 | Flags bit 5 | Toggle, reversible, state-checked |
| HUD combat / analysis mode | T1 | Flags bit 27 | Pure UI mode |
| Silent running on/off | T1 | Flags bit 10 | Reversible; spoken warning about heat when on |
| Galaxy map / system map open | T1 | GuiFocus 6 / 7 | UI only |
| Close map / panel ("back") | T1 | GuiFocus 0 | UI only |
| Left / right / comms / role panel | T1 | GuiFocus 1–4 | UI only |
| SRV handbrake, drive assist | T1 | Flags 12 / 15 | SRV only, toggles |
| Throttle to zero | T1 | (none reliable) | Stops the ship; refused in supercruise near a body |
| Boost, deploy heat sink, chaff, FSD jump/supercruise, docking request | **later** (T2) | — | Consumables, combat or navigation: need the confirm flow and more S5-style testing |

- **Read-only status questions (tier 0, no key ever pressed),** answered from Status.json, the
  journal, `Loadout`, `Cargo.json` and `NavRoute.json`:

| Question | Source |
|---|---|
| "Fuel?" / "How much fuel?" (main tank, reservoir, rough jumps left) | Status.json `Fuel`, Loadout `FuelCapacity` |
| "Cargo?" (used / capacity, top items) | Status.json `Cargo`, Cargo.json, Loadout `CargoCapacity` |
| "Jump range?" | Loadout `MaxJumpRange` (laden range: computed later) |
| "Where am I?" (system, body, station) | Location/FSDJump/Docked/ApproachBody |
| "Jumps left?" / "Next system?" | NavRoute.json |
| "Hull?" / "Shields?" | Loadout `HullHealth`, Status.json flag bit 3 |
| "Credits?" / "Rebuy?" | LoadGame, Loadout `Rebuy` |
| "What's my ship?" | Loadout `Ship`, `ShipName`, `ShipIdent` |
| "Legal state?" | Status.json `LegalState` |

**Shell.**
- Tray icon and popover (status, last commands, kill switch, open window), settings,
  first-run onboarding (§1.4), logs and the diagnostics bundle.
- Self-update, autostart at login, wakes when Elite runs (§4).

### 1.3 Later (with the reason)
| Item | Why not v1 |
|---|---|
| VR overlay (SteamVR, egui) | High value in VR but a large surface (ARCHITECTURE §15). v1 keeps VR usable through voice and spoken replies; the overlay is the first thing after v1. |
| Full binds editor (drag-to-rebind any action, conflict resolution, curves/deadzones) | The viewer + health check + "add keyboard key" covers the pain; a full editor needs much more testing against Elite's file quirks. |
| Tier-2 voice (FSD, docking request, heat sink, plotting) | Needs the confirm flow, more game-behaviour measurement, and has EULA questions for anything automated. |
| Second-screen web dashboard | The window covers flat-screen; VR needs the overlay instead. |
| Route, trading, engineering, logbook modules | Separate products in the module roadmap (§18.7). |
| Per-ship auto-switching of presets | Elite has no API to switch presets; it would mean editing `StartPreset.start` under the game. |
| Printable reference card / PDF export of binds | Nice, cheap later (the controller view already renders it). |

### 1.4 Out of scope for v1 (and probably for good)
- Cloud LLM / always-listening wake word (privacy, latency, cost; PTT is the right UX in a cockpit).
- Macros, timers, auto-honk and any automation that acts without a spoken request (EULA grey zone, ARCHITECTURE §11).
- Combat assistance of any kind.
- macOS/Linux builds (Elite is Windows; Proton users can run the Windows build later).
- Languages other than English for voice.

### 1.5 Other things worth adding (ranked)
Value and effort are relative (S ≈ ≤ 3 days, M ≈ 1–2 weeks, L ≈ 3+ weeks).

| # | Item | Value | Effort | v1? | Note |
|---|---|---|---|---|---|
| 1 | **First-run onboarding wizard**: find Elite, detect devices, press-to-bind PTT and kill switch, mic pick + level check + 3-phrase calibration, binds health check with one-click fixes | High | M | **v1** | Removes every failure Sumeet hit in the CLI rounds (misnamed config, wrong PTT, quiet mic, missing keyboard keys). |
| 2 | **Diagnostics bundle export** (zip: logs, redacted config, health report, device list, last N clip stats, versions; no audio unless ticked) | High | S | **v1** | Every remote debug round so far needed this. |
| 3 | **Binds backup/restore** | High | S | **v1** | Prerequisite for any edit; cheap. |
| 4 | **Personal phrase learning**: when the user confirms a did-you-mean, store "you're down → gear down" as a per-user alias that executes next time (still capped by policy) | High | S | **v1** | The most direct fix for *his* recurring mishearings; stays local. |
| 5 | **Spoken game-event callouts** (read-only, journal): low fuel, fuel scoop full, interdiction, under attack, heat warning, hull damage, scan complete; each with a toggle; master toggle default **on with three events**: low fuel, interdiction, heat | Med–High | S–M | **v1 (small set)** | Reuses the journal pipeline and Kokoro. Never presses keys. |
| 6 | VR-readable minimal overlay (status + last command + did-you-mean) | High (VR) | L | later | First post-v1 item. |
| 7 | Binds diff between two presets / before-after an update | Med | S | later | Small but not core. |
| 8 | Print/PDF reference card | Med | S | later | |
| 9 | HUD colour from the game (§3.1) | Low–Med | S | **v1** | Tiny; nice touch. |

---

## 2. Voice accuracy in VR

### 2.1 What we measured
- **Preview round 2:** in VR the clips come in at **−46 to −49 dBFS RMS, loudest 30 ms frame
  −38/−39 dBFS**. That's very quiet: speech at a normal desk mic usually peaks around −20 to
  −10 dBFS. "You're down." for "gear down" is typical of an under-driven input.
- PTT, bounce filtering and execution are fine (bounces of 29–53 ms now ignored; gear confirmed
  in 9 ms). The weak link is the audio reaching Parakeet.

### 2.2 Plan (in order)
1. **Make sure we record the right microphone.** SteamVR's audio setting can switch the
   Windows default recording device to the headset while SteamVR runs ("Audio → Set
   microphone to …"). The preview follows the *default* device, so in VR it may record the
   headset mic, not the BlackShark. Fix: astromech records a **chosen device by ID**, logs the
   device name on every clip, and warns when the default changes. Onboarding asks which mic
   he uses in VR.
2. **Gain normalisation before STT.** Per clip, after the existing gate: measure the speech
   level (loudest 30 ms frames above the noise floor), apply gain so speech peaks reach about
   **−6 dBFS** (max **+30 dB**), with a soft limiter, and no gain on frames under the noise
   floor + 6 dB (so we don't amplify hiss into words). At his −39 dBFS loudest frame that's
   about +30 dB. This is a pure Rust step, deterministic and testable offline.
3. **Per-user calibration in onboarding.** Three phrases ("gear down", "deploy hardpoints",
   "request docking"): measure noise floor and speech level, set the gain and the
   `min_level_dbfs` gate, check the transcripts, store per device. Re-run from settings.
4. **Domain-vocabulary matcher (already in).** v0.13 vocabulary normalisation, the mishearing
   cap and known-but-unavailable intents stay; add **personal phrase learning** (§1.5 #4).
5. **Second pass with a bigger model (option).** When the match lands in the ask band
   (0.60–0.85), re-decode the same clip with a larger model and take the better match. Candidates
   to measure: Parakeet TDT 0.6B **v3**, and a Whisper-small/medium int8 via sherpa-onnx. Only
   worth it if it adds < 400 ms on his CPU; it stays off by default until measured.
6. **Hotwords: blocked.** Supported for Parakeet TDT only with `modified_beam_search` and a
   `bpe.vocab` we'd have to generate, and that decoder currently hallucinates "Yeah." or returns
   empty text ~20% of the time (sherpa-onnx issue #3267; fix PR #3657 open). Re-test when it's
   merged.
7. **n-best / confidence.** The pinned sherpa-onnx Rust API (1.13.8) returns only text, tokens,
   timestamps and durations: **no n-best and no per-token probabilities.** So: (a) use the
   matcher score as the confidence (as now); (b) cheap proxy signals from durations (very short
   or very long token durations flag a bad decode); (c) if the C API's JSON result exposes
   `ys_log_probs` for offline transducers in a newer release, wire it through the worker and
   blend it with the matcher score. Beam search would give n-best, but it's blocked by #3267.

### 2.3 What to measure next
- **A labelled VR clip set**: 50+ PTT clips of his real commands in VR and 50 at the desk,
  saved opt-in with the device name, raw RMS/peak, transcript and what he meant (the confirmed
  command). This becomes the offline eval set (a `cargo run` harness).
- Command accuracy (exact intent) and execute/ask/miss rates for: raw vs normalised gain;
  default device vs chosen mic; Parakeet v2 vs v3; + second pass. **Target: ≥ 95% correct
  execute-or-ask, ≤ 1% wrong executes.**
- Second-pass latency on his Ryzen 9 7900X.
- Whether SteamVR actually switched his default mic (one look at the log line from step 1).

---

## 3. Visual design language

Goal: it should feel like a panel from the cockpit, not a web page. **No Frontier assets**:
no game fonts, logos, ship art, sounds or screenshots. Everything below is drawn or chosen by
us. Tokens: `app/src/design/tokens.css`; HUD tint: `app/src/design/hud.ts`; components in
`app/src/design/`; mockups in `app/src/mockups/` (open `#/mockup/<id>`).

### 3.1 Colour
| Token | Value | Use | Contrast on `--am-bg` |
|---|---|---|---|
| `--am-orange` | `#ff7a00` | Primary: titles, active state, lines | 7.8:1 |
| `--am-orange-bright` / `-dim` | `#ffa040` / `#b85700` | Hover / pressed | |
| `--am-orange-line` | 45% orange | Panel borders, dividers | |
| `--am-text` / `--am-text-strong` | `#ffd9b0` / `#fff1e0` | Body text (warm off-white, like HUD text) | 15.3:1 |
| `--am-muted` | `#b08a66` | Secondary text, labels | 6.5:1 |
| `--am-blue` | `#4ab6ff` | Info, live input, "yours", selection | 9.1:1 |
| `--am-red` | `#ff3b2f` | Danger, errors, kill switch | 5.7:1 |
| `--am-amber` | `#ffd23f` | Warnings | 14.1:1 |
| `--am-green` | `#7bd88f` | OK / armed | 11.7:1 |
| `--am-bg` / `--am-panel` | `#070504` / `#120c07` | Warm near-black surfaces | |

- **Why these:** the game's HUD is orange on dark with blue for friendly/info and red for
  hostile/danger; we mirror that structure, not exact game colours.
- **HUD colour from the game.** The in-game HUD colour is user-configurable through the
  `GUIColour/Default/MatrixRed|Green|Blue` rows in
  `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Graphics\GraphicsConfigurationOverride.xml`
  (identity = default orange; each output channel is a weighted mix of the default r, g, b).
  v1 reads it (read-only) and applies the same matrix to the orange ramp and text tokens
  (`hud.ts: parseMatrix/applyMatrix`). **Semantic red/amber/blue/green stay fixed** so warnings
  keep their meaning even with a green HUD. Setting: "Match my in-game HUD colour" (default on).

### 3.2 Typography
| Role | In game (community-identified) | Licence | We use (bundled, OFL 1.1) |
|---|---|---|---|
| Titles, panel headers | Eurostile (Extended) | Commercial (Linotype/Monotype); can't bundle | **Michroma** (Vernon Adams), a Eurostile-Extended-like wide square sans |
| UI text, numbers | Euro Caps / Eurostile | Euro Caps is "freeware" with no clear licence; avoid | **Saira Semi Condensed** (Omnibus-Type), square, condensed, readable small |
| Codes (`Joy_10`, `Key_L`, dBFS) | Telegrama (seen in extracted assets) | Commercial; avoid | **Share Tech Mono** |

- Fonts come from `@fontsource/*` packages (OFL), bundled into the app; no network font loads.
- Titles are uppercase with wide tracking (`--am-track-caps`); body text is sentence case for
  readability (the game is all-caps; for long text that hurts).
- Font identification is from the Frontier forums and fan asset packs, not confirmed by Frontier.

### 3.3 Shapes
- **Chamfered panels:** top-left and bottom-right corners cut at 45° (`.am-chamfer`, 10 px; 6 px
  for buttons). Thin 1 px orange borders at 45% alpha, a header strip with a solid orange "tick",
  a faint scanline texture (2.5% alpha, purely decorative).
- Active nav/tab = solid orange block with dark text (the game's selected-row style).
- Glow (`--am-orange-glow`) only on the few things that are "live": active state, the meter,
  the selected callout.

### 3.4 Motion
- Short and mechanical: 90 ms for hover/press, 180 ms for panels sliding in from the edge,
  ease-out. No bounces or spring physics.
- Live input (key tester, PTT listening) pulses a small square at 1 Hz.
- `prefers-reduced-motion` sets all durations to 0.

### 3.5 Sound cues
- Our own synthesized earcons (generated in code: short filtered sine/square blips), never game
  audio. Four cues: **listening** (rising two-tone, on PTT down, optional), **done** (single
  soft blip, only when no spoken confirmation), **ask** (rising-falling), **refused/killed**
  (falling two-tone). Distinct by *contour*, not just pitch. Volume setting; off by default
  except refused/killed.

### 3.6 Iconography
- Geometric line icons, 1.5 px strokes, square ends, built on the same 45° chamfer; status chips
  always pair a glyph (✓ ! ✕ i) with a word.
- Device drawings are generic (stick, throttle, HOTAS, gamepad, keyboard, mouse) drawn by us in
  SVG; never vendor product art or Frontier ship silhouettes.

### 3.7 Accessibility
- All text tokens meet WCAG AA on the panel colours (table above; disabled text at 2.8:1 is
  exempt by WCAG and is never the only carrier of information).
- **Colour-blind safety:** state is never colour-only: chips have glyph + word, warnings have
  "!" and a border style, the key tester labels everything in text. Orange/blue is the main
  pair (safe for the common red-green deficiencies); red vs green is never the only difference.
- Full keyboard navigation and visible focus (`:focus-visible` = solid orange fill), screen
  reader labels on toggles and meters, UI scale setting (90–150%).

### 3.8 Mockups
Static Svelte pages built from the tokens (sample data; device IDs are Sumeet's VKB pair):

| Mockup | File | Screenshot |
|---|---|---|
| Tray popover | `app/src/mockups/TrayPopover.svelte` | `docs/design/mockup-tray.png` |
| Keybinds: controller view (VKB OT L + EVO R) | `ControllerView.svelte`, `Stick.svelte` | `docs/design/mockup-keybinds.png` |
| Keybinds: search / list | `BindsList.svelte` | `docs/design/mockup-search.png` |
| Voice & controls settings (press-to-bind, mic meter, calibration) | `VoiceSettings.svelte`, `Meter.svelte` | `docs/design/mockup-voice.png` |

Regenerate: `scripts/mockup-shots.sh` (builds the app, serves it with `vite preview` and
captures with headless Chrome).

### 3.9 Device layouts
The controller view needs to know where buttons sit on each grip. v1 ships a small JSON per
**generic** layout (stick, throttle, HOTAS, gamepad) with anchor points, plus per-device
mappings (VID:PID → layout + `Joy_N` → anchor) for the devices we can test (his VKB pair first).
Unknown devices get the generic list layout until the user places buttons with the key tester
("press the button, click where it is").

---

## 4. Infrastructure

### 4.1 Process and tray model (recommendation)
**Recommendation: keep two processes, and the core owns the tray.**

- `astromech-core.exe` (Rust, no WebView) is the only process that's always running: tray icon
  (the `tray-icon` crate), game watcher, journal pipeline, input listeners, voice, the local API.
  Idle memory target ≈ 20–40 MB; models load only while Elite runs (§4.6).
- `astromech-shell.exe` (Tauri v2 + Svelte) is the window. It starts on demand (tray click,
  onboarding, update install) and talks to the core over the local API. Closing the window
  **exits the shell after 5 minutes idle**, freeing WebView2 (typically 100–200 MB across its
  processes).
- **The tray popover** (mockup) is a small frameless shell window anchored to the tray. A cold
  shell start takes about 0.5–1 s with WebView2 already installed; that's acceptable for a click,
  and the 5-minute warm period makes repeat opens instant. **Right-click** gives a native menu
  from the core (Open, Pause voice, Kill switch, Quit) that works even if the shell is broken.

Why not "the shell always running minimised"? It keeps WebView2 resident the whole time the PC
is on (and the game is a memory-heavy VR title); it couples voice and input reliability to the
webview's; and a renderer crash would take the tray down. ARCHITECTURE §18.1 (v0.7) already
chose two processes; the "always in tray" requirement doesn't change that, it only moves the
tray into the core (the v0.7 text already says the core stays in the tray while the game runs).

### 4.2 Packaging
- **Tauri bundler, NSIS installer, `installMode: "currentUser"`** → installs to
  `%LOCALAPPDATA%\Programs\astromech`, no admin, no UAC prompt. One installer carries
  `astromech-shell.exe`, `astromech-core.exe`, `astromech-voice-worker.exe` (as Tauri
  `externalBin`/resources) and the OFL fonts. Speech models download on first run (≈ 700 MB,
  SHA-256 checked, as now) so the installer stays small.
- WebView2: use the bootstrapper (`webviewInstallMode: downloadBootstrapper`); Windows 11 already has it.
- Uninstall removes the program; user data (`%LOCALAPPDATA%\astromech`) stays unless the
  "remove my data" box is ticked.

### 4.3 Self-update
- **Tauri updater plugin** with minisign-signed artefacts. Manifests on GitHub Releases:
  `latest.json` for **stable** and `beta.json` for **beta** (published on pre-release tags);
  the channel is a setting. The updater public key is compiled in.
- The update is the NSIS installer, so it **updates the shell, core and worker together**.
- **Never while Elite is running.** The core checks the manifest (at start and every 6 h, with
  the same minisign signature check via `minisign-verify`); if an update exists it shows
  "update ready · installs after Elite exits" in the tray. When the game isn't running (or the
  user clicks Install now while it isn't), the core launches the shell with `--install-update`;
  the shell downloads and verifies the update, asks the core to shut down (it cancels
  in-flight actions, releases keys, closes the mic), runs the installer in passive mode, and the
  installer restarts the core. If the user insists while Elite is running, we still refuse:
  updating mid-flight would kill voice and the kill switch.
- Rollback: the previous installer is kept; "Reinstall previous version" in diagnostics.

### 4.4 Code signing and SmartScreen
- **Start unsigned** for the first beta builds: Windows shows "Windows protected your PC" (More
  info → Run anyway) and some browsers warn on download. Acceptable for Sumeet and a few testers;
  documented in the README.
- **Apply to SignPath Foundation** (free code signing for open-source projects, with its own
  certificate and a CI-based signing flow) in parallel; requirements include an OSI licence
  (MIT is fine), a public repo and reproducible CI builds. Signed builds still have to build
  SmartScreen **reputation** over time (since 2024 even EV certificates don't skip it), but the
  warning shows the publisher and fades as downloads accumulate.
- Fallback if SignPath declines: Azure Trusted Signing (low monthly cost; availability for
  individuals depends on region).
- The minisign update signature is separate from Authenticode and is used from day one.

### 4.5 Autostart
- **HKCU `Run` key** (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, value
  `astromech` → `"…\astromech-core.exe" --background`), set by the installer and toggled in
  settings. No admin; it's what the Tauri autostart plugin uses. Task Scheduler would only add
  a start delay and restart-on-failure, at the cost of complexity (and some options need admin).
- `--background`: tray only, no window, low priority until the game starts.

### 4.6 Game detection and "becomes active"
- **Poll the process list every 3 s** (`CreateToolhelp32Snapshot`; under 1 ms per check) for
  `EliteDangerous64.exe`; also note `EDLaunch.exe` (launcher open → pre-warm). WMI process-start
  events would avoid polling but add COM/WMI dependencies and, for user processes, still poll
  internally; not worth it. **Journal folder activity** (a new `Journal.*.log`) is the second
  signal and confirms the session.
- **Idle** (game not running): tray, update checks, the window if opened (keybinds work fully,
  including the key tester). No mic, no PTT listener, no models in memory.
- **Active** (game running): load STT/TTS (≈ 4–6 s in the background), open the mic, start the
  PTT/kill-switch listener and the journal/Status pipeline, run the binds health check. Speak
  "Astromech online" (optional). When the game exits: unload models (free ≈ 1–2 GB), close the
  mic, go idle. Setting: "Keep voice loaded" for fast restarts.

### 4.7 Single instance, crash restart, logs
- **Single instance:** a named mutex `Local\astromech-core` in the core (second start → tell
  the first to open the window, then exit); the shell uses Tauri's single-instance plugin.
- **Crash restart:** the shell checks the core on start and restarts it if missing; the core
  installs a panic hook that writes the panic + backtrace to the log and a crash marker. On the
  next start the tray offers "astromech crashed last time · Save diagnostics". The voice worker is
  already a supervised child (restarted by the core).
- **Logs:** rolling files in `%LOCALAPPDATA%\astromech\logs` (7 days, 10 MB each), the same lines
  as the preview console; usernames redacted. **Diagnostics bundle** (§1.5 #2) from the tray or
  settings.

### 4.8 Release CI
On a `v*` tag (pre-release tags → beta channel):
1. Windows runner: build core, worker (`sherpa` feature), shell (`tauri build`) with `--locked`.
2. Run the test suite and the Wine-free Windows e2e.
3. Sign the exes and the installer (SignPath step once approved; skipped while unsigned).
4. Sign the updater artefact with minisign (`TAURI_SIGNING_PRIVATE_KEY` secret).
5. Create the GitHub Release with the installer, `.sig`, checksums and NOTICE files; generate
   and upload `latest.json` / `beta.json`.
6. Smoke test: download the release on a clean runner, install per-user, start, check version.

### 4.9 GPL caveat
`astromech-voice-worker.exe` links sherpa-onnx with espeak-ng (GPL-3.0), so that binary is
GPL-3.0 as a whole; everything else is MIT. The NSIS licence page shows the MIT licence and a
clear notice: "The speech worker (astromech-voice-worker.exe) is licensed under GPL-3.0. Its
complete source is at <repo URL and tag>." `COPYING-GPL-3.0.txt` and `NOTICE.md` are installed
next to the exe and linked from Settings → About. Because the worker is a separate process that
talks over a pipe, the rest of the app stays MIT.

---

## 5. Phased plan

Rough effort for one developer working with an agent; Sumeet tests at each milestone.

| Milestone | Contents | Effort | Exit test (Sumeet) |
|---|---|---|---|
| **M0 Foundation** | Core as tray app (single instance, autostart, game watcher, idle/active), core↔shell API, shell skeleton with the design tokens, logs, NSIS per-user installer, updater + release CI (unsigned) | ~2 weeks | Install without admin; tray appears at login; voice wakes when Elite starts; an update from beta 0.1 → 0.2 installs only after the game exits |
| **M1 Keybinds viewer** | Profiles, health check UI, controller view (generic layouts + VKB mapping), ship view, search + press-to-find, key tester, backup/restore | ~3 weeks | He finds any of his binds in < 10 s and sees every health warning with a fix |
| **M2 Voice v1** | Chosen mic + default-change warning, gain normalisation, calibration, personal phrase learning, the extra T1 commands, status questions, eval harness on his clip set | ~2–3 weeks | ≥ 95% correct execute-or-ask on his VR clip set; status questions answered |
| **M3 Onboarding + extras** | First-run wizard, diagnostics bundle, "add keyboard key in a copy", game-event callouts (3 defaults), HUD-colour matching | ~1.5 weeks | A fresh install to working gear-down by voice in < 5 minutes without reading docs |
| **M4 Release** | Signing (if approved), beta → stable, README/website, polish from testing | ~1–1.5 weeks | v1.0 on GitHub Releases |

Total ≈ **10–11 weeks**. The order puts infrastructure first because every later milestone ships
through the updater to Sumeet.

---

## 6. Decisions for Sumeet
1. **Scope:** are keybinds viewer + voice + the five v1 extras (onboarding, diagnostics bundle, backup/restore, phrase learning, three callouts) the right v1, with the VR overlay first after v1?
2. **Edits:** is "add a keyboard key to a free slot, in a copy" enough editing for v1, or do you need full rebinding in v1?
3. **New voice commands:** which of flight assist, HUD mode, silent running, maps/panels, SRV toggles and throttle-to-zero do you want in v1?
4. **Callouts:** default on with low fuel, interdiction and heat, or default off?
5. **VR mic:** which microphone do you use in VR: the BlackShark or the headset mic? Is SteamVR set to switch the default mic?
6. **Clip recording for the eval set:** OK to save your PTT clips locally (opt-in, deletable) for a few sessions so we can tune accuracy?
7. **Second-pass model:** accept up to ~400 ms extra on unsure commands for better accuracy?
8. **Updates:** stable only for you, or the beta channel? Is "never update while Elite is running" right, with no override?
9. **Signing:** fine to start unsigned (SmartScreen "Run anyway") while we apply to SignPath?
10. **Autostart:** start with Windows by default (tray only, idle until Elite runs), or opt-in?
11. **Look:** the mockups use a warm orange/blue palette and Michroma/Saira; does it feel right in and out of VR? Should it follow your in-game HUD colour by default?
12. **Name:** keep "astromech" as the product name?
