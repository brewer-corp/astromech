---
schema: atelier.review
schema_version: 1
id: astromech-gmmg
title: finding on astromech-zftk @b98a8650
target: astromech-zftk
identity: review-agent-zftk-1
commit: b98a8650910957a8ba247ee436f87d8ccc6b1244
created_at: 2026-11-04T23:08:39.750067701Z
event:
  event: finding
  recommendation: no_action
---

low: crates/astromech-core/src/pipeline.rs:413-418,637-650 open() now returns early when the file can't be opened, but follow_newest has already bumped stats.rollovers, flushed EDDN and set opened[old]=Some(committed) for the journal still followed. While the game's new journal stays unreadable this repeats once a second: the rollover count is inflated and EDDN's pending signal run is flushed early. Nothing goes live wrongly and no line is lost (the old journal keeps being read, and opened is rewritten on the real switch). Also a small window remains between open()'s readability probe and JournalTail::read_to's own open; if the lock lands exactly there, the old reader-at-0 behaviour returns. Both are cosmetic or vanishingly rare. If this code is touched again, probe readability in follow_newest before the bookkeeping.
