---
created_at: "2026-10-15T20:36:13.664487184+00:00"
id: "astromech-wu30"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-gpee"
  - kind: "issue"
    id: "astromech-tzc2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T04:57:48.927472607+00:00"
status: "done"
title: "Live system view with body values"
updated_at: "2026-10-17T04:57:48.927472607+00:00"
---

## Goal

Bodies as they're scanned (FSS, `Scan`, `SAASignalsFound`): class, terraformable, landable, rings, gravity, estimated value (FSS and mapped, first-discovery bonus) and a "worth mapping" flag.

## Done when

Value tests against published examples for each body class, and a replayed system shows every body.

## Context

Value formulas are community-derived; cite them and unit-test them.

## Close Reason

System view with body values built: astromech-exploration values tested against EDSM's published estimates for every body class; a replayed system shows every body; Exploration > System page.
