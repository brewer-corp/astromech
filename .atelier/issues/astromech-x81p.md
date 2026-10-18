---
created_at: "2026-10-17T21:26:01.678869282+00:00"
id: "astromech-x81p"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-x81p
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T23:04:44.639752219+00:00"
status: "done"
title: "Browser preview cannot show empty, loading or error states for fixture-backed pages"
updated_at: "2026-10-17T23:04:44.639752219+00:00"
---

## Goal

The example fixture and fake core let a walk reach empty, loading and error states for every page, not just Controls, Settings, Route and Setup.

## Done when

Engineering (ships), Materials, On foot, Engineers, Logbook, Session, Mining, Station, Trade, Colonisation, Exploration, Powerplay, Data sharing and Diagnostics each show a no-data state under ?core=empty, and a way to force a slow or failing core call (for example ?fail=<cmd> and ?slow=<ms>) so loading and error states can be screenshotted.

## Context

Found in the astromech-5qh0 walk: under ?core=empty these pages still render the sample fixture (Sample journal, CMDR Fixture, EDDN log), so their empty states were not seen. Evidence: docs/evidence/astromech-5qh0/ (full set kept locally in the worktree astromech-wt-walk).

## Close Reason

Preview can show empty, slow and failing states per call (?core=empty, ?fail=, ?slow=); every fixture-backed page walked in each state; hangs, leaked rejections and bare errors fixed with 39 state tests (15 fail on the old code). Reviewed twice.
