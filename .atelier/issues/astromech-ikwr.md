---
created_at: "2026-10-16T21:54:33.351831742+00:00"
id: "astromech-ikwr"
issue_type: "task"
labels: []
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-z4jb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Ship stats: same-grade plan rounding and unknown modifier labels"
updated_at: "2026-10-22T21:17:51.807287907+00:00"
---

## Goal

A plan target at the same grade as a fully rolled module with the same experimental drops the journal values, so the difference column can show a tiny phantom change; and any journal Modifier label missing from the table marks the ship Incomplete even when the stat isn't shown.

## Done when

Journal values kept when level, Quality 1 and experimental match; labels for stats that aren't shown don't mark the ship incomplete (or the table is completed from a real Loadout); tests.

## Context

`crates/astromech-engineering/src/plan.rs`, `crates/astromech-ship/src/module.rs`. Found in review of astromech-fdyn (R1, R2). Needs real Loadout lines (legacy module at Quality 0, V1 pre-engineered SCO OriginalValue) — see the note on astromech-fgv5.
