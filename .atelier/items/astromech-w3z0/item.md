---
schema: atelier.item
schema_version: 3
id: astromech-w3z0
title: Plotting macro
status: draft
mission: astromech-7q0z
labels:
- size-l
priority: high
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-15T20:36:13.654441995Z
---

## Goal

Layer 2 of [PLOTTING.md](docs/PLOTTING.md): plot the next hop in the galaxy map on one voice command or button press.

## Done when

- [ ] The key sequence for each state in PLOTTING.md's test list is generated and covered by tests, using the binds from the user's bindings file.

## Context

Designed already. One press per hop; no auto-plotting (strategy Scope).

## Notes

- 2026-10-16 [note] 2026-10-16, Sumeet: pasting into the galaxy-map search works. Selecting the search box and plotting the jump are much harder, so this macro is deferred. Copying the next system to the clipboard stays in scope.
