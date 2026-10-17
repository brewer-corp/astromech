---
created_at: "2026-10-17T18:51:24.602885009+00:00"
id: "astromech-2v40"
evidence_type: "test"
commit: "00b899f72d164b787b946f89530fad341a26cc5b"
result: "pass"
captured_at: "2026-10-17T18:51:21.558459582+00:00"
command: "cargo test -p astromech-core --test copy"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-yerd"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-yerd"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --test copy"
updated_at: "2026-10-17T18:51:24.603601436+00:00"
---

## Summary

cargo test -p astromech-core --test copy

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
test the_lookup_policy_and_service_lines_keep_to_the_copy_guide ... ok
test the_missing_key_help_lines_keep_to_the_copy_guide ... ok
test every_literal_the_core_says_keeps_to_the_copy_guide ... ok
test the_colonisation_answers_keep_to_the_copy_guide ... ok
test the_powerplay_target_answers_keep_to_the_copy_guide ... ok
test the_faction_lookup_answers_keep_to_the_copy_guide ... ok
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

## Stderr

Bytes: 454
Truncated: no

```text
   Compiling astromech-policy v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-policy)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-voice/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.83s
     Running tests/copy.rs (target/debug/deps/copy-a57b61b54ffa7e99)
```

