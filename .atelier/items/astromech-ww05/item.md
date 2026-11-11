---
schema: atelier.item
schema_version: 3
id: astromech-ww05
title: Click Coriolis and EDSY links in the Windows app
status: ready
mission: astromech-7q0z
labels:
- needs-human
- size-s
- validation
priority: high
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T05:00:27.647524127Z
trivial: null
caused_by: null
created_at: 2026-10-16T19:13:12.726110806Z
---

## Goal

Confirm the Coriolis and EDSY links built from the current loadout survive the Windows shell launch (open_external -> explorer.exe).

## Done when

- [ ] In the Windows app, one Coriolis and one EDSY link from the Builds page open the right build in the default browser; the long URL, including the EDSY # fragment, arrives intact (evidence).

## Notes

- 2026-10-17 [note] Use a large engineered ship (~6k-character Coriolis/EDSY URLs) to check long links open through explorer.exe (review finding astromech-f2dv on astromech-qiey); then tighten the URL-length assertion in the qiey test to the limit that held.
