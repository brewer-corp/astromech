---
schema: atelier.review
schema_version: 2
id: astromech-q0bw
title: finding on astromech-vucf @50e59010
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: 50e59010c8f51ecae55b994a55df7e4d1680b986
created_at: 2026-11-07T13:50:04.014319012Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 24220b3bac47c6a6584fd7126b3ef1b4f768e9de
    recorded_at: 2026-11-07T14:00:03.826149344Z
    resolution_commit: 83cee7e36811887abb56afa5592ff789bc0745ee
    reason: Cheap; fixed in 83cee7e3
---

medium: crates/astromech-core/src/app.rs:1699 (start sets EDDN_SHARING from the world's config) and world.rs:584-593 (write_config keeps the world's eddn/edsm settings). In world mode the EDDN and EDSM uploaders, update check and model download still use the real network. Nothing in d2nq reaches them (no game session), but i1ev is blocked only by d2nq and xk0n comes after i1ev, so i1ev's worlds/jump (FSDJump in a running session) would post to the real EDDN (/test schemas in debug) on every CI run, and a world with secrets.json would post to EDSM. Fix in i1ev, or before it: world mode forces eddn.enabled=false and the EDSM switch off until xk0n replaces those transports, or xk0n lands first. Defer to astromech-i1ev.
