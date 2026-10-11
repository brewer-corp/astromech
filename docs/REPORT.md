# Elite Dangerous AI Copilot: ecosystem research and feasibility report

*Prepared 8 Oct 2026 (America/New_York) for Sumeet. Raw notes (`notes/`) and cloned third-party source (`repos/`) stayed in the research workspace and are not in this repository.*

**Later survey.** Tool landscape as of 11 Oct 2026 (EliteIntel 1.1, COVAS 4.3, Directive 47, AURA, CanopyNav, EDMO, and what to copy) is in [COMPETITORS.md](COMPETITORS.md). This report stays the reference for data sources, `SendInput`, plotting, and EDCoPilot internals.

**How to read this report.** Every claim has a source next to it: a URL, or a repo file path for code I read.
- **(verified)** means I confirmed it today, either from a live request or the current official document.
- **(inferred)** means it is my reasoning from public evidence.
- **(unverified)** means it is reported but I did not confirm it.

---

## 1. Executive summary

- **Everything a copilot needs is reachable without touching the game process.** There are four sources, plus community APIs:
  1. The **Player Journal**, an append-only JSON-lines event log.
  2. About ten **companion JSON files**. The most important is **Status.json**, which holds near-realtime ship state: gear, lights, hardpoints, FSD, GUI focus, fuel and position.
  3. The **Frontier cAPI** (OAuth2 PKCE). It gives authoritative profile, market, shipyard and fleet-carrier data, but it is polled and rate-limited.
  4. The local **.binds XML files**.
  5. Community services: **Spansh**, **EDSM**, **Inara**, **EDDN** and **Ardent**, which fill in galaxy, market and route data.
  
  The official Journal Manual is now **v38 (May 2026)**, but it lags the game: colonisation events, for example, are undocumented (verified; see §3.1).
- **There is no input API.** Every existing tool "drives the ship" the same way: it reads the user's `.binds` file and synthesises **keyboard scancodes with Win32 `SendInput`** into the focused game window. This is true of EDCoPilot, COVAS:NEXT, EliteIntel, VoiceAttack and bindED. It requires:
  - a keyboard binding for every action,
  - the game window in focus,
  - matching privilege levels (UIPI),
  - hand-tuned timing.
  
  The way to make this reliable is closed-loop control: after each injected input, check the result in Status.json or the journal.
- **Course plotting is the hardest "simple" feature.** It is a blind macro that drives the galaxy-map UI:
  1. open the map,
  2. focus the search box,
  3. type the system name,
  4. select the result,
  5. click plot.
  
  It breaks on map-open latency, conflicting UI and camera binds, HOTAS axis noise, focus loss and stations versus systems. The robust pattern, used by COVAS:NEXT, is:
  - resolve the name through Spansh first,
  - gate each step on `GuiFocus`,
  - **confirm success by waiting for `NavRoute.json` to end at the target**.
  
  EliteIntel and EDCoPilot don't verify this way.
- **EDCoPilot is the most feature-complete "ship AI".** Its stack is legacy, though:
  - a Python/PyInstaller core with a Kivy GUI, talking over local TCP;
  - VoiceAttack for speech recognition (EDCoPilot has no STT of its own; VoiceAttack writes command strings to a text file that EDCoPilot polls);
  - Edge, Windows, cloud or ElevenLabs TTS;
  - keypress automation;
  - Spansh, EDSM and Inara lookups;
  - a private backend at `razzserver.com`.
  
  It has **no built-in LLM**: conversational AI comes from COVAS:NEXT over a ZeroMQ bridge (EDMesg). VR support means capturing its desktop window into third-party overlay apps.
- **I did NOT obtain or inspect EDCoPilot binaries.** Its EULA forbids reverse engineering, decompiling and disassembling, and downloading the software counts as accepting the EULA (`notes/eula.txt`). Everything about its internals in this report is inferred from its public wiki (112 pages archived), release notes, feature list and the open-source EDMesg bridge.
- **The modern AI pieces all exist today (verified).**
  - **OpenAI `gpt-live-1`** is real. It is a full-duplex voice model that delegates tool use to your backend, at $0.05/min.
  - Alternatives: OpenAI `gpt-realtime-2`, Google `gemini-3.8-live`, and xAI's Grok Voice Agent API.
  - For local or offline use: **NVIDIA Parakeet** speech-to-text (note: Parakeet is STT, not TTS), **Kokoro / Supertonic / Piper** TTS, all runnable through **sherpa-onnx**, which has both a Node addon and a Rust crate.
- **VR:** Elite runs on OpenVR/SteamVR or the legacy Oculus SDK, with **no native OpenXR**. A standalone **OpenVR overlay process** works without injecting into the game, as EliteIntel already does. Users on the Oculus runtime need OpenComposite plus OpenKneeboard, or an audio-only experience.
- **Recommendation:** a **Rust core**, with a **TypeScript UI in Tauri**.
  - **Rust core** handles: game I/O (journal and Status watchers, the binds parser/editor, the `SendInput` executor with verification), the audio pipeline (sherpa-onnx), and the OpenVR overlay.
  - **TypeScript (Tauri)** handles: the desktop UI and companion web panels.
  - **Brain:** cloud full-duplex voice (GPT-Live client delegation or Gemini Live) calling a **typed, deterministic tool layer**, plus a **local fast path** that sends "gear down"-style commands straight to keypresses without waiting on the LLM.
  - A TypeScript-only design (Electron or Node) is viable but weaker for low-level input and VR. Rationale is in §12.

---

## 2. Method and limits

- **Code I read in cloned repos** (commit and date in `notes/03_tools_code_findings.md`):
  - EliteIntel: github.com/SudoKrondor/EliteIntel; the old URL github.com/stone-alex/EliteIntel redirects there.
  - COVAS:NEXT: github.com/RatherRude/Elite-Dangerous-AI-Integration.
  - EDMesg, EliteAPI, bindED (Guntereno fork), OD-Explorer.
  - EDMarketConnector, EDDI, EDDN, and Athanasius/fd-api.
- **Live checks today:**
  - EDSM: system lookup and server-status endpoint.
  - Spansh: neutron route job, autocomplete, and dump file sizes.
  - EDAstro: the GEC JSON file.
- **Pages behind bot challenges:** the Inara and EDSM documentation pages block automated fetches, so I read their archived copies on web.archive.org from Mar–Sep 2026.
- **Not done:**
  - No EDCoPilot binary analysis (EULA; see §8.1).
  - No HCS VoicePacks or VoiceAttack internals (closed source).
  - Canonn API endpoints not verified.
  - ODUtils, OD-Explorer's core library, is private, so I couldn't read it.

---

## 3. Official data sources

### 3.1 Player Journal

- **Authoritative doc (verified):** Journal Manual v38, "Odyssey up to update26 (May 2026)": https://hosting.zaonce.net/community/journal/v38/Journal_Manual_v38.pdf. The v37 text is mirrored at https://elite-journal.readthedocs.io/.
- **Location and format:** `%USERPROFILE%\Saved Games\Frontier Developments\Elite Dangerous\Journal.<datestamp>.<part>.log`. It is line-delimited JSON with a header (`Fileheader`: part, `odyssey`, language, gameversion, build). Every record has `timestamp` (UTC) and `event`. Localised strings come as `X` / `X_Localised` pairs (manual §2).
- **Coverage:** about 160 documented events across startup, travel, combat, exploration, trade, station services, powerplay, squadrons, fleet carriers, Odyssey on-foot, and more.
- **The manual lags the game (verified):**
  - The v38 changelog lists only a Statistics change.
  - Colonisation events such as `ColonisationConstructionDepot` don't appear in the manual at all. They are handled in EDMC and COVAS:NEXT code.
  - Plan to use **community schemas** to fill the gaps: EDDN schemas (`repos/EDDN/schemas/`), jixxed's journal definitions, kayahr/ed-journal (TypeScript) and rster2002/ed-journals (Rust).
- **Copilot-relevant events:**
  - Travel: `FSDTarget`, `StartJump`, `FSDJump`, `SupercruiseEntry`/`Exit`, `SupercruiseDestinationDrop`.
  - Docking: `DockingRequested`/`Granted`/`Denied`, `Docked`, `Undocked`, `Touchdown`/`Liftoff`.
  - Route: `NavRoute`, `NavRouteClear`.
  - Exploration: `FSSDiscoveryScan`, `Scan`, `SAASignalsFound`, `ScanOrganic`.
  - Neutron boost: `JetConeBoost` (detects a supercharge, as auto-neutron routing uses).
  - Other: `ReceiveText` (comms), `Music` (a UI-state hint: EliteIntel uses it to detect the galaxy map), `Loadout`, `Cargo`, `Materials`, `Missions`, `Friends`.

### 3.2 Companion files (same folder)

| File | Written when | Use |
|---|---|---|
| **Status.json** | rewritten whenever state changes (§14) | realtime state; **verify every toggle** |
| NavRoute.json | a multi-jump route is plotted; cleared on `NavRouteClear` (v35+) | **plot verification**, next-jump info |
| Cargo.json | cargo changes | inventory |
| Market.json / Outfitting.json / Shipyard.json | the relevant station screen is opened | prices and stock at the current station |
| ModulesInfo.json | modules panel / `ModuleInfo` event | power/priority info |
| ShipLocker.json / Backpack.json | Odyssey inventory changes | on-foot materials |
| FCMaterials.json | carrier bartender opened | carrier materials |

**Status.json (manual §14, verified).**

Fields:
- `Flags`, `Flags2`, `Pips` (half-pips), `Firegroup`, `GuiFocus`
- `Fuel{FuelMain,FuelReservoir}`, `Cargo`, `LegalState`
- `Latitude`/`Longitude`/`Altitude`/`Heading`, `BodyName`, `PlanetRadius`
- `Balance`, `Destination{System,Body,Name}`
- On foot only: `Oxygen`, `Health`, `Temperature`, `SelectedWeapon`, `Gravity`

`Flags` bits:

| Bit | Meaning | Bit | Meaning | Bit | Meaning |
|---|---|---|---|---|---|
| 0 | Docked | 11 | Scooping fuel | 22 | In danger |
| 1 | Landed | 12 | SRV handbrake | 23 | Being interdicted |
| 2 | **Landing gear down** | 13 | SRV turret view | 24 | In main ship |
| 3 | Shields up | 14 | SRV turret retracted | 25 | In fighter |
| 4 | Supercruise | 15 | SRV drive assist | 26 | In SRV |
| 5 | FlightAssist off | 16 | FSD mass-locked | 27 | HUD analysis mode |
| 6 | Hardpoints deployed | 17 | FSD charging | 28 | Night vision |
| 7 | In wing | 18 | FSD cooldown | 29 | Altitude from average radius |
| 8 | Lights on | 19 | Low fuel | 30 | FSD jump |
| 9 | Cargo scoop deployed | 20 | Overheating | 31 | SRV high beam |
| 10 | Silent running | 21 | Has lat/long | | |

`Flags2` bits:

| Bit | Meaning | Bit | Meaning |
|---|---|---|---|
| 0 | OnFoot | 10 | VeryCold |
| 1 | InTaxi | 11 | VeryHot |
| 2 | InMulticrew | 12 | Glide mode |
| 3 | OnFootInStation | 13 | OnFootInHangar |
| 4 | OnFootOnPlanet | 14 | OnFootSocialSpace |
| 5 | AimDownSight | 15 | OnFootExterior |
| 6 | LowOxygen | 16 | BreathableAtmosphere |
| 7 | LowHealth | 17 | Telepresence multicrew |
| 8 | Cold | 18 | Physical multicrew |
| 9 | Hot | 19 | FSD hyperdrive charging |

`GuiFocus` values:

| Value | Screen | Value | Screen |
|---|---|---|---|
| 0 | none | 6 | **galaxy map** |
| 1 | right (internal) panel | 7 | system map |
| 2 | left (external) panel | 8 | orrery |
| 3 | comms panel | 9 | FSS |
| 4 | role panel | 10 | SAA |
| 5 | station services | 11 | codex |

Lat/long only update after a change of 0.02° in flight, or 0.0005° in an SRV. That is why EDCoPilot has a "light flasher" option to force more frequent updates during races (inferred from its wiki).

**What Status.json can't tell you:**
- throttle and speed,
- the current target,
- which tab is selected inside a panel,
- the galaxy-map cursor or search box state,
- whether a UI element is highlighted.

That gap is why plotting is a blind macro.

### 3.3 Frontier Companion API (cAPI)

Sources: `repos/fd-api/docs/FrontierDevelopments-CAPI-endpoints.md` and `oAuth2-notes.md` (https://github.com/Athanasius/fd-api), plus `repos/EDMarketConnector/companion.py`.

- **Auth:** OAuth2 Authorization Code with **PKCE (S256)** at `https://auth.frontierstore.net/{auth,token,decode}`.
  - Parameters: `audience=frontier,steam,epic`, `scope=auth capi`.
  - The **Client ID requires an approved app** in Frontier's Developer Zone (https://user.frontierstore.net/).
  - Access tokens last **4 h** (`expires_in` 14400). Refresh tokens work for at most **25 days** after the original authorisation; after that the user must log in again.
  - EDMC also checks that `/decode`'s `customer_id` matches the journal's FID, which guards against the wrong account.
- **Hosts:**
  - `companion.orerve.net`: Live (4.x / Odyssey)
  - `legacy-companion.orerve.net`: Legacy 3.8
  - `pts-companion.orerve.net`: beta
- **Endpoints:**
  - `/` lists the available links.
  - `/profile`, `/market`, `/shipyard` (includes outfitting), `/communitygoals`, `/fleetcarrier` (204 if you have no carrier), `/visitedstars`.
  - `/journal[/Y[/M[/D]]]` returns 200 / 206 (partial) / 204.
  - 418 means maintenance. Expired-token handling: fd-api documents 422 and EDMC handles 401.
- **Limits:** none are published. The guidance is to be polite, roughly one query per minute. EDMC enforces 60 s between station queries and 15 min for `/fleetcarrier`.
- **Other public Frontier endpoints:**
  - Community goals: `https://api.orerve.net/2.0/website/initiatives/list?lang=en`
  - GalNet JSON: `https://cms.zaonce.net/en-GB/jsonapi/node/galnet_article`
- **For a copilot:** use cAPI for things the journal lacks:
  - the full fleet and stored modules list after login,
  - carrier status,
  - authoritative market data,
  - remote journal reads (for example, the brain on another PC).
  
  It is **not** a realtime or control channel.

### 3.4 Keybindings (local, read/write)

- **Location:** `%LOCALAPPDATA%\Frontier Developments\Elite Dangerous\Options\Bindings\`
- **`StartPreset.<n>.start`** has four lines: the preset names for General, Ship, SRV and OnFoot. The older single-line `StartPreset.start` is what bindED still assumes.
- **`<Preset>.<maj>.<min>.binds`** files (e.g. `Custom.4.2.binds`) are XML: `<Control><Primary Device Key>[<Modifier/>]</Primary><Secondary/>…`. `{NoDevice}` means unbound.
- **Built-in presets aren't on disk** until the user customises them.
- **`BindingLoadingErrors.log`** reports missing devices. When a device in a custom file is missing, the game falls back to defaults, and the custom file can get overwritten if the user then edits binds. Custom files can be renamed and retitled so they appear by name in the game's preset list. (Spacejock KeybindsBackup: https://www.spacejock.com/KeybindsBackup.html)
- **Patterns in existing tools:**
  - **Require identical presets in all four sections:** EDCoPilot (wiki "missing_keybinds"), EliteAPI (`EliteDangerousApi.cs`), COVAS:NEXT (`src/lib/EDKeys.py`).
  - **Back up on every change:** EDCoPilot writes `.binds.bkp.NNNNN` files (wiki "automatic_binds_file_backups").
  - **Edit safely:** EliteIntel's `bindforge` makes surgical regex edits with SHA-256 baselines, stale-file guards, atomic writes, timestamped backups and conflict rules (`repos/EliteIntel/app/src/main/java/elite/intel/bindforge/PACKAGE.md`).
  - **Primary vs secondary:** COVAS:NEXT prefers the **secondary** slot for its required keyboard binds, so HOTAS mappings in primary stay untouched.
- **Out-of-game binding chores are fully feasible:** audit, back up, add missing keyboard binds to secondary slots, fix conflicts, and restore after device loss. The game must be restarted, or the preset re-selected, before it picks up changes.

---

## 4. Third-party services (status checked 8 Oct 2026)

| Service | Status | Auth | Limits / etiquette | Provides | Notes |
|---|---|---|---|---|---|
| **EDDN** https://github.com/EDCD/EDDN | Live (repo active Jul 2026) | none | Restarts Thursdays 07:06 UTC | **Live stream** `tcp://eddn.edcd.io:9500` (ZeroMQ SUB, zlib JSON). **Upload** `https://eddn.edcd.io:4430/upload/` | Stores nothing. Schemas in `EDDN/schemas/`. BSD-3 |
| **Spansh** https://spansh.co.uk | **Live** (route job queued, autocomplete OK) | none | No published limits; set a User-Agent | Official OpenAPI v3.1.1 (https://docs.spansh.co.uk/api.openapi.json) covers only `/api/{dump,system}/{id64}`, `/api/body/{id64}`, `/api/station/{marketId}`. **Routers are undocumented but widely used:** `POST /api/route` (neutron) → `{job}` → `GET /api/results/{job}`; `/api/generic/route` (EDDI); `/api/fleetcarrier/route`; `/api/trade/route`; `/api/nearest`; `/api/{systems,stations,bodies}/search[/save|/recall]`; `/api/systems/field_values/system_names?q=` | Daily dumps at https://downloads.spansh.co.uk/: `galaxy.json.gz` (~117 GB), `galaxy_populated` (~4.5 GB), `galaxy_stations` (~2 GB), `systems.json.gz` (~6.3 GB), plus 1-day and 7-day deltas. **Road-to-Riches and exobiology router API paths not verified.** No license stated |
| **EDSM** https://www.edsm.net | **Live** (API answered; status at `/api-status-v1/elite-server`) | API key for commander endpoints | Flight-log API: **360 requests/hour per user**. Cloudflare blocks the HTML pages | `api-v1` (system/systems/sphere/cube), `api-system-v1` (bodies, stations, factions), `api-logs-v1`, `api-journal-v1` (upload), `api-commander-v1`; nightly dumps | Community calls it under-maintained. Treat as best-effort |
| **Inara** https://inara.cz | Live (site is bot-protected) | Per-user API key; **app name must be whitelisted by the author** | "Be polite": batch events, send on session start/end and Docked/FSDJump; abuse gets keys invalidated. EDMC sends at most once per 35 s | Single endpoint `POST https://inara.cz/inapi/v1/` (header + batched events). Commander profile sync, some reads. **No market, system or station data via the API** | Dev guide (archived): https://web.archive.org/web/20260308173440/https://inara.cz/elite/inara-api-devguide/. Live game only |
| **Ardent Insight** https://ardent-insight.com | Live | none | Unknown | REST `https://api.ardent-insight.com/v2/...` (system, nearby, nearest service, commodity imports/exports, markets). Weekly SQLite dumps | Open source (github.com/iaincollins/ardent-api). Fed by EDDN. The closest replacement for EDDB's trade data |
| **EDAstro** https://edastro.com | Live | none | n/a | Maps and stats; **GEC POI JSON** `https://edastro.com/gec/json/all` (verified 200) | Used by OD-Explorer |
| **Canonn** https://canonn.science | Assumed live | n/a | n/a | Exobiology and codex data | Endpoints not verified |
| **EDDB** | **Shut down April 2023** | n/a | n/a | n/a | https://forums.frontier.co.uk/threads/moment-of-silence-for-eddb-io.615957/. Successors: Inara (trade UI), Spansh, Ardent, EDGalaxyData (https://edgalaxydata.space/) |

---

## 5. Ship control mechanics: how "toggle landing gear" actually works

1. **Read the binds.** Parse StartPreset and the `.binds` file and find a **keyboard** binding for `LandingGearToggle`. EDCoPilot and COVAS:NEXT can't press HOTAS buttons, so if the action is only bound to a joystick, the tool asks the user to add a keyboard secondary.
2. **Map the key name to a scancode.** EDCoPilot keeps a hardcoded "ScanMap" table per keyboard type (wiki "missing_scanmap_entries"). EliteIntel uses `MapVirtualKeyEx` with the foreground keyboard layout (`WindowsNativeKeyInput.java`). bindED maps key names to VK codes through `EDMap-<layout>.txt`.
3. **Focus the game.** The game is a DirectInput/RawInput consumer, so events must reach the foreground window.
   - EDCoPilot has the ini options `FocusUseAltSimulation` (a synthetic Alt press to get around `SetForegroundWindow` restrictions) and `FocusUseTopmostWorkaround`.
   - A second-monitor UI steals focus on every click, so EDCoPilot re-focuses the game after each button press.
4. **Inject.** Call `SendInput` with `KEYEVENTF_SCANCODE`, plus `KEYEVENTF_EXTENDEDKEY` for arrows, Ins/Del/Home/End/PgUp/PgDn, right Ctrl/Alt and numpad Enter or `/`.
   - Timing: hold for about 50–150 ms (EDCoPilot defaults to 0.15 s), then wait between keys. EliteIntel adds a random 99–201 ms post-input delay; COVAS:NEXT uses 10/200/100 ms.
   - Modifiers are pressed before the key and released after it.
   - **UIPI:** if the game runs elevated, input from a non-elevated process is silently dropped, so the copilot must run at the same integrity level (EDCoPilot and EliteIntel both warn about this).
5. **Verify.** Watch Status.json `Flags` bit 2 change within a timeout, e.g. 1.5 s. If it doesn't change, retry once or report failure.
   - EliteIntel's `WAIT_UNTIL` step polls a predicate every 50 ms (`ai/hands/InputSequenceExecutor.java`).
   - EliteAPI emits `Status.Gear` change events.
   - **Prefer idempotent intent over toggling blindly.** "Gear down" should press only if the bit is currently 0. Without this, a stale read or a double-fire flips the gear the wrong way.

**Alternatives to keyboard injection:**
- **vJoy** (a virtual joystick):
  - Maintained fork BrunnerInnovation 2.2.2.0, signed, released Dec 2023; the maintainer says little further investment is planned (https://github.com/BrunnerInnovation/vJoy).
  - You would bind actions to vJoy buttons in-game. Upside: it avoids keyboard-layout and focus problems for some inputs, because DirectInput devices are read even without focus (unverified for Elite).
  - Downside: it needs a driver install, and the user has to accept vJoy entries in their binds.
- **ViGEmBus** (a virtual Xbox pad) is **retired**: final 1.22.0, Nov 2023 (https://docs.nefarius.at/projects/ViGEm/End-of-Life/).
- **Memory or DLL hooking:** forbidden by the Frontier EULA and risky with anti-cheat. Don't.

**Linux:** EliteIntel uses XTest under X11. Wayland blocks synthetic input without portals or uinput. EliteAPI and EDCoPilot know the Proton paths.

**ToS boundary.** The Frontier EULA (https://www.frontier.co.uk/legal/elite-dangerous/eula) forbids "cheats, automation software, hacks, mods, or any other unauthorized software designed to modify or defeat the purpose or experience of the Game" and "any robot … or other automated … means to access the Game".
- Voice macros (VoiceAttack, EDCoPilot) have been tolerated publicly for years.
- I found no written Frontier whitelist.
- Keep a **human in the loop**: one user request leads to one bounded action sequence. No unattended loops such as auto-trading, auto-mining or AFK carrier routes. EDCoPilot's "auto" features (AutoHonk, auto-neutron next plot, auto shields) each trigger on a game event and perform a short action, which sits in a grey zone.

---

## 6. Course plotting automation

**Sequences in existing tools (from code):**
- **EliteIntel** (`repos/EliteIntel/app/src/main/java/elite/intel/gameapi/inputs/RoutePlotter.java`):
  1. Normalise the map toggle state (close and reopen if needed).
  2. Wait for `GuiFocus==6` or the `Music` event, then settle 1 s.
  3. Hold `CamZoomIn` for 500 ms.
  4. `UI_Left`, `UI_Right`, `UI_Select` to focus the search field.
  5. Type the name, press Enter, wait 500 ms.
  6. `UI_Right`, wait 500 ms, `UI_Select` (plot), wait 3 s, close.
  
  There's **no route verification**; it just beeps. Its code comments document real failures, including a HOTAS axis generating phantom `UI_Left` presses and a slow map open.
- **COVAS:NEXT** (`repos/Elite-Dangerous-AI-Integration/src/lib/actions/Plotter.py`):
  1. Resolve the user's phrase through Spansh system/station/body search with Levenshtein matching.
  2. Open the map, with a fallback of `UI_Back`×10.
  3. **Check for conflicts** between `UI_Up`/`UI_Right` and the camera-translate binds.
  4. `CamZoomIn`, `UI_Up`, `UI_Select`, type, Enter, `UI_Right`, `UI_Select`, `CamZoomOut`.
  5. Wait 3 s plus 1 s per 1,000 ly (distance from EDSM coordinates).
  6. Hold `UI_Select` for 1 s.
  7. **Verify that NavRoute.json's last `StarSystem` equals the target.**
  
  For in-system targets it OCRs the system map (`src/lib/ScreenReader.py`, rapidocr).
- **EDCoPilot** (wiki "fails_to_plot_course", "missing_keybinds"):
  - Needs keyboard binds for GalaxyMapOpen, UI Panel Select/Right/Up, UI Back, Next Panel Tab and Galaxy Cam Zoom In.
  - Fails if the UI-panel keys conflict with camera-translate keys, e.g. WASD.
  - Galaxy-map "Show Grid" must be on.
  - Has a configurable "additional pause after opening Galaxy Map", an alternate sequence, and a "partial plot" mode that only positions the map.
  - Refuses to plot from Station Services unless overridden in the ini.
  - Can't plot to a station or body, because the game's search can't. It lists this on its Frontier wish list.
- **OD-Explorer and EDMC SpanshRouter:** don't automate the map at all. They **copy the next waypoint to the clipboard** and the user pastes it into the galaxy-map search. This is the most reliable option and has zero ToS ambiguity, but it isn't hands-free.

**Why it's error-prone:**
- The game state isn't observable inside the map: search box focus and results list.
- Latency is variable: map load, server search and route calculation for long routes.
- Bind conflicts and keyboard layouts get in the way.
- Analog noise from HOTAS axes and hats.
- Focus loss.
- Map toggle semantics: pressing "open" while it's already open closes it.
- Names are ambiguous or partial, and the search autocompletes.
- Route length limits, and plotting failures such as "unreachable" or jump range.
- Odyssey on-foot or SRV contexts use different binds.

**Mitigations (recommended design):**
1. **Resolve before acting.** Turn speech into a canonical system name and id64 using Spansh autocomplete and fuzzy matching, and confirm aloud if ambiguous ("Did you mean Colonia or Colonia Dream?").
2. **Gate on state.** Require `GuiFocus==0` and not in station services before starting, and `GuiFocus==6` before typing. Abort if `GuiFocus` changes unexpectedly.
3. **Normalise.** Always press `UI_Back` until `GuiFocus==0`, then open the map, so the toggle state is never ambiguous.
4. **Type text with `KEYEVENTF_UNICODE`** (layout-independent) or the clipboard plus Ctrl+V (verify the game accepts paste in the search box; unverified).
5. **Verify the outcome.** Wait for a `NavRoute` journal event whose final system matches the resolved target; for single-jump targets, check `FSDTarget`. Use a timeout scaled by distance.
6. **Pre-flight bind audit.** Check that all required keyboard binds exist and don't conflict before offering the feature, and offer to fix them (§3.4).
7. **Fallback chain:** auto-plot → partial plot (open the map at the system) → clipboard with a spoken prompt.
8. For **neutron or multi-hop routes**, plot only the next waypoint. Trigger the next plot from a `JetConeBoost`/`FSDJump` event, ideally with the user's confirmation, as EDCoPilot's "Auto Jump" option does.

---

## 7. Survey of existing tools

| Tool | Stack | What it does | Key mechanisms (file refs) | Relevance |
|---|---|---|---|---|
| **EDCoPilot** (closed) | Python/PyInstaller, Kivy, VoiceAttack | Full "ship AI": announcements, panels, plotting, Spansh routes, chatter | See §8 | Benchmark for features |
| **COVAS:NEXT** (MIT) | Python + Electron/Angular | LLM voice copilot with actions | STT (faster-whisper, sherpa-onnx Parakeet), TTS (edge-tts, Kokoro), OpenAI-compatible LLMs with tool calls, `EDKeys.py`, `directinput.py`, `Plotter.py` (NavRoute verification), OCR, EDMesg bridge to EDCoPilot, electron-vr overlay (`@covas-labs/electron-vr`, npm license "Proprietary") | Closest open-source match to the vision |
| **EliteIntel** (CC0) | Java 21 | LLM copilot, local or cloud | sherpa-onnx Parakeet, Kokoro/Supertonic TTS, `InputSequenceExecutor` with `WAIT_UNTIL`, `bindforge` safe bind editing, `RoutePlotter`, C OpenVR overlay over a stdin protocol (`overlay/PROTOCOL.md`), Spansh, EDDN listener | Best reference for the executor, binds and VR overlay |
| **EliteAPI** (Somfic) | C# | Journal/Status library + VoiceAttack plugin | `EliteDangerousApi.cs` watchers (200 ms debounce), synthetic `Status.*` events, `BindingParser.cs` | Model for an event API |
| **bindED** | C# VoiceAttack plugin | Exposes binds to VoiceAttack | Single-line StartPreset; `Custom.3.0` only → **outdated for 4.x** | Shows the original approach |
| **OD-Explorer** | C# WPF .NET 8 | Exploration helper | Journal parsing, Spansh CSV → clipboard next hop, EDSM, EDAstro GEC | Clipboard-assist pattern |
| **EDMC** (GPL-2.0) | Python | Market/EDDN uploader, plugin host | cAPI OAuth (`companion.py`), `plugins/{eddn,edsm,inara}.py`; huge plugin ecosystem (SpanshRouter, EDMCOverlay, Canonn) | Reference cAPI client |
| **EDDI** | C# | Event-driven speech + VoiceAttack plugin | Cottle templates, SpeechService (SAPI, Windows.Media, Polly, Azure), Spansh (`generic/route`, `fleetcarrier/route`), Inara/EDSM/EDDN responders; writes `speechresponder.out` | Mature event→speech design |
| **EDDiscovery** (Apache-2.0) | C# | Logbook, 3D map, scripting | Journal DB, EDSM/Spansh, action packs with key presses | Data-rich, heavy |
| **Elite Observatory** (MIT) | C# | Plugin journal processor with notifications | TTS and popup notifiers | Plugin model |
| **ED Odyssey Materials Helper** (MIT) | Java | Materials, engineering, colonisation | Journal + data files | Domain data |
| **SrvSurvey** (GPL-3.0) | C# | Contextual bio, guardian and colonisation overlays | Status.json lat/long/heading geometry | Surface guidance math |
| **EDEngineer** (MIT) | C# | Blueprint tracker | Last push Mar 2024 | Stale |
| **Spacejock KeybindsBackup** | Windows app | Back up and rename binds | File copy, PresetName rename | Simple chore tool |
| **Newcomers (TS/Rust)** | | | | |
| Mallock/edmo (MIT) | **TypeScript + Rust/Tauri** | HUD, LM Studio guidance, Piper TTS | Rust tails the journal; React UI | Validates the Tauri approach |
| kayahr/ed-journal (MIT) | **TypeScript** | Typed journal reader/watcher | Active Oct 2026 | Reuse in a TS UI |
| rster2002/ed-journals (MIT) | **Rust** | Journal/Status models + live watching (crates.io `ed-journals`) | Active Oct 2026 | Reuse in a Rust core |
| voxta-module-elite-dangerous, KarlGrear/EDAI | C# | Small LLM companions | Journal + keybind actions | Minor |

---

## 8. EDCoPilot deep dive

### 8.1 What I could and couldn't do

- **Binaries:** the MSI is publicly downloadable: https://github.com/Razzafrag/EDCoPilot-Installer/releases, latest v1.11.756 as linked from razzafrag.com.
- **Why I stopped:** the **EULA** (https://www.razzafrag.com/eula, saved at `notes/eula.txt`) says you may not "decompile, disassemble or reverse engineer the Application or attempt to do any such things", and that downloading or using the application means agreeing to it.
- **Decision:** I did **not** download, unpack or statically inspect it. The brief's condition ("only if obtainable legitimately") isn't really met: getting it is legitimate, but inspecting it isn't. **Sumeet should decide** whether he wants to do any inspection on his own machine at his own risk. I recommend against it: the public docs already reveal the architecture, and clean-room design is safer.
- **Sources used instead:**
  - 112 wiki pages from https://razzserver.com/dokuwiki/ (raw exports in `notes/edcp_wiki/`)
  - the feature list (https://www.razzafrag.com/featurelist)
  - release notes up to v1.11.748 (1 Sep 2026)
  - the VR overlay post
  - open-source EDMesg

### 8.2 Architecture (inferred, high confidence unless noted)

- **Packaging:**
  - Python packaged with **PyInstaller**: the wiki mentions `_MEIxxxx` extraction folders, antivirus false positives "because it is packaged with PyInstaller", and Python stack traces.
  - **Kivy** GUI: release notes mention "kivy patches".
  - MSI installer to `C:\EDCoPilot`.
- **Processes:**
  - `LaunchEDCoPilot.exe` (launcher/updater), `EDCoPilot.exe` (core), `EDCoPilotGUI2.exe` (UI).
  - The UI talks to the core over **local TCP** (`serverprocessip=127.0.0.1` in `edcopilotgui.ini`, heartbeats, "GUI x has disconnected").
  - Multiple UI clients are supported, plus a **remote UI on another Windows PC**, which avoids focus stealing. A third-party browser UI, "EDCOPTER", also exists.
- **Config and state:**
  - `EDCoPilot.ini` (configparser), per-commander `EDCoPilot.SaveData.<cmdr>.LIVE.txt` (key=value records, including `Character:<Role>=voice`), the registry key `HKCU\SOFTWARE\EDCoPilot`, and a speech cache folder.
- **Backend:**
  - `razzserver.com` (behind Cloudflare), with fallback `raztest.ddns.net`.
  - Daily systems/stations reference files derived from the **Spansh daily extract**.
  - Time-trial leaderboards, community-published chatter and station facts, the supporter flag, and telemetry: startup commander and version, usage stats, crash traces.
- **Voice input:**
  - **None built in.** A locked **VoiceAttack profile** (about 3,000 commands) writes request strings to `EDCoPilot.request.txt`; the core polls it, executes the request and deletes the file.
  - EDCoPilot asks VoiceAttack to refresh dynamic phrase lists, such as recent system names.
  - AutoHotkey can write the same file.
  - The request vocabulary is documented on the "panel_navigation_commands" page, e.g. `SessionProgress:Mining`, `OpenMessagesPanel`.
- **Voice output:**
  - **Edge neural voices** by default (the unofficial edge-tts style endpoint, inferred), Windows SAPI, Google Cloud (including Chirp3-HD), Amazon Polly, Azure and ElevenLabs.
  - It tracks free-tier usage.
  - Translation through DeepL or Google.
  - DSP: radio effect, synthetic chorus/reverb, and a Bluetooth anti-clip hum.
  - A priority speech queue with interrupts, separate from background "space chatter".
  - **SpeechExtensions** phrase system: `@tags`, phrase lists, filters such as `(profanity)` and `(CMDRisMale)`, and tokens such as `<Commander>` and `<starsystem>`.
  - Multi-voice dialogue strings in the form `[<Role>]text[<Role2>]text`.
- **LLM:**
  - **None in EDCoPilot itself.** It integrates with **COVAS:NEXT** through **EDMesg**: ZeroMQ PULL on `tcp://127.0.0.1:15560` for actions and PUB on `15561` for events, with JSON `{type,data}` messages.
  - Actions: `OpenPanelAction`, `PanelNavigationAction`.
  - Events: `SpeakingPhraseEvent`, `PanelContentsEvent`, `LastVisitedEvent` (`repos/EDMesg/EDMesg/EDCoPilot.py`).
  - Result: COVAS's LLM replies are spoken by EDCoPilot's voices, and multi-character role tags map to different voices (wiki "using_the_llm_to_simulate_multiple_characters").
- **Ship control:** keyboard scancode injection exactly as in §5 (ScanMap table, 0.15 s press, focus workarounds, admin caveat). It needs **keyboard** binds and the same preset in all four sections. It backs up binds on every change.
- **Other integrations:**
  - It reads EDDI's `speechresponder.out` to voice EDDI's output.
  - It writes `EDCoPilot.Status.json` (speaking state), `LastSpoken.txt`, `shipai_speech.json` (for external TTS) and `panel.json`.
  - Discord webhooks, VLC (radio), VMX (music), GeForce Instant Replay, and EDMC Overlay (desktop only).
- **VR:** no native overlay. The user captures the EDCoPilot window with OVRDrop, Desktop+, OVR Toolkit, OpenKneeboard (via OpenComposite) or Oculus Dash (https://www.razzafrag.com/post/vr-overlay-for-oculus-using-openkneeboard). Control is by voice.

### 8.3 EDCoPilot feature catalogue and likely mechanism

Confidence key: **H** = documented in the wiki or feature list; **M** = mechanism inferred from documented behaviour plus the journal spec; **L** = guess.

| Feature (from feature list / wiki) | Likely mechanism | Data source | Conf. |
|---|---|---|---|
| Event announcements (scoopable stars on route, undiscovered system, last visit count, pad location on docking, limpet/crew reminders, materials full at trader, pirate scan warning, interdiction, friends online, wing join, wedding/funeral ships, comms read-out, HGE candidates) | Journal event handlers + local history DB + SpeechExtensions phrases | Journal: `FSDJump`, `NavRoute`, `DockingGranted` (`LandingPad` + station-layout table), `Docked`, `Materials`, `Scanned`, `Interdicted`, `Friends`, `WingJoin`, `ReceiveText`, `FSDJump` faction states | H/M |
| Voice commands (~3,000) | VoiceAttack grammar → request file → core | VoiceAttack | H |
| **Automatic course plotting** (bookmark click, voice "set course for X", last 120 destinations, partial plot) | Galaxy-map key macro (§6), name typed into search | Binds, Status `GuiFocus` | H |
| **Auto Jump on neutron route** | After supercharge (`JetConeBoost`) and a 5–20 s delay, plot the next waypoint, or just target it | Journal + Spansh route | H |
| Panel gesture trigger (left→bottom→left panel = plot next) | Watch the `GuiFocus` sequence in Status.json | Status | H |
| Spansh routes: Neutron, Road-to-Riches, Fleet Carrier, Trade (1–10 hops), Exo-Mastery, Galaxy Plotter CSV import | Spansh router API job/poll; results saved as bookmark groups | Spansh | H |
| Trade route market check (better buy list) | Compare the route against `Market.json` when the market opens | Market.json | H |
| Location finder (commodities, traders, brokers, modules, ships, megaships, Vista Genomics, bartenders…) | Spansh station/system search plus its own daily extract; ranks by jumps vs arrival distance | Spansh, backend | H/M |
| Bookmarks (unlimited, groups, filters, notes, per-system command menu, auto-bookmark) | Local DB | Local | H |
| System/Body/Stations panels, Station Facts (Inara Galpedia), faction info | Spansh/EDSM/Inara lookups + journal history | APIs | H |
| Guidance computer (bearing/distance to surface coordinates), saved surface locations | Great-circle maths on Status lat/long/heading/`PlanetRadius` | Status | H |
| Discovery estimator, biologicals tracking, colony distance rules, "moved far enough" | `Scan`/`SAASignalsFound`/`ScanOrganic` + species min-distance table + lat/long | Journal, Status | H/M |
| First-footfall detection | **Screen capture of the in-game banner + pixel-colour counting**, because the journal doesn't log it | Screen | H |
| Session progress (combat/mining/on-foot/colonisation), mining stats, prospector read-outs | Journal aggregation (`Bounty`, `ProspectedAsteroid`, `MiningRefined`, …) | Journal | H |
| Colonisation progress tracker | `ColonisationConstructionDepot`/`Contribution` events | Journal | M |
| Missions list with expiry reminders | `Missions`/`MissionAccepted`/… + timers | Journal | H |
| Ships, stored modules, locker, suits, weapons | `Loadout`, `StoredShips`, `StoredModules`, `ShipLocker`, `SuitLoadout` | Journal / companion files (not cAPI as far as docs show) | M |
| Blueprints and material trader tables | Static engineering data + `Materials` | Journal + static data | H |
| Controls panels (flight/combat/SRV/on-foot/camera buttons) | Each button = a keypress macro for a bind; greyed out if unbound | Binds | H |
| Preset messages sent in game | Likely opens comms and types text via key injection (mechanism not documented) | Binds | L |
| AutoHonk, auto shields on foot, auto target highest threat, Nomad hardpoint retract, light flasher | Event-triggered keypresses (`FSDJump` → hold fire on the discovery scanner firegroup, etc.) | Journal/Status + binds | H |
| Space/station/crew/deep-space/GalNet chatter, FC greetings, chit-chat | Phrase libraries + multiple cloud voices + radio DSP; GalNet via Frontier feed | Local + cloud TTS | H |
| Time trials (SRV/ship), leaderboards, Minibar directions | Lat/long waypoint geometry; handbrake flag as the start trigger; backend leaderboard | Status, backend | H |
| BGS tick and faction-system reports | Background checks (Spansh; EliteBGS was dropped in 2025) | APIs | H/M |
| EDSM unknown-system sound on target | EDSM lookup on `FSDTarget` | EDSM | H |
| Screenshots (convert, rename, post to Discord) | `Screenshot` event + file ops + webhook | Journal | H |
| Music player / VMX, triggered sound samples, radio | Event→folder mapping; VLC | Journal | H |
| Keybind backups | File watcher + copy | Binds | H |
| Translation (DeepL/Google) | Cloud API before TTS | Cloud | H |
| Remote UI, multi-UI, tablet | TCP client/server | n/a | H |
| COVAS:NEXT LLM conversation, multi-character voices | EDMesg ZeroMQ | n/a | H |
| EDMC overlay (pad directions, guidance) | EDMCOverlay's local socket protocol | n/a | M |

---

## 9. Feature matrix for the new copilot

Difficulty and risk scale: 1 = trivial, 5 = hard or fragile.

| Feature | Existing tools | Mechanism | Data source | Diff/Risk | TS/Rust notes |
|---|---|---|---|---|---|
| Live state awareness (location, ship, fuel, cargo, docking) | All | Tail the journal + watch Status.json and companion files | Journal, Status | 1 | Rust `notify` + `ed-journals`; TS `chokidar` + `kayahr/ed-journal`. Handle partial writes, `FileShare.ReadWrite`, debounce (~50–200 ms) |
| Event announcements / proactive commentary | EDCoPilot, EDDI, COVAS | Event → rule/LLM → TTS with a priority queue | Journal | 2 | Keep a deterministic rules layer; use the LLM to vary phrasing only. Rate-limit speech |
| Ship toggles (gear, lights, hardpoints, scoop, FA, night vision, silent running, SRV handbrake) | EDCoPilot, COVAS, EliteIntel, VoiceAttack | Binds → `SendInput` scancodes → verify via Status flag | Binds + Status | 2 | Rust `windows` crate `SendInput`; TS needs a native addon (N-API/koffi) |
| Actions without a Status flag (target next system, boost, request docking, deploy heatsink, FSS/DSS modes, panel navigation) | Same | Keypress; verify via journal (`FSDTarget`, `DockingRequested`, `HeatWarning`…) or `GuiFocus`; otherwise unverified | Binds + Journal | 3 | Mark "unverifiable" actions as such in the tool schema |
| Course plotting to a system | EDCoPilot, COVAS, EliteIntel | Galaxy-map macro with `GuiFocus` gating + NavRoute verification; fallback to clipboard | Spansh + binds + Status + NavRoute | **4–5** | Isolate in a state machine with timeouts and abort |
| Plot to a station/body | COVAS (OCR on system map) | System-map OCR click/selection | Screen | **5** | Avoid; announce "plotted to system; target station manually" |
| Neutron/R2R/carrier/trade routes | EDCoPilot, EliteIntel, EDDI, SpanshRouter | Spansh job API; then per-hop plot or clipboard | Spansh | 2 (API) / 4 (auto-hop) | Undocumented API → wrap with schema validation and graceful errors |
| "Nearest X" search | EDCoPilot, EliteIntel | Spansh `/api/nearest`, station search; Ardent nearest service | Spansh, Ardent | 2 | Cache results; rank by jumps + arrival distance |
| Market/trade advice | EDCoPilot, Inara web | Ardent API/SQLite, Spansh trade router, `Market.json` | Ardent, Spansh | 2–3 | Ardent SQLite dumps allow offline queries (`rusqlite`/`better-sqlite3`) |
| Exploration value / bio guidance | EDCoPilot, OD-Explorer, SrvSurvey | Journal scans + value formulas + species distance tables + lat/long geometry | Journal, Status | 3 | Value formulas are community-derived; cite and unit-test |
| Surface guidance / time trials | EDCoPilot, SrvSurvey | Great-circle bearing from Status | Status | 2 | Pure maths |
| Keybinding audit / fix / backup / restore | EDCoPilot (backup), EliteIntel bindforge, KeybindsBackup | Parse XML, atomic edits, backups, conflict check | Binds | 2–3 | `quick-xml` (Rust) or `fast-xml-parser` (TS); preserve unknown nodes, write atomically, never edit while the user is in the options menu |
| Commander/fleet overview | EDCoPilot, EDMC | Journal (`Loadout`, `StoredShips`), optional cAPI `/profile` | Journal, cAPI | 2 (journal) / 3 (cAPI approval) | cAPI needs a Frontier client-id approval and PKCE flow; tokens re-auth every 25 days |
| EDDN contribution | EDMC, EDDI | POST schemas | EDDN | 2 | Optional, a good-citizen feature |
| Natural conversation with game context | COVAS, EliteIntel, EDCoPilot+COVAS | LLM with tool calls + state summary | All | 3 | Keep the context small: a structured state snapshot, not raw journals (EliteIntel caps prompts at 8K tokens) |
| Voice I/O | All | Cloud S2S or local STT+TTS | n/a | 3 | sherpa-onnx (Node addon / Rust crate); WebRTC for GPT-Live in Tauri |
| VR presence | EliteIntel (native OpenVR), COVAS (electron-vr), EDCoPilot (window capture) | OpenVR overlay process | n/a | 3–4 | Rust FFI to `openvr_api` (the `ovr_overlay` crate is pre-0.1 and stale); audio-first fallback |
| First-footfall / other screen-only facts | EDCoPilot | Screen capture + pixel/OCR | Screen | 4 | Optional; fragile with HUD colour mods |
| In-game chat messages | EDCoPilot (presets) | Comms panel + typed text | Binds | 3 | Unicode `SendInput` |

---

## 10. Hard or error-prone features and mitigations

1. **Galaxy-map plotting.** See §6. Use a state machine with `GuiFocus` gates, normalising, Unicode typing, NavRoute verification, distance-scaled timeouts, a bind pre-flight check, and a partial-plot or clipboard fallback. Log every step for diagnosis.
2. **Keyboard-bind dependency.** Many users bind only HOTAS. Mitigations:
   - Auto-add keyboard **secondary** binds on unused keys (COVAS approach) through a safe bind editor, with a backup and an explicit user confirmation.
   - Require identical presets across the four sections, or handle mixed presets by reading the right file per section.
3. **Focus and UIPI.** A second-monitor UI steals focus. Mitigations:
   - Make the copilot UI non-activating (`WS_EX_NOACTIVATE`).
   - Restore focus to the game before injecting.
   - Detect an elevated game (`OpenProcessToken` integrity check) and warn.
   - Never inject when the game isn't foreground. Ask instead.
4. **Toggle-state ambiguity.** Use idempotent intents ("gear down"), check Status first, and verify after.
5. **Keyboard layouts.** Map through scancodes, not VKs. Use the game's key names to scancodes for the physical key, and Unicode only for text.
6. **Timing drift.** Make waits adaptive: poll Status/journal events with timeouts instead of fixed sleeps, and add small jitter (EliteIntel uses 99–201 ms).
7. **Journal/Status read races.** Status.json gets rewritten in place, so you will sometimes read a partial file. Retry the parse. Journal tailing must handle new files (`Fileheader`), `Continued` parts and the game restarting.
8. **Undocumented or community APIs** (Spansh routers, EDSM health, Inara whitelist). Mitigations:
   - Use adapters with schema validation, caching and circuit breakers.
   - Prefer the documented Spansh `/api/system` endpoints and the dumps where possible.
   - Register the app name with Inara before sending anything.
9. **LLM hallucinating actions or facts.** Mitigations:
   - Give the model only typed tools with enumerated actions.
   - The tool layer verifies results and returns ground truth.
   - Ask for spoken confirmation on irreversible actions (self-destruct, selling cargo, carrier jumps).
   - Never let the LLM invent system names: always resolve through Spansh.
10. **Latency.** Put a local intent fast path in front of the LLM for common commands, and use streaming speech-to-speech with barge-in.
11. **ToS.** Keep a human in the loop and no unattended automation. Use no game memory reads or DLL injection. Make "auto" features opt-in and event-bounded.
12. **VR runtime fragmentation.** Use OpenVR overlays for SteamVR users. Document OpenComposite+OpenKneeboard for Oculus users. Audio-only always works.

---

## 11. AI and voice stack (verified 8 Oct 2026)

| Layer | Option | Notes |
|---|---|---|
| **Full-duplex speech-to-speech** | **OpenAI GPT-Live-1** (`gpt-live-1`) | Listens and speaks at once and handles interruptions. **Delegates tools**: either Responses delegation (backend model e.g. `gpt-6-luna`/`gpt-6-sol`) or **client delegation**, where your app runs tools and returns results with `session.commentary.append`/`thinking.append` (500 tokens per append). $0.05/min + backend. Concurrency-limited (Build tier: 50). https://developers.openai.com/api/docs/models/gpt-live-1, https://developers.openai.com/api/docs/guides/live-delegation |
| | OpenAI `gpt-realtime-2` | Speech-to-speech with function calling and configurable reasoning effort; audio $32/$64 per 1M tokens in/out. https://developers.openai.com/api/docs/models/gpt-realtime-2. A newer 2.1/2.1-mini is reported in a community announcement (unverified) |
| | Google **`gemini-3.8-live`** | Native audio. **Async (non-blocking) function calls by default**, with scheduling `SILENT`/`WHEN_IDLE`/`INTERRUPT`, which suits game events. https://ai.google.dev/gemini-api/docs/models/gemini-3.8-live |
| | xAI Grok Voice Agent | `wss://api.x.ai/v1/realtime?model=grok-voice-latest`, OpenAI-Realtime-compatible events, tools. https://docs.x.ai/developers/model-capabilities/audio/speech-to-speech |
| **Local STT** | **NVIDIA Parakeet** (`parakeet-tdt-0.6b-v2/v3`, `parakeet-unified-en-0.6b` from Apr 2026 with streaming, ~160 ms minimum latency) | Parakeet is **speech-to-text**, not TTS. https://huggingface.co/nvidia/parakeet-unified-en-0.6b. Runs in **sherpa-onnx**, which has both a Node addon (`sherpa-onnx-node`) and a Rust crate (`sherpa-onnx`) |
| | faster-whisper / Whisper turbo, Moonshine | Alternatives. Add Silero VAD and a wake word (sherpa-onnx keyword spotting) |
| **Local TTS** | **Kokoro-82M**, Supertonic 3, Piper, Soprano | Kokoro is used by COVAS and EliteIntel; it runs in sherpa-onnx on CPU. edge-tts (EDCoPilot's default) is an unofficial endpoint, so there's a ToS and availability risk |
| **Cloud TTS** | ElevenLabs (streaming), Google Chirp3-HD, Azure, Polly | For character voices and chatter |
| **LLM (cascaded)** | Any tool-calling model, cloud or local (LM Studio/Ollama) | EliteIntel reports about 750 ms end-to-end for "gear down" via cloud |

**Recommended voice architecture.**
- **Primary mode (online):** GPT-Live-1 with **client delegation**, or Gemini Live with non-blocking tools.
  - The voice model handles conversation, barge-in and personality.
  - Every game action and lookup goes to *our* tool server, which executes, verifies and returns facts.
  - Game events are pushed as `thinking.append` (silent context) or `commentary.append` (things to say aloud).
- **Fast path:**
  - Run local streaming Parakeet in parallel on the mic.
  - A small grammar matcher catches high-frequency commands (gear, lights, hardpoints, boost, "target next system") and executes them within about 300 ms.
  - Then notify the live session ("Landing gear deployed, confirmed") so the conversation stays consistent.
- **Offline mode:** Parakeet STT → local LLM with tools → Kokoro TTS, all in-process via sherpa-onnx.
- **Push-to-talk:** bound to a HOTAS button, read by the copilot directly through SDL2 or HID, so it works in VR. Alternatively, always-on VAD plus a wake word.

---

## 12. VR delivery

- **What the game supports:** Elite supports **OpenVR/SteamVR** and the **legacy Oculus SDK**, with **no native OpenXR** (https://forums.frontier.co.uk/threads/a-few-quick-vr-things.618890/). OpenComposite can map OpenVR onto OpenXR runtimes.
- **OpenKneeboard:** its compatibility page lists Elite as OpenVR ✅, Oculus ❌, and "support for other APIs is blocked by the anti-cheat software" (https://openkneeboard.com/compatibility/games/).
- **Recommended tiers:**
  1. **Audio-first.** Voice plus earcons; works on every runtime.
  2. **Native OpenVR overlay** as a separate process: `IVROverlay` with a texture from our renderer and gaze or controller interaction. No game injection. This is what EliteIntel's C overlay does (`repos/EliteIntel/overlay/PROTOCOL.md`).
     - Rust: thin FFI to `openvr_api`. The `ovr_overlay` crate is 0.0.0 from 2022.
     - TypeScript: native addon, or COVAS's `@covas-labs/electron-vr`, whose license is "Proprietary", so check it before depending on it.
  3. **Window capture** (OpenKneeboard, OVR Toolkit, Desktop+) as a documented fallback for Oculus/OpenComposite users.
- **Design rules for VR:**
  - Large text and minimal interaction; anchor near cockpit panels.
  - Never require the mouse or keyboard.
  - Every UI action has a voice or HOTAS equivalent.

---

## 13. Architecture and stack recommendation

```
                 +-------------------------- Copilot Core (Rust, one process) --------------------------+
 Journal/*.json → | Watchers → Event bus → State store (typed snapshot)                                    |
 .binds/Start*  → | Binds service (parse / audit / safe edit / backup)                                    |
                  | Action executor (SendInput scancodes, focus mgr, UIPI check, WAIT_UNTIL verification) |
                  | Macro state machines (galaxy-map plot, docking request, panel nav) w/ timeouts       |
                  | Data adapters: Spansh, Ardent, EDSM, Inara(opt), EDDN(opt), cAPI(opt)  + cache/SQLite |
                  | Voice I/O: mic capture, VAD, Parakeet (sherpa-onnx), Kokoro TTS, audio mixer/queue   |
                  | Fast-path intent matcher                                                             |
                  | Tool server (typed JSON schema) ← used by LLM / live voice                           |
                  | OpenVR overlay renderer (separate thread/process)                                    |
                  +-----------------------------------------↑-------------------------------------------+
                                                            | IPC (Tauri commands / WebSocket)
              +---------------- UI (TypeScript, Tauri webview; also served to tablet/phone) --------------+
              | Panels, settings, bind doctor, route views, transcript; GPT-Live/Gemini Live session (WebRTC)|
              +----------------------------------------------------------------------------------------------+
```

**Why Rust for the core (vs TypeScript):**
- **Win32 input and VR:** `SendInput`, focus, integrity checks and the OpenVR FFI are first-class through the `windows` crate. In Node they need native addons such as koffi or N-API, which complicate packaging and add timing jitter from the GC and event loop.
- **Audio:** low-latency capture, mixing and DSP (radio effects) plus in-process sherpa-onnx are easier in Rust (`cpal`, `sherpa-onnx` crate) than in Node.
- **Reliability:** a single static binary, low memory use alongside a heavy game and a VR compositor, and strong typing for the journal event enums (`ed-journals` already exists).
- **Distribution:** Tauri gives a small installer and auto-update. Electron would add about 150 MB and a Chromium process competing with the game and VR.

**Why TypeScript for the UI and brain glue:**
- Fast UI iteration (React/Svelte); the UI can be reused as a web UI on a tablet (EDCoPilot's popular remote-UI pattern).
- The OpenAI/Gemini realtime SDKs and WebRTC are most mature in JS.
- `kayahr/ed-journal` gives typed events if any logic lives in the UI.

**If one language is a must:**
- **Rust-only** (egui or Tauri-less) is fine but has slower UI work.
- **TypeScript-only** (Node + Electron) is the fastest path to a prototype. COVAS's Electron UI and its `@covas-labs/electron-vr` prove it can work. Expect native-addon work for input, VR and STT, and a heavier runtime.

**Tool-layer design (LLM-facing), with example tools:**
- `get_state()`
- `set_ship_function({function: "landing_gear", state: "down"})`, which returns `{verified: true|false, observed}`
- `plot_route({destination, mode: "auto"|"partial"|"clipboard"})`, which returns the NavRoute summary
- `find_nearest({service, filters})`
- `spansh_route({type: "neutron"|"carrier"|"trade", ...})`
- `bindings_audit()` and `bindings_fix({plan_id, confirm: true})`

All of these:
- are enumerated and schema-validated,
- **never let the model press arbitrary keys**,
- require confirmation for destructive actions.

**Phased plan (suggested):**
1. Watchers, state, and spoken announcements (rules) with local TTS.
2. Bind doctor (audit, backup, safe secondary-bind insertion) and verified toggles.
3. Voice: GPT-Live client delegation, plus the local fast path.
4. Spansh routes and plotting state machine with NavRoute verification and fallbacks.
5. OpenVR overlay.
6. Optional cAPI, EDDN and Inara integrations (cAPI and Inara need registration or whitelisting).

---

## 14. Open questions and risks

1. **Frontier's stance on voice-driven automation** isn't written down beyond the EULA's "automation software" clause. Event-triggered auto-actions (auto-honk, auto-plot next neutron hop) are a grey zone. Keep them opt-in. Consider asking Frontier community management.
2. **EDCoPilot internals** are inferred only. Binary inspection is blocked by its EULA; Sumeet's call. If the goal is interoperability, EDMesg is the sanctioned public integration surface.
3. **Spansh router endpoints are undocumented** and could change. Spansh has no published rate limit or license for API use or dumps. Contacting the author is advisable before shipping a public tool.
4. **cAPI access** needs a Frontier Developer Zone client ID (approval timeline unknown). Refresh tokens expire 25 days after authorisation, which means periodic re-login prompts.
5. **Inara** requires app-name whitelisting by the site owner.
6. **Journal manual lag:** newer events need community schemas, and those can break with game updates.
7. **Plotting to stations or bodies** isn't possible through galaxy-map search; it needs OCR or system-map navigation, which is fragile.
8. **Elevated game / anti-cheat interactions** with `SendInput` work today, but could change.
9. **VR:** there's no maintained Rust OpenVR overlay crate. The COVAS electron-vr package is proprietary, and Oculus-runtime users depend on OpenComposite.
10. **Cloud voice cost and latency:** GPT-Live is $0.05/min (about $3/hour plus backend calls). Long play sessions argue for an always-available local or offline mode and push-to-talk or wake word, to avoid paying for idle minutes (verify billing for idle connected sessions).
11. **Unverified items flagged in the text:**
    - pasting into the galaxy-map search box,
    - vJoy input without focus,
    - gpt-realtime-2.1 details,
    - Canonn endpoints,
    - Spansh R2R and exobiology router paths.

---

## Appendix A: key sources

- Journal Manual v38: https://hosting.zaonce.net/community/journal/v38/Journal_Manual_v38.pdf
- fd-api (cAPI/OAuth docs): https://github.com/Athanasius/fd-api
- EDMC: https://github.com/EDCD/EDMarketConnector
- EDDN: https://github.com/EDCD/EDDN
- Spansh API docs: https://docs.spansh.co.uk/; dumps: https://downloads.spansh.co.uk/
- EDSM: https://www.edsm.net/en/api-v1 (archived copy used)
- Inara API dev guide (archived): https://web.archive.org/web/20260308173440/https://inara.cz/elite/inara-api-devguide/
- Ardent API: https://github.com/iaincollins/ardent-api
- EDDB closure: https://forums.frontier.co.uk/threads/moment-of-silence-for-eddb-io.615957/
- EDCoPilot: https://www.razzafrag.com/featurelist, https://razzserver.com/dokuwiki/doku.php?id=start, EULA https://www.razzafrag.com/eula
- EDMesg: https://github.com/RatherRude/EDMesg
- COVAS:NEXT: https://github.com/RatherRude/Elite-Dangerous-AI-Integration
- EliteIntel: https://github.com/SudoKrondor/EliteIntel (formerly stone-alex)
- EliteAPI: https://github.com/Somfic/EliteAPI
- bindED: https://github.com/Guntereno/bindED (fork of alterNERDtive/bindED)
- OD-Explorer: https://github.com/WarmedxMints/OD-Explorer
- EDDI: https://github.com/EDCD/EDDI
- KeybindsBackup: https://www.spacejock.com/KeybindsBackup.html
- Frontier ED EULA: https://www.frontier.co.uk/legal/elite-dangerous/eula
- OpenKneeboard compatibility: https://openkneeboard.com/compatibility/games/
- vJoy fork: https://github.com/BrunnerInnovation/vJoy; ViGEm end of life: https://docs.nefarius.at/projects/ViGEm/End-of-Life/
- GPT-Live-1: https://developers.openai.com/api/docs/models/gpt-live-1
- Gemini 3.8 Live: https://ai.google.dev/gemini-api/docs/models/gemini-3.8-live
- xAI voice: https://docs.x.ai/developers/model-capabilities/audio/speech-to-speech
- Parakeet: https://huggingface.co/nvidia/parakeet-unified-en-0.6b
- sherpa-onnx: https://k2-fsa.github.io/sherpa/onnx/
- Rust journal library: https://github.com/rster2002/ed-journals; TypeScript journal library: https://github.com/kayahr/ed-journal; Tauri example: https://github.com/Mallock/edmo
