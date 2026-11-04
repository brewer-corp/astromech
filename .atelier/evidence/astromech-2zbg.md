---
created_at: "2026-11-04T21:48:49.490647508+00:00"
id: "astromech-2zbg"
evidence_type: "test"
commit: "c4cce78bfd05c4548c3432098529904c7f9e2d3a"
result: "pass"
captured_at: "2026-11-04T21:48:29.750779411+00:00"
command: "cargo test -p astromech-store -p astromech-core"
exit_status: "0"
agent_identity: "implement-agent-f2k1"
target:
  kind: "issue"
  id: "astromech-f2k1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-f2k1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "astromech-store and astromech-core suites pass (store 19, core 187 lib tests). New tests: app::tests::the_window_opens_its_cache_in_the_running_cores_data_folder (Done-when 1: after a --config core quits, its file still present, the window's data_dir is the default the core it starts uses) and app::tests::a_config_not_written_yet_gives_the_window_the_cores_folders (Done-when 2: a --config file not written yet gives the window the core's default data and Bindings folders until written), plus store paths tests. All new tests failed with the fix reverted. The one-line shell change (app/src-tauri data_dir() calling astromech_store::window::data_dir) is not compiled on this Linux host; only Windows CI builds it."
updated_at: "2026-11-04T21:48:49.490957126+00:00"
---

## Summary

astromech-store and astromech-core suites pass (store 19, core 187 lib tests). New tests: app::tests::the_window_opens_its_cache_in_the_running_cores_data_folder (Done-when 1: after a --config core quits, its file still present, the window's data_dir is the default the core it starts uses) and app::tests::a_config_not_written_yet_gives_the_window_the_cores_folders (Done-when 2: a --config file not written yet gives the window the core's default data and Bindings folders until written), plus store paths tests. All new tests failed with the fix reverted. The one-line shell change (app/src-tauri data_dir() calling astromech_store::window::data_dir) is not compiled on this Linux host; only Windows CI builds it.

## Command

```console
cargo test -p astromech-store -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 22711
SHA-256: 3affa4a8f75af702ba5ccc0bb04c1264e8cc4a9d5265f6586cde02647ed3b907
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 2184
SHA-256: 77f39a1b4bfc762a644e6f0b6219c81d28b73a94547c0013a67de6c531a62936
