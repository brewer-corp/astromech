---
created_at: "2026-10-15T20:36:13.646802031+00:00"
id: "astromech-tyjz"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-tyjz
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-uxyq"
  children:
  - kind: "issue"
    id: "astromech-09s3"
  - kind: "issue"
    id: "astromech-5el6"
  - kind: "issue"
    id: "astromech-el1v"
  - kind: "issue"
    id: "astromech-lt2b"
  - kind: "issue"
    id: "astromech-thau"
  - kind: "issue"
    id: "astromech-w20r"
  - kind: "issue"
    id: "astromech-xvjs"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-6xyo"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-n16x"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:46:33.162186989+00:00"
status: "done"
title: "EDDN sender"
updated_at: "2026-10-16T20:46:33.162186989+00:00"
---

## Goal

astromech sends commodity, outfitting, shipyard, journal and the other live EDDN schemas from the live game, following the EDCD sender rules, with per-service settings. This is what EDMC does for EDDN.

## Done when

Message tests per schema against the EDDN schema files. Independent review. The game check (astromech-uxyq) runs after this merges. Delivers S6.

## Unknowns

Whether the `blackmarket` schema is still live.

## Notes

- 2026-10-16 [note] Unknown answered (EDCD schemas/blackmarket-README.md, live branch, read 16 Oct 2026): the blackmarket schema is deprecated; the same data goes in commodity/3's prohibited array since Sep 2017. Don't send blackmarket.

## Close Reason

All children done: sender core with outbox, retry rules and beta gate (el1v); commodity/3, outfitting/2, shipyard/2 (lt2b, thau); journal/1 and 12 event schemas (w20r, 5el6); running in the core from live lines only, off by default, outfitting/2 gated (xvjs); Settings → Data sharing (09s3). Message tests validate against the live schema files. The live-session check is astromech-uxyq, after the merge.
