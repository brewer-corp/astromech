---
created_at: "2026-10-15T20:36:13.655630401+00:00"
id: "astromech-ny26"
issue_type: "task"
labels:
- "size-s"
priority: "P0"
relationships:
  blocks:
  - kind: "issue"
    id: "astromech-w1q2"
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T20:36:54.533966583+00:00"
status: "done"
title: "My Powerplay standing"
updated_at: "2026-10-16T20:36:54.533966583+00:00"
---

## Goal

Power, rank, merits, time pledged, merits this week and this session, next rank and its reward.

## Done when

Replaying `Powerplay`, `PowerplayMerits` and `PowerplayRank` fixtures gives the expected numbers.

## Context

Journal only.

## Notes

- 2026-10-16 [note] Done in 51c7785. New crate astromech-powerplay (standing reducer + rules from data/catalog/powerplay.json, wiki-sourced). GameState.powerplay via new GameState::apply_line (hub/pipeline now pass the JournalLine so merits are dated). Voice: status.powerplay ('what's my Powerplay rank', 'powerplay status', 'how many merits'); normalise folds 'power play' to 'powerplay'. App: new top-level Powerplay tab, shell command powerplay_overview (journal replay cached on journal_version). Week merits = sum of MeritsGained since Thursday 07:00 UTC (community-reported tick); weekly assignment rewards aren't in the journal. The core's own week total only covers journals it replayed (current file); the app replays the whole folder. Shell code not compiled locally (Windows CI only).

## Close Reason

Replay of synthetic Powerplay/PowerplayMerits/PowerplayRank journals gives the expected standing (evidence astromech-ebxh, astromech-6dfw); awaiting independent review with the parent branch.
