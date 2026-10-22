---
created_at: "2026-10-22T19:30:24.928589104+00:00"
id: "astromech-6bjl"
issue_type: "feature"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-nk0u"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-vlsz"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Neutron plotter over the local file"
updated_at: "2026-10-22T21:11:35.787044259+00:00"
---

## Goal

`astromech_galaxy` plots a neutron route from the local neutron star file for a start, a destination, the ship's full-tank range R and its boost multiplier k, and returns the same `Plotted` the Spansh path returns, with no network.

## Done when

- A* over neutron stars: edges from the start to a neutron star cost `ceil(d / R)` jumps; from a neutron star to another, or to the destination, one boosted jump covers `k·R` and normal jumps cover the rest (`1 + ceil(max(0, d − k·R) / R)`); the route with no neutron stars is a candidate too. It minimises jumps, then stops.
- Unit tests on synthetic star fields where the optimal route is known (a straight chain, a detour that saves jumps, a gap no boost spans, start or destination on a neutron star, k = 4 and 6).
- A test against the recorded Spansh neutron routes (Sol to Colonia, plus at least one more recorded for this) at the same range: the local route has no more jumps than Spansh's, and the jump counts per leg use the same model (any difference explained in the test).
- Measured on this host and written in the module docs: time and peak memory for Sol to Colonia and Sol to Beagle Point at R = 60, k = 4 and R = 80, k = 6. Target: under 2 s each; names read only for the stops.
- `Plotted` from the local plotter has `source: "neutron plotter (this PC)"` and the same labels (`neutron boost`) the follower already speaks.

## Scope

`crates/astromech-galaxy` (new module next to `plot.rs` and `map.rs`). No UI or shell change; the wiring is a separate child.

## Context

`map.rs` has the grid and A* for jump counts to reuse or follow. `plot.rs` holds `Ship`, `Stop` and `Plotted`. The multiplier comes from `astromech_ship::Fsd` (astromech-vlsz). Start and destination coordinates come from `Galaxy::locate` (local star map, then Spansh's autocomplete).

## Unknowns

Neighbour counts at k·R up to about 500 ly near the galactic core. If plain A* misses the 2 s target, record the measurement and try a tighter heuristic (straight-line distance over k·R) or a coarser grid before anything cleverer.

## Notes

- 2026-10-22 [note] 2026-10-22: from review finding R3 (astromech-4h9h) on the neutron file: a loaded file must not read names from a file replaced underneath it (keep the handle opened at load, or re-check the header before reading names); on Windows a held handle also blocks the download's rename, so the download must cope.
