# astromech v1 design

Status: **proposal for review, revision 2** (branch `design/v1`). Date: 10 Oct 2026. Inputs:
the gear-down CLI preview (PR #1, 93687e5), Sumeet's real-game rounds, his review of
revision 1, `ARCHITECTURE.md` v0.13 and `KEYBINDS.md` v0.7. This document decides what v1 *is*;
the architecture doc stays the reference for how the pieces work inside.

**Revision 5 (11 Oct 2026).** Spoken replies stay on Kokoro, coloured as a cockpit PA so they
sit in the same register as Elite's COVAS (Verity / Victor). Default colour **Cockpit**. Do not
clone Frontier samples or the actors. Details in §6.4; DSP chain in ARCHITECTURE.md §5.10.

**Revision 4 (10 Oct, 11:59 PM ET).**
- The VKB sheets are recoloured to the app's dark palette by a build script, so they no longer
  glare. The stick photo is cut out automatically and gets a thin outline (§3.2).
- Ship part diagrams are planned for after v1: a ship drawing with lines from its parts to their
  bindings, as another way to view each activity. Plan and a concept sketch in §3.4.

**Revision 3 (10 Oct, 11:30 PM ET).** Sumeet's notes on revision 2:
- Use VKB's reference sheets directly. They're made for players to use, and this is a
  non-commercial hobby project. We bundle them with credit.
- VKB sticks ship with a fixed factory profile, so the stock button numbers are known. We ship
  them as built-in maps. Asking the user to press buttons is only for sticks someone remapped.
- The full list is good when you know what you want, but not for finding actions you didn't
  know about. On the **My controller** page, an activity
  filter (exploration, trading, …) lights the actions that matter for it on your stick.

**Revision 2 (10 Oct, 10:30 PM ET).** Sumeet rejected the revision-1 mockups as decoration,
not design: random colours, cards, icons, boxes and dividers, four equal-weight panels, a
calibration wizard inside settings and a press log inside the button config. This revision
starts from the tasks (§2), derives a minimal visual system from the game's own Controls screen
(§4), and only then draws (§5). Decisions applied:
- **VR is out of the app entirely.** No overlay, no in-headset UI. Voice still has to work
  while he flies in VR (§6), but nothing on screen needs to. *Update 15 Oct:* not yet, rather than never
  (see §1.4).
- **Releases are unsigned.**
- **Signed in-app updates.** The core checks GitHub Releases (`latest.json` + ed25519). Apply
  quits astromech, overlays the zip, and relaunches; Elite may still be running. First install
  is still the unsigned NSIS setup.exe.
- **No colour or theme customisation**, no following the in-game HUD colour: Elite's stock
  orange only.
- **"Save push-to-talk clips" is a permanent Debug setting** (off by default), not a one-off.

**In one paragraph:** v1 is a Windows tray app that wakes up when Elite starts. It has two
headline features: a **keybinds manager** (activity pictures on your own sticks for finding
what you need, plus the game's Controls list made searchable and honest about problems), and an
**always-on push-to-talk voice assistant** for hands-on-stick ship toggles and read-only status
questions. Everything is local. It installs per-user without admin.

**Peers (11 Oct).** A cluster of copilots shipped in Aug–Oct 2026 chasing the same jobs. The
survey, and a ranked list of properties to copy into this v1 (BindForge-style binds edits,
PTT-as-gate, router-before-model), is [COMPETITORS.md](COMPETITORS.md). This document still
decides what v1 *is*.

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
| **All keybinds** (full reference) | The game's Controls list, cloned and improved: same categories and expandable groups, primary/secondary columns and an "about this setting" panel, plus search, "press a button to jump to it", larger type, and inline health/conflict markers. | new; data from `astromech-keybinds` |
| **Health check** | Runs on app start, device plug/unplug, Bindings folder changes, before launch and on import. Missing devices / PID drift (wrong-firmware warning with reflash steps), buttons above Joy_32, axes the device doesn't have, keyboard-only presets, actions voice needs that have no keyboard key. Shown **inline** in the list (marker on the row, explanation in the about panel) and as an "Issues" filter, not on a separate dashboard. | k0 + `astromech-core::health` |
| **My controller** (opens first) | The maker's reference sheet for one device, with every binding in the sheet's own boxes. An activity filter (exploration, trading, combat, …) lights just that activity's actions and adds a short "not bound yet" list with "Bind this" and the keyboard-only bindings (§3.3); "All" shows everything. For VKB: their sheets, bundled with credit, plus built-in stock button maps (§3.2). A device without a sheet gets the list filtered to that device. | new |
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
- Autostart at login, wakes when Elite runs, signed in-app updates from GitHub Releases (§7.3).

### 1.3 Later (with the reason)
| Item | Why not v1 |
|---|---|
| Ship, SRV and suit part diagrams (§3.4) | Wanted, and planned. Needs original line art and a label layout engine; Activities on the sticks covers discovery for v1. |
| Full binds editor (drag-to-rebind any action, conflict resolution, curves/deadzones) | The viewer + health check + "add keyboard key" covers the pain; a full editor needs much more testing against Elite's file quirks. |
| Tier-2 voice (FSD, docking request, heat sink, plotting) | Needs the confirm flow, more game-behaviour measurement, and has EULA questions for anything automated. |
| Second-screen web dashboard | The window covers it. |
| Route, trading, engineering, logbook modules | Separate products in the module roadmap (§18.7). |
| Per-ship auto-switching of presets | Elite has no API to switch presets; it would mean editing `StartPreset.start` under the game. |
| Printable reference card / PDF export of binds | Nice, cheap later (the controller view already renders it). |

### 1.4 Out of scope for v1 (and probably for good)
- **Any UI in VR** (decided 10 Oct): no SteamVR overlay, no in-headset panels. The app is used
  at the desk; in VR the only interface is voice and spoken replies. *Update 15 Oct:* this is
  "not yet", a prioritisation call rather than a permanent one; the overlay is tracked as the
  "VR overlay" epic, after the daily lookups.
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
| 5 | **Spoken game-event callouts** (read-only, journal): low fuel, fuel scoop full, interdiction, under attack, heat warning, hull damage, scan complete; each with a toggle; master toggle default **on with three events**: low fuel, interdiction, heat | Med–High | S–M | **v1 (small set)** | Reuses the journal pipeline and Kokoro. Never presses keys. *Superseded 15 Oct:* low fuel, interdiction and heat are prominent on the HUD, so they aren't spoken; the reaction epics in the tracker replace this list. |
| 6 | Binds diff between two presets / before-after an update | Med | S | later | Small but not core. |
| 7 | Print/PDF reference card | Med | S | later | |

---

## 2. Information architecture

### 2.1 Tasks and how often
| Task | Frequency | Where |
|---|---|---|
| Install, find the game, detect devices, bind PTT + kill switch, pick and calibrate the mic, first health check | **Once** (and after a new PC or mic) | Onboarding |
| Re-calibrate after changing mic or headset | Rare | Settings → Microphone → Calibration: "Run again" (re-opens that onboarding step only) |
| "What should I bind for exploring / trading / mining?" | Occasional; often when starting a new activity | Activities |
| "What does this button do?" / "Where is X bound?" | **Occasional**, between sessions; very frequent while setting up a new ship or device | All keybinds (search, press a control) |
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
| **All keybinds** | Find and understand any binding; fix what's broken | Device pictures, statistics, voice settings, a separate health dashboard, colour-coded categories |
| **My controller** (default tab) | See one device as its manufacturer drew it, with what each control does; filter to one activity to see what matters for it and what's missing | Every action in the game (that's All keybinds); editing beyond "Bind this"; actions for other devices |
| **Presets** | Which preset is active, switch, back up, restore | Bindings themselves |
| **Materials** | Engineering materials in the three trader grids (raw / manufactured / encoded), with grade, category, storage cap, and High Grade Emissions notes (faction + state) | Blueprints, shopping lists, nearest trader |
| **Engineering: Ships** | A card per owned ship (modules from the newest `Loadout` for that ship in journal history); a ship's detail (`#/engineering/<id>`) sets blueprint / grade / experimental per module and shows the ship's stats now and with the plan done (jump range, shields, distributor, weapons, hull; Coriolis's calculations) with the difference, and need / have / short, trader swaps from surplus, and what is still to collect. Plans are kept per ship; rolls per grade are an editable estimate | Building a ship from scratch (outfitting designer), swapping modules, several builds per ship, Odyssey suit/weapon engineering |
| **Engineering: trip** | Which engineers to visit for the plan and in what order (fewest jumps), what to do at each stop, a note where grades could be rolled remotely (unlocked rank-5 engineer offers the blueprint, if pinned there), and which jobs need an engineer unlock. Every `EngineerCraft`, remote or in person, updates the ship at once, so the plan and trip shrink as you go | Tracking pinned blueprints (the game doesn't log them), unlock walkthroughs, neutron routing, per-base supercruise times |
| **Nearby lookups** (Materials and Engineering side panel) | Nearest systems for a farming source (Famine for wake scanning, the HGE faction pools) with jumps at the current ship's range and how old the report is | Plotting the route in game |
| **Settings** | Every voice and control setting in **one list** | Calibration flows, press logs, live transcripts, anything that isn't a setting |
| **Diagnostics** | Prove what the app sees: key tester (live press log), device list, health report, logs, "Save diagnostics" | Settings (except links to them) |
| **Voice tester** (next to Diagnostics) | Try a voice command safely: type a phrase, or use push-to-talk while the page is open, and see the transcript, the matched intent and its confidence, what astromech would say and the keys it would send. Tests are dry runs; **Run live** sends the command through the policy like a spoken one, once Elite is the active window (it waits 5 s for you to switch, with a countdown; needs Elite running; it skips the confirmation a spoken command would ask). While the page is open and visible, push-to-talk only tests; closing, hiding or minimising it hands push-to-talk back within 3 s | Editing the grammar or aliases, the eval harness |
| **Tray menu** (native, dark) | Is it running, open the window, pause voice, kill switch (while active), quit; one extra line only when something needs attention (crash, Finish setup, Restart to update) | Any status beyond one line; any webview; settings, updates, files (all in the window) |

Navigation is three plain text tabs in the window header, like the game's top-level options,
each with its pages as sub-tabs: **Controls** (My controller, All keybinds, Presets,
Key tester), **Engineering** (Builds, Materials) and **Settings** (General, Diagnostics, Voice
tester). While
setup is unfinished (no push-to-talk, no speech models) the header shows **Finish setup**.

**How the three binding views fit together.** They show the same bindings two ways:
- **My controller** answers "what does each control on this stick do?" ("All"), and "what do I
  need for this?" with an activity chosen. The activity lists are curated and short.
- **All keybinds** answers "where is everything?". It's complete and searchable.
Every action links to the others: "Show in all keybinds", "Show on my controller".

**Pressing a control.** In any of these views, pressing a button, hat or axis on a stick (or a
key) highlights every action bound to it. On a sheet, its box is outlined. In the list, the view
scrolls to the first match and marks all of them. Nothing is sent to the game; astromech only
listens. Onboarding is a separate full-window
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
  - **Voice responses**: spoken confirmations, voice colour (Natural / Cockpit / Harsh; default
    Cockpit, §6.4), speaker, speed, volume, game event callouts.
  - **Debug**: save push-to-talk clips (off), log every input event (off), bounce and
    minimum-clip thresholds (tuning knobs most people never touch).
- **Last commands / transcripts** → Diagnostics (log view), not the tray.
- **Check for updates** → Settings → Updates (and a daily check); downloads a signed zip.
  **Install and restart** (Settings, or the tray's "Restart to update" line) quits, overlays
  files, relaunches, and reopens the window if it was open (Elite may still be running).

---

## 3. The keybinds views

### 3.1 Three views, three questions
| View | Question it answers | Shape |
|---|---|---|
| **All keybinds** | "Where is X? What's on this key?" | The game's Controls list, improved |
| **My controller** (opens first) | "What does every control on this stick do?" With an activity: "What do I need for exploring, trading, …? What am I missing?" | One sheet, every binding; an activity filter lights its actions and adds a "not bound yet" list |

Why not one view? The list is the only view that covers everything (keyboard, every context,
both slots), and it's never wrong, because it's the file. But you can only find what you
already know exists. The game buries "Target next system in route" under Targeting, and you'd
never look there while learning to trade. Activities fixes discovery; the list stays the
complete reference.

Improvements in All keybinds over the game's list: a search box; pressing a control jumps to it;
15 px rows (the game is about 11 px at 1080p) and denser rows; plain text filters (Ship · SRV ·
On foot · General; All · Issues · Unbound); inline markers ("! No keyboard key", "! Joy 33:
ignored"); issue counts on groups; empty slots that look empty; `L ·` / `R ·` to say which stick.

### 3.2 Maker's sheets and stock button maps
**VKB's sheets.** VKB publishes reference sheets for its sticks on its Fillable Templates page
(<https://www.vkbgaming.com/fillable-templates/>). Sumeet's pair:
- Right: **"VKB Gladiator SCG (RH)" v1.0**.
- Left: **"VKB Gladiator SCG OTA (LH)" v1.0** (Omni Throttle).

Each is one page: a photo of the stick with leader lines to labelled tables. Each control has
two boxes: a small one for the button number and a wide one for what it does. We **bundle both
sheets** (`data/devices/vkb/`, credited to VKB Gaming and the sheet designer, UntoldForce) and
write into VKB's own boxes. The box positions come from the PDF itself (its form fields), so
labels land exactly where VKB put the boxes.

**Recoloured to match the app.** `scripts/recolour-sheets.py` runs at build time and writes the
PNGs the app shows (`app/public/devices/vkb/`). For each bundled PDF it:
1. Renders the page at a fixed **150 DPI**, so one PDF point is 150/72 px. The app places labels
   using page points, so the form-field boxes from the device maps stay valid at any size.
2. Cuts out the stick photo with **rembg** (the `isnet-general-use` model, CPU, about 3 s per
   sheet). It keeps the largest blob, which drops stray text and lines.
3. Recolours the rest by layer: white page → window black; grey table headers → the dark option
   cell; empty boxes → the dark unbound cell; black text and lines → light grey; red leader lines
   → muted orange; the blue Discord link → grey.
4. Lifts the photo's shadows so the black stick reads on black, and draws a 2 px grey outline
   round it.
The PDFs stay in the repo unchanged, with VKB's credit. Rerun the script whenever a sheet changes.

**Stock button numbers.** VKB sticks ship with a factory profile, so the numbers are known. We
ship them as built-in maps:
- `data/devices/vkb/gladiator-evo-r.231D0200.json`
- `data/devices/vkb/gladiator-evo-ot-l.231D3201.json`

Both sticks use the same numbering:

| Control | Joy | | Control | Joy |
|---|---|---|---|---|
| Main trigger, stage 1 / 2 | 1 / 2 | | A4 top hat up / right / down / left / push | 11 / 12 / 13 / 14 / 15 |
| A2 red button | 3 | | C1 thumb hat up / right / down / left / push | 16 / 17 / 18 / 19 / 20 |
| B1 side button | 4 | | Rapid-fire trigger push / pull | 21 / 22 |
| D1 pinky button | 5 | | En1 base wheel up / down | 23 / 24 |
| A3 centre hat up / right / down / left / push | 6 / 7 / 8 / 9 / 10 | | Sw1 base switch up / down | 25 / 26 (low confidence) |
| A1 ministick (hat mode) | POV1 up / right / down / left | | F1 / F2 / F3 base buttons | 27 / 28 / 29 |
| A1 ministick push | unknown | | Axes: X, Y, twist, wheel, ministick X / Y | X, Y, RZ, Z, RX, RY |

Where it comes from, and how sure we are:
- **Source:** EDRefCard's device data (MIT, `brammmers/edrefcard2`), which has entries for exactly
  these two IDs, `231D0200` and `231D3201`.
- **Checked against his sticks:** what his devices sent in the s7 runs, and his DualVKB preset,
  agree. Push-to-talk `Joy_10` is the A3 push on the left. `Joy_1` (the trigger) is bound to the
  frame shift drive, and `Joy_16` (C1 up) to throttle 100 %. Both ministicks send POV1. The right
  stick's `Joy_12` / `Joy_14` are A4 right / left.
- **Confidence per control** is in the JSON: **high** (stock map and seen on his sticks, 10 per
  stick), **medium** (stock map only, 24), **low** (6: ministick push, Sw1 up/down, which
  rapid-fire direction the sheet calls "up", and the base wheel axis).
- Not found: a published VKB list of default numbers, or the VKBDevCfg default profile file.
  I didn't find an "EliteCustomButtonNames" list either. EDRefCard is the best source we have.

**If a stick was remapped.** Pressing a control should light up the box the map expects. If a
press lands on a number the map doesn't have, or the user says "that's not where I pressed",
astromech asks about that one control ("Which control did you just press?") and the user
clicks it on the sheet. Only controls that disagree are ever asked about.

**Other devices.** The same approach works for any maker's sheet with boxes. EDRefCard's
pictures are the fallback, then a plain grid of buttons and axes.

### 3.3 Activities
Activities are a filter on My controller, not a separate view. The page shows one stick at a
time (a switcher picks which device); choosing an activity lights its actions in their boxes.
Each lit box shows the action and its other slot (often a key). Below the sheet:
- **Not bound yet:** the activity's important actions with no binding, each with "Bind this".
  Pressing a free control then binds it, in a copy of the preset.
- **Elsewhere:** the activity's actions bound only to the keyboard or mouse.

Actions can be in several activities. The catalogue below is a first draft, using Elite's
binding names. ★ marks the actions that go on the "not bound yet" list if missing.

**Flight basics**
★ `PitchAxisRaw`, ★ `RollAxisRaw`, ★ `YawAxisRaw`, ★ `ThrottleAxis`, `LateralThrustRaw`,
`VerticalThrustRaw`, ★ `UseBoostJuice`, ★ `ToggleFlightAssist`, `SetSpeedZero`, `SetSpeed50`,
`SetSpeed100`, `ToggleReverseThrottleInput`, ★ `LandingGearToggle`, `ShipSpotLightToggle`,
`NightVisionToggle`, `PlayerHUDModeToggle`, `HeadLookToggle`, `UIFocus`.

**Supercruise & jumps**
★ `HyperSuperCombination` (or `Supercruise` + `Hyperspace`), ★ `TargetNextRouteSystem`,
★ `GalaxyMapOpen`, `SystemMapOpen`, ★ `SetSpeed75` (approach), `SetSpeed100`, `SetSpeedZero`,
`SelectTarget`, `CycleNextTarget`, `FocusLeftPanel`, `OrbitLinesToggle`, `ToggleCargoScoop`
(silences the scoop), `DeployHeatSink` (fuel scooping), `LandingGearToggle`.

**Exploration (FSS / DSS)**
★ `ExplorationFSSEnter`, ★ `ExplorationFSSDiscoveryScan`, ★ `ExplorationFSSQuit`,
★ `ExplorationFSSZoomIn`, ★ `ExplorationFSSZoomOut`, `ExplorationFSSMiniZoomIn`,
`ExplorationFSSMiniZoomOut`, ★ `ExplorationFSSRadioTuningX_Raw` (or `_Increase` / `_Decrease`),
`ExplorationFSSTarget`, `ExplorationFSSCameraPitch`, `ExplorationFSSCameraYaw`,
★ `ExplorationSAAExitThirdPerson`, `ExplorationSAANextGenus`, `ExplorationSAAPreviousGenus`,
`ExplorationSAAChangeScannedAreaViewToggle`, ★ `PlayerHUDModeToggle`, ★ `TargetNextRouteSystem`,
★ `HyperSuperCombination`, `GalaxyMapOpen`, `SystemMapOpen`, `OpenCodexGoToDiscovery`.

**Trading**
★ `GalaxyMapOpen`, ★ `TargetNextRouteSystem`, ★ `HyperSuperCombination`, ★ `FocusLeftPanel`
(contacts and docking request), `FocusRightPanel`, `CycleNextPanel`, `CyclePreviousPanel`,
★ `LandingGearToggle`, `SetSpeedZero`, `SetSpeed75`, `UI_Select`, `UI_Back`, `UI_Up`,
`UI_Down`, `ToggleButtonUpInput` (silent running), `DeployHeatSink`, `ToggleCargoScoop`.

**Mining**
★ `DeployHardpointToggle`, ★ `PrimaryFire`, ★ `SecondaryFire`, ★ `CycleFireGroupNext`,
`CycleFireGroupPrevious`, ★ `ToggleCargoScoop`, ★ `PlayerHUDModeToggle`, `SelectTarget`,
`SetSpeedZero`, `UpThrustButton`, `DownThrustButton`, `ForwardThrustButton`, `EjectAllCargo`,
`IncreaseSystemsPower`, `IncreaseWeaponsPower`, `ResetPowerDistribution`.

**Combat**
★ `PrimaryFire`, ★ `SecondaryFire`, ★ `DeployHardpointToggle`, ★ `SelectTarget`,
★ `SelectHighestThreat`, `CycleNextHostileTarget`, `CycleNextTarget`, `CycleNextSubsystem`,
★ `IncreaseEnginesPower`, ★ `IncreaseSystemsPower`, ★ `IncreaseWeaponsPower`,
★ `ResetPowerDistribution`, `CycleFireGroupNext`, ★ `UseBoostJuice`, `ToggleFlightAssist`,
`DeployHeatSink`, `FireChaffLauncher`, `UseShieldCell`, `ChargeECM`, `OrderFocusTarget`.

**Docking & landing**
★ `LandingGearToggle`, ★ `FocusLeftPanel` (request docking), `UI_Select`, `UI_Back`,
★ `SetSpeedZero`, ★ `UpThrustButton`, ★ `DownThrustButton`, `LeftThrustButton`,
`RightThrustButton`, `ForwardThrustButton`, `BackwardThrustButton`, `VerticalThrust_Landing`,
`LateralThrust_Landing`, `PitchAxis_Landing`, `YawAxis_Landing`, `RollAxis_Landing`,
`ShipSpotLightToggle`, `NightVisionToggle`.

**SRV**
★ `SteeringAxis`, ★ `DriveSpeedAxis` (or `IncreaseSpeedButtonPartial` / `DecreaseSpeedButtonPartial`), ★ `AutoBreakBuggyButton`,
★ `VerticalThrustersButton`, `ToggleDriveAssist`, `HeadlightsBuggyButton`,
★ `ToggleBuggyTurretButton`, `BuggyPrimaryFireButton`, `BuggySecondaryFireButton`,
`ToggleCargoScoop_Buggy`, `RecallDismissShip`, `PlayerHUDModeToggle_Buggy`,
`GalaxyMapOpen_Buggy`, `SystemMapOpen_Buggy`, `FocusLeftPanel_Buggy`, `NightVisionToggle`.

**On foot**
★ `HumanoidForwardButton` / walk axes, ★ `HumanoidPrimaryInteractButton`,
`HumanoidSecondaryInteractButton`, ★ `HumanoidSprintButton`, `HumanoidJumpButton`,
`HumanoidCrouchButton`, ★ `HumanoidPrimaryFireButton`, `HumanoidZoomButton`,
`HumanoidReloadButton`, `HumanoidMeleeButton`, `HumanoidThrowGrenadeButton`,
★ `HumanoidItemWheelButton`, `HumanoidToggleFlashlightButton`,
`HumanoidToggleNightVisionButton`, `HumanoidSwitchToSuitTool`, `HumanoidToggleToolModeButton`,
`GalaxyMapOpen_Humanoid`, `SystemMapOpen_Humanoid`.

All names were checked against EDRefCard's control list (420 controls).

The mockup shows Exploration on his VKB pair. Its bindings are **example data in the shape
of his DualVKB preset**: we only have a few real lines from it (trigger → frame shift drive,
C1 up → throttle 100 %, A3 push → push-to-talk). Whether "Target next system in route" is
really unbound in his preset is unknown until we read the whole file.

---

### 3.4 Later: ship, SRV and suit diagrams
**What it is.** A drawing of the ship with thin lines from its parts to boxes showing their
bindings: lights, landing gear, hardpoints, cargo scoop, FSD, thrusters, shields, sensors,
cockpit panels. The same for the SRV and the suit. It's how people think ("how do I lower the
gear?"), it's the most game-like view, and pictures help people find controls faster than lists.

**How it fits.** Each activity gets a second view: **Sticks · Ship** (or SRV / Suit for those
activities). Same actions, same "not bound yet" list, same "Bind this". Sticks shows *where your
hands go*; Ship shows *what it does*. Not every action has a part (galaxy map, panels): those sit
in a plain "Cockpit" box near the canopy.

**Art.**
- Our own line art, not Frontier's models or screenshots. One **generic hull** first, then the
  SRV (Scarab) and a suit, all generic. Per-hull drawings (Cobra, Krait, …) come later, only if
  people want them.
- SVG, three-quarter view from above, so the belly, top and sides all show.
- Each drawing has an **anchor file**: part id → point on the drawing, plus which side its label
  should go. Example: `landing_gear: {at: [352, 352], side: "left"}`. Each action in the
  catalogue lists the part it belongs to (`LandingGearToggle → landing_gear`).

**Label layout.**
1. Put each label on its preferred side (left or right of the hull, or above/below).
2. Sort each side by the anchor's height and push labels apart to a minimum gap (one pass, like
   the game's own lists). Keep them as close to their anchor's height as possible.
3. Draw each leader as two straight segments: from the part dot to a bend 24 px from the box, then
   level into the box. No curves.
4. If two leaders cross, swap their labels and redo step 2. Repeat a few times. With 10–20 labels
   this is instant.
5. Labels never cover the hull. If a side is full, overflow goes above or below.

**Interaction.**
- Pressing a control highlights its part: the dot fills, the leader turns orange, the box gets an
  outline.
- Clicking a part (or its box) selects that action; "Bind this" then waits for a press.
- Unbound important actions: hollow dot, dashed orange leader, box says "Not bound · Bind this".

**Style.** Elite's stock orange only. Hull lines in orange at full strength for the outline and dim
orange for panel lines; a very faint orange fill, like the game's hologram ship panels. Leaders in
grey; part dots in orange. Boxes are the same filled cells as the list. No glow, no animation
beyond the highlight.

**Effort (after v1).**
| Work | Effort |
|---|---|
| Diagram component: SVG, anchor file, layout and crossing fix, highlight and bind | ~1 week |
| Generic ship line art + anchors for all ship activities | ~1 week |
| SRV and suit line art + anchors | ~1 week |
| Part ids in the catalogue, testing at different window sizes | ~3 days |
| **Total** | **~3.5 weeks** |
| Each extra per-hull drawing | ~2–3 days |

**Concept sketch:** `docs/design/mockup-ship.png` (`ShipDiagram.svelte`). Docking & landing on a
generic ship drawn in side view, marked as a concept, with example bindings. It shows lights,
landing gear (front and rear), top, belly, side and main thrusters, all stop, night vision and
the left panel (to request docking). The real version would use the three-quarter view.

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
| All keybinds (full list) | `app/src/mockups/KeybindsList.svelte` | `docs/design/mockup-keybinds.png` |
| Concept: Docking & landing ship diagram (after v1) | `ShipDiagram.svelte` | `docs/design/mockup-ship.png` |
| Activities: Exploration, on his VKB pair | `ActivityView.svelte`, `Sheet.svelte` | `docs/design/mockup-activities.png` |
| My controller (VKB sheet) | `MyController.svelte`, `Sheet.svelte` | `docs/design/mockup-controller.png` |
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
- Rev 4, sheets recoloured: the white flashbang is gone. Headers, boxes and text now use the
  list's own cells and greys, so the sheet reads as part of the window. The orange "in this
  activity" boxes stand out more than before, because nothing else on the sheet is bright.
- Rev 4a mapped empty boxes to the bound-cell brown. Every box was brown, the sheet looked
  busy, and bound and empty looked alike. **Fixed:** empty boxes use the darker unbound cell,
  like the game's empty slots.
- Rev 4a turned VKB's bold title letters into hollow outlines. **Fixed:** thick strokes now
  find their background from a wider area.
- Still busier than the game: VKB's photo and leader lines. Red leader lines *on top of* the
  photo stay red, because the script can't tell them from the red A2 button. Small; left as is.
- Ship concept: the drawing is flat and side-on, so it leaves empty space above and below, and
  some leaders cross. The planned layout pass (§3.4) and a three-quarter view fix both.
- Binding text on the sheets is light grey on the dark boxes; grey only for "not in this
  activity".
- Rev 3a printed hat directions ("Up") in the number box for the ministick. **Fixed:** `P1`
  (POV 1); axes have no number box.
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

### 6.4 Voice colour (COVAS register)

Kokoro is surprisingly realistic. That is a problem next to Elite's built-in COVAS: Verity and
Victor are voice actors (Verity Croft, Simon Paisley Day) treated as a ship computer — British,
clipped, a bit hollow — and the product name already leans droid. A warm neural voice in the
same headset is a person talking over the PA.

**Decision:** keep Kokoro (local, already in the worker, 446 ms first audio in S9). Colour the
PCM so replies sit in the same family as COVAS. Recreate the *register*; never clone Frontier
audio or the actors.

**What that register is.** Impersonal present-tense callouts, helmet/PA bandwidth, little
warmth, a short pip then the line. A ship system sitting beside COVAS, not a crewmate and not
a protocol droid.

**Colour presets** (Settings → Voice responses; also `[voice] colour` in the config). Applied
to live TTS and to the pre-rendered confirmation cache, so they match.

| Preset | What it does | When |
|---|---|---|
| **Natural** | Unprocessed Kokoro, `speed = 1.0` | Debug, or anyone who wants the raw model |
| **Cockpit** (default) | The chain below, `speed ≈ 0.92–0.95` | v1 default |
| **Harsh** | Narrower band (~3.5 kHz), more grit | If Cockpit still feels too human |

**Cockpit chain** (in order; numbers are a starting point, tuned by A/B in the headset with
COVAS in the other ear):

1. High-pass ~250 Hz, low-pass ~4 kHz.
2. Gentle presence bump ~1.5–2 kHz.
3. Compressor (even level, less human envelope).
4. Soft saturation. Optional 16 kHz downsample then upsample.
5. 1–2 ms comb / tiny metallic delay (hollow helmet).
6. A short two-tone pip before speech (own CC0/self-made earcon, never a game sample).

A small formant shift is a later knob if Cockpit is still too human. Vocoder, ring-mod, and
R2-style chirps stay out: they push the name into Star Wars.

**Speaker.** Kokoro v1.0 (`kokoro-multi-lang-v1_0`) has 54 voices, 8 of them British. The
voice is a setting by name (`[voice] speaker`, default `bf_emma`), picked in Settings. British
voices are phonemised as British English. Old configs with a v0.19 `speaker_id` move to the
same voice by name, or to the default.

**Copy.** Official, short, no contractions, no jokes, little "I" in callouts. Prefer
"Landing gear deployed." / "already deployed." Refusals stay one clause:
"Landing gear not available: supercruise." The gear-down slice's strings get a pass in M3.

**Rejected for v1 default**

| Option | Why it waits |
|---|---|
| eSpeak-ng as the voice | Already in the worker for G2P. Classic formant GPS, more 1998 computer than Verity. Optional later as "Classic computer". |
| Piper, SAM, DECtalk, SAPI | Extra models or novelty synths. SAM is the droid trap. |
| A second neural TTS | Timbre is a treatment problem. |
| Voice-clone of Verity / Victor | Frontier and actor IP. |

**Where it ships.** DSP, mixer barge-in and the copy pass are in the first M3 increment
(`astromech_voice::colour` / `mixer`; `[voice] colour`). The Settings row is **M4**.

---

## 7. Infrastructure

### 7.1 Process and tray model
**Two processes, and the core owns a native tray menu.**
- `astromech-core.exe` (Rust, no WebView) is the only process that's always running: tray icon
  and **native context menu** (the `tray-icon` crate's OS menu, dark via uxtheme: status line,
  Open astromech, Pause voice, Kill switch while active, Quit, plus a crash / Finish setup /
  Restart to update line when relevant),
  game watcher, journal pipeline, input listeners, voice, the local API. Idle memory target
  about 20–40 MB; models load only while Elite runs (§7.6).
- `astromech-shell.exe` (Tauri v2 + Svelte) is the window, started from the tray or on first
  run, talking to the core over the local API. Closing it exits it and frees WebView2
  (typically 100–200 MB across its processes).
- Revision 1's webview tray popover is dropped: a native menu is instant, needs no WebView2,
  and works even if the shell is broken.

### 7.2 Packaging
- **Tauri bundler, NSIS installer, `installMode: "currentUser"`** → installs to
  `%LOCALAPPDATA%\astromech` (Tauri's per-user default; M0 note: the template has no install-dir option, so user data moved to its `data\` subfolder), no admin, no UAC prompt. One installer carries
  `astromech-shell.exe`, `astromech-core.exe`, `astromech-voice-worker.exe` (as Tauri
  `externalBin`/resources) and the OFL fonts. Speech models download on first run (≈ 700 MB,
  SHA-256 checked, as now) so the installer stays small.
- WebView2: use the bootstrapper (`webviewInstallMode: downloadBootstrapper`); Windows 11 already has it.
- Uninstall removes the program; user data (`%LOCALAPPDATA%\astromech\data`) stays unless the
  "remove my data" box is ticked.

### 7.3 Updates
- The **core** owns check, download and apply. The Tauri updater plugin does not fit: the shell
  is not the long-lived process.
- **Channel:** GitHub Releases `v*` tags. Each release ships `latest.json` (version, zip name,
  SHA-256, notes) and `latest.json.sig` (ed25519 over the exact JSON bytes). The verifying key
  is baked into the core (`PUBLIC_KEY_HEX`); CI holds the private seed as `ASTROMECH_UPDATE_KEY`.
- **Payload:** a zip of `astromech-shell.exe`, `astromech-core.exe`, `astromech-voice-worker.exe`,
  `starmap.db` and the licence files. Not NSIS: overlaying files avoids SmartScreen on every `setup.exe`.
  `data\` (config, logs, models, DB) is never overwritten. `uninstall.exe` stays.
- **When:** check and download any time, including while Elite is running. Apply drops keys and
  the session, quits, copies from a helper in `%TEMP%` (the running `astromech-core.exe` is
  locked), then relaunches. The new process attaches to the running game the same way a mid-session
  start already does.
- **Policy** (`[update]` in `astromech.toml`): `check = true` (default) polls about daily;
  `apply = "ask"` (default) waits for **Install and restart** (Settings or tray); `on_quit` applies when
  astromech itself quits (no relaunch); `never` does not apply.
- First install, or a machine that never ran a self-updating build, still uses the NSIS
  `setup.exe`. NSIS still asks the core to exit cleanly before replacing files.

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
  including the key tester). No mic, no PTT listener, no models in memory. Update checks also
  run while active.
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
- **Logs:** rolling files in `%LOCALAPPDATA%\astromech\data\logs` (7 days, 10 MB each), the same lines
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
| **M1 Keybinds list** | All keybinds (groups, search, pressing a control, inline health), about panel, Presets, backups, "assign a key in a copy" | ~3 weeks | He finds any binding in under 10 s and every health issue says how to fix it |
| **M2 Sheets and activities** | Bundled VKB sheets (recoloured) and stock maps, My controller, Activities with the catalogue, "Bind this", the remapped-control question | ~2 weeks | Both sticks show the right actions in the right boxes with no setup; he finds "Target next system in route" from Exploration |
| **M3 Voice v1** | Chosen mic, level correction, calibration step, personal phrase learning, extra T1 commands, status questions, eval harness on clips saved via the Debug setting, **voice colour** (Cockpit PA DSP + copy pass, §6.4) | ~2–3 weeks | At least 95% correct execute-or-ask on his VR clip set; Cockpit replies sit in the same register as in-game COVAS |
| **M4 Onboarding, Settings, Diagnostics** | Onboarding flow, the settings list, key tester, diagnostics bundle, callouts | ~1.5 weeks | Fresh install to working gear-down by voice in under 5 minutes |
| **M5 Release** | Polish from testing, README | ~1 week | v1.0 on GitHub Releases |

Total about **11 weeks**.

---

### 7.10 As built in M0 (11 Oct 2026)
- `astromech-core.exe` with no options is the tray app; `--background` (Run key) skips the window;
  `--console` is the old console preview (the e2e uses it); `--quit` asks a running core to exit
  (the installer uses it). One-shot options (`--listen`, `--health`, …) still work.
- The window ↔ core link in M0 is a small local control API (JSON lines on 127.0.0.1, port and a
  per-run token in `data\core.json`): status, open window, save diagnostics, quit. The full
  WebSocket API replaces it as modules need it.
- Active = launcher or game running (the launcher pre-warms); idle after the game has been gone
  for two polls (6 s). The Raw Input listener starts at the first activation and stays.
- Start with Windows, Open config file and Download speech models moved from the tray to
  Settings (14 Oct); the tray links there with "Finish setup" while something is missing.
- Crash restart: the window's watchdog restarts the core if `core.json` exists but the core
  doesn't answer (a clean Quit removes the file, so it stays off).

### 7.11 As built in M1 (11 Oct 2026)
- **Where the keybinds code runs.** The app window reads and edits presets itself, through
  `astromech-keybinds` (`api`, `profile`, `edit`, `catalog`), and listens to sticks and keys with
  the same read-only listener as the core (`astromech-input`, Raw Input with INPUTSINK + XInput).
  The core keeps its own health watch on the Bindings folder. Nothing goes over the control API.
- **The only writer is `edit.rs`.** Before any write it copies the file to
  `Bindings\astromech-backups\<file>.<time>.bak` (a subfolder, so the game ignores it), writes a
  temp file and renames it over. A slot edit replaces only that slot element's text; every other
  byte stays as it was (tested). Built-in presets are never written: the first edit makes a named
  copy and makes it active wherever the original was. UTF-16 files and files outside the Bindings
  folder are refused. Presets → Restore puts a backup back (backing up the current file first).
- **Catalogue.** `data/catalog/controls.json` (420 actions; EDRefCard's list, MIT, labels in
  sentence case with clearer names for the common ones) and `data/catalog/activities.json`
  (generated from §3.3). The list groups are ours (Flight, Ship systems, Weapons and combat,
  Scanners, SRV driving, …), not the game's, because EDRefCard doesn't record the game's groups.
- **Health in the list.** No keyboard key (for the actions voice presses), `Joy_33`+ ignored,
  device not connected, and "same control as" (info only, and only inside one mode: FSS, galaxy
  map and menus reuse flight buttons on purpose). Per preset: duplicate names, XML errors, missing
  devices, and the VKB PID-mismatch warning (reflash or set the PID back in VKBDevCfg; astromech
  doesn't rewrite device IDs).
- **Remapped sticks.** A press the factory map doesn't know asks "Which control did you just
  press?"; the answer goes in `data\device-overrides.json` and replaces that one control's number.
- **Key tester** is on the Diagnostics page (with the connected controllers).
- **Modifiers.** Press-to-bind writes a single control. Existing modifier combos are shown and kept;
  making a new combo is left to the game's own menu for now.
- **Browser preview.** Outside Tauri the window serves `app/src/lib/fixtures/sample.json` (made by
  `cargo run -p astromech-keybinds --example fixture` from `data/samples/Example DualVKB.4.2.binds`,
  generated example data) and keeps edits in memory. `scripts/app-shots.sh` screenshots every screen.

### 7.12 As built in M3 output-path slice (11 Oct 2026)
- `[voice] colour = "natural" | "cockpit" | "harsh"` (default **cockpit**). DSP is `astromech_voice::colour`; the confirmation cache is coloured. Cockpit prepends a two-tone pip.
- Playback is `astromech_voice::mixer`: a new line replaces the queue; PTT down and the kill switch `cut` it (Kokoro cannot leak into the next PTT clip). cpal on Windows; async PlaySoundW if there is no device.
- Spoken gear lines: "Landing gear deployed." / "already deployed."

### 7.13 As built in M3 voice v1 (11 Oct 2026)
- **Chosen mic:** `[voice] mic` is a name substring; empty uses the Windows default. The device is logged on every PTT clip. `--list-mics` prints devices.
- **Level correction:** after the gate, `astromech_voice::gain` boosts speech frames toward −6 dBFS (max +30 dB) and leaves frames under the noise floor + 6 dB alone.
- **Calibration:** `--calibrate-files silent.wav speech1.wav …` stores per-device `min_level_dbfs` in SQLite. Settings UI: §7.14.
- **Personal aliases:** a did-you-mean "yes" stores transcript → command (cap 200). Policy still gates the command.
- **Extra T1:** flight assist (flag bit 5 = FA *off*), HUD analysis/combat, panels and maps (`GuiFocus`), throttle to zero (unverified; refused near a body in supercruise), SRV handbrake / drive assist. Silent running on asks "Silent running produces heat. Confirm?" then presses.
- **Status questions (T0):** fuel, cargo, jump range, location, jumps left, hull, shields, credits/rebuy, ship, legal state — from Status.json, journal `Loadout`/`LoadGame`, Cargo.json, NavRoute.json.
- **Eval:** `[voice] save_clips = true` writes WAV + JSON under `data/clips`. `astromech-core --eval DIR` scores execute-or-ask against `label` / `command` / `labels.json`.

### 7.14 As built in M4 onboarding and calibration (16 Oct 2026)
- **Calibration is one path.** `astromech_core::calibration` measures and stores the gate for the CLI
  (`--calibrate-files`) and the window (Settings → Microphone → Calibration, onboarding step 5). The
  key is the device name the session's recorder opens for `[voice] mic` (before, the CLI keyed it by
  the setting as typed, so a substring like "BlackShark" never matched the session's lookup).
- **Onboarding** is a full-window flow shown until finished or skipped (`onboarding.done` in SQLite):
  find Elite (Bindings and journal folders, install optional; a folder can be pasted), controllers
  (the window's device scan and a live last press), press-to-bind push-to-talk and kill switch, mic
  by name with a level check, calibration, and a health check whose rows each have a fix (go to the
  step, download models, or the press-to-bind dialog for a voice action with no keyboard key).
  `#/setup` re-runs it and `#/setup/<step>` opens one step; Settings links there.
- **Setup edits go through the core.** The control API takes `args`; `set_ptt`, `set_kill_switch`,
  `set_mic` and `set_folders` edit `astromech.toml` with `toml_edit` (comments kept, never written
  if the result doesn't parse) and swap the core's config. A running voice session picks them up
  the next time Elite starts. Recording (`mic_check`, `calibrate_*`) happens in the core, on Windows.
- **Voice responses** (Settings): speaker (English Kokoro v1.0 voices, British first), colour and
  speed, written with `set_voice` through the same locked config edit. The running session's
  speaker switches at once (`Speaker::set_voice`), so the next reply uses the new voice; the TTS
  cache key already includes model, voice, speed and colour. Preview says "Landing gear deployed."
  through the session's speaker, or a Kokoro the core starts for the preview while Elite isn't
  running (released when a session starts or after a minute unused; refused while a session
  loads, and when speech output is off). After a voice change the fixed phrases re-render in the
  background, but replies always render first and warm-ups for voices no longer chosen are skipped.
- **Browser preview.** `?core=empty|new|ready` answers the core's commands from a recorded state
  (`app/src/lib/fakecore.ts`); `empty` also hides the example preset and controllers.

## 9. Decided (11 Oct 2026)
Settled with Sumeet before implementation started:
1. **Default screen:** the window opens on **My controller** (the Activities page was merged into it as a filter on 16 Oct 2026).
2. **Row labels:** **mixed case**, not the game's all-caps labels.
3. **v1 voice adds safe toggles:** flight assist, silent running (asks for a **spoken
   confirmation** first), HUD mode, panels, and throttle to zero.
4. **Game callouts:** **only a low-fuel warning**, on by default at **25%**, threshold
   configurable in Settings. No interdiction or heat callouts: the game already alerts those
   clearly. *Update 15 Oct:* the low-fuel warning is prominent on the HUD too, so
   it isn't spoken either; the fuel callout says the next scoopable star on the route, and
   warns only when there isn't enough fuel to reach one (tracker: "Fuel: next scoopable star").
5. **Autostart:** **start with Windows on by default** (tray only, idle until Elite runs), with
   an opt-out in setup and in Settings.
6. **Six uncertain VKB buttons** (ministick push, both base-switch directions, rapid-fire
   direction, base wheel axis): resolved later with the key tester (M4), one press each.
7. **Signed in-app updates** from GitHub Releases (§7.3), **unsigned binaries** (Authenticode;
   payload is ed25519), and **Elite's stock colours** (no custom theming).
8. **Voice colour:** Kokoro stays; replies are coloured as a **cockpit PA** (default **Cockpit**,
   §6.4). Natural and Harsh are the other presets. No Frontier-sample clone, no vocoder default.

Still open, not blocking: activity list and catalogue review (§3.3), VR mic choice, second-pass
model latency budget, product name, which Kokoro voice is the flattest announcer.
