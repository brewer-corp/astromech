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
  - kind: "issue"
    id: "astromech-v3u4"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Run the one-process probe's restart mode on Sumeet's PC"
updated_at: "2026-11-07T14:07:11.000043748+00:00"
---

## Goal

Learn whether Windows restart registration restarts a crashed astromech on a real Windows 10/11 desktop, before astromech-v3u4 removes the shell's watchdog, so the watchdog question is settled early rather than after merge.

## Done when

- [ ] On Sumeet's PC (not in game is fine), the spike built from branch astromech-y6d6 runs through the probe's restart mode, and its `restart.md` is attached here as an evidence receipt.
  - **The exe:** `astromech-spike.exe` from the `astromech-spike-exe` artifact of the "One process (Windows)" workflow run on that branch: run 42384570621 (head b418f604; https://github.com/brewer-corp/astromech/actions/runs/42384570621, artifact id 13579199136, expires 2027-02-05), or `cargo build --release` in `spikes/o1-one-process`.
  - **Prerequisites:** Python 3 with `pip install psutil pillow`, and a checkout of the branch for `scripts/one_process_probe.py`.
  - **The command:** `python scripts/one_process_probe.py --restart --app <exe> --endpoint %LOCALAPPDATA%\astromech-spike\core.json --out <dir> --config "Sumeet PC"`. It crashes the spike with abort, access_violation and main_thread_panic after 65 s, ends it with TerminateProcess, and aborts it after 10 s, then waits 90 s each for a restart: about 13 minutes.
- [ ] If neither crash kind restarts with WER at its defaults, astromech-dl53's stop rule applies: the watchdog question (a third process, against success criterion one-process) goes to Sumeet as a decision.

## Context

The spike (astromech-wgtr, evidence astromech-fqw9) could not show a restart on the GitHub Windows runner: no restart, WER report or WerFault in 15 crashes across three WER configurations. Raised in the wgtr review as an open question.
