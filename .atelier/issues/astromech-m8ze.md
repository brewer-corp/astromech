---
created_at: "2026-10-16T22:35:43.304823496+00:00"
id: "astromech-m8ze"
issue_type: "bug"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Docking quiet can stick after re-reading an older journal"
updated_at: "2026-10-16T22:35:43.304823496+00:00"
---

## Goal

When an older journal is re-read mid-session (e.g. a sync tool touches it) and it ends after DockingGranted with no docking-end event (a crash), the reaction engine's docking quiet stays on and mutes non-urgent speech until the next docking-end event. Replayed lines shouldn't leave docking quiet set.

## Done when

A pipeline test re-reads an older journal ending after DockingGranted and non-urgent speech is not muted afterwards.

## Context

Deferred from the review of the EDDN merge into astromech-cmhv (pipeline.rs:252-258, astromech-announce lib.rs:245-249).
