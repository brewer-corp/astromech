---
created_at: "2026-10-22T21:44:00.541828311+00:00"
id: "astromech-ya9f"
evidence_type: "test"
commit: "b4e118b5345dafd978718e915eb4f40562203b41"
result: "pass"
captured_at: "2026-10-22T21:43:55.793408584+00:00"
command: "bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-engineering links'"
exit_status: "0"
agent_identity: "claude-implementer-0k88"
target:
  kind: "issue"
  id: "astromech-6yt1"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6yt1"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "links::tests::only_our_links_are_openable: GitHub help and release-notes URLs pass is_openable_url; GitHub URLs with spaces, quotes, commas, control chars or look-alike paths are refused (fails with the GitHub prefix removed). open_external now calls only this predicate (shell edit, CI-verified build)"
updated_at: "2026-10-22T21:44:00.542296343+00:00"
---

## Summary

links::tests::only_our_links_are_openable: GitHub help and release-notes URLs pass is_openable_url; GitHub URLs with spaces, quotes, commas, control chars or look-alike paths are refused (fails with the GitHub prefix removed). open_external now calls only this predicate (shell edit, CI-verified build)

## Command

```console
bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-engineering links'
```

Exit status: 0

## Stdout

Bytes: 680
Truncated: no

```text

running 6 tests
test links::tests::coriolis_round_trips_the_loadout ... ok
test links::tests::only_our_links_are_openable ... ok
test links::tests::edsy_round_trips_the_loadout ... ok
test links::tests::urls_snapshot ... ok
test links::tests::large_loadout_urls_stay_openable_and_short ... ok
test tests::ship_links_follow_the_loadout_until_a_craft ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.08s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

## Stderr

Bytes: 390
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.15s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-e5e36a3c27b68c87)
     Running tests/live_journal.rs (target/debug/deps/live_journal-c046d0d16d4ab265)
     Running tests/transfers.rs (target/debug/deps/transfers-2c6b58e66ad3c98b)
```

