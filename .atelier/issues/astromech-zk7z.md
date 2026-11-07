---
created_at: "2026-11-07T14:39:48.513952213+00:00"
id: "astromech-zk7z"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: atelier-lifecycle
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move astromech onto Atelier's new lifecycle"
updated_at: "2026-11-07T14:39:49.120218666+00:00"
---

## Goal

Move this repository onto Atelier's new lifecycle (atelier-mmpv in spadaval/atelier): the starter's policy shape, statuses the new policy lacks moved first, AGENTS.md in Atelier's words. The one installed Atelier binary serves every repository on this host, and astromech still runs the old starter (fast_forward_only, blocked and review statuses, a close step for work).

## Done when

- [ ] `.atelier/workflow.yaml` has the shape `atelier init` writes, with `merge_strategy: merge_commit`: no `blocked` or `review` status, no `close` transition for work, and `loose_end` and `validation` on the `on_base` workflow. `base_branch: main` and the issue types stay.
- [ ] The six backlog validations (astromech-278z, astromech-8tml, astromech-dq9y, astromech-iw0s, astromech-leio, astromech-rk4s) move to `todo`, since `on_base` has no backlog; astromech-278z by its status line alone, so `feature/astromech-v6r3`, which edits that record, still merges cleanly. No record is left in a status the new policy lacks.
- [ ] AGENTS.md describes the lifecycle in the same words as Atelier's AGENTS.md and keeps the PR rules; the SessionStart hook in `.claude/settings.json` runs `atelier status`, not the deprecated `prime`.
- [ ] `.github/workflows/atelier.yml` no longer says `atelier merge` fast-forwards `main`.
- [ ] The After merge lines are ready to migrate once this merges: on a clone, `atelier merge` of this branch and then `atelier migrate after-merge --apply` on `main` make loose ends for astromech-1hh9 and astromech-ok29, and `atelier land` and `atelier doctor` pass there. That records-only commit follows the merge on `main`; the in-flight astromech-v6r3 and astromech-y6d6 migrate their own lines on their branches.
- [ ] `atelier check` and `atelier doctor` pass on the branch.

## Scope

The policy, the records whose status the policy drops, AGENTS.md, the Claude Code hook and the comment in `.github/workflows/atelier.yml`. Not `atelier migrate dispositions`, and not the records of items in flight on other branches.

## Context

atelier-nn67 in spadaval/atelier (the rollout), ADR 0028 there, and its starter policy. The Atelier workflow in CI builds Atelier at `ATELIER_REV` and runs `atelier doctor` on every PR.
