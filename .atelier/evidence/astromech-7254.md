---
created_at: "2026-10-22T21:04:22.518466176+00:00"
id: "astromech-7254"
evidence_type: "test"
commit: "8fb56609d361be548cd201d6ca79556ecbadb8af"
result: "pass"
captured_at: "2026-10-22T21:04:18.471400478+00:00"
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
title: "Re-run at the d4of commit (neutron.rs gained the download code): script still builds the committed fixture byte for byte; the reader's position, name, order and cell-range tests pass; clippy and fmt clean."
updated_at: "2026-10-22T21:04:22.519017042+00:00"
---

## Summary

Re-run at the d4of commit (neutron.rs gained the download code): script still builds the committed fixture byte for byte; the reader's position, name, order and cell-range tests pass; clippy and fmt clean.

## Command

```console
bash -c 'python3 scripts/test_gen_neutron.py && CARGO_BUILD_JOBS=4 cargo test -p astromech-galaxy --test neutron && CARGO_BUILD_JOBS=4 cargo clippy -p astromech-galaxy --all-targets -- -D warnings && cargo fmt --all --check'
```

Exit status: 0

## Stdout

Bytes: 296
Truncated: no

```text

running 4 tests
test a_cell_with_no_stars_is_empty ... ok
test refuses_other_files ... ok
test drops_the_systems_without_a_neutron_main_star ... ok
test reads_every_neutron_star_in_cell_order ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 814
Truncated: no

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.103s

OK
   Compiling astromech-data v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-1842/crates/astromech-data)
   Compiling astromech-engineering v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-1842/crates/astromech-engineering)
   Compiling astromech-ship v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-1842/crates/astromech-ship)
   Compiling astromech-galaxy v1.0.0-alpha.2 (/home/supadava/projects/astromech-wt/astromech-1842/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.16s
     Running tests/neutron.rs (target/debug/deps/neutron-04bbc83f4c78a3aa)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
```

