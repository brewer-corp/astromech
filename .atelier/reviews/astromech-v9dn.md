---
schema: atelier.review
schema_version: 2
id: astromech-v9dn
title: finding on astromech-vucf @4744ece0
target: astromech-vucf
identity: claude-reviewer-1hh9
commit: 4744ece07c9a33b4183c5178bf79bbd8e9b5c504
created_at: 2026-11-07T18:48:11.575275127Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-core/tests/world_voice.rs:91-99 and worlds/kill-switch/world.jsonl (the kill-during-verify race). If the kill lands before the verify thread reaches StateHub::wait_flag (hub.rs:122-141), the wait checks cancelled() before it sleeps and returns Cancelled. So the test still passes, with the same reply, and doesn't hang: the race can't make it flaky. The only loss is that the 25 ms slice is sometimes not exercised. Measured: I copied the world with the slice advance at 24 ms and drove the built core over the control protocol. The mutant got no reply in 3 of 3 runs, and the 25 ms baseline replied 'Pressed, then the kill switch cancelled it.' in 3 of 3. The gap between the confirmation and the wait is microseconds, against milliseconds of control round trips before the kill. Not worth a 'verify waiting' observation.
