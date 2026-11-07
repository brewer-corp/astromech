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

astromech's Atelier CI job pins `ATELIER_REV` 523a6502, older than the binary every agent on this host now runs. That binary writes `## Report` sections, which the pinned one rejects, so the Atelier job fails on astromech-bp23's PR (run 42367751665) and on any branch with new records. Move the pin to atelier 12efdfd9 (master of spadaval/atelier, the merge of its light-path item atelier-4nsl in mission atelier-mmpv), which reads every record form the installed binary writes, including the light path's version-3 review records.

## Done when

- [ ] `.github/workflows/atelier.yml` pins `ATELIER_REV` to 12efdfd9ce899e5a768a82163e0758e227b686f6, and nothing else changes.
- [ ] The Atelier workflow passes on this branch's pull request with the new pin, its log showing 12efdfd9 being built.
- [ ] `atelier check` and `atelier doctor` pass locally with a binary built from that commit.
