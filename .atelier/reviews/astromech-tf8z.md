---
schema: atelier.review
schema_version: 1
id: astromech-tf8z
title: 'Reviewed git diff main...HEAD at b72a76cb, focused on fix commits 97d51bb1 (yfje) and 746d65b4 (ek06); the rest had no code change since review-agent-g3qt-1 (98e68277; merge 69b8c20e touched no crates). In a scratch copy: cargo test -p astromech-core -p astromech-data all pass (core lib 173); fmt --check clean; clippy -D warnings on core+data clean. Mutations: reverting edsm_bodies to ttl::SYSTEM fails a_count_refreshed_after_an_hour_is_edsms_new_one (1 vs 2 bodies); dropping .or_else(|_| e.metadata()) fails a_journal_that_cannot_be_opened_at_startup_is_still_a_replay_up_to_its_length (3 EDDN sends vs 1). R1 effects: edsm_bodies has one caller (known_bodies::EdsmBodies); KnownBodiesRule asks once per arrival plus ticks for 60 s, so EDSM volume rises only for revisits more than an hour apart (one request per such arrival, rate limited); cache expiry = fetch start + 3600 s is always past by the time Background''s KEEP_FOR (from completion) lapses, so the refresh reaches EDSM. Docs (ARCHITECTURE, guide/callouts) updated. Limits: Linux only; the ek06 test is cfg(unix) (runs on the ubuntu CI job, not Windows) and Windows file-sharing behaviour was judged by reading; cargo test --workspace not rerun.'
target: astromech-g3qt
identity: review-agent-g3qt-2
commit: b72a76cb2a16a77adc1bd664ddd800829e2cb69b
created_at: 2026-10-22T21:17:00.677514335Z
event:
  event: review
---

Reviewed git diff main...HEAD at b72a76cb, focused on fix commits 97d51bb1 (yfje) and 746d65b4 (ek06); the rest had no code change since review-agent-g3qt-1 (98e68277; merge 69b8c20e touched no crates). In a scratch copy: cargo test -p astromech-core -p astromech-data all pass (core lib 173); fmt --check clean; clippy -D warnings on core+data clean. Mutations: reverting edsm_bodies to ttl::SYSTEM fails a_count_refreshed_after_an_hour_is_edsms_new_one (1 vs 2 bodies); dropping .or_else(|_| e.metadata()) fails a_journal_that_cannot_be_opened_at_startup_is_still_a_replay_up_to_its_length (3 EDDN sends vs 1). R1 effects: edsm_bodies has one caller (known_bodies::EdsmBodies); KnownBodiesRule asks once per arrival plus ticks for 60 s, so EDSM volume rises only for revisits more than an hour apart (one request per such arrival, rate limited); cache expiry = fetch start + 3600 s is always past by the time Background's KEEP_FOR (from completion) lapses, so the refresh reaches EDSM. Docs (ARCHITECTURE, guide/callouts) updated. Limits: Linux only; the ek06 test is cfg(unix) (runs on the ubuntu CI job, not Windows) and Windows file-sharing behaviour was judged by reading; cargo test --workspace not rerun.
