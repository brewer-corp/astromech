---
created_at: "2026-11-04T23:43:47.897142998+00:00"
id: "astromech-9516"
evidence_type: "test"
commit: "157996b4587d973b430cb4c8e6ffddf1d34fd8c0"
result: "pass"
captured_at: "2026-11-04T23:43:46.184165598+00:00"
command: "cargo test -p astromech-eddn"
exit_status: "0"
agent_identity: "implement-agent-1o77"
target:
  kind: "issue"
  id: "astromech-u6ai"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-u6ai"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-eddn: all pass, incl. new a_failed_refused_batch_counts_a_refusal_only_against_the_event_failing_alone and the poisoned drain at 0, 49, 50, 100, 199. Mutation (refuse_at refusals+1 for every row in the failed refused-batch branch) fails the new test; reverted. fmt and clippy -D warnings clean."
updated_at: "2026-11-04T23:43:47.897652830+00:00"
---

## Summary

cargo test -p astromech-eddn: all pass, incl. new a_failed_refused_batch_counts_a_refusal_only_against_the_event_failing_alone and the poisoned drain at 0, 49, 50, 100, 199. Mutation (refuse_at refusals+1 for every row in the failed refused-batch branch) fails the new test; reverted. fmt and clippy -D warnings clean.

## Command

```console
cargo test -p astromech-eddn
```

Exit status: 0

## Stdout

Bytes: 12109
SHA-256: a8835813f73116875149f6c8bfc6bd2425a906c192da9fb022b96390906158ff
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 360
SHA-256: 200ac822e332f67d255458d194dc4e57f2f1daf20f7f8d9ff6ed45336bf53ea0
