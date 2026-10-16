---
created_at: "2026-10-16T20:54:23.887353568+00:00"
id: "astromech-3ct0"
evidence_type: "test"
commit: "53fbc221b9282183e33c953f4a0409931f30fb62"
result: "pass"
captured_at: "2026-10-16T20:53:57.709270563+00:00"
command: "cargo test -p astromech-core --locked lookups"
exit_status: "0"
agent_identity: "claude"
target:
  kind: "issue"
  id: "astromech-tw2y"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-tw2y"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Voice phrase tests (19 services -> find.service) and spoken answer from a recorded response with saved filters and ship pad"
updated_at: "2026-10-16T20:54:23.887777600+00:00"
---

## Summary

Voice phrase tests (19 services -> find.service) and spoken answer from a recorded response with saved filters and ship pad

## Command

```console
cargo test -p astromech-core --locked lookups
```

Exit status: 0

## Stdout

Bytes: 659
Truncated: no

```text

running 3 tests
test lookups::tests::spoken_answer_uses_the_ships_pad_and_saved_filters ... ok
test tester::tests::lookups_say_what_they_would_look_up ... ok
test lookups::tests::service_phrases_resolve_to_lookups ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.19s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s
```

## Stderr

Bytes: 774
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-galaxy)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-m30a/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 25.82s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-9ead6ca7f9b9a7b4)
     Running unittests src/main.rs (target/debug/deps/astromech_core-1620298b589ad367)
     Running tests/gear_down.rs (target/debug/deps/gear_down-f08141c738926b64)
     Running tests/voice_tester.rs (target/debug/deps/voice_tester-dff127e51d0f749e)
```

