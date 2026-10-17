---
created_at: "2026-10-17T17:07:47.746520768+00:00"
id: "astromech-3fzm"
evidence_type: "test"
commit: "b96aad0f8ef564f33f4c72b38963cd569c71bb71"
result: "pass"
captured_at: "2026-10-17T17:07:46.572167132+00:00"
command: "cargo test -p astromech-voice"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-xtq1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xtq1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-voice"
updated_at: "2026-10-17T17:07:47.746796331+00:00"
---

## Summary

cargo test -p astromech-voice

## Command

```console
cargo test -p astromech-voice
```

Exit status: 0

## Stdout

Bytes: 1846
Truncated: no

```text

running 30 tests
test catalog::tests::kokoro_v1_assets_and_voices ... ok
test client::tests::exit_codes_are_explained ... ok
test colour::tests::default_speed ... ok
test catalog::tests::urls_and_digests ... ok
test gain::tests::pick_name_exact_then_substring ... ok
test colour::tests::natural_is_identity ... ok
test mixer::tests::new_play_replaces ... ok
test mixer::tests::play_then_cut_is_silence ... ok
test colour::tests::resample_roundtrip_length ... ok
test gain::tests::already_loud_is_not_boosted ... ok
test colour::tests::utterance_adds_pip_except_natural ... ok
test download::tests::zip_directory_parsing ... ok
test gain::tests::quiet_speech_is_boosted_hiss_is_not ... ok
test download::tests::manual_archive_is_verified_extracted_and_filtered ... ok
test gain::tests::calibration_sets_gate_between_noise_and_speech ... ok
test models::tests::finds_s9_layout ... ok
test models::tests::half_extracted_v1_is_not_installed ... ok
test proto::tests::response_lines ... ok
test proto::tests::request_roundtrip ... ok
test score::tests::normalisation ... ok
test score::tests::scoring ... ok
test models::tests::parakeet_files_without_the_marker_are_missing ... ok
test models::tests::retired_kokoro_is_removed_only_after_v1_is_installed ... ok
test speakable::tests::reading_rules_table ... ok
test models::tests::small_v0_19_users_get_small_v1 ... ok
test colour::tests::harsh_is_narrower_than_cockpit ... ok
test wav::tests::roundtrip ... ok
test colour::tests::output_stays_in_range ... ok
test colour::tests::cockpit_kills_bass_and_air_keeps_presence ... ok
test download::tests::bad_manual_archive_is_discarded ... ok

test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 298
Truncated: no

```text
   Compiling astromech-voice v0.1.5 (/home/supadava/projects/astromech-wt-bugs-state/crates/astromech-voice)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running unittests src/lib.rs (target/debug/deps/astromech_voice-cad272ffca9a263d)
   Doc-tests astromech_voice
```

