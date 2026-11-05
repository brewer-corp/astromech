---
created_at: "2026-11-05T00:13:18.782505277+00:00"
id: "astromech-vvuz"
evidence_type: "test"
commit: "0699c857ea01072b91804c82ac9d9b510604d78c"
result: "pass"
captured_at: "2026-11-05T00:12:42.251304339+00:00"
command: "cargo test -p astromech-core -p astromech-journal"
exit_status: "0"
agent_identity: "orchestrator"
target:
  kind: "issue"
  id: "astromech-73w9"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-73w9"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Fresh run on the merged branch with a clean target dir outside the repo (the earlier failure astromech-3np4 was a stale shared-cache build): core and journal suites pass, including a_refused_touched_journals_header_is_read_once"
updated_at: "2026-11-05T00:13:18.782805527+00:00"
---

## Summary

Fresh run on the merged branch with a clean target dir outside the repo (the earlier failure astromech-3np4 was a stale shared-cache build): core and journal suites pass, including a_refused_touched_journals_header_is_read_once

## Command

```console
cargo test -p astromech-core -p astromech-journal
```

Exit status: 0

## Stdout

Bytes: 25944
SHA-256: 97382f05d43cfdcc641f113a51517776352fad9a807ad20e528eaadf1d573dba
Test summary: test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 2543
SHA-256: 5d5cc87905a8eb23be9c45c69f3c7485657bf2a00fef5cecb80d0cb58f7688a1
