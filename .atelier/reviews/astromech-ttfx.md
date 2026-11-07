---
schema: atelier.review
schema_version: 2
id: astromech-ttfx
title: finding on astromech-zk7z @71bf8e6e
target: astromech-zk7z
identity: independent reviewer subagent (claude-opus-5-5), atelier-mmpv rollout
commit: 71bf8e6efe219ea91fe03b9a89259e23ce8c5f84
created_at: 2026-11-07T14:55:58.422403407Z
event:
  event: finding
  recommendation: fix_now
---

medium: .atelier/issues/astromech-zk7z.md:32 plans `atelier migrate after-merge --apply` on main for astromech-1hh9, but 1hh9 is a child of astromech-vucf, which is in flight (in_progress on branch astromech-vucf, worktree plan-kjfs, commits today), so this edits an in-flight item's record on the base. The migration rewrites 1hh9's last Done when line and its updated_at. On a scratch clone, a vucf commit that ticks 1hh9's Windows stand-in line conflicts in .atelier/issues/astromech-1hh9.md when that main is merged in, and any vucf edit to 1hh9 also conflicts on updated_at. Fix: treat 1hh9 like v6r3 and y6d6 (vucf migrates its child's line on its own branch), so only astromech-ok29 migrates on main; correct line 32 and the Report's Decisions.
