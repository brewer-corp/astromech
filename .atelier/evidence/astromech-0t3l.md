---
created_at: "2026-10-17T19:18:38.284322085+00:00"
id: "astromech-0t3l"
evidence_type: "test"
commit: "db710854f0e9f85a36289c23b479587b482ca2dc"
result: "pass"
captured_at: "2026-10-17T19:18:36.438698929+00:00"
command: "cargo test -p astromech-core --lib the_sample_config"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-k20i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-k20i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib the_sample_config"
updated_at: "2026-10-17T19:18:38.284628426+00:00"
---

## Summary

cargo test -p astromech-core --lib the_sample_config

## Command

```console
cargo test -p astromech-core --lib the_sample_config
```

Exit status: 0

## Stdout

Bytes: 198
Truncated: no

```text

running 1 test
test app::tests::the_sample_config_written_by_open_config_changes_no_setting ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 122 filtered out; finished in 0.00s
```

## Stderr

Bytes: 262
Truncated: no

```text
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-config/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.68s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

