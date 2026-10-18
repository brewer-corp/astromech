---
schema: atelier.review
schema_version: 1
id: astromech-zhdu
title: 'R1 [low] lib.rs:418: while_off is called for every input of a disabled rule, Tick included (about 1/s); default no-op and RouteFollow returns early, negligible cost; doc says each input. No action.'
target: astromech-lo81
identity: review-agent (sonnet, independent)
commit: 810d8bd932424fffe8f71379eed3b8b68cb7e417
created_at: 2026-10-17T19:12:33.996595527Z
event:
  event: finding
  recommendation: no_action
---

R1 [low] lib.rs:418: while_off is called for every input of a disabled rule, Tick included (about 1/s); default no-op and RouteFollow returns early, negligible cost; doc says each input. No action.
