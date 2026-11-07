---
schema: atelier.review
schema_version: 2
id: astromech-t9m8
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:26.334200452Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: fix_now
    identity: claude-author
    commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    recorded_at: 2026-11-07T04:55:53.428094367Z
    resolution_commit: 2dc1e59b1ef18b668bef415e41cfe21d826f7442
    reason: Folded into the plan in 2dc1e59b.
---

medium: astromech-y6d6 missing scope/unknowns No upgrade path from installed two-process versions (a Start with Windows entry and a running tray core pointing at astromech-core.exe, the updater living in the core's update.rs, the single-instance mutex in V1-DESIGN 7.7, astromech-shell.log vs astromech-core.log in diagnostics). Also unnamed: user guide pages that describe the background process (docs/guide/getting-started.md:70,127, settings.md:9), V1-DESIGN 7.7, release.yml's existing install/start/status/quit smoke, stage-sidecars.sh and externalBin. Fix when planning y6d6: add the upgrade path as an Unknown/Done-when line and name these files.
