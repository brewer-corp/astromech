---
schema: atelier.item
schema_version: 3
id: astromech-3340
title: Check the text size and the new colours in the installed app on Windows
status: ready
mission: null
labels:
- needs-human
- loose_end
priority: medium
blocked_by:
- astromech-b2cz
- astromech-3kf5
implementers: []
ready:
  by: conversion
  at: 2026-11-10T05:55:46.920933807Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:46.920933807Z
---

## Goal

Text size (astromech-b2cz) and the calmer colours (astromech-3kf5) were built and checked in Chrome on Linux. WebView2 on Sumeet's monitor is the real target: the step a first run picks from the display, how the six steps read from the seat next to the game, and whether Windows display scaling adds to it as expected. The height zoom this item first checked (astromech-k6pl) is gone.

## Done when

- [ ] On Sumeet's PC, in a release that has astromech-b2cz and astromech-3kf5: the step the first run picked (or migrated to) is comfortably readable from his seat; the six Text size steps change it and resizing or maximising the window does not; Windows display scaling adds to it as expected; nothing is clipped at the smallest window; and the pages read calm, with orange where it matters.
