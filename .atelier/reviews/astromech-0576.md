---
schema: atelier.review
schema_version: 1
id: astromech-0576
title: finding on astromech-zftk @87957374
target: astromech-zftk
identity: review-agent-zftk-1
commit: 87957374e2e0aa1cf0537d7dd22d097cdd88ce63
created_at: 2026-11-04T22:37:17.360130231Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/pipeline.rs:705-712 Pre-existing, not introduced here. The handle measurement covers only the latest-by-name and followed journals. When the game's current journal is neither (a foreign journal stamped later is latest by name and newest by mtime, the scenario of a_line_the_game_writes_to_its_journal_during_the_catch_up_is_live), it is measured by the listing, which on Windows lags the open file; lines in that gap are pre-startup history but are read past the recorded length when it is next followed, and as Class::Gated those stamped within FRESH_LINE_SECS (120 s) of now go live. Blast radius is bounded to about the last two minutes before startup. Suggested follow-up: also measure through a handle any journal whose name stamp is within the last day (a handful of files, so startup cost on years of journals stays one listing plus a few opens).
