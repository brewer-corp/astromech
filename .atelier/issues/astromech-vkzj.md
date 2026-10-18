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
updated_at: "2026-10-18T01:43:05.869679296+00:00"
---

## Goal

Stop a single word that merely shares the crude phonetic key with a one-word key-pressing phrase from executing it. word_sim floors any key match at 0.85, which is exactly EXECUTE, and one-word phrases get no help from other words, so "boost" executes paste and "page", "peek", "pick", "pack" execute "back" (panel.close).

## Done when

A phonetic-key match alone cannot reach EXECUTE for a key-pressing command (for example, keep the floor below 0.85 when the edit similarity is low, or for one-word phrases); "boost" -> paste and "page" -> back ask or are rejected; the voice eval keeps 0 wrong executes and 64/64 correct rejects, and no command drops below its current score.

## Context

Review finding R1 of astromech-8997 (pre-existing on main). The reviewer counted, over ~58k single English words: 57 that execute panel.close, 47 that execute galaxy_map.paste ("boost", "best", "past", "post", "pushed"), 72 that execute request_docking (speaks only). "Boost" is everyday Elite vocabulary. Scratch data: /home/supadava/.cache/astromech-review/diff-8997/.
