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
closed_at: "2026-11-07T15:09:57.592559448+00:00"
status: "done"
title: "Move astromech onto Atelier's new lifecycle"
updated_at: "2026-11-07T15:09:57.592559448+00:00"
---

## Goal

Move this repository onto Atelier's new lifecycle (atelier-mmpv in spadaval/atelier): the starter's policy shape, statuses the new policy lacks moved first, AGENTS.md in Atelier's words. The one installed Atelier binary serves every repository on this host, and astromech still runs the old starter (fast_forward_only, blocked and review statuses, a close step for work).

## Done when

- [x] `.atelier/workflow.yaml` has the starter's shape, as Atelier's own policy has it on atelier-nn67, with `merge_strategy: merge_commit`: no `blocked` or `review` status, no `close` transition for work, and `loose_end` and `validation` on the `on_base` workflow, which keeps `backlog` with `plan` and `defer`. `base_branch: main` and the issue types stay.
- [x] No record is in a status the new policy lacks. The six backlog validations (astromech-278z, astromech-8tml, astromech-dq9y, astromech-iw0s, astromech-leio, astromech-rk4s) stay in `backlog`, which `on_base` allows, so this branch changes no existing issue record.
- [x] AGENTS.md describes the lifecycle in the same words as Atelier's AGENTS.md and keeps the PR rules, and its backlog bullet holds for every issue type, loose ends and validations included; the SessionStart hook in `.claude/settings.json` runs `atelier status`, not the deprecated `prime`.
- [x] `.github/workflows/atelier.yml` no longer says `atelier merge` fast-forwards `main`.
- [x] Once this merges, only astromech-ok29's After merge line migrates on `main`, in the coordinator's records-only commit; astromech-1hh9 (a child of astromech-vucf), astromech-v6r3 and astromech-y6d6 are in flight and migrate their own lines on their branches. On a clone, `atelier merge` of this branch and then that migration on `main` make one loose end blocked by astromech-ok29, and `atelier land` and `atelier doctor` pass there.
- [x] The in-flight branches need no sync after this merges: on a clone, `atelier merge <owner> --check` on unsynced feature/astromech-v6r3, astromech-y6d6 and astromech-vucf refuses only each branch's own gates, and `atelier land` passes on each.
- [x] `atelier check` and `atelier doctor` pass on the branch.

## Scope

The policy, AGENTS.md, the Claude Code hook and the comment in `.github/workflows/atelier.yml`. Not `atelier migrate dispositions`, and not the records of items in flight on other branches.

## Context

atelier-nn67 in spadaval/atelier (the rollout), ADR 0028 there, and its starter policy. The Atelier workflow in CI builds Atelier at `ATELIER_REV` and runs `atelier doctor` on every PR.

## Report

astromech's policy has the starter's shape with `merge_commit`, its `on_base` workflow keeping `backlog`, so no record moves; AGENTS.md, the SessionStart hook and the Atelier workflow's comment describe the new lifecycle, and `atelier check` and `atelier doctor` pass on the branch.

- Policy: `diff` of `.atelier/workflow.yaml` against a fresh `atelier init` policy with `merge_strategy: merge_commit` added shows only `on_base`'s `plan`, `defer` and `supersede` from `backlog`, which are identical to the `on_base` section of Atelier's policy in the atelier-nn67 worktree.
- Statuses: `git diff main` touches no issue record but this one; `atelier check` passes under the new policy with the six validations in `backlog`.
- AGENTS.md and hook: read against Atelier's AGENTS.md (spadaval/atelier master). On a clone with this branch merged, `atelier issue create --status backlog` succeeds for a `loose_end` and a `validation`, and `atelier work ready` lists neither, so the backlog bullet holds for every type unchanged. `.claude/settings.json` runs `atelier status --hook claude`.
- CI comment: `grep fast-forward .github/workflows/*.yml` finds nothing.
- Migration: on a clone, a stand-in approval and `atelier merge astromech-zk7z` passed. `atelier migrate after-merge --apply` on `main`, keeping only its change to astromech-ok29 and that item's new loose end, then passed `atelier land`, `atelier doctor` and `atelier check`.
- In-flight branches: on the same clone, `atelier merge <owner> --check` ran on feature/astromech-v6r3, astromech-y6d6 and astromech-vucf before and after this merge, and after the ok29 migration, none of them synced. Each refuses only its own gates (review.current, claims.reviewed, and findings or blockers already open); the old Report requirement from work's `close` drops out, and no `workflow_issue_status_invalid` or conflict appears. `atelier land` passes on all three. None needs `atelier merge --sync` for this change.
- `atelier check` passes, noting the 4 open After merge lines and the older disposition form; `atelier doctor` passes.

### Deviations

- The branch began on the light path, but `atelier land` refuses an unowned branch that changes `.atelier/workflow.yaml` (`land_work_untraceable`), so `doctor` failed. This issue owns the branch instead; the fix is being made in Atelier on atelier-nn67.
- The first version moved the six backlog validations to `todo`, since `on_base` had no backlog. That made every unsynced in-flight branch that edits one of them fail `atelier merge --check` with `workflow_issue_status_invalid` (finding astromech-f9t1). The coordinator's design change gives `on_base` a `backlog` with `plan` and `defer`, as Atelier's starter now does on atelier-nn67, so the validations stay where they were.
- The migration is not on this branch. With its loose ends here, `atelier merge` refused them (`workflow_issue_type_unknown`): merge judges the merged records by `main`'s policy, which gains `loose_end` only when this merges. For the same reason no loose end for the post-merge migration can be filed here; the coordinator makes that records-only commit on `main`.

### Decisions

- Only astromech-ok29 migrates on `main`. astromech-1hh9 is a child of astromech-vucf (in progress on branch astromech-vucf, which edits 1hh9's record), and astromech-v6r3 and astromech-y6d6 are in flight on their own branches; y6d6's branch has also reworded its line to add the crash restart (astromech-dl53). Migrating them on `main` would conflict with those branches and make `atelier merge` refuse them with `merge_base_records` (finding astromech-ttfx); Agent Factory files an in-flight item's loose ends on its work branch.
- AGENTS.md's backlog bullet stays as written: with `backlog` on both workflows it is true for every type (finding astromech-8lvk).

### Open items

- astromech-v3u4, a child of astromech-y6d6, has a fifth After merge line that is not in the `After merge (<reason>):` form, so neither the migration nor `atelier check` sees it; y6d6's line names the same S11 check. The y6d6 branch handles it.
- The migration turns each stand-in into a line such as `- [ ] the Windows test above.`, worth rewording in the post-merge commit.
- Pin 12efdfd9 was not built here. It is two Atelier merges behind the installed binary, and neither changes how policies or records are read; CI's doctor on the PR shows it.
