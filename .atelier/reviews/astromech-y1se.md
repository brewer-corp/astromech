---
schema: atelier.review
schema_version: 2
id: astromech-y1se
title: review on astromech-bp23 @791ff421
target: astromech-bp23
identity: claude-reviewer-se44
commit: 791ff421536f6dbcc4eac9a98c4a0ad985c55af0
created_at: 2026-11-07T09:14:42.712740854Z
event:
  event: review
---

Review of astromech-se44 (13fb58a1, 960792f2, ddbf1145, 173e3184; 791ff421 is a planning note) against its Done when and decision note, bp23, unh9/1zhg and the one-tested-process Authority section. Read bindings.rs, reply.rs, the Registry changes, Core::commands, status, route_set/route_add, setup::args, biology/hub/calibration typed replies; diffed the 22 domain crates (only derive/ts attributes, no serde change); spot-checked CoreStatus, Biology, Mining, Snapshot, SamplingView, KnownView, Clearance, Toast, Inventory/Cell/FarmStep against app/src/lib/core.ts, exobiology.ts, materials.ts, engineering.ts (shapes match; serde(default) Vec skips are optional; one Option skip without ts(optional) is SetFoldersArgs, covered by optional_fields). Fixture recorded at 960792f2 and unchanged since. status: each lock taken and dropped before the struct literal; active() and setup_needed() take their own; no guard held across status_line; no deadlock seen. Route arg changes reach only malformed input the window never sends; no in-game behaviour change. No json! reply left outside cfg(test) in the core's six files, hub or astromech-api besides the control envelope. Fresh: cargo test --workspace 1299 passed 0 failed 2 ignored; clippy -D warnings, fmt, check-deps (32), check-guide, cargo deny (ts-rs 12.0.1 MIT, new crates ts-rs, ts-rs-macros, termcolor) clean; pnpm check 0 errors (tsconfig includes src/lib/api/types), lint clean, test 528 passed. In a scratch worktree: drift test fails on a hand-edited type, a stale file and an edited commands.ts line; JsonValue test fails on Vec<HashMap<String, Option<Value>>> in save_diagnostics' reply. core_replies_keep_their_json fails with another HOME. Not approved: one fix_now (fixture host-specific). Not run: Windows CI, the shell's window build.
