---
schema: atelier.review
schema_version: 2
id: astromech-ed64
title: finding on astromech-bp23 @36ac2e58
target: astromech-bp23
identity: claude-reviewer-ndzx
commit: 36ac2e58e02be587b011992816354b14f2ca84fd
created_at: 2026-11-07T07:42:03.897334801Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: dd3c44e8ea254af9e81bfcdd06c25465a660a076
    recorded_at: 2026-11-07T07:43:30.371748474Z
    follow_up: astromech-ravn
    reason: Filed as bp23 child astromech-ravn
---

low: crates/astromech-core/src/session.rs:163 The known-biology lookup still builds its own astromech_data::Client (in-memory cache, its own limiter), so ndzx's Done when ('astromech-data's rule of one client per process holds') and ARCHITECTURE §5.8's new 'one limiter spaces every lookup' are not quite true: its Spansh bodies searches are not spaced against the window and voice lookups. It predates ndzx (a63r) and its session-only cache looks deliberate, so moving it to the shared client changes caching. Fix: either share the client (decide the TTL) or name the exception in ARCHITECTURE. File a follow-up.
