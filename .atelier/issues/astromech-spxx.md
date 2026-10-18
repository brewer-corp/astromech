---
created_at: "2026-10-17T22:48:16.486252976+00:00"
id: "astromech-spxx"
issue_type: "task"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-spxx
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T00:06:13.832749976+00:00"
status: "done"
title: "Polling pages hide a failure after the first successful read"
updated_at: "2026-10-18T00:06:13.832749976+00:00"
---

## Goal

Ships, Materials, Powerplay and Station show a later failed poll, not only a failed first read.

## Done when

When a poll fails after data was shown, the page keeps the data and says it is stale (or shows the error beside it). A test per page.

## Context

Found in the review of the astromech-x81p PR: these pages show their error only while they hold no data (!fleet, err && !ov), so a core that goes away mid-session leaves old numbers on screen with no sign.

## Notes

- 2026-10-17 [note] Cause: pages showed their poll error only while they held no data (or, on Station, Session, Exploration, Logbook, Exobiology, replaced the data with the error). Fix: pollFailure() in lib/ui/errors.ts and a shared Stale.svelte line (Could not refresh: <reason>.), used by Ships, Materials, On foot, Logbook, Session, Mining, Station, Colonisation, Exploration (system and unsold), Exobiology, Powerplay, Route, Data sharing (EDDN and EDSM) and Diagnostics. Not covered, filed as astromech-cvxf: the trip planner's engineer poll and the coreStatus polls.

## Close Reason

Polling pages keep their last data and show 'Could not refresh: <reason>' on a failed poll, cleared on the next good read; the real loaders now reject in the app; production-path tests fail on the old code. Reviewed twice.
