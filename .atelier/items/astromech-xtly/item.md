---
schema: atelier.item
schema_version: 3
id: astromech-xtly
title: 'System chip: announce copies and show failures; chips in prose'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T18:56:53.876435294Z
---

## Goal

The shared SystemChip has a fixed aria-label, so screen readers miss "Copied", and a clipboard failure is swallowed. System names inside data prose (Materials source hints such as "Nearest encoded trader: Ray Gateway, Diaguandri") are plain text, not chips.

## Done when

- [ ] The chip has a polite live status and an error state (component test); source hints carry structured system names rendered as chips.

## Context

`app/src/lib/ui/SystemChip.svelte`, Materials data. Found in review of astromech-q318 (round 1, R4, R5).

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-q318 (v1.0 UI refinement)
