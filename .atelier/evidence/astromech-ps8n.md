---
created_at: "2026-10-16T21:54:58.999768701+00:00"
id: "astromech-ps8n"
evidence_type: "test"
commit: "7bdf6080b1ede3015d034190cb9c5e00c553fb1e"
result: "pass"
captured_at: "2026-10-16T21:54:48.873461251+00:00"
command: "bash -c 'cargo test -q -p astromech-galaxy --test sites && cargo test -q -p astromech-core --test site_lookup && cargo test -q -p astromech-intent && cargo test -q -p astromech-tools && cargo test -q -p astromech-materials && cargo test -q -p astromech-gamestate'"
exit_status: "0"
agent_identity: "claude-implementer"
target:
  kind: "issue"
  id: "astromech-osjr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-osjr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Site lookups from fixed positions (Sol, HIP 36601) on recorded Spansh bodies searches: nearest of each kind with distance and jumps (estimate and exact on the star map), filters, ignored-filter warning; voice 'nearest crystalline shards' resolves to the lookup and speaks the nearest site"
updated_at: "2026-10-16T21:54:59.000080893+00:00"
---

## Summary

Site lookups from fixed positions (Sol, HIP 36601) on recorded Spansh bodies searches: nearest of each kind with distance and jumps (estimate and exact on the star map), filters, ignored-filter warning; voice 'nearest crystalline shards' resolves to the lookup and speaks the nearest site

## Command

```console
bash -c 'cargo test -q -p astromech-galaxy --test sites && cargo test -q -p astromech-core --test site_lookup && cargo test -q -p astromech-intent && cargo test -q -p astromech-tools && cargo test -q -p astromech-materials && cargo test -q -p astromech-gamestate'
```

Exit status: 0

## Stdout

Bytes: 1412
Truncated: no

```text

running 6 tests
......
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 10 tests
..........
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 7 tests
.......
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 13 tests
.............
test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 9 tests
.........
test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

