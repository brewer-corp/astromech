---
created_at: "2026-11-07T23:48:54.231542958+00:00"
id: "astromech-lltb"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Replace the hand-edited Spansh route fixtures with recorded ones"
updated_at: "2026-11-07T23:48:54.231542958+00:00"
---

## Goal

The Spansh route fixtures are recorded raw answers (decision astromech-1zhg), not edited ones.

## Done when

- [ ] `crates/astromech-data/tests/fixtures/spansh-route-*.json` and `spansh-exact-*.json` (all-zero job ids, a 3-stop Sol to Colonia route) are re-recorded with `--record-net` from Spansh's public endpoints (no key, no commander) and checked for redaction; worlds/preview/sol, worlds/every-command and the crate tests that use them pass, and the preview drift test stays green.

## Context

Found by astromech-cwtv's implementer (mission astromech-kjfs), which re-recorded its own route for the journeys.
