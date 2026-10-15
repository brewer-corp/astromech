---
created_at: "2026-10-15T20:36:13.663793110+00:00"
id: "astromech-w8ai"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-6jzn"
  - kind: "issue"
    id: "astromech-nugx"
  - kind: "issue"
    id: "astromech-omi1"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Name reading rules before TTS"
updated_at: "2026-10-15T20:36:13.663793110+00:00"
---

## Goal

Read procedural names ("Synuefe BY-F d12-39": letters spelled, digits as digits), catalogue prefixes (HIP, HD, LHS, Col 285), ore and commodity names, percentages and large numbers ("19 M") correctly.

## Done when

A table test maps each example to the expected TTS input text.

## Context

Needed whichever TTS engine we use. Prospector lines need ore names and percentages read right.
