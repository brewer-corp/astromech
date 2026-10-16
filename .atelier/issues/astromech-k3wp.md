---
created_at: "2026-10-15T20:36:13.646672720+00:00"
id: "astromech-k3wp"
issue_type: "feature"
labels: []
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-k3wp
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-e8ux"
  - kind: "issue"
    id: "astromech-huab"
  - kind: "issue"
    id: "astromech-xf8j"
  children:
  - kind: "issue"
    id: "astromech-05bi"
  - kind: "issue"
    id: "astromech-6qk1"
  - kind: "issue"
    id: "astromech-i9w7"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-0x68"
    type: "related"
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-y9cj"
    type: "related"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T23:04:41.272087976+00:00"
status: "done"
title: "Colonisation tracker"
updated_at: "2026-10-16T23:04:46.392764407+00:00"
---

## Goal

The user sees every construction project's progress, what's left to deliver in loads for the current ship, and where to buy each commodity. This replaces Raven Colonial's Architect Tracker and EDCoPilot's tracker.

## Done when

Journal replay tests for the tracker and shopping list; recorded-response tests for where to buy. Independent review. The game check (astromech-xf8j) runs after this merges. Delivers S5.

## Notes

- 2026-10-16 [note] Merge note: feature/astromech-ipb5 adds system_address to JournalEvent::Docked; crates/astromech-colonisation/src/lib.rs:224 matches Docked without .. and will need updating when either branch lands after the other (review of u3f4, R5).
- 2026-10-16 [note] Docked system_address merge note settled in a64fd2e (Merge main into feature/astromech-k3wp): the colonisation Docked match takes the new field with `..`; astromech-store re-exports both rusqlite (main) and StoreError (this branch).

## Close Reason

All children done on this branch: construction tracker (6qk1) and shopping list (i9w7) with journal replay tests, reviewed; where to buy (05bi) with recorded Ardent tests (evidence dn8s), component tests and UI check. Independent review of 05bi and the main merge (a64fd2e) is outstanding and gates atelier merge; the game check astromech-xf8j runs after the merge.
