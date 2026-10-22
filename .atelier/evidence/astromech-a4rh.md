---
created_at: "2026-10-22T23:05:22.429252513+00:00"
id: "astromech-a4rh"
evidence_type: "test"
commit: "416cc5cbb08c26000f092e08ad8ac59a2701879d"
result: "pass"
captured_at: "2026-10-22T23:05:20.857566907+00:00"
command: "cargo test -p astromech-core --test copy"
exit_status: "0"
agent_identity: "claude-implementer-uuqt"
target:
  kind: "issue"
  id: "astromech-joma"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-joma"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review R2 (416cc5cb): copy.rs passes with NGC 7822 Sector AB-C d14-123 (12 spoken words) added; the next-waypoint answer now says the name as its own sentence then 'For <label>.', so answers are checked with the name-only exemption and every other sentence keeps to 16 words. Workspace: 1124 passed, 0 failed."
updated_at: "2026-10-22T23:05:22.429544517+00:00"
---

## Summary

After review R2 (416cc5cb): copy.rs passes with NGC 7822 Sector AB-C d14-123 (12 spoken words) added; the next-waypoint answer now says the name as its own sentence then 'For <label>.', so answers are checked with the name-only exemption and every other sentence keeps to 16 words. Workspace: 1124 passed, 0 failed.

## Command

```console
cargo test -p astromech-core --test copy
```

Exit status: 0

## Stdout

Bytes: 658
Truncated: no

```text

running 9 tests
test the_cargo_answer_limits_its_partial_buyer_sentences ... ok
test the_route_lines_keep_to_the_copy_guide ... ok
test the_missing_key_help_lines_keep_to_the_copy_guide ... ok
test every_literal_the_core_says_keeps_to_the_copy_guide ... ok
test the_colonisation_answers_keep_to_the_copy_guide ... ok
test the_faction_lookup_answers_keep_to_the_copy_guide ... ok
test the_powerplay_target_answers_keep_to_the_copy_guide ... ok
test the_lookup_policy_and_service_lines_keep_to_the_copy_guide ... ok
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

## Stderr

Bytes: 261
Truncated: no

```text
   Compiling astromech-core v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-uuqt/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.33s
     Running tests/copy.rs (target/debug/deps/copy-3c4e88bffbeb9cf6)
```

