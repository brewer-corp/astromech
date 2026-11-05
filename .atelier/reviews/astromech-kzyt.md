---
schema: atelier.review
schema_version: 1
id: astromech-kzyt
title: finding on astromech-ki3j @6b1e09b8
target: astromech-ki3j
identity: review-agent-ki3j-1
commit: 6b1e09b8067d2575b27e0b2d43c86cefc9a6292c
created_at: 2026-11-04T20:55:50.614885706Z
event:
  event: finding
  recommendation: no_action
---

low: crates/astromech-core/src/pipeline.rs:1908-1944 and 1636-1648 a_restored_journal_written_late_stays_a_replay_whatever_its_header_says and an_older_journal_written_after_it_appears_is_not_followed no longer reach classify for a restored journal (header agrees with name, old lines; dropped assert!(!t.live)), because the journal is now never followed. That path is still reachable when the followed journal has no Fileheader (an empty game journal followed at startup: begins_before is false, so any headered newest is switched to, and opened as Gated with may_be_live). The freshness gate it relies on is still covered by gated_lines_are_judged_one_by_one and a_gated_batch_with_a_stale_line_then_a_fresh_one_makes_only_the_fresh_one_live, so no action needed; a test of that exact shape would pin it if this code changes again.
