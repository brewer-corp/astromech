---
created_at: "2026-10-22T21:44:02.690417761+00:00"
id: "astromech-g2d4"
evidence_type: "test"
commit: "b4e118b5345dafd978718e915eb4f40562203b41"
result: "pass"
captured_at: "2026-10-22T21:44:00.806875765+00:00"
command: "bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-store config && CARGO_BUILD_JOBS=4 cargo test -p astromech-core config'"
exit_status: "0"
agent_identity: "claude-implementer-0k88"
target:
  kind: "issue"
  id: "astromech-6a7c"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6a7c"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "paths::tests::the_window_reads_the_config_the_core_published: a published --config file deleted since falls back to the default (fails without the fix); core's window-config test still passes"
updated_at: "2026-10-22T21:44:02.690802949+00:00"
---

## Summary

paths::tests::the_window_reads_the_config_the_core_published: a published --config file deleted since falls back to the default (fails without the fix); core's window-config test still passes

## Command

```console
bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-store config && CARGO_BUILD_JOBS=4 cargo test -p astromech-core config'
```

Exit status: 0

## Stdout

Bytes: 2526
Truncated: no

```text

running 2 tests
test paths::tests::journal_dir_is_the_configured_one_else_under_the_user_profile ... ok
test paths::tests::the_window_reads_the_config_the_core_published ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 8 tests
test setup::tests::a_broken_config_is_not_overwritten ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test config::tests::finds_look_alike_config_names ... ok
test config::tests::one_bad_value_costs_only_that_value ... ok
test app::tests::callout_settings_round_trip_through_the_config_file ... ok
test app::tests::fixing_a_broken_config_clears_the_error_without_a_restart ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 158 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s
```

## Stderr

Bytes: 1648
Truncated: no

```text
   Compiling astromech-store v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-0k88/crates/astromech-store)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.76s
     Running unittests src/lib.rs (target/debug/deps/astromech_store-44ebb2436e59e65f)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-74cd69d29e207e81)
     Running unittests src/main.rs (target/debug/deps/astromech_core-49be82ca53cc4ca8)
     Running tests/colonisation_voice.rs (target/debug/deps/colonisation_voice-8235a6f3218cd067)
     Running tests/copy.rs (target/debug/deps/copy-3c4e88bffbeb9cf6)
     Running tests/faction_voice.rs (target/debug/deps/faction_voice-f191b20098a65af0)
     Running tests/gear_down.rs (target/debug/deps/gear_down-faf2fc4d72be2d30)
     Running tests/known_bodies.rs (target/debug/deps/known_bodies-33dea5791167f5b8)
     Running tests/mining_status.rs (target/debug/deps/mining_status-fb4b6503077187a0)
     Running tests/powerplay_voice.rs (target/debug/deps/powerplay_voice-e759527d35cb383f)
     Running tests/real_journals.rs (target/debug/deps/real_journals-f6b5cf6328b2582d)
     Running tests/session_summary.rs (target/debug/deps/session_summary-27b2307587ec22ec)
     Running tests/site_lookup.rs (target/debug/deps/site_lookup-462819d7b8e292c6)
     Running tests/trade_voice.rs (target/debug/deps/trade_voice-954e9e78fe087070)
     Running tests/voice_eval_phrases.rs (target/debug/deps/voice_eval_phrases-1a9379231cb4b68a)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-d1ad4564ffda83b6)
```

