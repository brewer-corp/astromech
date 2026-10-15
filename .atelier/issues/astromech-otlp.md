---
created_at: "2026-10-15T20:21:44.897944418+00:00"
id: "astromech-otlp"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Name reading rules before TTS"
updated_at: "2026-10-15T20:21:44.897944418+00:00"
---

## Goal

Read procedural names ("Synuefe BY-F d12-39": letters spelled, digits as digits), catalogue prefixes (HIP, HD, LHS, Col 285), ore and commodity names, percentages and large numbers ("19 M") correctly.

## Done when

A table test maps each example to the expected TTS input text.

## Context

Needed whichever TTS engine we use.
