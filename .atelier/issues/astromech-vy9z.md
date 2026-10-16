---
created_at: "2026-10-16T20:09:16.942762236+00:00"
id: "astromech-vy9z"
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
status: "backlog"
title: "Ships page: accessible name for unnamed ships; link-open error test"
updated_at: "2026-10-16T20:09:16.942762236+00:00"
---

## Goal

An unnamed ship's card link shows "Unnamed" but is labelled "CU-01, Imperial Cutter", so voice control saying "click Unnamed" misses it; and no test shows "Couldn't open the link" when opening a Coriolis/EDSY/Spansh/Inara link fails.

## Done when

The label includes the visible text ("Unnamed CU-01, Imperial Cutter"); a component test covers the open error.

## Context

`app/src/modules/engineering/View.svelte:84`, `Build.svelte` `open()`. Found in review of astromech-z4jb (round 5, N1, N2).
