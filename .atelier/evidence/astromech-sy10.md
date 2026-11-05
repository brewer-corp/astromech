---
created_at: "2026-11-04T22:52:54.303358563+00:00"
id: "astromech-sy10"
evidence_type: "test"
commit: "2882f5ff32697b23c83279d8dc8dc7d2c4b1171e"
result: "pass"
captured_at: "2026-11-04T22:52:08.829532444+00:00"
command: "cargo test --no-fail-fast -p astromech-intent -p astromech-tools -p astromech-galaxy -p astromech-core"
exit_status: "0"
agent_identity: "implement-agent-1n9b"
target:
  kind: "issue"
  id: "astromech-1n9b"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-1n9b"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test for astromech-intent, -tools, -galaxy, -core at 2882f5ff: all pass, including the new tests a_system_after_in_or_around_is_the_origin_and_other_places_are_here and a_named_system_after_a_non_lookup_is_no_match (intent; both seen failing on the pre-change grammar), where_to_buy_from_a_named_system_searches_there_and_says_so (colonisation_voice), a_command_with_a_system_after_it_is_not_heard_as_a_lookup (trade_voice), the_named_system_lines_keep_to_the_copy_guide and the named variants in the target/faction copy tests (copy)."
updated_at: "2026-11-04T22:52:54.303835864+00:00"
---

## Summary

cargo test for astromech-intent, -tools, -galaxy, -core at 2882f5ff: all pass, including the new tests a_system_after_in_or_around_is_the_origin_and_other_places_are_here and a_named_system_after_a_non_lookup_is_no_match (intent; both seen failing on the pre-change grammar), where_to_buy_from_a_named_system_searches_there_and_says_so (colonisation_voice), a_command_with_a_system_after_it_is_not_heard_as_a_lookup (trade_voice), the_named_system_lines_keep_to_the_copy_guide and the named variants in the target/faction copy tests (copy).

## Command

```console
cargo test --no-fail-fast -p astromech-intent -p astromech-tools -p astromech-galaxy -p astromech-core
```

Exit status: 0

## Stdout

Bytes: 34141
SHA-256: eef996d0431962094dc5d3f91a23e5e2860867e32557f496b474b8877d057f08
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 6617
SHA-256: bced5363cf4abc43fc1d90899826f1ec53f261ee423488580e539b230a0044de
