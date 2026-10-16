---
created_at: "2026-10-16T19:50:25.749334885+00:00"
id: "astromech-i4gx"
issue_type: "task"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-ipb5"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "History: commander id on events and ledgers"
updated_at: "2026-10-16T19:51:00.767285663+00:00"
---

## Goal

`astromech-history` has no commander id: ships are keyed by ShipID alone and every ledger mixes commanders (and Legacy-mode journals), so one commander's Anaconda can become another's sold Sidewinder. ARCHITECTURE §18.5 planned `cmdr_fid` on journal files and events.

## Done when

Each file's commander (from Commander/LoadGame FID) is stored; ledgers are keyed by commander; a two-commander fixture keeps them apart; a migration derives it for existing databases and rebuilds ledgers.

## Context

Found in review of astromech-aonb (R1). Must land before the logbook (fxp0) or session summary (bg0x) ship.
