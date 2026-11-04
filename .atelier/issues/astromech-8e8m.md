---
created_at: "2026-10-23T00:00:38.915965180+00:00"
id: "astromech-8e8m"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-8e8m
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T22:09:08.610015928+00:00"
status: "done"
title: "CI: check that commits pushed to main were reviewed"
updated_at: "2026-11-04T22:09:08.610015928+00:00"
---

## Goal

A push to `main` that carries unreviewed code or a done claim made outside review fails CI, as `atelier land` on `main` refuses it locally before a push.

## Done when

On push runs, the Atelier workflow checks the commits between `github.event.before` and the pushed commit as base commits (e.g. a scratch upstream ref at `event.before` and doctor on branch `main`), or Atelier's doctor checks approvals for done claims already in history; a test push marking a leaf issue done without review fails the job, and CONTRIBUTING.md says so.

## Context

Review finding R1 (astromech-tjok) on task/astromech-mk30: `check_base_commits` (land_base_done_claim, land_base_unreviewed) runs only on branch main for unpushed commits, so the CI step skips it on its detached head. Reproduced by marking astromech-1b43 done in one commit: CI step passes.

## Close Reason

Implementer report. Outcome: .github/workflows/atelier.yml gains a push-only step 'atelier land (pushed commits)' (if: github.event_name == 'push' && !cancelled()): with BEFORE=github.event.before it exits 0 for an all-zeros before, fetches before if absent, checks out main at GITHUB_SHA (-B), points refs/remotes/origin/main at before, sets main's upstream, asserts main@{upstream} == before, and runs atelier land (non-mutating without --push). That is how land on main picks base commits at the pinned rev: check_base_commits checks rev-list HEAD --not main@{upstream}. The doctor step and PR path are unchanged. CONTRIBUTING's tracker-records paragraph now says the push job fails on land_base_unreviewed or land_base_done_claim, after the fact (fix is a revert). Proof: docs/evidence/astromech-8e8m/report.md, scenarios (a)-(g) via the workflow's own run: scripts in a scratch clone with Atelier built at ATELIER_REV; (a) and (d) fail, (b), (c), (e), (f) pass, (g) fails closed. YAML parsed with PyYAML; actionlint not installed. Deviations: the installed atelier (5abb3651) is 227 commits past the pin and its check_base_commits differs (base exemptions), so the pinned rev was built into the scratchpad for the proof. Open: only a real push proves the GitHub side (refs/remotes/origin/main created by actions/checkout with fetch-depth 0, github.event.before set, fetching a force-pushed-away before by SHA from GitHub). A force push whose old main GitHub will not serve fails the job with no override. Moving ATELIER_REV changes what land checks (newer revs honour base exemptions). Independent review outstanding.
