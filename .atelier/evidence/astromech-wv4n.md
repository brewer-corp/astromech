---
created_at: "2026-10-16T22:03:25.695525897+00:00"
id: "astromech-wv4n"
evidence_type: "test"
commit: "7b5fc669d61807de8b34d9d2205cd5244d975507"
result: "pass"
captured_at: "2026-10-16T22:03:21.832265552+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-galaxy --lib && cd app && npx vitest run src/lib/ui/Nearby.test.ts'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-n0qb"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-n0qb"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After replaying an FSDJump fixture (journal-arrivals.log, 16 Oct) over the recorded Spansh famine answer (14 Oct): Ross 154's states (Famine, Civil Unrest) and age (2026-10-16T12:00:00Z) come from the journal; Tau Ceti, whose famine ended on arrival, drops out and shows up for the Boom lookup; an older visit does not undo a newer one; state names take Spansh spelling. Nearby.test.ts shows 'seen on your visit'."
updated_at: "2026-10-16T22:03:25.695969614+00:00"
---

## Summary

After replaying an FSDJump fixture (journal-arrivals.log, 16 Oct) over the recorded Spansh famine answer (14 Oct): Ross 154's states (Famine, Civil Unrest) and age (2026-10-16T12:00:00Z) come from the journal; Tau Ceti, whose famine ended on arrival, drops out and shows up for the Boom lookup; an older visit does not undo a newer one; state names take Spansh spelling. Nearby.test.ts shows 'seen on your visit'.

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-galaxy --lib && cd app && npx vitest run src/lib/ui/Nearby.test.ts'
```

Exit status: 0

## Stdout

Bytes: 382
Truncated: no

```text

running 16 tests
................
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-sfys/app


 Test Files  1 passed (1)
      Tests  3 passed (3)
   Start at  18:03:24
   Duration  1.34s (environment 56%, setup 16%, transform 15%, tests 11%, import 2%, worker 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

