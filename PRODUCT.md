# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

Elite Dangerous commanders on Windows, on the same PC as the game.

Two jobs:

1. Hands-on-sticks push-to-talk voice, plus keybinds that are searchable and honest about problems.
2. Looking things up in the window between jumps: engineering, galaxy, trade, exploration, Powerplay, logbook.

Success is a spoken ship toggle that does what was asked, and a window that is usable between jumps without becoming the thing they fly from.

## Product Purpose

astromech is a local-first unofficial copilot for Elite Dangerous. Hold a button, say "gear down", and the landing gear goes down. Ask where to sell cargo and it answers out loud. While they play it speaks up about what the HUD does not show. A window next to the game holds keybinds, ships, materials, trade, and the logbook.

It exists because Elite's Controls list is miserable to search, and because commanders need ship toggles and lookups without taking their hands off the sticks.

## Positioning

Local-first grammar push-to-talk with no LLM required. An LLM, if it ever appears, is opt-in and never the default. Combined with that: a binds doctor that can hear.

That is the inverse of LLM ship-AIs that also parse `.binds`. astromech is an unofficial fan project, not a personality and not a competing ship AI.

## Operating Context

Windows, same machine as Elite Dangerous. astromech lives in the tray, starts with Windows unless turned off, and waits until Elite starts. The first run finds Elite, picks push-to-talk and a microphone, and downloads speech models (~800 MB).

The window is for setup and for looking things up between jumps. While they fly, voice does the work and the window can stay closed. In-flight UI is voice plus tray. There is no VR overlay or in-headset UI in v1.

Header areas: Controls, Engineering, Galaxy, Trade, Exploration, Powerplay, Logbook, Settings. Default window route is My controller. Ctrl+K jumps to a page, material, ship, blueprint, or module by name. System names copy to the clipboard for pasting into Elite's galaxy map; astromech does not type into the game.

Journal-backed pages refresh every few seconds. Voice and history stay on the PC. Network is used for lookups (Spansh, Ardent Insight, EDSM), updates, the first speech-model download, and, only if turned on, EDDN/EDSM sharing.

## Capabilities and Constraints

**Voice.** Push-to-talk ship controls (gear, lights, hardpoints, cargo scoop, flight assist, panels, and more), checked against what the game reports. Spoken answers about fuel, cargo, location, and the like. Recognition and TTS run on the PC (Parakeet + Kokoro via sherpa-onnx). Callouts cover what the HUD leaves out. Policy and a kill switch sit between intent and keypress. Actions go through keyboard binds via SendInput; a keyboard secondary bind remains required.

**Keybinds.** Maker's controller sheet (VKB sheets bundled with credit), the game's Controls list made searchable, health checks, press-to-bind, presets with backups. The only file write in v1 is a copy of a `.binds` file, always backed up first.

**Lookups in the window.** Fleet (one selected ship with computed specs and a build planner; the rest of the fleet as a rail), stored modules, materials, engineer unlocks as a tech tree, Odyssey blueprints, nearest services/modules/ships, commodity prices and trade routes, the system you are in, Powerplay, colonisation, mining, logbook. Ship specs come from the same Coriolis-based calculations as the planner.

**Constraints (binding).**

- English only.
- Unofficial Frontier fan project; not affiliated with or endorsed by Frontier Developments. No Frontier art, audio, fonts, copies of game files, or datamined data.
- Unsigned per-user Windows install (`%LOCALAPPDATA%\astromech`). SmartScreen on first run is expected.
- Stock Elite orange only. No colour or theme customisation, and no following the in-game HUD colour.
- Read-only by default: a copy of `.binds`, and SendInput only for voice actions the commander asked for.
- Kill switch and policy are product ground.
- No VR UI in v1. Voice still has to work while they fly in VR.
- Source is MIT; installed binaries that include the sherpa voice worker are GPL-3.0 as a whole.

Alpha: the product works day to day and is still changing.

## Brand Commitments

Name: **astromech**. Voice-first copilot, not a ship personality.

Spoken replies sit in the same register as Elite's COVAS: impersonal present tense, no contractions, no jokes, no "I". Default colour Cockpit. Recreate the register; do not clone Frontier samples or actors.

UI stays in the Elite Controls dialect: the game's own Controls screen as the visual register. That dialect is binding; it is not a licence to invent a new colour world.

Ship hulls in the window are CMDR Qohen Leth's cartoon vectors from EDAssets (CC BY-NC-SA 4.0), credited in NOTICE.md. They stand in for the hull because Frontier 3D and game screenshots are off limits. Engineer access uses CMDR Robert Phillips' engineer mark from EDAssets, one mark per rank as the game does.

VKB Gladiator reference sheets are used as VKB drew them, with credit.

## Evidence on Hand

- Product copy and install path: `README.md`, `NOTICE.md`, `docs/guide/`.
- v1 product decisions: `docs/V1-DESIGN.md`.
- Competitive landscape (planning only): `docs/COMPETITORS.md`.
- Architecture: `docs/ARCHITECTURE.md`.
- Shipped and in-progress UI: `app/src/` (Tauri v2 + Svelte 5 window; Rust core in the tray).
- Guide screenshots and design evidence: `docs/guide/`, `docs/design/`, `docs/evidence/`.
- Ship cartoons: `app/public/ships/`. Engineer access mark: `app/public/engineers/`. Credits: `NOTICE.md`.
- VKB sheets: `data/devices/vkb/`.
- Journal and search fixtures under `crates/*/tests` and `app/src/lib`.

Do not invent testimonials, commander quotes, benchmarks, pricing, or affiliation with Frontier.

## Product Principles

1. Hands stay on the sticks. Anything needed in flight works by voice or a bound button.
2. Local and grammar-first. The copilot must work with no cloud model and no LLM.
3. Honest about binds and about the game. Health marks, Status.json checks, and copy that does not pretend astromech typed into Elite.
4. Small and solid. Two headline jobs done well beat a panel zoo or a personality.
5. Stay unofficial and stay in Elite's own register. Stock orange, Controls dialect, English, no Frontier assets.

## Accessibility & Inclusion

English only in v1. No other product-specific accessibility standard has been set.
