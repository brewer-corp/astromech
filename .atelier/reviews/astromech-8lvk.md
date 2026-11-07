---
schema: atelier.review
schema_version: 2
id: astromech-8lvk
title: finding on astromech-zk7z @71bf8e6e
target: astromech-zk7z
identity: independent reviewer subagent (claude-opus-5-5), atelier-mmpv rollout
commit: 71bf8e6efe219ea91fe03b9a89259e23ce8c5f84
created_at: 2026-11-07T14:55:58.791960422Z
event:
  event: finding
  recommendation: fix_now
---

low: AGENTS.md:42-45 says someday work sits in backlog and to file it with `atelier issue create --status backlog`. That no longer holds for validations and loose ends: on_base has no backlog (the six someday validations moved to todo for this reason), and `atelier issue create --issue-type loose_end --status backlog` is refused (issue_create_status_invalid). This repository files many someday in-game checks, so an agent following the bullet is refused. Fix: say backlog is for work items, and validations and loose ends wait in todo, labelled needs-human.
