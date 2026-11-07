---
schema: atelier.review
schema_version: 2
id: astromech-june
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:26.530169666Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author
    commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    recorded_at: 2026-11-07T04:55:53.610649122Z
    resolution_commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    reason: Folded into the plan in 2dc1e59b.
---

medium: astromech-bp23 Unknowns Moving every window command (binds edits that write .binds files, colonisation_buy, open_external) onto the TCP control API widens what a local client with the token can do; today the shell allow-lists core_call commands (core.ts:114) and ARCHITECTURE 18.2 routes commands through astromech-policy. Fix when planning bp23: add which commands go through policy and what the control API exposes to non-window clients.
