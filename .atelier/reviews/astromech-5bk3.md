---
schema: atelier.review
schema_version: 1
id: astromech-5bk3
title: 'Reviewed git diff main...HEAD at 7974320 against the issue. v5 migration: partial expression index built in the existing Immediate-transaction migrate; on a synthetic 900k-row/700 MB events table CREATE INDEX took 0.25 s (warm cache), one-off; the app''s read-only open refuses the schema mismatch until the core migrates (unchanged behaviour). EXPLAIN QUERY PLAN of landings_before''s SQL: SEARCH events USING INDEX events_landing_system (<expr>=? AND ts<?), with and without ANALYZE. SystemViewCache signature (path+length via open handle of the two newest journals) catches new file, growth, rollover and truncation; UnsoldCache keys on db+WAL length/mtime and commander. main.rs read for compile errors: Manager imported, AppHandle moved into blocking, types line up; Windows CI green. cargo test -p astromech-history -p astromech-exploration -p astromech-journal all pass; landings_before_is_fast_with_a_large_history passed 15/15 in debug at load average ~11. Two NO ACTION findings. Note for integration: astromech-0n4h''s system_view needs the live SystemFold, so whichever of the two lands second must make this cache hold the fold (finding recorded on astromech-0n4h).'
target: astromech-dt5h
identity: review-agent (opus, independent)
commit: 797432034fbbec3d2aa0eaa7a812437b9455afe2
created_at: 2026-10-17T08:00:13.836417310Z
event:
  event: approval
---

Reviewed git diff main...HEAD at 7974320 against the issue. v5 migration: partial expression index built in the existing Immediate-transaction migrate; on a synthetic 900k-row/700 MB events table CREATE INDEX took 0.25 s (warm cache), one-off; the app's read-only open refuses the schema mismatch until the core migrates (unchanged behaviour). EXPLAIN QUERY PLAN of landings_before's SQL: SEARCH events USING INDEX events_landing_system (<expr>=? AND ts<?), with and without ANALYZE. SystemViewCache signature (path+length via open handle of the two newest journals) catches new file, growth, rollover and truncation; UnsoldCache keys on db+WAL length/mtime and commander. main.rs read for compile errors: Manager imported, AppHandle moved into blocking, types line up; Windows CI green. cargo test -p astromech-history -p astromech-exploration -p astromech-journal all pass; landings_before_is_fast_with_a_large_history passed 15/15 in debug at load average ~11. Two NO ACTION findings. Note for integration: astromech-0n4h's system_view needs the live SystemFold, so whichever of the two lands second must make this cache hold the fold (finding recorded on astromech-0n4h).
