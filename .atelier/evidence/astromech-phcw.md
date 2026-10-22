---
created_at: "2026-10-22T21:03:03.581759511+00:00"
id: "astromech-phcw"
evidence_type: "test"
commit: "66d97a389deab854a52b5273b49bbd3ac365ca6d"
result: "pass"
captured_at: "2026-10-22T21:02:46.831210563+00:00"
command: "cargo test -p astromech-eddn --lib edsm::tests"
exit_status: "0"
agent_identity: "claude-implementer-ux0h"
target:
  kind: "issue"
  id: "astromech-dxv1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-dxv1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "EDSM courier tests pass: reset-on-stored test (fails with the reset removed), three side-by-side poisoned events no longer hold good ones (failed before the reorder), outage test gives up at most OUTAGE_AFTER"
updated_at: "2026-10-22T21:03:03.582269923+00:00"
---

## Summary

EDSM courier tests pass: reset-on-stored test (fails with the reset removed), three side-by-side poisoned events no longer hold good ones (failed before the reorder), outage test gives up at most OUTAGE_AFTER

## Command

```console
cargo test -p astromech-eddn --lib edsm::tests
```

Exit status: 0

## Stdout

Bytes: 2132
Truncated: no

```text

running 26 tests
test edsm::tests::a_list_from_an_earlier_run_is_used_until_it_is_a_day_old ... ok
test edsm::tests::another_commanders_events_are_held_back_and_logged_once ... ok
test edsm::tests::a_key_that_is_still_wrong_shows_its_error_from_the_same_pass ... ok
test edsm::tests::a_wrong_commander_or_key_keeps_the_events_for_later ... ok
test edsm::tests::an_event_stored_between_poisoned_ones_resets_the_outage_guard ... ok
test edsm::tests::a_single_event_that_keeps_failing_is_given_up_and_later_events_go ... ok
test edsm::tests::a_413_batch_is_split_the_same_way ... ok
test edsm::tests::once_the_list_is_known_discarded_events_are_not_even_queued ... ok
test edsm::tests::beta_and_crew_sessions_are_not_sent ... ok
test edsm::tests::events_queued_for_another_commander_are_not_sent_with_this_key ... ok
test edsm::tests::each_event_is_settled_by_its_own_answer ... ok
test edsm::tests::off_or_without_credentials_nothing_is_queued_and_the_queue_is_discarded ... ok
test edsm::tests::a_request_holds_one_game_session_and_at_most_a_batch ... ok
test edsm::tests::a_poisoned_event_costs_only_itself ... ok
test edsm::tests::what_counts_as_refused_and_what_is_only_held ... ok
test edsm::tests::the_startup_replay_is_not_sent ... ok
test edsm::tests::a_corrected_key_sends_what_a_wrong_one_held_back_at_once ... ok
test edsm::tests::turning_sharing_off_does_not_wake_or_release_anything ... ok
test edsm::tests::the_queue_behind_a_poisoned_event_moves_while_it_waits ... ok
test edsm::tests::fixture_session_requests_exclude_everything_on_the_discard_list ... ok
test edsm::tests::three_poisoned_events_side_by_side_do_not_hold_the_good_ones_behind ... ok
test edsm::tests::without_a_discard_list_nothing_is_sent ... ok
test edsm::tests::an_outage_of_5xx_answers_gives_up_a_few_events_not_the_queue ... ok
test edsm::tests::outages_do_not_use_up_a_messages_refusals ... ok
test edsm::tests::a_known_poisoned_event_is_not_sent_back_through_the_halving ... ok
test edsm::tests::two_poisoned_events_do_not_hold_the_queue ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 119 filtered out; finished in 0.09s
```

## Stderr

Bytes: 219
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-724ff17d7908aa6d)
```

