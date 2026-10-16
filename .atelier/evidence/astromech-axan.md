---
created_at: "2026-10-16T18:44:01.745409318+00:00"
id: "astromech-axan"
evidence_type: "test"
commit: "6cfe4df3da000a4e128e9aea0dfdc26c1e63f012"
result: "pass"
captured_at: "2026-10-16T18:44:00.567096090+00:00"
command: "env CARGO_BUILD_JOBS=4 cargo test -p astromech-voice -p astromech-core -- kokoro retired speaker cache_names sample_parses request_roundtrip finds_s9"
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
title: "Catalog entries (v1.0 URLs, digests, 54-voice list matching voices.bin size, British voices, v0.19 mapping), config migration (speaker_id -> v1.0 name or default; name not ID written back), retired v0.19 folder removed only after v1.0 is installed, cache key includes model and voice, protocol round-trip"
updated_at: "2026-10-16T18:44:01.745795067+00:00"
---

## Summary

Catalog entries (v1.0 URLs, digests, 54-voice list matching voices.bin size, British voices, v0.19 mapping), config migration (speaker_id -> v1.0 name or default; name not ID written back), retired v0.19 folder removed only after v1.0 is installed, cache key includes model and voice, protocol round-trip

## Command

```console
env CARGO_BUILD_JOBS=4 cargo test -p astromech-voice -p astromech-core -- kokoro retired speaker cache_names sample_parses request_roundtrip finds_s9
```

Exit status: 0

## Stdout

Bytes: 1083
Truncated: no

```text

running 3 tests
test speech::tests::cache_names_are_stable ... ok
test config::tests::old_speaker_id_migrates_to_a_v1_voice_name ... ok
test config::tests::sample_parses_and_buttons_match ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 26 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 4 tests
test proto::tests::request_roundtrip ... ok
test catalog::tests::kokoro_v1_assets_and_voices ... ok
test models::tests::finds_s9_layout ... ok
test models::tests::retired_kokoro_is_removed_only_after_v1_is_installed ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 22 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 469
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.80s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-3ef5b9319d242e60)
     Running unittests src/main.rs (target/debug/deps/astromech_core-8db4e83b3fbc02c2)
     Running tests/gear_down.rs (target/debug/deps/gear_down-940437f5cc7f0d85)
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-75b209c31c0e6ada)
   Doc-tests astromech_core
   Doc-tests astromech_voice
```

