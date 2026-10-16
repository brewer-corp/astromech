---
created_at: "2026-10-16T22:18:22.689803896+00:00"
id: "astromech-fwbh"
issue_type: "task"
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
title: "Don't store repeated identical Outfitting.json snapshots"
updated_at: "2026-10-16T22:18:22.689803896+00:00"
---

## Goal

Stop the station book growing by about 100 MB a year from byte-identical outfitting snapshots, which the game writes with a new timestamp on every visit to the outfitting screen.

## Done when

A test shows a repeated snapshot with the same items as the station's latest adds no raw row, and ARCHITECTURE notes the expected growth.

## Context

Deferred from the astromech-zig3 review.
