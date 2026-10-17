---
created_at: "2026-10-17T00:03:38.072471576+00:00"
id: "astromech-ee8j"
evidence_type: "test"
commit: "e3cd7deb8351358554dcfecde039bb0fd64e9a96"
result: "pass"
captured_at: "2026-10-17T00:03:36.331594891+00:00"
command: "cargo test -p astromech-core --lib set_eddn_flips_the_shared_switch"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6xyo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6xyo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R3+R5: set_eddn flips EDDN_SHARING and switching off empties the queue at once (fails with the discard or the swap removed)"
updated_at: "2026-10-17T00:03:38.072764723+00:00"
---

## Summary

R3+R5: set_eddn flips EDDN_SHARING and switching off empties the queue at once (fails with the discard or the swap removed)

## Command

```console
cargo test -p astromech-core --lib set_eddn_flips_the_shared_switch
```

Exit status: 0

## Stdout

Bytes: 214
Truncated: no

```text

running 1 test
test app::tests::set_eddn_flips_the_shared_switch_and_switching_off_empties_the_queue_at_once ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 0.00s
```

## Stderr

Bytes: 391
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-6xyo/crates/astromech-eddn)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-6xyo/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.61s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-094d1cbbf733166e)
```

