---
schema: atelier.review
schema_version: 1
id: astromech-iqwt
title: 'R2 [medium] Proof gap: no test covers a catch-up that ends in Shutdown (quit to desktop) and nothing unit-tests GameState::logged_in. Mutating the Shutdown clear (event == "Shutdown" -> never matches) leaves all astromech-announce and astromech-gamestate tests green (checked); only the MainMenu-ended catch-up is tested. Add a replay test: catch-up LoadGame then Shutdown, live fresh launch menu (Commander, MainMenu, Shutdown) says nothing; and gamestate tests for logged_in across LoadGame, MainMenu, Shutdown and the Fileheader rule from R1.'
target: astromech-627q
identity: review-agent (opus, independent)
commit: 4910e731b0539fd52abe0edc195f7fe40f8d6ebe
created_at: 2026-10-17T05:38:06.556233993Z
event:
  event: finding
  recommendation: fix_now
---

R2 [medium] Proof gap: no test covers a catch-up that ends in Shutdown (quit to desktop) and nothing unit-tests GameState::logged_in. Mutating the Shutdown clear (event == "Shutdown" -> never matches) leaves all astromech-announce and astromech-gamestate tests green (checked); only the MainMenu-ended catch-up is tested. Add a replay test: catch-up LoadGame then Shutdown, live fresh launch menu (Commander, MainMenu, Shutdown) says nothing; and gamestate tests for logged_in across LoadGame, MainMenu, Shutdown and the Fileheader rule from R1.
