---
created_at: "2026-11-07T14:07:11.000043748+00:00"
id: "astromech-fmub"
issue_type: "task"
labels:
- "needs-human"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "evidence"
    id: "astromech-fqw9"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run the one-process probe's restart mode on Sumeet's PC"
updated_at: "2026-11-07T14:07:11.000043748+00:00"
---

## Goal

Learn whether Windows restart registration restarts a crashed astromech on a real Windows 10/11 desktop, before astromech-v3u4 removes the shell's watchdog, so the watchdog question is settled early rather than after merge.

## Done when

- [ ] On Sumeet's PC (not in game is fine), with the spike built from branch astromech-y6d6 (the `one-process` workflow uploads it, or `cargo build --release` in `spikes/o1-one-process`), `python scripts/one_process_probe.py --restart ...` runs the abort and access-violation crashes after 65 s, and its `restart.md` is attached here as an evidence receipt.
- [ ] If neither crash kind restarts with WER at its defaults, astromech-dl53's stop rule applies: the watchdog question (a third process, against success criterion one-process) goes to Sumeet as a decision.

## Context

The spike (astromech-wgtr, evidence astromech-fqw9) could not show a restart on the GitHub Windows runner: no restart, WER report or WerFault in 15 crashes across three WER configurations. Raised in the wgtr review as an open question.
