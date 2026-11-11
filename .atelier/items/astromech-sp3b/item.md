---
schema: atelier.item
schema_version: 3
id: astromech-sp3b
title: 'Powerplay page: show collect and deliver points'
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-17T03:01:20.026364299Z
---

## Goal

Show Galaxy::powerplay_logistics (collect, deliver, data hand-in with distance limits) on the Powerplay page: a tauri command modelled on powerplay_targets, a lib/powerplay.ts loader, and a section opened from a target row, with component tests.

## Done when

- [ ] Picking a target in Where to work lists its collect and deliver points with the commodity and limit, and component tests pass.
