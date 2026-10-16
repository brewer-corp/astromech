---
created_at: "2026-10-16T23:03:27.313139531+00:00"
id: "astromech-bh0u"
evidence_type: "test"
commit: "2e9bc9387a1f0325594aeb596114eb021636e7e8"
result: "pass"
captured_at: "2026-10-16T23:03:26.783834868+00:00"
command: "cargo test -p astromech-colonisation journal_folder_view"
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
title: "Journal-folder view: projects being built first, each with its shopping list (market symbol, aboard, to buy, loads) in the JSON shape the app reads"
updated_at: "2026-10-16T23:03:27.313439360+00:00"
---

## Summary

Journal-folder view: projects being built first, each with its shopping list (market symbol, aboard, to buy, loads) in the JSON shape the app reads

## Command

```console
cargo test -p astromech-colonisation journal_folder_view
```

Exit status: 0

## Stdout

Bytes: 209
Truncated: no

```text

running 1 test
test tests::shopping_tests::journal_folder_view_lists_projects_with_their_shopping_lists ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
```

## Stderr

Bytes: 302
Truncated: no

```text
   Compiling astromech-colonisation v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-k3wp/crates/astromech-colonisation)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running unittests src/lib.rs (target/debug/deps/astromech_colonisation-5823c5891198ddd4)
```

