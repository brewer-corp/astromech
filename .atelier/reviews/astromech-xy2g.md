---
schema: atelier.review
schema_version: 1
id: astromech-xy2g
title: finding on astromech-ki3j @6b1e09b8
target: astromech-ki3j
identity: review-agent-ki3j-1
commit: 6b1e09b8067d2575b27e0b2d43c86cefc9a6292c
created_at: 2026-11-04T20:55:37.092672311Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-core/src/pipeline.rs:605 follow_newest never switches to a journal whose Fileheader is earlier than the followed one's, so a followed journal with a header in the future pins the pipeline for the rest of the run. Shapes: a synced journal from a PC whose clock runs ahead (astromech-36g1 shape 2 names a skewed clock) arrives newest by mtime with a later header and is switched to; or this PC's clock is corrected backwards (NTP after a dual-boot RTC offset) so the game's next session or part begins 'before' the one followed. The game's own journal is then never read again until a journal begins after the bad header: state, callouts, route moves and EDDN all stop silently. Before this change the game's next write took the session back (the old test the_games_journal_takes_the_session_back_from_a_touched_foreign_one, now rewritten with an earlier header so it no longer covers this). Fails closed (nothing is wrongly live), so the safety property holds. The decision note names only two concurrent instances as the trade-off. Suggest a follow-up issue: e.g. treat a header later than now plus a margin as untrusted when comparing, or let a journal that classifies as the game's (fresh lines) take the session back; and widen the decision note's trade-off.
