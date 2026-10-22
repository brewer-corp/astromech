---
created_at: "2026-10-22T21:03:08.327020866+00:00"
id: "astromech-jw5x"
evidence_type: "test"
commit: "66d97a389deab854a52b5273b49bbd3ac365ca6d"
result: "pass"
captured_at: "2026-10-22T21:03:03.806535869+00:00"
command: "cargo test -p astromech-core --lib edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay"
exit_status: "0"
agent_identity: "claude-implementer-ux0h"
target:
  kind: "issue"
  id: "astromech-st8o"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-st8o"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Old live lines still go to EDSM by decision (no age guard); core pipeline and EDSM tests that pin it pass; guide and code comment record why"
updated_at: "2026-10-22T21:03:08.327390955+00:00"
---

## Summary

Old live lines still go to EDSM by decision (no age guard); core pipeline and EDSM tests that pin it pass; guide and code comment record why

## Command

```console
cargo test -p astromech-core --lib edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay
```

Exit status: 0

## Stdout

Bytes: 209
Truncated: no

```text

running 1 test
test pipeline::tests::edsm_gets_live_lines_with_their_state_and_nothing_from_the_replay ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 165 filtered out; finished in 0.03s
```

## Stderr

Bytes: 219
Truncated: no

```text
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-badbc78a43d6af96)
```

