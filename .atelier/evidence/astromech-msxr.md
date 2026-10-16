---
created_at: "2026-10-16T21:59:11.098433546+00:00"
id: "astromech-msxr"
evidence_type: "test"
commit: "7f41910b1b3b72d60fd8446873bc1400035b125e"
result: "pass"
captured_at: "2026-10-16T21:59:06.759833368+00:00"
command: "bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-galaxy --lib hge && cargo test -q -p astromech-materials && cd app && npx vitest run src/lib/ui/Nearby.test.ts'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-bwmz"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-bwmz"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "From a fixed position on a recorded-shape Spansh answer: a 2-billion Federation system 30 ly away (score 72) ranks above a 200-million one 5 ly away (score 5); Famine keeps nearest-first without a score; hge_score properties; Nearby.test.ts shows 'score N · pop X' on HGE rows only"
updated_at: "2026-10-16T21:59:11.098765404+00:00"
---

## Summary

From a fixed position on a recorded-shape Spansh answer: a 2-billion Federation system 30 ly away (score 72) ranks above a 200-million one 5 ly away (score 5); Famine keeps nearest-first without a score; hge_score properties; Nearby.test.ts shows 'score N · pop X' on HGE rows only

## Command

```console
bash -c 'export CARGO_BUILD_JOBS=3 && cargo test -q -p astromech-galaxy --lib hge && cargo test -q -p astromech-materials && cd app && npx vitest run src/lib/ui/Nearby.test.ts'
```

Exit status: 0

## Stdout

Bytes: 711
Truncated: no

```text

running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-sfys/app


 Test Files  1 passed (1)
      Tests  2 passed (2)
   Start at  17:59:09
   Duration  1.15s (environment 60%, transform 15%, setup 15%, tests 7%, import 2%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

