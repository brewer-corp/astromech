---
created_at: "2026-10-23T00:08:31.390679291+00:00"
id: "astromech-58a7"
issue_type: "task"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-58a7
priority: "P3"
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
closed_at: "2026-11-04T22:14:40.878166605+00:00"
status: "done"
title: "Warn when stations list outfitting modules astromech doesn't know"
updated_at: "2026-11-04T22:14:40.878166605+00:00"
---

## Goal

After a game update adds modules, astromech's maintainers learn quickly that outfitting messages are being refused for unknown modules, instead of only a per-visit Diagnostics line.

## Done when

The real-journal harness reports unknown outfitting symbols as problems, and a scheduled or release-time check (or the release checklist) re-runs scripts/gen-eddn-outfitting.py against the latest FDevIDs and fails on new symbols; documented in CONTRIBUTING or the release notes process.

## Context

Review finding R3 (astromech-tbrk) on task/astromech-dlrb: crates/astromech-eddn/src/companion.rs:254 refuses a station's whole outfitting message when a module is in neither list.

## Close Reason

Implemented at 25b99687 (plus data regen 9f08b20f). scripts/gen-eddn-outfitting.py --check <FDevIDs checkout> compares the checkout's station/commander lists with data/catalog/eddn-outfitting.json and exits 1 listing new, moved and gone symbols (fails on any difference, not only new symbols: any difference means the file is stale). Tested offline by scripts/test_gen_eddn_outfitting.py (fixture outfitting.csv built from the committed file), added to ci.yml. New .github/workflows/eddn-outfitting.yml runs the check against FDevIDs' default branch every Monday 06:17 UTC and on workflow_dispatch; it has not run on GitHub yet. companion::unknown_modules (pub) lists every module in an Outfitting.json that would refuse the visit; modules() and it now share one owner() classification (behaviour unchanged, all 152 eddn tests pass). The real-journal harness's EDDN section reports any as a PROBLEM, reading the folder's Outfitting.json directly, so it reports even when the run did not send that file. Documented in CONTRIBUTING (Before a PR list). Surprise: the network run found the committed file already stale against FDevIDs@c7c29f7 (int_largecargorack_size6_class1, a station module, and six Anaconda Mk II armour symbols), so outfitting/2 has been refused at stations selling a size 6 class 1 cargo rack; regenerated in a separate commit 9f08b20f so it can be reviewed or dropped on its own. Proof: evidence astromech-znvd and two validation receipts (synthetic harness folder; network check). Not run: the harness on a real commander folder; full workspace tests (only touched crates). Review outstanding.
