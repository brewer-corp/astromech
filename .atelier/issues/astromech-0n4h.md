---
created_at: "2026-10-17T05:07:39.115726457+00:00"
id: "astromech-0n4h"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-wu30"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "System view: bodies scanned on an earlier visit"
updated_at: "2026-10-17T05:07:39.115726457+00:00"
---

## Goal

Coming back to a system shows the bodies scanned there before, with their values, not just the ones scanned since this arrival.

## Done when

History fixture: a system scanned on one visit and entered again later shows the earlier bodies, with what was mapped, before any new scan.

## Context

The Exploration → System view (astromech-wu30) folds the newest journals from the arrival on. The game doesn't write `Scan` again for bodies already scanned, so a return visit shows only the arrival star's automatic scan until bodies are rescanned. The history keeps every `Scan` line, so the view can seed itself from the stored lines for the system's address (as `History::add_landings` does for landings).
