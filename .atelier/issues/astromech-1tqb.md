---
created_at: "2026-11-08T18:51:23.938228715+00:00"
id: "astromech-1tqb"
issue_type: "feature"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-6ec9"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-7f11"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Shadow-decode push-to-talk clips with Parakeet Unified to compare it with TDT v2 in real play"
updated_at: "2026-11-08T18:51:23.938228715+00:00"
---

## Goal

Learn from real play whether Parakeet Unified would beat TDT v2 for astromech commands, at no risk: TDT v2 keeps acting, and Unified transcribes the same clip afterwards, with both transcripts saved next to the clip.

## Done when

- [ ] A `voice.shadow_stt` setting (off by default) loads Unified int8 in the parakeet-rs STT worker beside TDT v2.
- [ ] With it on, each push-to-talk clip is decoded by Unified after the TDT v2 command has run, and the clip sidecar written by `voice.save_clips` (`session.rs:596`) gets `shadow.transcript`, `shadow.decode_ms` and the command it would have run.
- [ ] A command lists the clips where the two disagree, and scores them once labelled, through `astromech_core::eval`.
- [ ] Unified is downloaded only when the setting is on; its URL and SHA-256 are in `catalog.rs`, and NOTICE.md carries the NVIDIA Open Model License notice.
- [ ] Unified never acts, and no new sherpa code is added.

## Context

- Evidence so far (docs/evidence/astromech-ggcx/): Unified transcribes better (WER 5.6% against 7.0% on Kokoro, 5.4% against 6.9% on cloned voice, level on real dictation) but misses more commands. It drops short first words in noise ("Gear up" heard as "Up."), returns nothing for some one-word commands, and twice flipped "off" to "on" on Kokoro (none on cloned voice).
- The shadow decode costs about 100 ms CPU per command after the action and about 650 MB RAM while loaded. Elite is CPU-bound, hence the setting.

## Unknowns

- Whether the shadow decode should wait until the game is idle on CPU, or run straight away.
