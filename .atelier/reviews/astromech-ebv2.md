---
schema: atelier.review
schema_version: 1
id: astromech-ebv2
title: 'R1 [medium] crates/astromech-gamestate/src/lib.rs logged_in: a crash (no Shutdown, no MainMenu) leaves logged_in true, so starting astromech after a crashed session arms the summary, and the next game launch''s MainMenu (in the Commander-before-MainMenu order astromech-d2e4 allows) speaks the crashed session''s totals. On main this case is silent (armed only by a live LoadGame), so the branch regresses the fresh-launch requirement. Confirmed with a temporary replay test (removed): catch-up LoadGame, then live Fileheader part 1, Commander, Music MainMenu, tick -> all three summary lines spoken; the real history would pass the end>=ts filter because the new file''s Commander and Music lines are attributed to the commander inside the still-open session window. Fix: clear logged_in on a Fileheader that is not a continuation (part absent or 1; a new game process), keep it across part>1 rollover, and add that replay test.'
target: astromech-627q
identity: review-agent (opus, independent)
commit: 4910e731b0539fd52abe0edc195f7fe40f8d6ebe
created_at: 2026-10-17T05:38:06.466801062Z
event:
  event: finding
  recommendation: fix_now
---

R1 [medium] crates/astromech-gamestate/src/lib.rs logged_in: a crash (no Shutdown, no MainMenu) leaves logged_in true, so starting astromech after a crashed session arms the summary, and the next game launch's MainMenu (in the Commander-before-MainMenu order astromech-d2e4 allows) speaks the crashed session's totals. On main this case is silent (armed only by a live LoadGame), so the branch regresses the fresh-launch requirement. Confirmed with a temporary replay test (removed): catch-up LoadGame, then live Fileheader part 1, Commander, Music MainMenu, tick -> all three summary lines spoken; the real history would pass the end>=ts filter because the new file's Commander and Music lines are attributed to the commander inside the still-open session window. Fix: clear logged_in on a Fileheader that is not a continuation (part absent or 1; a new game process), keep it across part>1 rollover, and add that replay test.
