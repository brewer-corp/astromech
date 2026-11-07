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

- [x] `.github/workflows/atelier.yml` pins `ATELIER_REV` to 12efdfd9ce899e5a768a82163e0758e227b686f6, and nothing else changes.
- [x] The Atelier workflow passes on this branch's pull request with the new pin, its log showing 12efdfd9 being built.
- [x] `atelier check` and `atelier doctor` pass locally with a binary built from that commit.

## Report

The Atelier CI job now pins spadaval/atelier 12efdfd9ce899e5a768a82163e0758e227b686f6, the merge of its light-path item, which reads every record form the binary on the dev host writes. On PR #217 the `atelier doctor` job built atelier-cli at 12efdfd9 from scratch and passed (run 42369239411). `atelier check` and `atelier doctor` pass locally with the installed binary 0.3.1+12efdfd9.

### Deviations
- The first pin commit (b36548fc) matched an older pin in a stale checkout and changed nothing, and a later edit cut this record's Done when; the reviewer caught the first (astromech-vzcs, -rmk5, -4lt6) and CI caught the second. Both are fixed.

### Open items
- The PR's general Linux CI job fails in the frontend tests (`src/lib/palette.test.ts`, 1 failed), which this one-line workflow change cannot affect; main's own CI run 42352408463 was already failing.
