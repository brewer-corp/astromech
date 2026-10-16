---
created_at: "2026-10-16T18:54:55.783547699+00:00"
id: "astromech-ay3a"
evidence_type: "test"
commit: "d8b2724a885aa7afec1f60f49a3538928cde901e"
result: "pass"
captured_at: "2026-10-16T18:54:52.827512242+00:00"
command: "env CARGO_BUILD_JOBS=3 cargo test -p astromech-voice -p astromech-core -- kokoro retired half_extracted small_v0_19 speaker cache_names sample_parses request_roundtrip finds_s9"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-a50f"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-a50f"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes: v1.0 counts as installed only with its download marker (half-extracted folder is re-downloaded and v0.19 kept; mutation check: dropping the marker filter fails half_extracted_v1_is_not_installed), int8-only v0.19 users get int8 v1.0, v0.19 left alone in a folder models_dir points at, plus the earlier catalog/config/cache tests"
updated_at: "2026-10-16T18:54:55.783858068+00:00"
---

## Summary

After review fixes: v1.0 counts as installed only with its download marker (half-extracted folder is re-downloaded and v0.19 kept; mutation check: dropping the marker filter fails half_extracted_v1_is_not_installed), int8-only v0.19 users get int8 v1.0, v0.19 left alone in a folder models_dir points at, plus the earlier catalog/config/cache tests

## Command

```console
env CARGO_BUILD_JOBS=3 cargo test -p astromech-voice -p astromech-core -- kokoro retired half_extracted small_v0_19 speaker cache_names sample_parses request_roundtrip finds_s9
```

Exit status: 0

## Stdout

Bytes: 1280
Truncated: no

```text

running 4 tests
test speech::tests::cache_names_are_stable ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok
test app::tests::retired_models_stay_in_a_folder_models_dir_points_at ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 6 tests
test catalog::tests::kokoro_v1_assets_and_voices ... ok
test proto::tests::request_roundtrip ... ok
test models::tests::finds_s9_layout ... ok
test models::tests::half_extracted_v1_is_not_installed ... ok
test models::tests::retired_kokoro_is_removed_only_after_v1_is_installed ... ok
test models::tests::small_v0_19_users_get_small_v1 ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 703
Truncated: no

```text
   Compiling astromech-voice v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-a50f/crates/astromech-voice)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-a50f/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.58s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3ef5b9319d242e60)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8db4e83b3fbc02c2)
     Running tests/gear_down.rs (target/debug/deps/gear_down-940437f5cc7f0d85)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-75b209c31c0e6ada)
   Doc-tests astromech_core
   Doc-tests astromech_voice
```

