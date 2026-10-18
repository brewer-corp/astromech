---
created_at: "2026-10-17T19:30:02.493599671+00:00"
id: "astromech-ir9i"
issue_type: "task"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "My controller: alternatives on the other stick, and edge-case tests"
updated_at: "2026-10-17T19:30:02.493599671+00:00"
---

## Goal

Close the edge cases the review of the My controller restore left open.

## Done when

(1) An activity's action whose alternative is bound only on the other stick (or only elsewhere) shows in the other-stick or Elsewhere list instead of nowhere. (2) Tests for an invalid stored activity id, selId resetting on an activity change, an action bound on both sticks, and the Ship/General box preference.

## Context

Review findings R3 and R4 on PR #133 (astromech-54hd), both low and not regressions: elsewhere already ignored alternatives.

Second review of PR #133 (the fix commit): `showRow()` does not clear `unknown`, so after an unmapped press, clicking an action row lights its control but the panel keeps showing the unknown press (the template checks `unknown` before `sel`); the device switch buttons do not clear `unknown` either. One line each plus a test (press an unmapped key, click a row, expect "Action"). Not a regression from the PR.
