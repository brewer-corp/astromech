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
closed_at: "2026-10-18T13:42:12.120954651+00:00"
status: "done"
title: "Voice: a phonetic-key match alone executes one-word key presses ('boost' -> paste, 'page' -> back)"
updated_at: "2026-10-18T13:42:12.120954651+00:00"
---

## Goal

A match that needed the crude phonetic key for any word (spelling alone under 0.85, the shared key lifting it to 0.85, the execute threshold) must never execute, for any command, not only key presses. It is a veto, not a score floor: scoring, ranking and margins stay as on main, and a sound-alike winner's Execute becomes Ask for the same command and slot. Before this, "boost" executed paste, "page", "peek", "pick" and "pack" executed "back" (panel.close), and "show chalk maeby" opened the galaxy map.

## Done when

Nothing newly executes compared with main: the only difference from main is Execute -> Ask with the same command, slot and score, checked line by line over the probe corpora (docs/evidence/astromech-vkzj/main-vs-head.txt); "boost" -> paste and "page" -> back ask; the voice eval keeps 0 wrong executes and 64/64 correct rejects.

## Context

Review finding R1 of astromech-8997 (pre-existing on main). The reviewer counted, over ~58k single English words: 57 that execute panel.close, 47 that execute galaxy_map.paste ("boost", "best", "past", "post", "pushed"), 72 that execute request_docking (speaks only). "Boost" is everyday Elite vocabulary. Scratch data: /home/supadava/.cache/astromech-review/diff-8997/.

## Notes

- 2026-10-17 [note] Method and proof (review R2). Option chosen: route a sound-alike-only match to 'Did you mean' for key-pressing commands whose phrase has a single content word (longer than two letters), rather than requiring an exact match for one-word phrases only (missed 'post it', 'go page') or asking on every sound-alike (turned 18 correctly executing eval transcripts, 47 clips, into questions: 'Coms panel', 'Roll panel', 'Open cargo skeep.', 'Hand break off.'). Word list: 59,332 lines = zxcvbn english_wikipedia + us_tv_and_film + surnames (alphabetic) + the 200 eval phrases + phonetic variants (the astromech-8997 reviewer's gen.py), plus 116,734 'X it' / 'go X' pairs from its single words; each line through astromech_core::assistant::grammar().best() by a scratch probe example (not committed). Single-word inputs executing a key press: 104 -> 0 (57 back, 47 paste). Multi-word lines: 132 -> 131. 'X it'/'go X': 147 -> 32; the 32 left are close spellings, not key matches ('go black', 'haste it', 'pasted it'). Every change is Execute -> Ask; nothing newly executes. List: docs/evidence/astromech-vkzj/word-list-changes.tsv. Voice eval: 3115/3136 before and after, 0 wrong executes, 64/64 rejects; lost correct executes: 'Pazed.' (bm_george clean and cockpit, paste) now asks.
- 2026-10-17 [note] Round-2 fix: a sound-alike counts as backed up only by a content word (longer than two letters) or exact slot heard on its spelling. Reviewer's 276 all-sound-alike inputs ('rice care', 'chalk maeby'): Execute 276 -> 0; 709 one-sound-alike-plus-real-word inputs: still execute. 59k list, 'X it'/'go X' pairs and the 592 eval transcripts: identical to the previous round. Voice eval at 66f4cced: 3115/3136, 0 wrong executes, 64/64 rejects, misses 13; only 'Pazed.' (2 clips) lost its execute. Known limit (astromech-8f5g): particles longer than two letters ('down', 'off') count as backing words.
- 2026-10-18 [note] Change of direction (Sumeet, via the coordinator): the one-word, lone-sound-alike and backed rules are removed. One rule: if any phrase word matched only through the phonetic key (spelling alone under 0.85), the phrase score is capped at SOUND_CAP 0.80, so it can ask but never execute. Applied to every command, not only key presses: the eval shows no regression either way (all-commands: 81 clips execute -> ask; key-press commands alone would be 47), and one rule is simpler than a per-command predicate. Re-scored cached transcripts, clips execute -> ask per family: ui 34, status 16, route 15, ship 5, srv 4, faction 2, powerplay 2, trade 2, confirm 1; no new miss, wrong ask or wrong execute; rejects 64/64. Fresh voice-eval at 8e2ceaea: 3115/3136, 0 wrong executes, 64/64 rejects, misses 13. Word lists (key-press executes before -> after): 59k words 237 -> 70 (single words 104 -> 0; all executes 1019 -> 257); 'X it'/'go X' pairs 147 -> 32; reviewer's all-sound-alike 276 -> 0; one-sound-alike 709 -> 1; round-3 dictionary-swap probe 2024 -> 1. The two left ('fall stop' -> all stop at 0.875) are close spellings, not key matches. Nothing newly executes. astromech-8f5g superseded (nothing backs a sound-alike any more; 'car down' asks).
- 2026-10-18 [note] Switch to a pure veto (decided by the orchestrator after round 6; 73d4e007). This supersedes the SOUND_CAP 0.80 note above: scoring, ranking, margins, slots, aliases and the mishearing path are main's unchanged; best() only turns a winner's Execute into Ask when its phrase matched some word solely through the phonetic key, keeping the command, slot and score. Line by line against main (docs/evidence/astromech-vkzj/main-vs-head.txt, 987,531 lines over 12 sets: 59k words, 'X it'/'go X' pairs, round-2 all/one-sound-alike, round-3 probe, eval transcripts, round-4 one-edit and corpora, round-5 slot probe and two-edit, round-6 adversarial and aliases), the only difference is Execute -> Ask with the same command, slot and score; 0 other differences. Execute -> Ask counts: words 762, pairs 115, round-2 all-sound-alike 276, one-sound-alike 709, round-3 probe 2,024 (including 'fall stop', which the veto build asks), eval transcripts 35, round-4 one-edit 12,530, round-4 corpora 15,387, slot probe 29,466, two-edit 17,817, adversarial 2,976, aliases 41. Eval clips: 81 Execute -> Ask, 0 wrong executes, 64/64 rejects, misses 13. word-list-changes.tsv regenerated from the veto build (3,885 rows).
- 2026-10-18 [note] Goal and Done when restated to match what was decided (round 7 R2): the rule covers all commands, it is a veto rather than a score floor, and 'no command drops below its current score' is replaced by 'nothing newly executes compared with main; only Execute -> Ask with the same command, slot and score; eval 0 wrong executes and 64/64 rejects'. The round-7 R3 follow-up (an Ask-level sound-alike of 'yes' confirms a pending question; pre-existing on main) is filed as astromech-m2bl.

## Close Reason

Pure veto: a winning match that used a sound-only word asks instead of executing. Nothing newly executes vs main (987k probe lines, only Execute->Ask same command/slot/score); eval 0 wrong executes, 64/64 rejects. Follow-ups: astromech-clju (recogniser uses grammar), astromech-m2bl (yes sound-alike).
