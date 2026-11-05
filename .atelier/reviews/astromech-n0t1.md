---
schema: atelier.review
schema_version: 1
id: astromech-n0t1
title: finding on astromech-rz0f @2828a4c0
target: astromech-rz0f
identity: reviewer-fable-rz0f
commit: 2828a4c0e6b4301e193a1587c21be434e740b88f
created_at: 2026-11-05T07:28:22.724070113Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/route.rs:438 follow() parks whatever unfinished route of ours it replaces, even one identical to the route being set, in place of the older parked one. Set destination clicked twice on the same result (or Follow on the engineer trip twice) parks the one-stop route it just set and drops the parked trip; the note then says 'Replaced Deciat', so it is not silent, but the trip is gone. Suggested fix: skip the park when the replaced route equals the new one (same source and waypoints), keeping the older parked route.
