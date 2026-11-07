---
schema: atelier.review
schema_version: 2
id: astromech-1k9v
title: finding on astromech-bp23 @e0fac0ea
target: astromech-bp23
identity: claude-reviewer-i8vr
commit: e0fac0ea40744ef51d4ea921bc10933300547cd8
created_at: 2026-11-07T09:47:38.814826506Z
event:
  event: finding
  recommendation: fix_now
---

low: scripts/onboarding_smoke.py:413-416 (star_map_preload) galaxy_systems always asks Spansh (Galaxy::system_names -> spansh_system_names, crates/astromech-galaxy/src/lib.rs:214), and core_call's socket has a 5 s timeout (line 197). After the page tour has queued lookups on Spansh's 1 req/s slot, or when Spansh is slow, the read times out with TimeoutError (an OSError), which 'except ValueError' doesn't catch. The Release smoke on the bp23 PR then fails on network timing, not on the preload check. Fix: catch (ValueError, OSError) there. The star map is already warmed at session start (session.rs:142), so the call is only a fallback.
