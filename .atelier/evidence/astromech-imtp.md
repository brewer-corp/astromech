---
created_at: "2026-10-22T21:17:33.317980904+00:00"
id: "astromech-imtp"
evidence_type: "test"
commit: "9c69fe72f8f0fb65a25d229f5d7608e5bd449c12"
result: "pass"
captured_at: "2026-10-22T21:17:31.765763166+00:00"
command: "bash -c 'python3 scripts/test_gen_neutron.py && CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test neutron && CARGO_BUILD_JOBS=4 cargo clippy -p astromech-galaxy --all-targets -- -D warnings && cargo fmt --all --check'"
exit_status: "0"
agent_identity: "claude-implementer-1842"
target:
  kind: "issue"
  id: "astromech-nk0u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-nk0u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After review fixes (format v2, name group in header): the script rebuilds both fixtures byte for byte (35-star sample, 600-star synthetic across three name groups); the reader reads every position, name and cell range of both. Mutation: script group 128 with the header-driven reader passes the name reads; a reader hard-coding 256 fails them. Clippy and fmt clean."
updated_at: "2026-10-22T21:17:33.318252840+00:00"
---

## Summary

After review fixes (format v2, name group in header): the script rebuilds both fixtures byte for byte (35-star sample, 600-star synthetic across three name groups); the reader reads every position, name and cell range of both. Mutation: script group 128 with the header-driven reader passes the name reads; a reader hard-coding 256 fails them. Clippy and fmt clean.

## Command

```console
bash -c 'python3 scripts/test_gen_neutron.py && CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test neutron && CARGO_BUILD_JOBS=4 cargo clippy -p astromech-galaxy --all-targets -- -D warnings && cargo fmt --all --check'
```

Exit status: 0

## Stdout

Bytes: 339
Truncated: no

```text

running 5 tests
test a_cell_with_no_stars_is_empty ... ok
test refuses_other_files ... ok
test drops_the_systems_without_a_neutron_main_star ... ok
test reads_every_neutron_star_in_cell_order ... ok
test reads_names_across_name_groups ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```

## Stderr

Bytes: 320
Truncated: no

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.120s

OK
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running tests/neutron.rs (target/debug/deps/neutron-04bbc83f4c78a3aa)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s
```

