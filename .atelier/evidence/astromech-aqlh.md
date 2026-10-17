---
created_at: "2026-10-16T23:35:09.007162192+00:00"
id: "astromech-aqlh"
evidence_type: "test"
commit: "ba77ef85bf23f20da5764d5aa18a93113fe17139"
result: "pass"
captured_at: "2026-10-16T23:35:07.700601357+00:00"
command: "cargo test -p astromech-data --test recorded spansh_trade"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mljg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mljg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Spansh trade route job/poll replay: submit form, two queued (HTTP 202) polls then ok, plus refused submit, failed job, expired job, timeout, empty result and schema errors"
updated_at: "2026-10-16T23:35:09.008055219+00:00"
---

## Summary

Spansh trade route job/poll replay: submit form, two queued (HTTP 202) polls then ok, plus refused submit, failed job, expired job, timeout, empty result and schema errors

## Command

```console
cargo test -p astromech-data --test recorded spansh_trade
```

Exit status: 0

## Stdout

Bytes: 227
Truncated: no

```text

running 2 tests
test spansh_trade_route_polls_until_the_job_is_done ... ok
test spansh_trade_route_errors_are_plain_words ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
```

## Stderr

Bytes: 266
Truncated: no

```text
   Compiling astromech-data v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-mljg/crates/astromech-data)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.15s
     Running tests/recorded.rs (target/debug/deps/recorded-3dcf0cb75e4928d2)
```

