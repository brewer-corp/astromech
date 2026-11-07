---
created_at: "2026-11-07T04:51:25.107542998+00:00"
id: "astromech-km2s"
issue_type: "feature"
labels:
- "size-l"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "related"
  - kind: "issue"
    id: "astromech-vucf"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "The static preview runs the real core compiled to WASM"
updated_at: "2026-11-07T04:51:25.107542998+00:00"
---

## Goal

The published preview link (scripts/preview-publish) runs the real core compiled to WebAssembly against a scripted world in the browser, so the artifact shows real core output and time passing, and the remaining hand-written fixtures go away.

## Done when

- [ ] The command layer and the logic crates build for wasm32-unknown-unknown in CI (journal, gamestate, announce, materials, engineering, exploration and ship already do; SQLite needs sqlite-wasm-rs and a wasm C toolchain; HTTPS is switched off).
- [ ] A wasm-bindgen facade over the command layer is the preview's transport, single-stepped by the world script; time comes from the clock trait.
- [ ] The preview artifact stays inside the artifact limits (15 MB a binary file).

## Context

Pick up after mission astromech-kjfs's command layer and world seam are done; probe of 6 Nov 2026 in the Route redesign conversation. Lookups stay recorded (the artifact blocks other hosts) and the neutron star file is too big for an artifact.
