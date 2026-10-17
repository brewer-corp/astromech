---
created_at: "2026-10-17T07:27:41.075653802+00:00"
id: "astromech-ys47"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: task/astromech-ys47
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T08:06:37.530910164+00:00"
status: "done"
title: "Exobiology prediction: star class for moons and binary orbits; drop single-sighting conditions"
updated_at: "2026-10-17T08:06:37.530910164+00:00"
---

## Goal

Findings on astromech-znhx: Spansh's bodies search gives only a body's immediate parent, so moons and bodies orbiting a pair of stars get no star class and the star gate is learned from about a quarter of the sightings (astromech-v12n); 56 of 338 conditions come from a single sighting, some apparently bad data such as Bacterium Aurasus on ammonia, argon-rich and methane bodies (astromech-rzd1). Work out the nearest star from the system's full body list in scripts/gen-exobiology.py, require at least two sightings per condition, and fix the two NOTICE.md slips (a missing full stop; 'below' for 'above').

## Done when

The regenerated catalogue's held-out recall stays at or above 0.98 with fewer candidates per body, the star gate is exercised on most held-out bodies, and no condition rests on one sighting.

## Close Reason

Nearest star worked out from each system's stars in gen-exobiology.py, two sightings per condition, NOTICE slips fixed; held-out recall 0.987 (>=0.98) with 10.2 candidates per body (was 11.5), star known on 91% of held-out bodies, no single-sighting condition.
