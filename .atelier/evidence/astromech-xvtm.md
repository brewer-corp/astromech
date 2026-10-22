---
created_at: "2026-10-22T21:30:39.923086406+00:00"
id: "astromech-xvtm"
evidence_type: "test"
commit: "dfcb12fd136d61438891084e3e8672dc52cabd6c"
result: "pass"
captured_at: "2026-10-22T21:30:39.495375250+00:00"
command: "cargo test -p astromech-history --lib -- logbook read_only"
exit_status: "0"
agent_identity: "claude-implementer-fdh3"
target:
  kind: "issue"
  id: "astromech-hloh"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-hloh"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Logbook search finds non-ASCII capitals (ÉCOLE via école, Ölberg via ölb/ÖLBERG) in places and timeline; read-only open sets a 2 s busy timeout; existing logbook tests pass"
updated_at: "2026-10-22T21:30:39.923549609+00:00"
---

## Summary

Logbook search finds non-ASCII capitals (ÉCOLE via école, Ölberg via ölb/ÖLBERG) in places and timeline; read-only open sets a 2 s busy timeout; existing logbook tests pass

## Command

```console
cargo test -p astromech-history --lib -- logbook read_only
```

Exit status: 0

## Stdout

Bytes: 350
Truncated: no

```text

running 4 tests
test tests::read_only_open_waits_when_busy ... ok
test tests::logbook_search_folds_non_ascii_capitals ... ok
test tests::read_only_open_reads_what_the_writer_wrote ... ok
test tests::logbook_search_finds_places_with_their_counts ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 31 filtered out; finished in 0.02s
```

## Stderr

Bytes: 162
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/astromech_history-8b4e2bbed412436b)
```

