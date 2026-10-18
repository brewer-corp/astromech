---
created_at: "2026-10-18T00:59:54.360818931+00:00"
id: "astromech-kb0n"
issue_type: "task"
labels:
- "voice"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Voice: count phrase confirmations by normalised text"
updated_at: "2026-10-18T00:59:54.360818931+00:00"
---

## Goal

Count "Did you mean" confirmations per normalised phrase, not per exact transcript string, so "Dear dawn." and "dear dawn" add up.

## Done when

Confirmations of transcripts that normalise the same count together toward the two-yes rule for key presses; the learned list still shows what was heard; tests cover it.

## Context

Review finding R4 of astromech-1c66. Errs on the safe side today: variation only delays learning.
