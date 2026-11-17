---
schema: atelier.item
schema_version: 3
id: astromech-zqqx
title: Preview with ?core=empty shows every engineer locked; the real core shows the starting engineers as known
status: ready
mission: astromech-xajc
labels:
- tooling
- size-s
priority: low
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T12:23:38.509294697Z
trivial: null
caused_by: null
created_at: 2026-11-17T12:23:38.401706190Z
---

## Goal

The browser preview's empty state matches the app's. With `?core=empty`, Engineering > Engineers shows every engineer as locked; the real core, with no journal read yet, shows the engineers every commander starts knowing as known. Both of astromech-m6gw's reviewers deferred it (19 Nov 2026). A screen checked against the wrong empty state can pass with a first-run page nobody will see.

## Done when

- [ ] The preview's empty answers for the Engineers page are the core's reply for an empty world, recorded through `app/src-tauri/tests/preview_fixtures.rs` like the other fixtures, not written by hand; the fixture drift test passes.
- [ ] The Engineers tree's empty-state capture is retaken from it.
- [ ] The Report lists any other page whose `?core=empty` answer is hand-written and differs from the core's, as items or as fixed here if small.

## Scope

Preview fixtures for the empty state.
