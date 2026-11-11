---
schema: atelier.item
schema_version: 3
id: astromech-mf7w
title: Offer the sustained-DPS charge-time fix to Coriolis
status: ready
mission: null
labels:
- needs-human
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: conversion
  at: 2026-10-17T16:28:50.729966932Z
trivial: null
caused_by: null
created_at: 2026-10-17T16:28:50.729966932Z
---

## Goal

Coriolis (github.com/EDCD/coriolis) shows sustained DPS above burst DPS for charged weapons; offer it the fix astromech makes in astromech-tgar.

## Done when

- [ ] A PR (or an issue, if the maintainers prefer) is open on EDCD/coriolis, linked here; or a note says why it wasn't sent.

## Context

Coriolis's `getSustainedFactor` in `Module.js` leaves the last burst's charge time out of the reload cycle (astromech-bnnh). An agent can prepare the change and a description with a worked example (rail guns on a Fer-de-Lance), but the PR goes out under Sumeet's GitHub account, so he approves and opens it. Check first whether an issue or PR already covers it upstream.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-tgar (Ship stats: count charge time in sustained DPS)
