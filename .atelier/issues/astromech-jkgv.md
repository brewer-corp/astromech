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
  relates:
  - kind: "issue"
    id: "astromech-xk0n"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Dev bridge from pnpm dev to the headless core, and preview fixtures dumped from it"
updated_at: "2026-11-07T11:57:01.792266735+00:00"
---

## Goal

The browser can show the real core's output. `pnpm dev` with a world passes the page's `call()` to a headless core through a Vite middleware, and the preview fixtures the core can produce are dumped from it, with a test that fails when they drift. Today, outside Tauri, every call goes to the hand-written fake (app/src/lib/transport.ts, `previewTransport`), and most of app/src/preview/fixtures/ has no producer.

## Done when

- [ ] A Vite plugin in app/dev/ (outside src/, so no bundle carries it) works with `ASTROMECH_WORLD=<dir> pnpm dev`. It starts the headless core binary (`ASTROMECH_CORE`, default target/debug/astromech-core) on that world. Its only endpoint source is the JSON line the child prints on stdout (port and token); it doesn't read `core.json`. It serves `POST /__astromech/call`, which sends the call with `origin: "app_window"` and streams the progress lines and then the reply.
- [ ] transport.ts picks a bridge transport when the plugin is on. Its progress is published on the command's name, as the Tauri transport does. `pnpm build` drops it, and `scripts/check-dist.py` fails if a bridge string is in app/dist.
- [ ] The control client the plugin uses is a small Node module that astromech-yis2's driver can import to spawn a world core, read its stdout line, and call `world_step` and `world_wait`.
- [ ] A check in CI's Linux job fails if the bridge doesn't return the real core's reply: start the plugin on `worlds/minimal`, call `status`, and compare its `status_line` with the core's own. The author also runs `scripts/ui --url http://127.0.0.1:5173/ --wait <the world's commander> --text` against a world with a commander, and records the receipt.
- [ ] Each JSON file in app/src/preview/fixtures/ is in a table in a Rust test (crates/astromech-core/tests/). The table gives either the command and args the headless core answers it with over `worlds/preview` (synthetic journals and recorded network), or why the core can't produce it, for example static catalogue data a generator script writes. The test compares the core's reply, with volatile fields masked, against the file. `ASTROMECH_UPDATE_FIXTURES=1` rewrites them. A hand edit to a dumped fixture fails it. The hand-written ones the core can produce are replaced by dumps, and `pnpm check` and `pnpm test` still pass.
- [ ] CONTRIBUTING.md documents both loops: `ASTROMECH_WORLD=... pnpm dev` for the real core in the browser, and `tauri dev` at the PC for the real window.

## Scope

The plugin, the bridge transport, the Node client, the fixture table and test, `worlds/preview`, and the docs. The shell's own push topics (`input` from its raw-input listener, `route`, `core-restarted`; app/src-tauri/src/window.rs:73, :96-140, :184) are not served through the bridge. Say so in the transport's doc comment. The preview fake stays for UI behaviour tests and the static preview link (strategy, Outcome). Its inline scenario replies in fakecore.ts are out of scope, and the core-reply-shapes contract test keeps covering them.

## Context

The existing dumpers (crates/astromech-galaxy/examples/dump-local-route.rs, dump-sites.rs, crates/astromech-market/examples/dump-trade.rs, crates/astromech-odyssey/examples/dump-odyssey-fixture.rs, crates/astromech-ship/examples/dump-ships-fixture.rs, crates/astromech-keybinds/examples/fixture.rs) dump once and have no drift check. The pattern to copy is `core_replies_keep_their_json` (crates/astromech-core/src/app.rs:2592) and the bindings update flag (CONTRIBUTING.md, `ASTROMECH_UPDATE_BINDINGS`). Journals in worlds are synthetic, and never personal logs (AGENTS.md, Constraints).
