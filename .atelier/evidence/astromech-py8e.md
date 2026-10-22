---
created_at: "2026-10-22T22:48:35.043879168+00:00"
id: "astromech-py8e"
evidence_type: "test"
commit: "eb6a7bdfa771c9edef343b1e23c2350fc30165ad"
result: "pass"
captured_at: "2026-10-22T22:48:34.686661833+00:00"
command: "cargo test -p astromech-intent eval_mishearings"
exit_status: "0"
agent_identity: "claude-implementer-uuqt"
target:
  kind: "issue"
  id: "astromech-xi2i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xi2i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "eval_mishearings passes (No toriety -> status.legal, Engage/and gauge frames of drive -> hyperspace_jump, Fuel state -> status.fuel). scripts/voice-eval.sh rerun after the last fix (report docs/evidence/astromech-xi2i/report.md): 3123/3136 execute-or-ask, 0 wrong executes, miss 9 (was 3114, miss 16 before). status.legal 87.5->100, status.fuel 93.8->100, hyperspace_jump 90.6->100, unavailable/cockpit+noise 93.8->100, status/noise 96.7; status.shields and status.ship 96.9. status.credits ('rebuy' heard 'Raban', 'Goodbye.' in noise) stays 93.8."
updated_at: "2026-10-22T22:48:35.044771720+00:00"
---

## Summary

eval_mishearings passes (No toriety -> status.legal, Engage/and gauge frames of drive -> hyperspace_jump, Fuel state -> status.fuel). scripts/voice-eval.sh rerun after the last fix (report docs/evidence/astromech-xi2i/report.md): 3123/3136 execute-or-ask, 0 wrong executes, miss 9 (was 3114, miss 16 before). status.legal 87.5->100, status.fuel 93.8->100, hyperspace_jump 90.6->100, unavailable/cockpit+noise 93.8->100, status/noise 96.7; status.shields and status.ship 96.9. status.credits ('rebuy' heard 'Raban', 'Goodbye.' in noise) stays 93.8.

## Command

```console
cargo test -p astromech-intent eval_mishearings
```

Exit status: 0

## Stdout

Bytes: 149
Truncated: no

```text

running 1 test
test tests::eval_mishearings ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 29 filtered out; finished in 0.04s
```

## Stderr

Bytes: 161
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-dd31d636140f577e)
```

