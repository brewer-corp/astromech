---
created_at: "2026-10-22T20:56:50.711842849+00:00"
id: "astromech-gjr8"
evidence_type: "test"
commit: "500d718b7fcec6cc234d50d40bafc56ad7d08e44"
result: "pass"
captured_at: "2026-10-22T20:56:49.861795876+00:00"
command: "cargo test -p astromech-core --lib pipeline"
exit_status: "0"
agent_identity: "claude-implementer-g3qt"
target:
  kind: "issue"
  id: "astromech-01c0"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-01c0"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Pipeline test: a replayed Market trigger retried after the live half begins stays silent (fired Market. before the fix), a live one retried from idle still speaks; all pipeline tests pass"
updated_at: "2026-10-22T20:56:50.712138360+00:00"
---

## Summary

Pipeline test: a replayed Market trigger retried after the live half begins stays silent (fired Market. before the fix), a live one retried from idle still speaks; all pipeline tests pass

## Command

```console
cargo test -p astromech-core --lib pipeline
```

Exit status: 0

## Stdout

Bytes: 4140
Truncated: yes

```text

running 45 tests
test pipeline::tests::a_line_appended_to_the_open_journal_is_applied_when_idle ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_on_the_next_status_read ... ok
test pipeline::tests::a_pending_companion_event_is_loaded_when_idle ... ok
test pipeline::tests::a_pending_event_whose_file_never_comes_is_given_up_once ... ok
test pipeline::tests::a_dock_in_the_live_half_of_a_reopened_journal_still_gets_its_market_callout ... ok
test pipeline::tests::a_plotted_route_reaches_the_callouts_once_and_not_from_the_startup_read ... ok
test pipeline::tests::a_pending_market_event_is_retried_as_live_only_if_its_line_was ... ok
test pipeline::tests::a_conflict_copy_is_never_live_and_does_not_silence_the_game ... ok
test pipeline::tests::a_retry_never_runs_ahead_of_the_journal_lines_of_its_batch ... ok
test pipeline::tests::a_journal_left_while_empty_is_live_when_reopened ... ok
test pipeline::tests::a_journal_left_twice_is_split_at_the_latest_read_point ... ok
test pipeline::tests::a_journal_stamped_long_ago_is_not_taken_for_the_games ... ok
test pipeline::tests::a_reopened_journal_replays_only_what_was_read_before ... ok
test pipeline::tests::an_idle_pipeline_follows_a_new_journal_within_a_second ... ok
test pipeline::tests::a_new_scheme_journal_started_after_an_old_scheme_one_is_live ... ok
test pipeline::tests::backpack_json_is_read_again_after_a_backpack_change ... ok
test pipeline::tests::a_new_session_named_earlier_than_the_followed_one_is_live ... ok
test pipeline::tests::a_gated_batch_with_a_stale_line_then_a_fresh_one_makes_only_the_fresh_one_live ... ok
test pipeline::tests::companion_file_is_read_after_its_event ... ok
test pipeline::tests::callouts_are_silent_for_the_startup_catch_up_and_live_after_it ... ok
test pipeline::tests::a_journal_started_in_the_repeated_hour_after_the_clocks_go_back_is_live ... ok
test pipeline::tests::following_a_journal_that_is_not_the_latest_is_logged_once ... ok
test pipeline::tests::a_later_part_of_a_session_judged_the_games_is_live ... ok
test pipeline::tests::missing_or_stale_file_is_logged_then_used_once_the_game_writes_it ... ok
test pipeline::tests::a_restored_journal_written_late_stays_a_replay_whatever_its_header_says ... ok
test pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay ... ok
test pipeline::tests::an_older_journal_touched_later_is_read_as_a_replay ... ok
test pipeline::tests::an_older_journal_written_after_we_follow_it_is_still_a_replay ... ok
test pipeline::tests::a_touched_first_journal_of_the_repeated_hour_is_not_live_once_the_second_is ... ok
test pipeline::tests::an_earlier_session_is_not_live_once_a_newer_one_has_started ... ok
test pipeline::tests::eddn_gets_nothing_from_the_startup_replay_and_live_lines_after_it ... ok
test pipeline::tests::the_header_agrees_with_the_name_across_daylight_saving ... ok
test pipeline::tests::gated_lines_are_judged_one_by_one ... ok
test pipeline::tests::the_app_starting_in_the_repeated_hour_follows_the_games_journal_live ... ok
test pipeline::tests::the_game_running_before_we_start_is_live_beside_a_later_foreign_journal ... ok
test pipeline::tests::the_games_journal_first_seen_long_after_it_started_is_live ... ok
test pipeline::tests::the_games_journal_is_a_replay_up_to_startup_after_a_catch_up_on_a_touched_older_one ... ok
test pipeline::tests::the_games_journal_stays_live_beside_a_foreign_one_stamped_later ... ok
test pipeline::tests::the_games_journal_followed_while_empty_is_live_once_written ... ok
test pipeline::tests::outfitting_file_is_kept_when_fresh_and_only_then ... ok
test pipeline::tests::the_startup_replay_of_a_gated_journal_with_fresh_lines_is_silent ... ok
test pipeline::tests::the_games_journal_takes_the_session_back_from_a_touched_foreign_one ... ok
test history::tests::a_stuck_backfill_never_blocks_the_pipeline ... ok
test pipeline::tests::a_stream_of_unrelated_notifications_does_not_starve_the_journal_read ... ok
test pipeline::tests::the_pipeline_loop_reads_the_journal_without_any_notification ... ok

test result: ok. 45 passed; 0 failed; 0 ignored; 0 mea
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-badbc78a43d6af96)
```

