---
schema: atelier.item
schema_version: 3
id: astromech-m2bl
title: 'Voice: a sound-alike of ''yes'' still confirms a pending key press and counts toward learning'
status: draft
mission: null
labels:
- post-alpha
- voice
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-18T13:28:06.946737439Z
---

## Goal

Decide, and then enforce, whether a sound-alike of "yes" may confirm a pending question. Today any confirm match at Ask level counts as yes (`crates/astromech-core/src/assistant.rs:400-406`), so a sound-alike like "yous", "yaz" or "yeas" (flagged as sound-only, Ask at 0.85) runs the pending key press and counts toward learning the alias.

## Done when

- [ ] - The behaviour is decided with Sumeet. Options: (a) keep it, since a question is pending and a yes-like sound is very likely yes; (b) a sound-only confirm re-asks once; (c) a sound-only confirm runs the press but does not count toward KEY_CONFIRMATIONS learning. - The chosen behaviour has tests. - The voice eval confirm family does not regress.

## Context

Round-7 review finding R3 on astromech-vkzj (PR #164). This predates that PR, and the vkzj veto does not touch this path. Nothing in it newly executes compared with main.

## Links

Relationships of the old record that the new format has no field for.

- derived_from: astromech-vkzj (Voice: a phonetic-key match alone executes one-word key presses ('boost' -> paste, 'page' -> back))
