---
schema: atelier.item
schema_version: 3
id: astromech-2dai
title: 'Browser tests in two worktrees at once fail to start: vitest holds port 63315'
status: approved
mission: null
labels:
- size-s
- tooling
priority: high
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A40e7bd6a-75d7-481f-bc07-e18b62a688e1%3Adelegate-task%3A7af34a08-e960-4a02-98c2-b0dd1dd1ba67
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T05:23:28.198951236Z
trivial: null
caused_by: null
created_at: 2026-11-17T05:20:19.125189673Z
---

## Goal

Two agents can run `pnpm test` in two worktrees at the same time. Today the second run's browser tests fail to start because Vitest's browser server takes port 63315 in the first; three implementers hit it on 12 Nov 2026 (astromech-drnv, astromech-v4bw, astromech-irrv) and each worked around it by hand.

## Done when

- [x] `pnpm test` passes while another process holds port 63315 (the state a second worktree's run leaves the first in), with no flag or config outside the repository; shown by one run with the port held by a plain listener, not by two full runs at once, which exceeds an agent's 4 GB memory limit on this machine.
- [x] The fix is at the cause (the fixed port in the browser test setup, `app/vite.config.ts` or what it loads), not a retry; CONTRIBUTING.md's Tests says nothing about ports because nothing needs saying.

## Scope

Test configuration only.

## Report

Browser tests now start when another worktree's run holds port 63315: `pnpm test` passed (84 files, 627 tests) with a plain Node listener on `127.0.0.1:63315`, and the port was free again afterwards.

The cause was in `app/vite.config.ts`. The root config sets `server: { port: 5173, strictPort: true }` for `tauri dev`, and the browser project inherits it through `extends: true`. Vitest 5's browser server calls Vite's `listen(63315)`, which honours `strictPort` and fails with "Port 63315 is already in use" instead of trying 63316. The browser project now sets `server: { strictPort: false }`, so Vite moves to the next free port. Vitest has no port retry of its own, so this is the fix at the cause, not a retry. `tauri dev` still gets 5173 strictly, and CONTRIBUTING.md needs nothing about ports.

Before the fix, `pnpm vitest run --project browser` with the listener on 63315 failed at once with that error and ran no tests. After it, the same run passed, and so did the full `pnpm test`. `eslint vite.config.ts` and `pnpm check` are clean.

The first Done when line changed during the work. It asked for `pnpm test` in two worktrees at once. My attempt at that ran out of memory: each agent thread here has a 4 GB memory cgroup, and two full runs went over it (the kernel log showed 4.5 GB). The lead changed the line to what one agent can prove: a single run while a plain listener holds 63315, which is the state a second worktree's run leaves the first in.

