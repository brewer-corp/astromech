---
schema: atelier.review
schema_version: 2
id: astromech-4zdn
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:03.491953569Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 24220b3bac47c6a6584fd7126b3ef1b4f768e9de
    recorded_at: 2026-11-07T14:00:03.302731358Z
    resolution_commit: 83cee7e36811887abb56afa5592ff789bc0745ee
    reason: Helper owns an empty home
---

medium: crates/astromech-core/tests/support/mod.rs:865 WorldCore::start isolates nothing itself: the child inherits the real HOME, XDG_DATA_HOME, LOCALAPPDATA, APPDATA and USERPROFILE unless each caller passes overrides, and only world_mode.rs does. Every sibling test (i1ev, xk0n, 1hh9, gllh, jkgv) spawns through this helper, so a regression to Paths::current() or any env fallback in world mode would write into the real HOME from those tests, which is the incident from this child's own mutation run. Fix: the helper makes its own empty home tempdir, always points those five variables (and XDG_CONFIG_HOME, XDG_CACHE_HOME, XDG_STATE_HOME) at it, and quit() asserts it is still empty; world_mode.rs keeps its explicit check.
