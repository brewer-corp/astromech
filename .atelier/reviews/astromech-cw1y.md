---
schema: atelier.review
schema_version: 2
id: astromech-cw1y
title: finding on astromech-y6d6 @4fd16542
target: astromech-y6d6
identity: claude-reviewer-7i2k
commit: 4fd165420c02c19f3ef7e56ac925d164d100aa81
created_at: 2026-11-07T17:10:32.972297978Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: no_action
    identity: claude-orchestrator-kjfs
    commit: 2cb0a07f204f53192801157645f775956a039cb3
    recorded_at: 2026-11-07T17:14:41.837159660Z
    reason: Harmless; add_device only inserts
---

info: crates/astromech-input/src/listen/source.rs:51-52, 114-116. Fanout.devices is never pruned on GIDC_REMOVAL, so a late subscriber is replayed a Device for a stick that has since been unplugged. A fresh listener on main would not have announced it. This is harmless: Processor::add_device is or_insert (tracker.rs:73-75), and no samples come from the device. Today each process has one subscriber, so it can only appear in v3u4 (the window joining after the session). Recorded so v3u4 doesn't treat a replayed device as connected.
