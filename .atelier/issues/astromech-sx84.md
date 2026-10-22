---
created_at: "2026-10-17T19:31:03.063222350+00:00"
id: "astromech-sx84"
issue_type: "bug"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "atelier land fails 'review event history missing' for review records that main got through a merge"
updated_at: "2026-10-22T19:44:28.832556637+00:00"
---

## Goal

`atelier land` and `atelier merge` work on a branch that has defer or no_action dispositions, whatever the history of the older review records it carries from main.

## Done when

`atelier land` on task/astromech-8sx3 or feature/astromech-94j4 (as of 17 Oct 2026) no longer stops with "review event history missing". The fix is upstream in spadaval/atelier, with a test where a review record is introduced in a merge commit.

## Context

In `crates/atelier-app/src/review_gate.rs`, `original_independent` finds a review record's introducing commit with `introduced_query`. That is `git log --root --reverse --diff-filter=A HEAD -- <file>`, without `-m`/`--first-parent`. So a record added on main by a merge commit (for example `.atelier/reviews/astromech-0xky.md`, from merge 9e9f64f "Merge branch 'main' into task/astromech-ah0i") has no introducing commit. The `?` on the defer/no_action path in global mode then fails the whole land. 20 such records exist on main. `atelier land` on main passes, because main has no work owner. Found during astromech-94j4 and astromech-8sx3; the plotter, 8sx3 and dpoc branches are affected. It is not caused by their records.

## Notes

- 2026-10-22 [note] 2026-10-22: it also stops `atelier land` on main when an unpushed records-only commit adds an `advances` link to a mission (34a56908, planning astromech-1842). Bisected in a scratch clone: a new unlinked issue, a note, or an edit to an unlinked issue land fine; one new issue advancing astromech-7q0z fails. The link puts the mission in the touched set, so the mission's done issues are re-checked in global mode, and the defer/no_action dispositions on znhx and ah0i hit the `?`. Those records (3h36, 332q, 0xky and 17 more) were created inside merge commits: neither parent of 201c809b has astromech-3h36.md, so `git log --diff-filter=A` without `-m` never lists an adding commit. Pushing passes once the commit is upstream (the trusted point moves past it). Fix upstream: add `--diff-merges=first-parent` (or `-m --first-parent`) to `introduced_query`, and treat a missing history like the approval path (`unwrap_or(false)`) instead of aborting.
- 2026-10-22 [note] 2026-10-22: filed upstream as atelier-as7i (the merge-commit lookup and the abort) and atelier-ss25 (adding an issue to a mission re-checks the whole mission), in spadaval/atelier's tracker. astromech-mk30 runs `atelier doctor` in CI here.
