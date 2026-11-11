---
schema: atelier.item
schema_version: 3
id: astromech-k5s5
title: 'Play check at Sumeet''s PC: the one app next to Elite, and his install updating to it'
status: ready
mission: null
labels:
- needs-human
- loose_end
priority: medium
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-11-08T15:51:58.268871937Z
trivial: null
caused_by: null
created_at: 2026-11-08T15:51:58.268871937Z
---

## Goal

Confirm on Sumeet's PC, next to Elite, what Windows CI stands in for: the one app (astromech-y6d6) plays as before, and his two-process install updates to it. Needs his PC and the game, so no agent can do it.

## Done when

- [ ] From his two-process install, Settings → Install and restart brings back one `astromech-core.exe`. His Start-menu and pinned shortcuts open the window, and at the next login Start with Windows shows the tray only (astromech-9lh3; stand-in: the `upgrade` job).
- [ ] At login, Start with Windows shows the tray and no window. Left-clicking the tray icon opens the window. Pause voice and the kill switch work from the tray menu while Elite runs (astromech-v3u4, S11).
- [ ] Push-to-talk and the kill switch work with the window open, and after it is closed.
- [ ] Opening and closing the window in play causes no hitch he can see. Quit releases everything.

## Context

Stand-ins on Windows CI at 916c97c3: the onboarding smoke's kill-switch runs, App window cost (receipt astromech-osyy) and the upgrade job from v1.0.0-alpha.4. Do it after v1.0.0-alpha.5 is published.
