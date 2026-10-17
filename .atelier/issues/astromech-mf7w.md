---
created_at: "2026-10-17T16:28:50.729966932+00:00"
id: "astromech-mf7w"
issue_type: "task"
labels:
- "needs-human"
- "size-s"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-tgar"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Offer the sustained-DPS charge-time fix to Coriolis"
updated_at: "2026-10-17T16:28:50.729966932+00:00"
---

## Goal

Coriolis (github.com/EDCD/coriolis) shows sustained DPS above burst DPS for charged weapons; offer it the fix astromech makes in astromech-tgar.

## Done when

A PR (or an issue, if the maintainers prefer) is open on EDCD/coriolis, linked here; or a note says why it wasn't sent.

## Context

Coriolis's `getSustainedFactor` in `Module.js` leaves the last burst's charge time out of the reload cycle (astromech-bnnh). An agent can prepare the change and a description with a worked example (rail guns on a Fer-de-Lance), but the PR goes out under Sumeet's GitHub account, so he approves and opens it. Check first whether an issue or PR already covers it upstream.
