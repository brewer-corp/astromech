---
created_at: "2026-10-17T18:42:48.946147535+00:00"
id: "astromech-csfm"
evidence_type: "test"
commit: "2be14b9efcb47ebc38ffdba33676f1777c0e6cf6"
result: "pass"
captured_at: "2026-10-17T18:42:34.568202040+00:00"
command: "cargo test -p astromech-keybinds"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-2za5"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-2za5"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-keybinds"
updated_at: "2026-10-17T18:42:48.946630326+00:00"
---

## Summary

cargo test -p astromech-keybinds

## Command

```console
cargo test -p astromech-keybinds
```

Exit status: 0

## Stdout

Bytes: 1315
Truncated: no

```text

running 25 tests
test binds::tests::decode_variants ... ok
test binds::tests::file_names ... ok
test binds::tests::device_mappings ... ok
test active::tests::keyboard_bindings ... ok
test binds::tests::start_preset ... ok
test binds::tests::keys ... ok
test catalog::tests::humanizes_ids ... ok
test binds::tests::malformed_position ... ok
test binds::tests::parse_sample ... ok
test devices::tests::fake_file ... ok
test devices::tests::axis_mapping ... ok
test game::tests::epic ... ok
test game::tests::vdf ... ok
test game::tests::products ... ok
test edit::tests::adds_missing_actions_and_axes ... ok
test active::tests::resolves_ship_context ... ok
test edit::tests::edits_only_the_slot ... ok
test report::tests::redaction ... ok
test catalog::tests::bundled_catalogue_loads ... ok
test profile::tests::view_rows_and_issues ... ok
test profile::tests::device_and_pid_warnings ... ok
test doctor::tests::end_to_end ... ok
test profile::tests::paste_keys_in_the_galaxy_map_are_flagged ... ok
test edit::tests::file_edit_backup_restore_copy_and_activate ... ok
test api::tests::api_round_trip ... ok

test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 255
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/astromech_keybinds-1d6462455cf97723)
   Doc-tests astromech_keybinds
```

