---
created_at: "2026-11-05T09:13:45.458444355+00:00"
id: "astromech-y14z"
issue_type: "task"
labels:
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-f7i0"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run the voice eval with the Mining hotspots phrases before the next release"
updated_at: "2026-11-05T09:13:45.458444355+00:00"
---

## Goal

The new `mining.hotspots` phrases and `{mineral}` slot words (painite, LTDs, low temp diamonds, void opals and the rest) are measured by speech recognition before they ship.

## Done when

- `scripts/voice-eval.sh` runs on a release sherpa build (on boat, per the boat skill; it is too heavy for the live host) with the current 200-phrase set, and its results are recorded as evidence on this issue, including the two hotspot phrases and the trade.sell / find.service phrases they replaced.
- Any phrase below the eval's bar gets an issue.

## Context

Deferred finding astromech-geeg from the f7i0 review: the eval set swap is within CONTRIBUTING's rules and the phrase-coverage test passes, but the audio eval was not rerun.
