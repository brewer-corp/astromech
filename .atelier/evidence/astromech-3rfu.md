---
created_at: "2026-10-22T22:12:08.862036902+00:00"
id: "astromech-3rfu"
evidence_type: "test"
commit: "21e7add372f503d5323f41c13760c817f4f152c9"
result: "pass"
captured_at: "2026-10-22T22:12:06.759618046+00:00"
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
title: "copy.rs passes with the route test covering Hyades Sector DB-W b2-6 (9 spoken words) and Col 285 Sector AB-C d14-123 (10) in the arrival callout, skip/back replies and the next-waypoint answer with 'neutron boost and fuel'; without the name-only exemption it fails (8 over-budget callouts). Rule written in V1-DESIGN 6.5."
updated_at: "2026-10-22T22:12:08.862534249+00:00"
---

## Summary

copy.rs passes with the route test covering Hyades Sector DB-W b2-6 (9 spoken words) and Col 285 Sector AB-C d14-123 (10) in the arrival callout, skip/back replies and the next-waypoint answer with 'neutron boost and fuel'; without the name-only exemption it fails (8 over-budget callouts). Rule written in V1-DESIGN 6.5.

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
test every_literal_the_core_says_keeps_to_the_copy_guide ... ok
test the_colonisation_answers_keep_to_the_copy_guide ... ok
test the_route_lines_keep_to_the_copy_guide ... ok
test the_missing_key_help_lines_keep_to_the_copy_guide ... ok
test the_powerplay_target_answers_keep_to_the_copy_guide ... ok
test the_faction_lookup_answers_keep_to_the_copy_guide ... ok
test the_lookup_policy_and_service_lines_keep_to_the_copy_guide ... ok
test every_line_keeps_to_the_copy_guide ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s
```

## Stderr

Bytes: 261
Truncated: no

```text
   Compiling astromech-core v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-uuqt/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running tests/copy.rs (target/debug/deps/copy-c8814142d665d5b3)
```

