---
created_at: "2026-10-22T19:29:51.528661596+00:00"
id: "astromech-1842"
issue_type: "feature"
labels:
- "size-l"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-1842
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-3x0x"
  - kind: "issue"
    id: "astromech-6bjl"
  - kind: "issue"
    id: "astromech-7z3m"
  - kind: "issue"
    id: "astromech-d4of"
  - kind: "issue"
    id: "astromech-nk0u"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Plot neutron routes on this PC"
updated_at: "2026-10-22T20:42:08.174497721+00:00"
---

## Goal

**Plot with Spansh → Neutron** is worked out on the user's PC from a file of every known neutron star, and gives routes with no more jumps than Spansh's at the default efficiency, for the ship's own boost (×4, or ×6 for the Mk II drive). Plotting takes seconds, not a Spansh queue, works offline once the file is downloaded, and stops loading Spansh's servers. Until the file is downloaded, the Spansh neutron plotter is the fallback.

## Done when

- Every child is done, reviewed and merged on this branch.
- The validation item for this increment (child `validation`) passes: local routes compared with Spansh's for the same ships and endpoints, and a plotted route loads into the follower.
- docs/guide/routes.md and docs/PLOTTING.md describe the local plotter, the download, the fallback and the limits.

## Unknowns

- Search cost: a boost range of 6 × 80 ly is a 480 ly neighbour radius, which can hold thousands of neutron stars near the core. The plotter child measures it; if A* is too slow, a coarser heuristic or a jump-count cap per expansion is the fallback.
- How Spansh counts the normal jumps between neutron stars. We assume `ceil((d − k·R) / R)` after a boosted jump; the comparison test checks it against recorded Spansh routes.

## Background

Discussed with Sumeet on 22 Oct 2026.

- **Data:** Spansh publishes `https://downloads.spansh.co.uk/systems_neutron.json.gz` (182 MB gzipped, rebuilt with its other dumps; same schema as `systems.json`). On 22 Oct 2026 it held 5,057,885 systems: 5,045,154 with a neutron main star, 12,475 black holes, about 110 others. Only neutron main stars are usable: the boost comes from the arrival star. Packed as id64 and three i32 coordinates in 1/32 ly, the neutron stars take 96 MB (60 MB gzipped) and their names 102 MB (23 MB gzipped).
- **Policy:** Spansh's data is public and fine to process in CI and ship. Neutron stars rarely change, so the file is refreshed by hand, like the star map preload (`.github/workflows/starmap.yml`), not on a schedule.
- **Download, not bundled:** about 80 MB compressed is too much to add to the installer and to every update, so the app downloads it once into the data folder from a GitHub pre-release, as the star map preload is published.
- **The model:** a route's stops are neutron stars plus the destination; the galaxy map plots each leg. From a neutron star a boosted jump reaches `k·R` (k the drive's multiplier, R the full-tank range with the cargo aboard), then normal jumps cover the rest. Start and destination are ordinary systems located with `Galaxy::locate`. The search minimises total jumps, then the number of stops (each stop is a galaxy-map plot), so the efficiency setting goes away for local routes.
- **Out of scope here:** fuel planning, checking sparse gaps against real stars, white-dwarf boosts, via systems, FSD injections. Fuel and sparse gaps are the backlog increment that follows this one.
- The in-game plotter's "use jet-cone boost" only takes neutron stars almost on the straight line (Spansh's author puts it near 99.5% efficiency) and plots at most about 20,000 ly; that is why this tool is worth having.
