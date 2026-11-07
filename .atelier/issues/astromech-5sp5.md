---
created_at: "2026-11-07T11:28:47.118470266+00:00"
id: "astromech-5sp5"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: astromech-5sp5
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move the CI's Atelier pin to a binary that reads the records this host writes"
updated_at: "2026-11-07T11:28:47.568980904+00:00"
---

## Goal

astromech's Atelier CI job pins `ATELIER_REV` 1f5fd1c2, older than the binary every agent on this host now runs. That binary writes `## Report` sections, which the pinned one rejects, so the Atelier job fails on astromech-bp23's PR (run 42367751665) and on any branch with new records. Move the pin to atelier 55cf6072 (master of spadaval/atelier, part of mission atelier-mmpv there), which reads every record the installed binary writes.

## Done when

- [x] `.github/workflows/atelier.yml` pins `ATELIER_REV` to 55cf6072b9234df735138177370456fa828ce404, and nothing else changes.
- [x] The Atelier workflow passes on this branch's pull request with the new pin.
- [x] `atelier check` and `atelier doctor` pass locally with a binary built from that commit.

## Report

The Atelier CI job now pins spadaval/atelier 55cf6072b9234df735138177370456fa828ce404, and passes on this branch's PR (brewer-corp/astromech#217, run on b36548fc). `atelier check` and `atelier doctor` pass locally with the installed binary, whose code is identical to that commit (no change under `crates/` between d292ba31 and 55cf6072).

### Open items
- Atelier mission atelier-mmpv moves the pin again, at or past its light-path item (atelier-4nsl), before its rewritten skill goes live; astromech should not use the light path until then.

