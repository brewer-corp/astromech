---
schema: atelier.review
schema_version: 1
id: astromech-4ylo
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:18.276134597Z
event:
  event: finding
  recommendation: defer
---

defer: astromech-rz0f. "Set destination parks the route it replaced" is a change to Desk::set, which today forgets the parked route (crates/astromech-core/src/route.rs:407); only plotted() parks (route.rs:484). set is also what route import and the engineer trip (modules/engineering/Build.svelte:41) call, and rz0f says Build is unchanged. Fix: state whether parking is a new command or a change to set that import and the engineer trip inherit, and whether their notes also say Replaced <name>. RESUME.
