---
created_at: "2026-10-17T05:19:41.130479584+00:00"
id: "astromech-dt5h"
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
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Exploration: index landings by system and stop re-reading the journal for the system view"
updated_at: "2026-10-17T05:19:41.130479584+00:00"
---

## Goal

`History::landings_before` reads every Touchdown and Disembark row and json_extracts SystemAddress with no index; it runs on the pipeline thread at each live jump and on every 3 s System page poll (~40 ms at 60k rows, finding P4LS-B1 on astromech-ce4s). The shell re-parses the two newest journal files every 3 s and the unsold query re-folds all scan lines every 15 s (finding P4LS-B2 on astromech-tzc2). Index or ledger landings by system address, and cache the system view and unsold totals by journal file length/mtime.

## Done when

A history of 100k landing rows answers landings_before in under 5 ms (bench or timed test), and an unchanged journal is not re-parsed by the page poll (test).
