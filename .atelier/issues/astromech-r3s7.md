---
created_at: "2026-10-17T22:31:25.544034237+00:00"
id: "astromech-r3s7"
issue_type: "bug"
labels:
- "alpha"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-r3s7
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-17T23:21:11.711081544+00:00"
status: "done"
title: "A journal once judged the game's stays live after a newer session starts"
updated_at: "2026-10-17T23:21:11.711081544+00:00"
---

## Goal

Journal liveness rule: the folder's latest journal by name stamp is the game's (Class::Game, every appended line live). In the repeated hour after a daylight-saving fall-back the game's newer session can sort before the earlier one: after a restart (Journal...T011000 after Journal...T013000), a tool that touches and appends to the first journal makes old lines live, because it is still the latest by name.

## Done when

A pipeline test with those two journals shows lines appended late to the first (013000) journal, with old timestamps, are a replay once the game is writing the second (011000); the 0d7l, 36g1, t16r and 782v tests keep passing.

## Context

Review finding on the astromech-782v PR (reproduced: the EDDN queue went from 1 to 2). The fix is a design change: for example treat latest-by-name as the game's only until another journal has been judged the game's by its header and has produced a fresh line, or gate latest-by-name journals by line freshness too (many older fixtures use fixed old stamps and rely on the unconditional rule). Narrow: it needs two game starts in the repeated hour and a sync tool touching the first journal (EU 25 Oct, US 1 Nov 2026).

## Notes

- 2026-10-17 [note] 17 Oct, re-review of #149: reproduced with no clock change. The app follows A live; the game starts B (later name); a tool appends an old-stamped line to A and touches it; EDDN queue 1 -> 2. Cause: game_sessions (pipeline.rs ~203, 414-419) is never cleared, and a reopen applies the part past the read point as live. Fix: stop treating an older session as the game's once a newer one is, or check line freshness for any journal that isn't the current one. Same class as 0d7l; not an alpha blocker (needs a tool appending different content to an earlier journal while the app spans two sessions).
- 2026-10-17 [note] Chosen: one current session. game_session is the stamp of the session the game is writing; starting a newer one (a journal that is the latest by name the first time we see it, or a header-matching journal's first fresh line) retires the previous for good, so it is judged like any other journal (header agrees: only lines stamped now are live; otherwise a replay). Rejected: gating every non-current journal on line freshness without retiring, because the latest-by-name rule still made a touched first journal of the repeated hour live. Limits found in review: (1) a foreign later-named journal touched while the game is quiet still becomes the session and its stale lines are live until the game's next fresh line takes it back (pre-existing 0d7l behaviour, now bounded; test the_games_journal_takes_the_session_back_from_a_touched_foreign_one); (2) a retired game journal whose header disagrees with its name (a machine whose offset changed mid-session) stays Foreign: rare, not changed; (3) after a pause over 120 s the game's catch-up lines of a retired-then-recovered journal count as replay (the 36g1/t16r trade-off).

## Close Reason

One current session: starting a newer one retires the earlier for good, so a touched earlier journal, including the repeated hour's latest-by-name first journal, is judged like any other (stale lines replay). Tests: an_earlier_session_is_not_live_once_a_newer_one_has_started failed before (queue 2 to 3); the repeated-hour test fails without retirement; a flip-flop test. 0d7l, 36g1, t16r and 782v tests unchanged; 50 of 50 under load. Review recorded.
