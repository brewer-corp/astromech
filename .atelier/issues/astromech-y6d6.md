---
created_at: "2026-11-07T04:51:23.056349964+00:00"
id: "astromech-y6d6"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-01ft"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-bp23"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "One process: the core and the window in one Tauri app"
updated_at: "2026-11-07T04:51:23.056349964+00:00"
---

## Goal

astromech ships as one Tauri process holding the tray, pipeline, voice, input and the window (decision astromech-6i8s), with the second executable and the start-the-core plumbing gone. The speech worker stays a separate supervised process.

## Done when

- [ ] A spike, first, shows the tray working with no window and through a window crash, and that destroying the window ends WebView2's processes on Windows.
- [ ] Start with Windows creates no window; closing the window destroys it; a second start focuses or opens it at `--route`.
- [ ] Crash restart works without the shell (Windows restart registration or a minimal watchdog), shown by a test that kills the process.
- [ ] Installer, updater and Start with Windows move to the one executable; the release build and its smoke run pass.
- [ ] An existing two-process install upgrades cleanly: the old Start with Windows entry, a tray core already running, the updater (crates/astromech-core/src/update.rs), the single-instance mutex and the log files; shown on Windows CI from an install of the last release.
- [ ] `scripts/webview_cost.py` and `.github/workflows/webview-cost.yml` measure the one-process criterion (app process idle with no window against today's core alone; the window's rise plus WebView2 processes with it open), and the job passes.
- [ ] ARCHITECTURE.md (§5.10, §18) and docs/V1-DESIGN.md §7.7 describe one process and link astromech-6i8s; docs/guide pages that mention the background process match; `scripts/stage-sidecars.sh`, `externalBin` and the smoke step in .github/workflows/release.yml fit the new layout.
- [ ] After merge (Sumeet's PC, with Elite running): `scripts/webview_cost.py` shows the one-process criterion's bounds hold next to the game. Stand-in: the webview-cost job on Windows CI.

## Unknowns

- Whether tao's event loop can own the tray with no window on every path the core uses today (tray.rs).
- What the windowless Tauri process costs at idle.
- Whether `core.json` (the control API's endpoint file) stays: the one-command-list test, a second start, scripts/onboarding_smoke.py, scripts/webview_cost.py and the dev bridge all find the control API through it today.
