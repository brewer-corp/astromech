# astromech v1 design

Status: **proposal for review, revision 2** (branch `design/v1`). Date: 10 Oct 2026. Inputs:
the gear-down CLI preview (PR #1, 93687e5), Sumeet's real-game rounds, his review of
revision 1, `ARCHITECTURE.md` v0.13 and `KEYBINDS.md` v0.7. This document decides what v1 *is*;
the architecture doc stays the reference for how the pieces work inside.

**Revision 2 (10 Oct, 10:30 PM ET).** Sumeet rejected the revision-1 mockups as decoration,
not design: random colours, cards, icons, boxes and dividers, four equal-weight panels, a
calibration wizard inside settings and a press log inside the button config. This revision
starts from the tasks (§2), derives a minimal visual system from the game's own Controls screen
(§4), and only then draws (§5). Decisions applied:
- **VR is out of the app entirely.** No overlay, no in-headset UI. Voice still has to work
  while he flies in VR (§6), but nothing on screen needs to.
- **Releases are unsigned.**
- **No automatic updates.** No updater plugin, no update channels, no "never while Elite runs"
  rule. Only a manual "Check for updates…" that opens the GitHub release page.
- **No colour or theme customisation**, no following the in-game HUD colour: Elite's stock
  orange only.
- **"Save push-to-talk clips" is a permanent Debug setting** (off by default), not a one-off.

**In one paragraph:** v1 is a Windows tray app that wakes up when Elite starts. It has two
headline features: a **keybinds manager** (the game's Controls list, made searchable and
honest about problems, plus the manufacturer's sheet for "my controller"), and an
**always-on push-to-talk voice assistant** for hands-on-stick ship toggles and read-only status
questions. Everything is local. It installs per-user without admin.

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
long list per context, tiny all-caps text, no search, no "what does this button do?", no
warnings. The main view keeps the game's structure (so it's instantly familiar) and fixes
those problems; see §3 for why this beats a pictorial main view.

| Feature | What it does | Seed / source |
|---|---|---|
| **Keybinds list** (main view) | The game's Controls list, cloned and improved: same categories and expandable groups, primary/secondary columns and an "about this setting" panel, plus search, "press a button to jump to it", larger type, and inline health/conflict markers. | new; data from `astromech-keybinds` |
| **Health check** | Runs on app start, device plug/unplug, Bindings folder changes, before launch and on import. Missing devices / PID drift (wrong-firmware warning with reflash steps), buttons above Joy_32, axes the device doesn't have, keyboard-only presets, actions voice needs that have no keyboard key. Shown **inline** in the list (marker on the row, explanation in the about panel) and as an "Issues" filter, not on a separate dashboard. | k0 + `astromech-core::health` |
| **My controller** (secondary view) | The manufacturer's own reference sheet for each connected device, with our labels (button number + bound action) overlaid on the sheet's own fields. For VKB: their fillable PDF templates, used from the user's own copy (§3.2). Devices without a sheet get the list filtered to that device. | new |
| **Presets** | Every preset in the Bindings folder and which one is active per context (`StartPreset*.start`), duplicate `PresetName`s and file-name mismatches, backups and restore. | `astromech-keybinds::active`, k0 |
| **Small edits, on a copy** | Exactly one class of edit in v1: **assign a keyboard key to an action's empty slot** (the fix voice needs), plus the manual "replace device ID in a copy" from KEYBINDS §7.3. Always: back up, write a copy, tell the user to reselect the preset. | KEYBINDS §7 |
| **Key tester** | Live press log (device, Elite name, bound actions, bounce detection), in Diagnostics. Read-only Raw Input, proven in S7 not to disturb the game. The same listener powers "press a button to jump to it". | s7-input-listener |

**Voice (the second value).**
- PTT voice commands with the §5.12 matcher, did-you-mean, policy, kill switch and silent
  Status.json verification exactly as in the preview, plus the VR accuracy work in §6.
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
- A native tray menu (status, open, pause voice, kill switch, check for updates, quit), one
  settings list, first-run onboarding, Diagnostics (key tester, health report, logs, bundle).
- Autostart at login, wakes when Elite runs, a manual "check for updates" (§7).

### 1.3 Later (with the reason)
| Item | Why not v1 |
|---|---|
| Full binds editor (drag-to-rebind any action, conflict resolution, curves/deadzones) | The viewer + health check + "add keyboard key" covers the pain; a full editor needs much more testing against Elite's file quirks. |
| Tier-2 voice (FSD, docking request, heat sink, plotting) | Needs the confirm flow, more game-behaviour measurement, and has EULA questions for anything automated. |
| Second-screen web dashboard | The window covers it. |
| Automatic updates | Decided: none for now. A manual "check for updates" opens the release page (§7.3). |
| Route, trading, engineering, logbook modules | Separate products in the module roadmap (§18.7). |
| Per-ship auto-switching of presets | Elite has no API to switch presets; it would mean editing `StartPreset.start` under the game. |
| Printable reference card / PDF export of binds | Nice, cheap later (the controller view already renders it). |

### 1.4 Out of scope for v1 (and probably for good)
- **Any UI in VR** (decided 10 Oct): no SteamVR overlay, no in-headset panels. The app is used
  at the desk; in VR the only interface is voice and spoken replies.
- Colour themes or following the in-game HUD colour (decided): the stock orange palette only.
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
| 2 | **Diagnostics bundle export** (zip: logs, redacted config, health report, device list, clip stats, versions; saved clips only if the Debug setting is on and the box is ticked) | High | S | **v1** | Every remote debug round so far needed this. |
| 3 | **Binds backup/restore** | High | S | **v1** | Prerequisite for any edit; cheap. |
| 4 | **Personal phrase learning**: when the user confirms a did-you-mean, store "you're down → gear down" as a per-user alias that executes next time (still capped by policy) | High | S | **v1** | The most direct fix for *his* recurring mishearings; stays local. |
| 5 | **Spoken game-event callouts** (read-only, journal): low fuel, fuel scoop full, interdiction, under attack, heat warning, hull damage, scan complete; each with a toggle; master toggle default **on with three events**: low fuel, interdiction, heat | Med–High | S–M | **v1 (small set)** | Reuses the journal pipeline and Kokoro. Never presses keys. |
| 6 | Binds diff between two presets / before-after an update | Med | S | later | Small but not core. |
| 7 | Print/PDF reference card | Med | S | later | |

---

## 2. Information architecture

### 2.1 Tasks and how often
| Task | Frequency | Where |
|---|---|---|
| Install, find the game, detect devices, bind PTT + kill switch, pick and calibrate the mic, first health check | **Once** (and after a new PC or mic) | Onboarding |
| Re-calibrate after changing mic or headset | Rare | Settings → Microphone → Calibration: "Run again" (re-opens that onboarding step only) |
| "What does this button do?" / "Where is X bound?" | **Occasional**, between sessions; very frequent while setting up a new ship or device | Keybinds (search, press-to-jump) |
| Fix a health issue (missing key for voice, Joy > 32, device drift) | Occasional, prompted by a marker | Keybinds row → about panel |
| See the whole stick at a glance, print it | Occasional | My controller |
| Switch / back up / restore a preset | Rare | Presets |
| Change PTT, mic, voice responses | Rare | Settings |
| Speak a command, hear a reply, hit the kill switch | **In session**, constantly | Voice + hardware buttons; **no screen** |
| Check "is it running and listening?" | In session, at a glance | Tray icon + tray menu first line |
| Debug something with me remotely | Rare | Diagnostics → Save diagnostics |

The in-session tasks need no window at all, so the window can be optimised for calm, careful
setup work at a desk: big readable text, one thing at a time.

### 2.2 Screens (each with one purpose)
| Screen | Single purpose | Deliberately NOT here |
|---|---|---|
| **Onboarding** (first run; individual steps re-runnable) | Get from install to "gear down works" in under 5 minutes | Any setting not needed for that; explanations beyond one line per step |
| **Keybinds** (main window, default tab) | Find and understand any binding; fix what's broken | Device pictures, statistics, voice settings, a separate health dashboard, colour-coded categories |
| **My controller** | See one device as its manufacturer drew it, with what each control does | Editing; actions for other devices; anything not on that device |
| **Presets** | Which preset is active, switch, back up, restore | Bindings themselves |
| **Settings** | Every voice and control setting in **one list** | Calibration flows, press logs, live transcripts, anything that isn't a setting |
| **Diagnostics** | Prove what the app sees: key tester (live press log), device list, health report, logs, "Save diagnostics" | Settings (except links to them) |
| **Tray menu** (native) | Is it running, open the window, pause voice, kill switch, quit | Any status beyond one line; any webview |

Navigation is five plain text tabs in the window header (Keybinds, My controller, Presets,
Settings, Diagnostics), like the game's top-level options. Onboarding is a separate full-window
flow, not a tab.

### 2.3 Where each piece lives
- **Calibration** → onboarding step 4; "Run again" in Settings → Microphone opens just that
  step. Not a panel in settings.
- **Button press log / bounce display** → Diagnostics → Key tester. Settings only shows the
  bound button.
- **Health check results** → inline markers on Keybinds rows + an "Issues" filter; the full
  report (all checks, including passed ones) → Diagnostics.
- **All voice and control settings** → one Settings list, game style (label left, value right,
  description panel for the selected row), under five plain headings:
  - **Push-to-talk**: button.
  - **Kill switch**: button, hold-to-re-arm time.
  - **Microphone**: device, live input level, level correction, calibration ("Run again").
  - **Voice responses**: spoken confirmations, voice, volume, game event callouts.
  - **Debug**: save push-to-talk clips (off), log every input event (off), bounce and
    minimum-clip thresholds (tuning knobs most people never touch).
- **Last commands / transcripts** → Diagnostics (log view), not the tray.
- **Check for updates** → tray menu and Diagnostics; opens the release page.

---

## 3. The keybinds view

### 3.1 Pictorial main view vs the game's list, honestly
| | Pictorial (device drawing with callouts) | Game list, cloned and improved |
|---|---|---|
| Answers "what does this button do?" | Well, *if* the drawing is accurate for this exact grip and firmware profile | Well, via "press the button → jump to the row" |
| Answers "where is landing gear bound?" | Poorly: you scan pictures | Well: search, or the group you already know from the game |
| Coverage | Only devices we have accurate layouts for; keyboard is hopeless as a picture | Every action, every device, keyboard and mouse included |
| Contexts (ship / SRV / on foot / UI), modifiers, both slots | Crowded: 2 slots × several contexts per control | Natural: rows and columns |
| Familiarity | New | Same structure as the game (he already knows where things are) |
| Accuracy risk | High: wrong positions or numbering are worse than none | Low: it's the file |
| Effort | Large per device | One generic view |

**Recommendation: the cloned-and-improved list is the main view**, and "My controller" (the
manufacturer's sheet with our overlays) is the secondary view for seeing a stick at a glance.
A pictorial main view only wins for one question and fails the others; the list wins most
tasks and is never wrong.

Improvements over the game's list (all in the mockup):
- A search field at the top ("Search actions, or press any button…"): free text over action
  names and the game's UI labels, plus press-to-jump via the S7 listener.
- Larger type (15 px rows vs about 11 px in-game at 1080p), higher contrast, and denser rows
  (34 px) so a group fits on screen.
- Context and filter as plain text toggles (Ship · SRV · On foot · General; All · Issues ·
  Unbound) instead of separate menu trees.
- Inline markers in a fourth column ("! No keyboard key", "! Joy 33: ignored"), group headers
  show their issue count, the about panel explains the issue and offers the fix.
- Device names that say which stick (`L · JOY 3`, `R · POV1 PUSH`) instead of the game's
  ambiguous joystick glyphs; empty slots are visibly empty.
- One footer: Undo, Save to a copy…, and the file and last backup time.

### 3.2 "My controller": the manufacturer's sheet
**Findings (VKB, checked 10 Oct 2026):**
- **Where:** VKB's official site has a "Fillable Templates" page
  (<https://www.vkbgaming.com/fillable-templates/>) with fillable PDFs for the Gladiator
  NXT/EVO family, including exactly his pair:
  - Left: **"VKB Gladiator SCG OTA (LH)" v1.0**,
    `…/wp-content/uploads/2026/03/VKB-Gladiator-SCG-OTA-LH-v1.0.pdf` (Omni Throttle grip).
  - Right: **"VKB Gladiator SCG (RH)" v1.0**, `…/VKB-Gladiator-SCG-RH-v1.0.pdf`.
  - The sheets are credited "Design by UntoldForce v1.0" (a community designer) and published
    by VKB; one letter-size page (792 × 612 pt) with a product photo and labelled tables.
- **Numbering:** the sheets name *controls*, not button numbers: B1 (side button), A4 (top
  hat: push/up/down/left/right), A3 (centre hat), A1 (ministick), A2 (red button), the rapid-fire
  trigger (up/down), the main trigger (stages 1 and 2), C1 (thumb hat), D1 (pinky), base buttons
  F1–F3, Sw1 and En1 (up/down), and axes x, y, twist, throttle, A1(x), A1(y). Each control has
  two form fields: a narrow `…_1` cell for the button number and a wide `…_2` cell for its
  binding (e.g. `A2_A_1` / `A2_A_2`, `MT_A_1` for trigger stage 1; 76 widgets per sheet). **VKB
  doesn't print default numbers** because they depend on the firmware profile set in
  VKBDevCfg (modes, shift layers and the OTA variant all renumber buttons; VKB's older
  Gladiator manual shows e.g. trigger = 14 in mode 1 and 16 in mode 2). So we must **capture**
  the numbering from his device rather than assume it.
- **Licence:** neither the page nor the PDFs carry a licence or redistribution permission, and
  the photo is VKB product artwork. Treat them as all rights reserved: **we don't put them in
  the repo or the installer.**

**Chosen approach (legal and accurate):**
1. In My controller, an unmatched device shows "Get VKB's sheet": it opens VKB's templates page
   in the browser (or downloads the known URL on the user's click), and the user's own copy is
   stored under `%LOCALAPPDATA%\astromech\sheets`. "Choose sheet…" accepts any local PDF.
2. The app renders page 1 with pdf.js (Apache-2.0) and reads the PDF's **own AcroForm field
   rectangles** at runtime, so overlays land exactly in VKB's boxes and survive layout changes.
   We ship only a small mapping from field names to controls (`A2_A` → "A2 red button"), which
   is a list of facts, not VKB's artwork. The file's SHA-256 is recorded; if VKB publishes a
   new version with different field names, the mapping falls back to "unmapped fields".
3. **Capture buttons…**: the key tester walks the fields ("Press A2 (red button)"), recording
   the `Joy_N` / POV / axis each control sends. Skippable per control, re-runnable, stored per
   device ID and firmware profile name. Then each field shows the number (narrow cell) and the
   bound Elite action (wide cell); clicking a field jumps to the row in Keybinds, and pressing
   a control selects its field.
4. Other manufacturers: the same mechanism for any fillable PDF we add a mapping for; without a
   sheet, My controller shows the Keybinds list filtered to that device.

The mockup uses a **clearly labelled neutral placeholder** in place of VKB's photo; the table
boxes are drawn at the PDF's real field rectangles and the control names are VKB's. The button
numbers in it are an **example capture**, not his real numbering.

---

## 4. Visual system

Derived from the game's Options → Controls screen (colours sampled from Sumeet's screenshot).
What the game gets right, and we copy: a neutral near-black background, one orange accent,
light-grey upper-case row labels, filled value cells instead of boxes and borders, an "about
this setting" panel, two thin orange rules (under the title, above the footer actions) and
almost nothing else. Tokens: `app/src/design/tokens.css`.

| Token | Value | Use | Game reference |
|---|---|---|---|
| `--bg` | `#0c0d0f` | Window background | `#0c0d0f` |
| `--bg-list` | `#08090b` | (reserved) list column | `#08090b` |
| `--accent` | `#fa6e00` | The only colour: active tab, group open, selection, markers, rules, text actions | `#f56b02`–`#fd6f03` |
| `--text-hi` | `#ffffff` | Titles in the about panel | white |
| `--text` | `#e8e8e8` | Rows and body (16.4:1 on `--bg`) | `#e8e8e8` |
| `--text-lo` | `#a0a0a0` | Column heads, secondary (7.6:1) | (game uses grey sparingly) |
| `--cell` | `#3d220f` | Binding value cell | `#42230e` |
| `--cell-opt` | `#2b2d2e` | Option value cell (on/off, numbers, choices) | `#2c2e2d` ("REGULAR") |
| `--cell-empty` | `#1c1610` | An empty binding slot | (game: same as filled; we make empty visible) |

States, without adding colours:
- **Row focus/selection:** label turns orange and its cells get a 1 px orange outline (the game
  turns the label orange).
- **Listening / press-to-bind:** the value cell fills solid orange with dark text ("PRESS A
  BUTTON…  ESC TO CANCEL").
- **Warning / health issue:** orange "!" + plain words in the marker column; never a new colour.
- **Ignored binding** (Joy > 32): the cell text struck through and dimmed, plus the marker.
- **Conflict:** marker "! Also: <other action>"; the about panel lists both.
- **Unbound:** an empty, dimmer cell.
- **Disabled:** `--text-lo`.
- Expand/collapse: the game's orange outlined square with + / −.

Type: **Saira Semi Condensed** (OFL) for everything, **Share Tech Mono** (OFL) only for
numbers like dBFS. Rows 15 px upper case with slight tracking, about-panel body 15 px sentence
case (all-caps prose is one of the game's readability problems), titles 24 px. No icons, no
cards, no shadows, no gradients, no glow, no motion beyond instant state changes; respects
`prefers-reduced-motion` trivially.

Accessibility: every text level is at or above the game's contrast; state is always carried by
words or shape (outline, strike-through, "!") as well as orange, so it works for colour-blind
users; keyboard navigation follows the list (↑/↓, ←/→ to collapse/expand, Enter to edit, Esc
to cancel), and Ctrl+F focuses search.

---

## 5. Mockups

Static Svelte pages (open `#/mockup/<id>`), rendered with `scripts/mockup-shots.sh`:

| Mockup | File | Screenshot |
|---|---|---|
| Keybinds list (main view) | `app/src/mockups/KeybindsList.svelte` | `docs/design/mockup-keybinds.png` |
| My controller (VKB sheet, placeholder) | `MyController.svelte`, `sheet.ts` | `docs/design/mockup-controller.png` |
| Settings (one list) | `Settings.svelte` | `docs/design/mockup-settings.png` |
| Tray menu (native) | `TrayMenu.svelte` | `docs/design/mockup-tray.png` |

### 5.1 Side-by-side critique against the game
Better than the game:
- Text is bigger and the list is denser: a whole group plus its neighbours fits on screen,
  where the game shows about a dozen rows in a third of the screen.
- Search and press-to-jump exist; the game has neither.
- Problems are visible where they are (inline markers, issue counts on groups) instead of
  silently not working.
- Empty slots look empty; struck-through ignored bindings; "L · / R ·" says which stick.
- No blurred cockpit background behind the list: flat `--bg` is calmer and more legible.
- No game-pad glyph legend eating a quarter of the about panel.

Still noisier than the game (and what I changed):
- Revision 2a had two vertical lines between the list and the about panel (a dim list
  scrollbar and the panel's orange edge). **Fixed:** one orange separator, as in the game.
- Settings 2a put bounce and minimum-clip thresholds under Push-to-talk, making a three-row
  group out of one real setting. **Fixed:** they live in Debug.
- Settings 2a had a "Log folder" link row in the list. **Fixed:** removed; it's in Diagnostics.
- The tray menu 2a showed "Pause voice" ticked under "voice ready". **Fixed.**
- Two rows of filter toggles (contexts and All/Issues/Unbound) above the column heads are one
  more line than the game has; kept, because they replace the game's separate context menus.
- The VKB sheet is a bright paper page inside a dark window. That's a deliberate trade:
  accuracy over aesthetics; it's VKB's document, shown as VKB made it.
- Upper-case row labels are copied from the game for familiarity, though they read slower than
  mixed case. Open question for Sumeet (§9).

---

## 6. Voice accuracy in VR

### 6.1 What we measured
- **Preview round 2:** in VR the clips come in at **−46 to −49 dBFS RMS, loudest 30 ms frame
  −38/−39 dBFS**. That's very quiet: speech at a normal desk mic usually peaks around −20 to
  −10 dBFS. "You're down." for "gear down" is typical of an under-driven input.
- PTT, bounce filtering and execution are fine (bounces of 29–53 ms now ignored; gear confirmed
  in 9 ms). The weak link is the audio reaching Parakeet.

### 6.2 Plan (in order)
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

### 6.3 What to measure next
- **A labelled VR clip set**: 50+ PTT clips of his real commands in VR and 50 at the desk,
  recorded with the permanent Debug setting **"Save push-to-talk clips"** (off by default; WAV
  + device name, raw RMS/peak, transcript and the confirmed command, kept in
  `%LOCALAPPDATA%\astromech\clips` until deleted). This becomes the offline eval set (a
  `cargo run` harness) and stays useful for any future accuracy bug report.
- Command accuracy (exact intent) and execute/ask/miss rates for: raw vs normalised gain;
  default device vs chosen mic; Parakeet v2 vs v3; + second pass. **Target: ≥ 95% correct
  execute-or-ask, ≤ 1% wrong executes.**
- Second-pass latency on his Ryzen 9 7900X.
- Whether SteamVR actually switched his default mic (one look at the log line from step 1).

---

## 7. Infrastructure

### 7.1 Process and tray model
**Two processes, and the core owns a native tray menu.**
- `astromech-core.exe` (Rust, no WebView) is the only process that's always running: tray icon
  and **native context menu** (the `tray-icon` crate's OS menu: status line, Open astromech,
  Keybinds, Settings, Pause voice, Kill switch, Check for updates…, Save diagnostics…, Quit),
  game watcher, journal pipeline, input listeners, voice, the local API. Idle memory target
  about 20–40 MB; models load only while Elite runs (§7.6).
- `astromech-shell.exe` (Tauri v2 + Svelte) is the window, started from the tray or on first
  run, talking to the core over the local API. Closing it exits it and frees WebView2
  (typically 100–200 MB across its processes).
- Revision 1's webview tray popover is dropped: a native menu is instant, needs no WebView2,
  and works even if the shell is broken.

### 7.2 Packaging
- **Tauri bundler, NSIS installer, `installMode: "currentUser"`** → installs to
  `%LOCALAPPDATA%\Programs\astromech`, no admin, no UAC prompt. One installer carries
  `astromech-shell.exe`, `astromech-core.exe`, `astromech-voice-worker.exe` (as Tauri
  `externalBin`/resources) and the OFL fonts. Speech models download on first run (≈ 700 MB,
  SHA-256 checked, as now) so the installer stays small.
- WebView2: use the bootstrapper (`webviewInstallMode: downloadBootstrapper`); Windows 11 already has it.
- Uninstall removes the program; user data (`%LOCALAPPDATA%\astromech`) stays unless the
  "remove my data" box is ticked.

### 7.3 Updates (manual only)
- **No automatic updates in v1**: no Tauri updater plugin, no manifests, no channels, no
  background checks.
- "Check for updates…" (tray menu and Diagnostics) opens
  `https://github.com/spadaval/astromech/releases/latest` in the browser. Diagnostics shows the
  installed version so it's easy to compare.
- Installing a new version = running the new installer; NSIS closes the core first (it asks the
  core to exit cleanly via the local API, which cancels in-flight actions and releases keys).

### 7.4 Code signing
- **Releases are unsigned.** Windows SmartScreen shows "Windows protected your PC" on first run
  (More info → Run anyway) and some browsers warn on download; the README says so, with the
  SHA-256 of each installer in the release notes.

### 7.5 Autostart
- **HKCU `Run` key** (`HKCU\Software\Microsoft\Windows\CurrentVersion\Run`, value
  `astromech` → `"…\astromech-core.exe" --background`), set by the installer and toggled in
  settings. No admin; it's what the Tauri autostart plugin uses. Task Scheduler would only add
  a start delay and restart-on-failure, at the cost of complexity (and some options need admin).
- `--background`: tray only, no window, low priority until the game starts.

### 7.6 Game detection and "becomes active"
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

### 7.7 Single instance, crash restart, logs
- **Single instance:** a named mutex `Local\astromech-core` in the core (second start → tell
  the first to open the window, then exit); the shell uses Tauri's single-instance plugin.
- **Crash restart:** the shell checks the core on start and restarts it if missing; the core
  installs a panic hook that writes the panic + backtrace to the log and a crash marker. On the
  next start the tray offers "astromech crashed last time · Save diagnostics". The voice worker is
  already a supervised child (restarted by the core).
- **Logs:** rolling files in `%LOCALAPPDATA%\astromech\logs` (7 days, 10 MB each), the same lines
  as the preview console; usernames redacted. **Diagnostics bundle** (§1.5 #2) from the tray or
  settings.

### 7.8 Release CI
On a `v*` tag:
1. Windows runner: build core, worker (`sherpa` feature) and shell (`tauri build`) with `--locked`.
2. Run the test suite and the Windows e2e.
3. Create the GitHub Release with the installer, SHA-256 checksums and NOTICE files.
4. Smoke test: download the release on a clean runner, install per-user, start, check the version.

### 7.9 GPL caveat
`astromech-voice-worker.exe` links sherpa-onnx with espeak-ng (GPL-3.0), so that binary is
GPL-3.0 as a whole; everything else is MIT. The NSIS licence page shows the MIT licence and a
clear notice: "The speech worker (astromech-voice-worker.exe) is licensed under GPL-3.0. Its
complete source is at <repo URL and tag>." `COPYING-GPL-3.0.txt` and `NOTICE.md` are installed
next to the exe and linked from Settings → About. Because the worker is a separate process that
talks over a pipe, the rest of the app stays MIT.

---

## 8. Phased plan

Rough effort for one developer working with an agent; Sumeet tests at each milestone.

| Milestone | Contents | Effort | Exit test (Sumeet) |
|---|---|---|---|
| **M0 Foundation** | Core as tray app (native menu, single instance, autostart, game watcher, idle/active), core↔shell API, shell frame with the visual system, logs, NSIS per-user installer, release CI | ~1.5 weeks | Install without admin; tray at login; voice wakes when Elite starts |
| **M1 Keybinds** | The list view (groups, search, press-to-jump, inline health), about panel, Presets, backups, "assign a key in a copy" | ~3 weeks | He finds any binding in under 10 s and every health issue says how to fix it |
| **M2 My controller** | pdf.js rendering of the user's VKB sheets, field mapping for his two sheets, capture flow | ~1.5 weeks | Both sticks shown with correct numbers and actions after one capture |
| **M3 Voice v1** | Chosen mic, level correction, calibration step, personal phrase learning, extra T1 commands, status questions, eval harness on clips saved via the Debug setting | ~2–3 weeks | At least 95% correct execute-or-ask on his VR clip set |
| **M4 Onboarding, Settings, Diagnostics** | Onboarding flow, the settings list, key tester, diagnostics bundle, callouts | ~1.5 weeks | Fresh install to working gear-down by voice in under 5 minutes |
| **M5 Release** | Polish from testing, README | ~1 week | v1.0 on GitHub Releases |

Total about **10–11 weeks**.

---

## 9. Decisions for Sumeet
1. **Main view:** OK with the cloned-and-improved Controls list as the main keybinds view and the VKB sheet as "My controller"?
2. **Capture:** OK to press each control once (about 40 per stick, skippable) so the sheet shows correct numbers?
3. **Sheets:** fine to download VKB's two PDFs yourself (one click from the app opens VKB's page), rather than us shipping them?
4. **Labels:** keep the game's upper-case row labels, or mixed case (easier to read)?
5. **Edits:** is "assign a keyboard key to an empty slot, in a copy" enough editing for v1?
6. **New voice commands:** which of flight assist, HUD mode, silent running, maps/panels, SRV toggles and throttle-to-zero do you want in v1?
7. **Callouts:** default off, or on with low fuel, interdiction and heat?
8. **VR mic:** which microphone do you use in VR, and does SteamVR switch the Windows default?
9. **Second-pass model:** accept up to about 400 ms extra on unsure commands for better accuracy?
10. **Autostart:** start with Windows by default (tray only, idle until Elite runs), or opt-in?
11. **Name:** keep "astromech" as the product name?
