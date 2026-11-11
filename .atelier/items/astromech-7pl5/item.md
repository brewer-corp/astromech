---
schema: atelier.item
schema_version: 3
id: astromech-7pl5
title: 'Upgrade test between one-process releases: the newest release upgraded to this build by its own helper and setup.exe /S'
status: ready
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-07T19:48:33.637663433Z
trivial: null
caused_by: null
created_at: 2026-11-07T19:48:33.637663433Z
---

## Goal

A regression in upgrading from one one-process release to the next is caught on Windows CI before it ships. The Release workflow's `upgrade` job is pinned to v1.0.0-alpha.4, the last two-process release (finding astromech-s3rq): it starts the old core and the old window (`astromech-shell.exe`), which a one-process release has not got. Once a one-process release is published, nothing tests an upgrade from it.

## Done when

- [ ] A job in .github/workflows/release.yml (or a route of scripts/upgrade_smoke.py) installs the newest published `v*` release that is one-process, starts it as a player has it (`astromech-core.exe --background`, then a second start opening the window), and upgrades it to this build by both routes: that release's own helper (copied to `%TEMP%\astromech-apply-<pid>.exe`, `--relaunch --route settings`) over this build's zip, and this build's `setup.exe /S` over the running app.
- [ ] After each route, within 30 s: the old app quit cleanly (it logged `quitting` and removed core.json), exactly one `astromech-core.exe` of this build runs, the updater route's window is open at Settings (`ui_state` `page`), the Run value is unchanged, and `astromech-shell.exe` opens the window in the running app.
- [ ] Until a one-process release exists the job is skipped with a message naming why, and it picks that release by itself once one does (no edit at each release).
- [ ] `publish` waits for it, as it does for the two-process `upgrade` job.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from review: astromech-s3rq
