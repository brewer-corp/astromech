---
created_at: "2026-10-17T16:53:48.217576239+00:00"
id: "astromech-j42m"
issue_type: "feature"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-05k2"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T17:01:44.473214545+00:00"
status: "superseded"
title: "Alpha: test everything that runs without the game"
updated_at: "2026-10-17T17:01:44.473214545+00:00"
---

## Goal

Every v1.0 criterion that can be measured without Elite running is measured, on Linux and on the Windows CI runner, and recorded as evidence; what is left needs the game and nothing else.

## Done when

Every child issue is done, and one evidence receipt per measured criterion is attached to astromech-05k2 or the child that measured it.

## Context

The mock game (`crates/astromech-core/examples/mockgame.rs`) is Windows-only; Windows CI already runs an installer smoke test (`.github/workflows/release.yml`). Screens can be walked with `scripts/app-shots.sh` and `scripts/ui` against the example fixture.

## Close Reason

Its issues merge independently on their own branches (a parent owns one shared branch); the alpha is tracked by the label alpha and astromech-5ggk's blockers
