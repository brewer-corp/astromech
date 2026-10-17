---
created_at: "2026-10-17T22:31:25.544034237+00:00"
id: "astromech-r3s7"
issue_type: "bug"
labels:
- "alpha"
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "A touched journal that is the latest by name but not the game's is live in full (repeated hour after a fall-back)"
updated_at: "2026-10-17T22:31:25.544034237+00:00"
---

## Goal

Journal liveness rule: the folder's latest journal by name stamp is the game's (Class::Game, every appended line live). In the repeated hour after a daylight-saving fall-back the game's newer session can sort before the earlier one: after a restart (Journal...T011000 after Journal...T013000), a tool that touches and appends to the first journal makes old lines live, because it is still the latest by name.

## Done when

A pipeline test with those two journals shows lines appended late to the first (013000) journal, with old timestamps, are a replay once the game is writing the second (011000); the 0d7l, 36g1, t16r and 782v tests keep passing.

## Context

Review finding on the astromech-782v PR (reproduced: the EDDN queue went from 1 to 2). The fix is a design change: for example treat latest-by-name as the game's only until another journal has been judged the game's by its header and has produced a fresh line, or gate latest-by-name journals by line freshness too (many older fixtures use fixed old stamps and rely on the unconditional rule). Narrow: it needs two game starts in the repeated hour and a sync tool touching the first journal (EU 25 Oct, US 1 Nov 2026).
