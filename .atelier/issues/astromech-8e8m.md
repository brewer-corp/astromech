---
created_at: "2026-10-23T00:00:38.915965180+00:00"
id: "astromech-8e8m"
issue_type: "task"
labels:
- "size-s"
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
status: "todo"
title: "CI: check that commits pushed to main were reviewed"
updated_at: "2026-10-23T00:00:38.915965180+00:00"
---

## Goal

A push to `main` that carries unreviewed code or a done claim made outside review fails CI, as `atelier land` on `main` refuses it locally before a push.

## Done when

On push runs, the Atelier workflow checks the commits between `github.event.before` and the pushed commit as base commits (e.g. a scratch upstream ref at `event.before` and doctor on branch `main`), or Atelier's doctor checks approvals for done claims already in history; a test push marking a leaf issue done without review fails the job, and CONTRIBUTING.md says so.

## Context

Review finding R1 (astromech-tjok) on task/astromech-mk30: `check_base_commits` (land_base_done_claim, land_base_unreviewed) runs only on branch main for unpushed commits, so the CI step skips it on its detached head. Reproduced by marking astromech-1b43 done in one commit: CI step passes.
