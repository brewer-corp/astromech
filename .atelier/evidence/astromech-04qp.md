---
created_at: "2026-10-16T22:15:13.826177681+00:00"
id: "astromech-04qp"
evidence_type: "test"
commit: "c3ba251b6d1aa555a5ed56aadd41c6dc59803135"
result: "pass"
captured_at: "2026-10-16T22:15:11.853021721+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-engineering unlocks && cd app && npx vitest run src/modules/unlocks'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-361l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-361l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Given the fixture EngineerProgress (tests/unlocks, login snapshot plus a later single-engineer change), the tree marks unlocked (Felicity rank 5; Liz Ryder from the newer event), invited (Kit Fowler), known (Marco Qwent; never-logged common-knowledge Tod McQuinn) and locked engineers, and names what is left for each locked one (introduction from an unlocked introducer, suit referral task, or 'Unlock X first'); depths follow introductions; unlock data covers all 25 ship and 13 suit engineers incl. every engineer the trip planner knows. View.test.ts checks the page."
updated_at: "2026-10-16T22:15:13.826539546+00:00"
---

## Summary

Given the fixture EngineerProgress (tests/unlocks, login snapshot plus a later single-engineer change), the tree marks unlocked (Felicity rank 5; Liz Ryder from the newer event), invited (Kit Fowler), known (Marco Qwent; never-logged common-knowledge Tod McQuinn) and locked engineers, and names what is left for each locked one (introduction from an unlocked introducer, suit referral task, or 'Unlock X first'); depths follow introductions; unlock data covers all 25 ship and 13 suit engineers incl. every engineer the trip planner knows. View.test.ts checks the page.

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-engineering unlocks && cd app && npx vitest run src/modules/unlocks'
```

Exit status: 0

## Stdout

Bytes: 356
Truncated: no

```text

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 21 filtered out; finished in 0.00s


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-sfys/app


 Test Files  1 passed (1)
      Tests  1 passed (1)
   Start at  18:15:12
   Duration  1.04s (environment 49%, tests 27%, setup 13%, transform 10%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

