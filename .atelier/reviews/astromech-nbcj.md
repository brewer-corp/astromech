---
schema: atelier.review
schema_version: 1
id: astromech-nbcj
title: finding on astromech-ki3j @6b1e09b8
target: astromech-ki3j
identity: review-agent-ki3j-1
commit: 6b1e09b8067d2575b27e0b2d43c86cefc9a6292c
created_at: 2026-11-04T20:55:37.365643139Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/pipeline.rs:600-606 while a touched older journal stays newest by mtime and is refused, every follow_newest call (each journal activity and Status.json notification, several per second in game, plus the 1 s idle scan) opens and reads the first line of the newest journal twice and of the followed journal once. On Windows the game's open journal's listing mtime can lag (as the code's own comments note), so this state can last a whole session. Cheap per call, but it is repeated file opens on journals that AV and sync tools also watch. Suggest caching the header time per path (the followed one's never changes) when convenient.
