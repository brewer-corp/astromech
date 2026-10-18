---
created_at: "2026-10-17T20:10:24.648177355+00:00"
id: "astromech-c9i7"
issue_type: "task"
labels:
- "alpha"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-18T01:50:55.556243410+00:00"
status: "superseded"
title: "Decide whether the onboarding smoke test gates tagged releases"
updated_at: "2026-10-18T01:50:55.556243410+00:00"
---

## Goal

Decide whether a failing onboarding smoke test (scripts/onboarding_smoke.py, Release build) should stop a tagged release from publishing.

## Done when

The Release build either keeps the step as a gate for tags (after a record of green runs) or runs it with continue-on-error on tags, and the choice is noted in the workflow.

## Context

Review of astromech-flru (R2). The step runs after the installer artifact is uploaded, but publish needs the installer job to pass, so a flaky run (window timing, foreground rules, fixed DevTools port) would hold up a release. It has one green PR run so far.

## Notes

- 2026-10-17 [note] 17 Oct 2026, Sumeet: keep the gate. A failing onboarding smoke test stops a tagged release from publishing; it is the check that catches a broken installer or first run before the updater ships it. The gate is already in place (astromech-flru, merged), so nothing to change.

## Close Reason

Decided by Sumeet on 17 Oct 2026: keep the release gate, which astromech-flru already implements; no work left.
