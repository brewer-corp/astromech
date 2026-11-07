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

- [x] `.atelier/workflow.yaml` has the shape `atelier init` writes, with `merge_strategy: merge_commit`: no `blocked` or `review` status, no `close` transition for work, and `loose_end` and `validation` on the `on_base` workflow. `base_branch: main` and the issue types stay.
- [x] The six backlog validations (astromech-278z, astromech-8tml, astromech-dq9y, astromech-iw0s, astromech-leio, astromech-rk4s) move to `todo`, since `on_base` has no backlog; astromech-278z by its status line alone, so `feature/astromech-v6r3`, which edits that record, still merges cleanly. No record is left in a status the new policy lacks.
- [x] AGENTS.md describes the lifecycle in the same words as Atelier's AGENTS.md and keeps the PR rules; the SessionStart hook in `.claude/settings.json` runs `atelier status`, not the deprecated `prime`.
- [x] `.github/workflows/atelier.yml` no longer says `atelier merge` fast-forwards `main`.
- [x] The After merge lines are ready to migrate once this merges: on a clone, `atelier merge` of this branch and then `atelier migrate after-merge --apply` on `main` make loose ends for astromech-1hh9 and astromech-ok29, and `atelier land` and `atelier doctor` pass there. That records-only commit follows the merge on `main`; the in-flight astromech-v6r3 and astromech-y6d6 migrate their own lines on their branches.
- [x] `atelier check` and `atelier doctor` pass on the branch.

## Scope

The policy, the records whose status the policy drops, AGENTS.md, the Claude Code hook and the comment in `.github/workflows/atelier.yml`. Not `atelier migrate dispositions`, and not the records of items in flight on other branches.

## Context

atelier-nn67 in spadaval/atelier (the rollout), ADR 0028 there, and its starter policy. The Atelier workflow in CI builds Atelier at `ATELIER_REV` and runs `atelier doctor` on every PR.

## Report

astromech's policy has the starter's shape with `merge_commit`, the six backlog validations are in `todo`, and AGENTS.md, the SessionStart hook and the Atelier workflow's comment describe the new lifecycle; `atelier check` and `atelier doctor` pass on the branch.

- Policy: `diff` of `.atelier/workflow.yaml` against a fresh `atelier init` policy with `merge_strategy: merge_commit` added is empty.
- Statuses: the six validations read `backlog` on `main` and `todo` here. astromech-278z's diff is its status line alone, and `git merge-file` of it with `feature/astromech-v6r3`'s version has no conflict. No record is in `blocked` or `review`, and no validation is outside `todo`, `done` or `superseded`.
- AGENTS.md and hook: read against Atelier's AGENTS.md (spadaval/atelier master); `.claude/settings.json` runs `atelier status --hook claude`.
- CI comment: `grep fast-forward .github/workflows/*.yml` finds nothing.
- Migration: in a throwaway clone, a stand-in approval and `atelier merge astromech-zk7z` passed; then `atelier migrate after-merge --apply` on `main`, with its changes to astromech-v6r3 and astromech-y6d6 left out, made loose ends blocked by astromech-1hh9 and astromech-ok29, and `atelier land`, `atelier doctor` and `atelier check` passed.
- `atelier check` passes, noting the 4 open After merge lines and the older disposition form; `atelier doctor` passes.

### Deviations

- The branch began on the light path, but `atelier land` refuses an unowned branch that changes `.atelier/workflow.yaml` (`land_work_untraceable`), so `doctor` failed. This issue owns the branch instead; the fix is being made in Atelier on atelier-nn67.
- The migration is not on this branch. With its loose ends here, `atelier merge` refused them (`workflow_issue_type_unknown`): merge judges the merged records by `main`'s policy, which gains `loose_end` only when this merges. For the same reason no loose end for the post-merge migration can be filed here; the coordinator makes that records-only commit on `main`.

### Decisions

- astromech-v6r3 and astromech-y6d6 keep their After merge lines. Both are in flight on their own branches (`feature/astromech-v6r3`, `astromech-y6d6`), which edit those records, and y6d6's branch has reworded its line to add the crash restart (astromech-dl53). Migrating them on `main` would conflict with those branches and make `atelier merge` refuse them with `merge_base_records`; Agent Factory files an in-flight item's loose ends on its work branch.
- astromech-278z changes only its status line, not `updated_at`, so it merges cleanly with `feature/astromech-v6r3`, which edits the lines around it.

### Open items

- astromech-v3u4, a child of astromech-y6d6, has a fifth After merge line that is not in the `After merge (<reason>):` form, so neither the migration nor `atelier check` sees it; y6d6's line names the same S11 check. The y6d6 branch handles it.
- The migration turns each stand-in into a line such as `- [ ] the Windows test above.`, worth rewording in the post-merge commit.
- Pin 12efdfd9 was not built here. It is two Atelier merges behind the installed binary, and neither changes how policies or records are read; CI's doctor on the PR shows it.
