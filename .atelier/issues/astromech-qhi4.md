---
created_at: "2026-10-16T18:12:15.158728707+00:00"
id: "astromech-qhi4"
issue_type: "bug"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T22:10:41.780076154+00:00"
status: "done"
title: "Engineer trips suggest engineers who can't reach the grade"
updated_at: "2026-10-16T22:10:41.780076154+00:00"
---

## Goal

Engineer trip predictions are often wrong: they suggest engineers whose maximum grade for a blueprint is lower than the grade the plan asks for. Find out why and fix it.

## Done when

A trip test with a plan for a grade above an engineer's cap for that blueprint excludes or splits that engineer. The test fails on the current code. Spot-check the engineer grade table against the wiki for every engineer, and record the result.

## Context

`crates/astromech-engineering/src/trip.rs` already filters on `engineers::offers(..) >= job.grade` (around line 143). So the cause is probably the grade table, or a path that skips the filter. Reproduce with a real plan before changing anything.

## Close Reason

Cause: blueprint engineer lists were merged across module types. Now per module group; the failing trip test passes; engineer grade table spot-checked against the wiki for every engineer (two corrections). Evidence recorded.
