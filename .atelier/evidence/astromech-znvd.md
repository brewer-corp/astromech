---
created_at: "2026-11-04T22:14:31.251861571+00:00"
id: "astromech-znvd"
evidence_type: "test"
commit: "25b99687091461269a101cf75274c72fc734b4e7"
result: "pass"
captured_at: "2026-11-04T22:14:27.456761689+00:00"
command: "bash -c 'python3 scripts/test_gen_eddn_outfitting.py && cargo test -p astromech-eddn && cargo test -p astromech-core --test real_journals'"
exit_status: "0"
agent_identity: "implement-agent-58a7"
target:
  kind: "issue"
  id: "astromech-58a7"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-58a7"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Offline check-mode tests (4 pass: same lists pass; new, moved, gone symbols fail and are named; --check writes nothing; 3 of 4 fail against the pre-change script), astromech-eddn tests incl. unknown_modules_lists_every_one_the_visit_would_be_refused_for (152 pass), real_journals harness compiles and skips without a folder"
updated_at: "2026-11-04T22:14:31.252145421+00:00"
---

## Summary

Offline check-mode tests (4 pass: same lists pass; new, moved, gone symbols fail and are named; --check writes nothing; 3 of 4 fail against the pre-change script), astromech-eddn tests incl. unknown_modules_lists_every_one_the_visit_would_be_refused_for (152 pass), real_journals harness compiles and skips without a folder

## Command

```console
bash -c 'python3 scripts/test_gen_eddn_outfitting.py && cargo test -p astromech-eddn && cargo test -p astromech-core --test real_journals'
```

Exit status: 0

## Stdout

Bytes: 11986
SHA-256: 947dec5035d66b28c1fc6e9ef23eec71a9e0199d61c2e4aa0926630d3dbd6d24
Test summary: test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

## Stderr

Bytes: 964
SHA-256: c4fd5015927e6b46c4803265a512d4968392bfd6dcd67930e00b15985353989e
