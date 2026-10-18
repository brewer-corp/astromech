---
created_at: "2026-10-18T13:28:06.946737439+00:00"
id: "astromech-m2bl"
issue_type: "task"
labels:
- "post-alpha"
- "voice"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-vkzj"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Voice: a sound-alike of 'yes' still confirms a pending key press and counts toward learning"
updated_at: "2026-10-18T13:28:06.946737439+00:00"
---

## Goal

Decide, and then enforce, whether a sound-alike of "yes" may confirm a pending question. Today any confirm match at Ask level counts as yes (`crates/astromech-core/src/assistant.rs:400-406`), so a sound-alike like "yous", "yaz" or "yeas" (flagged as sound-only, Ask at 0.85) runs the pending key press and counts toward learning the alias.

## Done when

- The behaviour is decided with Sumeet. Options: (a) keep it, since a question is pending and a yes-like sound is very likely yes; (b) a sound-only confirm re-asks once; (c) a sound-only confirm runs the press but does not count toward KEY_CONFIRMATIONS learning.
- The chosen behaviour has tests.
- The voice eval confirm family does not regress.

## Context

Round-7 review finding R3 on astromech-vkzj (PR #164). This predates that PR, and the vkzj veto does not touch this path. Nothing in it newly executes compared with main.
