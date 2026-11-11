---
schema: atelier.item
schema_version: 3
id: astromech-um0g
title: Overlay panels
status: draft
mission: null
labels:
- size-l
priority: medium
blocked_by:
- astromech-d4p9
- astromech-iw0s
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.664349527Z
---

## Goal

PTT and confirmation toast, then a kneeboard with prospector results, route, colonisation list and nearest service.

## Done when

- [ ] Each panel renders from replayed journal data and updates as events arrive, covered by tests.

## Context

Only if the spike is a go.

## Links

Relationships of the old record that the new format has no field for.

- parent: astromech-nf25 (VR overlay)
