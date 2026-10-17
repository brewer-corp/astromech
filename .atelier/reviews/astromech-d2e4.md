---
schema: atelier.review
schema_version: 1
id: astromech-d2e4
title: 'R1 [medium] crates/astromech-announce/src/session_summary.rs ends_session/react_all: the Music MainMenu trigger is not checked against a real journal. New journal files start with no commander (history lib.rs import), so a MainMenu line before the Commander event at game launch is harmless; but if the game writes Music MainMenu after Commander and before LoadGame at launch, an app started fresh (spoken=false) would arm, and History::session(at) would return the previous session (open-ended window, end >= ts because the line is attributed to that commander), speaking the last session''s summary again at launch. Recommend checking the line order in a real launch journal (needs-human) and, if it occurs, arming the MainMenu trigger only after a live LoadGame.'
target: astromech-oo8y
identity: review-agent (opus, independent)
commit: e9945e7abf52f8d8b47f45c1bc67e8733379a76b
created_at: 2026-10-17T05:12:58.162934002Z
event:
  event: finding
  recommendation: defer
---

R1 [medium] crates/astromech-announce/src/session_summary.rs ends_session/react_all: the Music MainMenu trigger is not checked against a real journal. New journal files start with no commander (history lib.rs import), so a MainMenu line before the Commander event at game launch is harmless; but if the game writes Music MainMenu after Commander and before LoadGame at launch, an app started fresh (spoken=false) would arm, and History::session(at) would return the previous session (open-ended window, end >= ts because the line is attributed to that commander), speaking the last session's summary again at launch. Recommend checking the line order in a real launch journal (needs-human) and, if it occurs, arming the MainMenu trigger only after a live LoadGame.
