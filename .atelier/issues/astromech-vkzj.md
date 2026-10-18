---
created_at: "2026-10-18T01:00:41.224826830+00:00"
id: "astromech-vkzj"
issue_type: "task"
labels:
- "alpha"
- "voice"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-vkzj
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Voice: a phonetic-key match alone executes one-word key presses ('boost' -> paste, 'page' -> back)"
updated_at: "2026-10-18T02:43:43.803144868+00:00"
---

## Goal

Stop a single word that merely shares the crude phonetic key with a one-word key-pressing phrase from executing it. word_sim floors any key match at 0.85, which is exactly EXECUTE, and one-word phrases get no help from other words, so "boost" executes paste and "page", "peek", "pick", "pack" execute "back" (panel.close).

## Done when

A phonetic-key match alone cannot reach EXECUTE for a key-pressing command (for example, keep the floor below 0.85 when the edit similarity is low, or for one-word phrases); "boost" -> paste and "page" -> back ask or are rejected; the voice eval keeps 0 wrong executes and 64/64 correct rejects, and no command drops below its current score.

## Context

Review finding R1 of astromech-8997 (pre-existing on main). The reviewer counted, over ~58k single English words: 57 that execute panel.close, 47 that execute galaxy_map.paste ("boost", "best", "past", "post", "pushed"), 72 that execute request_docking (speaks only). "Boost" is everyday Elite vocabulary. Scratch data: /home/supadava/.cache/astromech-review/diff-8997/.

## Notes

- 2026-10-17 [note] Method and proof (review R2). Option chosen: route a sound-alike-only match to 'Did you mean' for key-pressing commands whose phrase has a single content word (longer than two letters), rather than requiring an exact match for one-word phrases only (missed 'post it', 'go page') or asking on every sound-alike (turned 18 correctly executing eval transcripts, 47 clips, into questions: 'Coms panel', 'Roll panel', 'Open cargo skeep.', 'Hand break off.'). Word list: 59,332 lines = zxcvbn english_wikipedia + us_tv_and_film + surnames (alphabetic) + the 200 eval phrases + phonetic variants (the astromech-8997 reviewer's gen.py), plus 116,734 'X it' / 'go X' pairs from its single words; each line through astromech_core::assistant::grammar().best() by a scratch probe example (not committed). Single-word inputs executing a key press: 104 -> 0 (57 back, 47 paste). Multi-word lines: 132 -> 131. 'X it'/'go X': 147 -> 32; the 32 left are close spellings, not key matches ('go black', 'haste it', 'pasted it'). Every change is Execute -> Ask; nothing newly executes. List: docs/evidence/astromech-vkzj/word-list-changes.tsv. Voice eval: 3115/3136 before and after, 0 wrong executes, 64/64 rejects; lost correct executes: 'Pazed.' (bm_george clean and cockpit, paste) now asks.
