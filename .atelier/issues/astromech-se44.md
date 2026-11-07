---
created_at: "2026-11-07T06:33:38.680504277+00:00"
id: "astromech-se44"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Export every command's request and reply types to TypeScript (ts-rs)"
updated_at: "2026-11-07T06:33:38.680504277+00:00"
---

## Goal

Every registered command's request and reply types are exported to TypeScript from Rust, so the window and its fake can be typed against the real core's shapes. Today a fixture can drift from the core unseen.

## Done when

- [ ] Types are derived with `ts-rs` on every astromech-api handler's args and reply, the domain types they return, and the core handlers' args and replies.
- [ ] A registry test writes the files:
  - ts-rs's types go to `app/src/lib/api/types/`;
  - `app/src/lib/api/commands.ts` maps each command name to `{ args, reply, exposure, progress? }`, using the generated type names.

  `cargo test -p astromech-api` fails when the checked-in files differ from what it would write. CONTRIBUTING.md names the command that updates them.
- [ ] The core commands whose replies are built with `json!` reply with typed structs of the same shape. They are in crates/astromech-core/src/app.rs, calibration.rs, callouts.rs, learned.rs, biology.rs and setup.rs.
  - The shapes test `setup_and_calibration_reply_shapes_match_the_window_fake` and the existing control-API tests pass unchanged.
  - No reply's JSON changes: a test compares a few before and after.
- [ ] `serde_json::Value`, which ts-rs types as `JsonValue`, is used only for args or replies named in an allow-list in the registry test with a reason each. The test fails for any other. An expected entry is `set_search_filters`' empty reply only if it can't be `()`. The Close Reason lists the final allow-list.
- [ ] Generated files and the TypeScript check:
  - `cd app && pnpm -s check` passes with the generated files included;
  - `pnpm -s lint` passes, with `commands.ts` linted and `app/src/lib/api/types/` (ts-rs output, not hand-written) excluded in the eslint config, named there;
  - `cargo deny check` passes with the new dependency;
  - `app/src/lib/api/client.ts`, the unused WebSocket placeholder, is deleted.

## Scope

Rust, the generated `app/src/lib/api/` files and the eslint ignore entry. The window starts using them in the typed child.

There are two error paths, and that is accepted. A failed call is the envelope's `{"ok": false, "error"}`. A core command that reports a failure inside its reply (`{"ok": false, "error": …}` within `value`) keeps doing so, typed as such. Folding those into the envelope would rewrite the `.ok` checks in many window modules that astromech-xil2 has just moved. It needs no Rust-side design and can be done later as its own item if wanted.

## Context

ts-rs over tauri-specta: once the shell forwards everything through one proxy, commands aren't Tauri commands. The same types must serve the control API, the headless core and later WASM. ARCHITECTURE.md:421 marks specta 2 as a pre-release. The moved handlers return typed structs already (the two move children).
