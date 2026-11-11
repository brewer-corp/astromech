---
schema: atelier.item
schema_version: 3
id: astromech-usck
title: 'Outfitting search: nearest stations first, not newest rows'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T22:18:23.023963321Z
---

## Goal

outfitting_search applies LIMIT 100 to module rows before sorting by distance, so for broad words the nearest station can be cut and one station's many matches crowd others out.

## Done when

- [ ] A test with more than 100 matching rows returns the nearest station first, limited by station.

## Context

Deferred from the astromech-zig3 review.
