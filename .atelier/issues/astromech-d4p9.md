---
created_at: "2026-10-15T20:36:13.664211831+00:00"
id: "astromech-d4p9"
issue_type: "spike"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-iw0s"
  - kind: "issue"
    id: "astromech-um0g"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Prototype the SteamVR overlay with cost logging"
updated_at: "2026-10-16T04:38:23.155501483+00:00"
---

## Goal

Spike S2, the build half: an overlay in Rust that logs GPU/CPU and frame-time cost, with both raw upload and texture sharing behind a switch.

## Done when

The prototype builds and runs against SteamVR's null driver or a headset, logging its costs (evidence).

## Context

`astromech-overlay` has the toast view only.
