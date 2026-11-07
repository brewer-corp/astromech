---
created_at: "2026-11-07T11:57:01.792266735+00:00"
id: "astromech-jkgv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Dev bridge from pnpm dev to the headless core, and preview fixtures dumped from it"
updated_at: "2026-11-07T20:46:26.217983037+00:00"
---

## Goal

The browser can show the real core's output. `pnpm dev` with a world passes the page's `call()` to a headless core through a Vite middleware, and the preview fixtures the core can produce are dumped from it, with a test that fails when they drift. Today, outside Tauri, every call goes to the hand-written fake (app/src/lib/transport.ts, `previewTransport`), and most of app/src/preview/fixtures/ has no producer.

## Done when

- [x] A Vite plugin in app/dev/ (outside src/, so no bundle carries it) works with `ASTROMECH_WORLD=<dir> pnpm dev`. It starts the headless core binary (`ASTROMECH_CORE`, default target/debug/astromech-core) on that world. Its only endpoint source is the JSON line the child prints on stdout (port and token); it doesn't read `core.json`. It serves `POST /__astromech/call`, which sends the call with `origin: "app_window"` and streams the progress lines and then the reply.
- [x] transport.ts picks a bridge transport when the plugin is on. Its progress is published on the command's name, as the Tauri transport does. `pnpm build` drops it, and `scripts/check-dist.py` fails if a bridge string is in app/dist.
- [x] The control client the plugin uses is a small Node module that astromech-yis2's driver can import to spawn a world core, read its stdout line, and call `world_step` and `world_wait`.
- [x] A check in CI's Linux job fails if the bridge doesn't return the real core's reply: start the plugin on `worlds/minimal`, call `status`, and compare its `status_line` with the core's own. The author also runs `scripts/ui --url http://127.0.0.1:5173/ --wait <the world's commander> --text` against a world with a commander, and records the receipt.
- [x] Each JSON file in app/src/preview/fixtures/ is in a table in a Rust test (crates/astromech-core/tests/). The table gives either the command and args the headless core answers it with over `worlds/preview` (synthetic journals and recorded network), or why the core can't produce it, for example static catalogue data a generator script writes. The test compares the core's reply, with volatile fields masked, against the file. `ASTROMECH_UPDATE_FIXTURES=1` rewrites them. A hand edit to a dumped fixture fails it. The hand-written ones the core can produce are replaced by dumps, and `pnpm check` and `pnpm test` still pass.
- [x] CONTRIBUTING.md documents both loops: `ASTROMECH_WORLD=... pnpm dev` for the real core in the browser, and `tauri dev` at the PC for the real window.

## Scope

The plugin, the bridge transport, the Node client, the fixture table and test, `worlds/preview`, and the docs. The shell's own push topics (`input` from its raw-input listener, `route`, `core-restarted`; app/src-tauri/src/window.rs:73, :96-140, :184) are not served through the bridge. Say so in the transport's doc comment. The preview fake stays for UI behaviour tests and the static preview link (strategy, Outcome). Its inline scenario replies in fakecore.ts are out of scope, and the core-reply-shapes contract test keeps covering them.

## Context

The existing dumpers (crates/astromech-galaxy/examples/dump-local-route.rs, dump-sites.rs, crates/astromech-market/examples/dump-trade.rs, crates/astromech-odyssey/examples/dump-odyssey-fixture.rs, crates/astromech-ship/examples/dump-ships-fixture.rs, crates/astromech-keybinds/examples/fixture.rs) dump once and have no drift check. The pattern to copy is `core_replies_keep_their_json` (crates/astromech-core/src/app.rs:2592) and the bindings update flag (CONTRIBUTING.md, `ASTROMECH_UPDATE_BINDINGS`). Journals in worlds are synthetic, and never personal logs (AGENTS.md, Constraints).

## Notes

- 2026-11-07 [note] From finding astromech-1uyn (d2nq review): world mode quits when its stdin closes, so the Vite plugin must pipe the world core's stdin and hold it open for the dev server's life; state it in this child's Done when. Optional: give world_mode.rs's kill test a hang guard.
- 2026-11-07 [note] From finding astromech-lwic (1hh9 review): the world 'focus' step and gui/vehicle reactions are only covered by a script-load test; add a voice world case where a key is refused after Elite loses focus, and one gui reaction.
- 2026-11-07 [decision] The bridge is chosen by a value the plugin defines (import.meta.env.VITE_ASTROMECH_BRIDGE, the call's path), behind import.meta.env.DEV; transport.ts loads lib/bridge.ts with a dynamic import inside that branch, and main.ts skips loading the preview when bridged()
  Reason: A define exists only when the dev server applies the plugin, so tests, preview builds and pnpm build see no bridge, and Rollup drops the dead branch and its chunk (check-dist proves it, and fails with the guard removed). lib/bridge.ts imports nothing from the app, so transport.ts may import it without the cycle the preview has
  Rejected: main.ts registering the bridge as it registers the preview (usePreview): the spec asks transport.ts to pick it, and the bridge has no import cycle to avoid
  Rejected: A runtime probe of /__astromech/call: every page load would make a request, and a release build would carry the probe
- 2026-11-07 [decision] The Node control client and the plugin are TypeScript with erasable syntax only (app/dev/*.ts, imports with .ts), run by node's type stripping; @types/node is a new dev dependency, referenced only from app/dev files
  Reason: yis2's driver and scripts import it with plain node (Node 22.18+ strips types by default; CI's setup-node 22 resolves to that), Vite's config bundles it, and svelte-check typechecks it with the app; the app's own sources keep only vite/client types
  Rejected: Plain .mjs with JSDoc: no typecheck without allowJs and checkJs across the app
- 2026-11-07 [decision] CI's bridge check is a pnpm script (pnpm bridge-check, app/dev/bridge-check.ts) run in the Linux job after cargo test built the core, not a Vitest test; it compares status_line with a second world core's and checks the config path is a world core's temp copy
  Reason: It needs the built binary, which pnpm test must not need on a frontend-only checkout; and status_line alone ('Waiting for Elite') is what the preview's fake says too, so the path check is what tells a real core's reply
- 2026-11-07 [decision] The client gives each world core an empty home (every HOME/XDG/LOCALAPPDATA/APPDATA/USERPROFILE variable) and clears the proxy variables, as the Rust tests' helper does; the homes are removed on exit, Ctrl+C or a kill
  Reason: Defence in depth for the dev bridge and yis2's driver: world mode already keeps to its temp folder, and quit() checks the home is still empty
- 2026-11-07 [decision] worlds/preview is a folder of small worlds (history, odyssey, materials, engineering, krait, station, binds, voice, sol, colonies, powerplay, colonisation, explorer, market, mining, unlocks), one per sample's synthetic commander, not one game folder
  Reason: The samples come from different synthetic commanders and places (CMDR Fixture's history, the engineering fleet, Test Cmdr docked at Test Orbital, a commander at Sol, Lembava, Greae Phoea, LHS 2435). Probed as one game folder, they merged into a different commander: the logbook, fleet, docked station and plot ship all changed. Small worlds reproduce each sample from its own inputs, and yis2 can reuse them
  Rejected: One worlds/preview game folder with marks: history and fleet accumulate across marks, so no order reproduces every sample
- 2026-11-07 [decision] Lookup times read the remote-data client's clock: lookups.rs now(&State) at its 6 sites, Galaxy::now_s() in ensure_covered, nearby, sites, stations, colonies.rs:221 and powerplay.rs:120
  Reason: Age filters and offer ages read the wall clock, so a dump of recorded October data would drift day by day. The client's clock is SystemClock in the installed app, which is unchanged, and the world's manual clock in world mode. zgoo still owns the clippy rule and the other reads

## Report

All six Done when lines hold on astromech-vucf (commits d3cf4d9b..ce1e1b6d plus the record commit). Each line and its proof:
- Line 1, the plugin: `ASTROMECH_WORLD=../worlds/<w> pnpm dev` starts the core from app/dev/bridge.ts and serves `POST /__astromech/call` as NDJSON. Live runs on worlds/minimal, every-command and preview/history. The endpoint comes only from the stdout line (app/dev/core.ts `startWorld`). Finding 1uyn: stdin is a pipe held for the server's life. After `kill -9` of vite's node process, the world core exited and its temp world was gone.
- Line 2, the bridge transport: transport.ts picks lib/bridge.ts under `import.meta.env.DEV && VITE_ASTROMECH_BRIDGE`. `src/lib/bridge.test.ts` (3 tests) shows progress published on the command's name, the core's error as the rejection, and a truncated stream failing. Mutation: progress published on another topic fails it. `pnpm build` plus check-dist prints "no browser preview or dev bridge". Mutation: with the DEV guard removed and the env set at build, check-dist fails on '/__astromech/call' and 'the dev bridge closed before'.
- Line 3, the Node client: app/dev/core.ts. It runs under plain `node` and has `startWorld`, `callWindow`, `step`, `wait`, `observed` and `quit`. A smoke script imported it and called status (ok), `world_observed` from the window (refused) and quit (exit 0).
- Line 4, the CI check: `pnpm bridge-check`, a step in ci.yml's Linux job, passes. It compares status_line with a second world core's and checks the config path is a world temp copy. Mutations: a fake reply fails with "not a world core's", and a wrong command fails. Receipt: `scripts/ui --url http://127.0.0.1:5173/#/logbook --wait Fixture --text` on worlds/preview/history gave ok:true and "CMDR Fixture · 4 systems · 3 stations · 6 arrivals · 4 dockings", with the real core's places and timeline. This ran after `--click-text "SKIP SETUP"` at /: the root opens onboarding for a fresh world, and the home page is the controller page. Finding igbw: the bridge refuses with 403 any call not from its own page (Host must be the dev server's loopback name and port, the body application/json, and Sec-Fetch-Site same-origin or the Origin the server's own). dev/bridge.test.ts (6 tests) covers a foreign text/plain POST, a foreign-origin JSON POST, a cross-site one, one with no origin and a rebound Host, all refused before reaching the core, and a same-origin call answered. With the check removed, 5 of the 6 fail. bridge-check also expects a foreign call to get 403. Live, the reviewer's text/plain POST from https://evil.example and a foreign-origin set_folders both got 403, and the page still showed the logbook through the bridge.
- Line 5, the fixtures table: crates/astromech-core/tests/preview_fixtures.rs covers all 28 files. 24 are dumped whole from the core. Three are dumped except one hand-written part, each with its reason in the table: trade /route/loop (no recorded Spansh loop answer), plot /local (needs the neutron star file) and exobiology /known (other players' reports: Spansh's known-biology search is recorded only for Luhman 16; the rest of exobiology.json is the exobiology world's reply, finding nk8x). One is hand-written whole: eddn-log (the EDDN outbox stamps and retries on the wall clock). nearby-hge, the black-market, 5A FSD and Krait MkII station searches and the Painite hotspots are dumped from Spansh pages recorded for the preview on 7 Nov 2026 (worlds/preview/sol-nov, whose config gives each page's provenance and trim; finding v0xh). Coverage is checked at every level a part's key descends into (finding ptge): the test `a_hand_added_entry_beside_the_parts_is_caught` passes, and a fourth headline hand-added to ships.json fails the drift test with `/headlines/99 has no part in the table`. Proof: a hand edit to logbook.json ("Fixtur") fails with `/commander/name: "Fixtur" ≠ "Fixture"`, and passes again once reverted. After the review fixes, the test passed 3 runs in a row (about 1.1 s each). `pnpm check` gives 0 errors and `pnpm test` 544 passed.
- Line 6, the docs: CONTRIBUTING.md "Running the app" covers both loops and the client. "Before a PR" covers the fixture refresh. ARCHITECTURE §18.2 gains a dev bridge bullet.
- Finding lwic: `world_voice::a_key_is_refused_once_elite_loses_focus_and_a_gui_reaction_opens_the_map` passes. With Notepad in front, "lights on" is refused with "Elite is not the active window. Nothing pressed." and no key is sent. "galaxy map" sends Key_M, and a gui reaction sets GuiFocus 6. Mutations: focus left on Elite fails the refusal wait, and the gui reaction removed fails at gui_focus.
- Workspace (Linux, after the last change): `cargo test --workspace --locked` 1372 passed, 0 failed. Clippy `-D warnings` and fmt are clean. check-deps and check-guide pass.

### Deviations
- worlds/preview is a folder of small worlds, one per sample (decision note).
- Lookup times now read the remote-data client's clock (decision note). This overlaps zgoo's "galaxy dates".
- The kill test got its optional hang guard (finding 1uyn).
- The world client's empty homes are removed on Ctrl+C and kill.

### Decisions
Six decision notes on this record: the bridge's selection, TypeScript under node type stripping with @types/node, bridge-check as a pnpm script, the client's isolated home, worlds/preview as many worlds, and the lookup clock.

### Surprises
- Several fixtures had already drifted from the core: links, material hints, the session shown, the keybinds problems, the fuel line, plot range and route.
- The UI tests that read those values were updated to the dumped data.
- The Spansh pages first copied into worlds/preview/sol turned out to be field-trimmed crate fixtures (station rows cut to the asked-for module, body rows to 16 of their 46 fields). They were replaced by pages recorded for the preview, about 1.6 MB in worlds/preview/sol-nov: a station row lists every module and ship it sells.

### Open items
- The old dumper examples (dump-odyssey-fixture, dump-ships-fixture, dump-trade, dump-sites, dump-local-route, astromech-engineering and astromech-materials dump-fixture, dump-unlocks, keybinds' fixture) now duplicate the drift test. Their docs still say they write app/src/preview/fixtures. Remove or re-point them in a follow-up.
- `scripts/ui --url http://127.0.0.1:5173/` on a fresh world shows onboarding, not the commander. The receipt used #/logbook after skipping setup.
- Windows CI hasn't run any of this.
- Review is outstanding.
