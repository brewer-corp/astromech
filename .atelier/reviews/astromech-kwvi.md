---
schema: atelier.review
schema_version: 1
id: astromech-kwvi
title: 'Re-reviewed 5de26c9 (merge of main with dt5h plus fixes). Every shell call checked against the crates: SystemViewCache::fold, History::system_view(SystemFold), SystemFold Clone/view/absorb, History::open_read_only all exist with matching signatures; the Ok/Err match arms move live once each. Cache holds the fold, system_view runs every poll; a_cached_poll_still_shows_the_earlier_bodies passes with parses()==1, and dt5h''s unchanged-journal test adapted to fold() passes. v6 is a plain CREATE INDEX (partial, idempotent on existing rows); EXPLAIN QUERY PLAN shows SEARCH events USING INDEX events_signal_system; system_lines_are_fast_with_many_signal_rows passes (<5 ms at 100k rows). cargo test -p astromech-history -p astromech-exploration all pass. PR 92 CI on the pushed head b805161: Linux, Windows MSVC build + app shell, Windows installer all pass.'
target: astromech-0n4h
identity: review-agent (sonnet, independent)
commit: 33ab3a02e629a30612d5ef2cc743d11e4034df96
created_at: 2026-10-17T08:27:32.418437703Z
event:
  event: approval
---

Re-reviewed 5de26c9 (merge of main with dt5h plus fixes). Every shell call checked against the crates: SystemViewCache::fold, History::system_view(SystemFold), SystemFold Clone/view/absorb, History::open_read_only all exist with matching signatures; the Ok/Err match arms move live once each. Cache holds the fold, system_view runs every poll; a_cached_poll_still_shows_the_earlier_bodies passes with parses()==1, and dt5h's unchanged-journal test adapted to fold() passes. v6 is a plain CREATE INDEX (partial, idempotent on existing rows); EXPLAIN QUERY PLAN shows SEARCH events USING INDEX events_signal_system; system_lines_are_fast_with_many_signal_rows passes (<5 ms at 100k rows). cargo test -p astromech-history -p astromech-exploration all pass. PR 92 CI on the pushed head b805161: Linux, Windows MSVC build + app shell, Windows installer all pass.
