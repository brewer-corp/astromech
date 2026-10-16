---
created_at: "2026-10-16T23:03:26.722737496+00:00"
id: "astromech-dn8s"
evidence_type: "test"
commit: "2e9bc9387a1f0325594aeb596114eb021636e7e8"
result: "pass"
captured_at: "2026-10-16T23:03:26.135997292+00:00"
command: "cargo test -p astromech-market --test list"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-05bi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-05bi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Done when: recorded Ardent exports (4 colonisation commodities near Sol, 16 Oct 2026, trimmed) rank Efremov Plant (3 of the list, 8.6 ly) first and above the nearer fleet carrier FCX-801 (1 of the list, 6.0 ly); pad/carrier/age/min-stock filters apply per commodity; unavailable commodities named. A distance-only ranking fails the test (mutation checked)."
updated_at: "2026-10-16T23:03:26.723022438+00:00"
---

## Summary

Done when: recorded Ardent exports (4 colonisation commodities near Sol, 16 Oct 2026, trimmed) rank Efremov Plant (3 of the list, 8.6 ly) first and above the nearer fleet carrier FCX-801 (1 of the list, 6.0 ly); pad/carrier/age/min-stock filters apply per commodity; unavailable commodities named. A distance-only ranking fails the test (mutation checked).

## Command

```console
cargo test -p astromech-market --test list
```

Exit status: 0

## Stdout

Bytes: 319
Truncated: no

```text

running 3 tests
test stations_stocking_more_of_the_list_rank_above_closer_ones ... ok
test commodities_nobody_sells_are_named_and_finished_lines_are_not_asked ... ok
test the_lookup_filters_apply_to_every_commodity ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 265
Truncated: no

```text
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-k3wp/crates/astromech-market)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running tests/list.rs (target/debug/deps/list-008bdfd0fbb58d4a)
```

