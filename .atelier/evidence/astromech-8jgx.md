---
created_at: "2026-10-16T20:20:04.953291959+00:00"
id: "astromech-8jgx"
evidence_type: "test"
commit: "b2e317ac1896ea665ee09663db58513b06b8786a"
result: "pass"
captured_at: "2026-10-16T20:20:02.176670092+00:00"
command: "cargo test -p astromech-eddn stale"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-xvjs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xvjs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "N1 at b2e317a: a_run_of_stale_lines_is_logged_once_and_so_is_recovery (5 stale lines -> one log line, recovery -> one; fails if every stale line logs). Workspace at b2e317a: fmt, clippy -D warnings clean, cargo test --workspace 40 binaries pass, check-deps OK."
updated_at: "2026-10-16T20:20:04.953491832+00:00"
---

## Summary

N1 at b2e317a: a_run_of_stale_lines_is_logged_once_and_so_is_recovery (5 stale lines -> one log line, recovery -> one; fails if every stale line logs). Workspace at b2e317a: fmt, clippy -D warnings clean, cargo test --workspace 40 binaries pass, check-deps OK.

## Command

```console
cargo test -p astromech-eddn stale
```

Exit status: 0

## Stdout

Bytes: 193
Truncated: no

```text

running 1 test
test feed::tests::a_run_of_stale_lines_is_logged_once_and_so_is_recovery ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 98 filtered out; finished in 0.00s
```

## Stderr

Bytes: 278
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-eddn)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.70s
     Running unittests src/lib.rs (target/debug/deps/astromech_eddn-56ff1c1f80a1c470)
```

