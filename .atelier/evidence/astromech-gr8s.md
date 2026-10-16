---
created_at: "2026-10-16T19:21:59.558232059+00:00"
id: "astromech-gr8s"
evidence_type: "test"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:21:58.819158189+00:00"
command: "cargo test -p astromech-engineering planned_modules"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-engineering planned_modules"
updated_at: "2026-10-16T19:21:59.558629261+00:00"
---

## Summary

cargo test -p astromech-engineering planned_modules

## Command

```console
cargo test -p astromech-engineering planned_modules
```

Exit status: 0

## Stdout

Bytes: 178
Truncated: no

```text

running 1 test
test tests::planned_modules_follow_the_plan_like_the_game ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.01s
```

## Stderr

Bytes: 299
Truncated: no

```text
   Compiling astromech-engineering v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-z4jb/crates/astromech-engineering)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running unittests src/lib.rs (target/debug/deps/astromech_engineering-ff71dd009829e65d)
```

