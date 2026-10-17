---
created_at: "2026-10-17T05:58:02.281968993+00:00"
id: "astromech-z7ar"
evidence_type: "test"
commit: "f02beb56ce2cd9f6e69b068a202dbbfa894e166b"
result: "pass"
captured_at: "2026-10-17T05:58:02.281924049+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-zs2q"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-zs2q"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At f02beb5 (tree identical to the gate run): /tmp/landq/gates.sh exit 0 (fmt, clippy -D warnings, cargo test --workspace, check-deps, tauri metadata, pnpm lint/check/build/test 179 passed). New tests: astromech-data recorded.rs a_schema_error_is_one_spoken_sentence_without_the_serde_detail and expired_rows_are_pruned_so_the_cache_holds_only_live_answers (both seen failing before the fix), a_broken_cache_still_returns_the_network_answer (asserts the logged read/write failures), a_cache_locked_by_the_other_process_waits_for_it_then_caches (fails with busy_timeout 0); cache.rs unit tests for busy_timeout and the v0.1.5 table; astromech-core trade_voice.rs the_sessions_voice_lookups_and_star_map_share_one_client via session::remote_data and Client::shared_with_transport (fails when the star map gets its own client: 'Ardent asked twice within 1.5ms'; fails when Client::shared points elsewhere: cache len 0). Shell (app/src-tauri) not compiled on Linux; checked by reading, Windows CI builds it."
updated_at: "2026-10-17T05:58:02.282269043+00:00"
---

At f02beb5 (tree identical to the gate run): /tmp/landq/gates.sh exit 0 (fmt, clippy -D warnings, cargo test --workspace, check-deps, tauri metadata, pnpm lint/check/build/test 179 passed). New tests: astromech-data recorded.rs a_schema_error_is_one_spoken_sentence_without_the_serde_detail and expired_rows_are_pruned_so_the_cache_holds_only_live_answers (both seen failing before the fix), a_broken_cache_still_returns_the_network_answer (asserts the logged read/write failures), a_cache_locked_by_the_other_process_waits_for_it_then_caches (fails with busy_timeout 0); cache.rs unit tests for busy_timeout and the v0.1.5 table; astromech-core trade_voice.rs the_sessions_voice_lookups_and_star_map_share_one_client via session::remote_data and Client::shared_with_transport (fails when the star map gets its own client: 'Ardent asked twice within 1.5ms'; fails when Client::shared points elsewhere: cache len 0). Shell (app/src-tauri) not compiled on Linux; checked by reading, Windows CI builds it.
