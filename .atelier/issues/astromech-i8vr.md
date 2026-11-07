---
created_at: "2026-11-07T06:33:38.679868984+00:00"
id: "astromech-i8vr"
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
    id: "astromech-7rcn"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-se44"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "One shell proxy for every window call; forwarders, core_call and core_status gone"
updated_at: "2026-11-07T09:21:40.548247277+00:00"
---

## Goal

The window reaches every command through one shell proxy, and the shell's per-command forwarders, `core_call` and `core_status` are gone. The shell then holds only the window, startup and its link to the core: the proxy, the core start, the watchdog and the input listener, which the single-process increment (astromech-y6d6) removes.

## Done when

- [ ] The shell registers one Tauri command, `call(cmd, args, progress)`. It forwards any command to the core with `origin: "app_window"` and no read timeout, resolves with `value`, rejects with `error`, and forwards progress lines to the channel. `generate_handler!` lists only it.
- [ ] The shell's direct dependencies are tauri, tauri-plugin-single-instance, serde, serde_json, anyhow, astromech-api (the control client), astromech-store (`Paths`), astromech-log, astromech-keybinds (the log `Redactor`, window.rs:1320) and astromech-input (the input listener, window.rs:1145-1195). It drops astromech-materials, -journal, -engineering, -ship, -data, -galaxy, -market, -colonisation, -history, -odyssey, -station, -powerplay and -exploration.
- [ ] `app/src/lib/transport.ts` (from astromech-xil2) sends every call through it. The `CORE_COMMANDS` split, the `text`-string special case if the wire dropped it, and the `CHANNELS` table go. A progress command's topic comes from the generated command map. `core_status` callers use `status`, with a failed call meaning the core isn't running, as `core_status` returning null means now. `powerplay_targets` sends `within_ly`.
- [ ] scripts/onboarding_smoke.py opens every top-level page of the installed app after the wizard (the modules in app/src/modules) and fails when a page shows "not allowed", "unknown command", an internal error or "the core is not running". It passes in the Release workflow on the PR. The core log it uploads shows the star map's open line (from the galaxy-move child) naming the installed `starmap.db` as the preload.
- [ ] ARCHITECTURE.md §18.2 describes the registry in astromech-api, the exposure levels and policy's role (exposure decision), JSON lines with progress lines on the wire, and ts-rs types. The process-model text stays for astromech-y6d6.
- [ ] `cd app && pnpm -s check && pnpm -s lint && pnpm -s test` and the Rust checks pass, and so does Windows CI's shell build.

## Scope

window.rs, app/src/lib/transport.ts and the few lib modules that named shell-only commands, scripts/onboarding_smoke.py, ARCHITECTURE §18.2. The fake transport keeps its handlers, typed in the next child.

## Notes

- 2026-11-07 [note] From finding astromech-tabr (ndzx review), must be fixed here before bp23 merges: shell_info is now answered by the core, so Diagnostics (app/src/modules/diagnostics/View.svelte:29,49-51) can no longer show 'window X, background Y' after a half-applied update, and with the core down it shows '…'. Give the window its own version (built into the bundle, e.g. a Vite define from the workspace version, since it ships inside the shell exe), or drop the comparison only if y6d6 makes it moot — keep it until then.
