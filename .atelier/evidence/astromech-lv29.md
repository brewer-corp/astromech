---
created_at: "2026-10-22T20:53:20.241270816+00:00"
id: "astromech-lv29"
evidence_type: "test"
commit: "6e4acb802b5cb85ae61927727acf03040cd8db40"
result: "pass"
captured_at: "2026-10-22T20:53:14.611340881+00:00"
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
title: "Script builds the committed fixture byte for byte from the 39-row sample (black hole, B, O and no-mainStar rows dropped); the Rust reader returns every star's position, name and cell range in the expected order, refuses bad magic, other versions and short files. Real dump (run by hand, workflow build step with curl swapped for a local copy): 5,045,154 stars, 1,645,100 cells, 184 MB, neutron.bin.gz 71,139,428 bytes (< 100 MB) with SHA-256 file; reader opens it in 96 ms (release). Mutation check: sorting by name within a cell fails both the Python and the Rust test."
updated_at: "2026-10-22T20:53:20.241723480+00:00"
---

## Summary

Script builds the committed fixture byte for byte from the 39-row sample (black hole, B, O and no-mainStar rows dropped); the Rust reader returns every star's position, name and cell range in the expected order, refuses bad magic, other versions and short files. Real dump (run by hand, workflow build step with curl swapped for a local copy): 5,045,154 stars, 1,645,100 cells, 184 MB, neutron.bin.gz 71,139,428 bytes (< 100 MB) with SHA-256 file; reader opens it in 96 ms (release). Mutation check: sorting by name within a cell fails both the Python and the Rust test.

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

Bytes: 380
Truncated: no

```text
..
----------------------------------------------------------------------
Ran 2 tests in 0.117s

OK
cargo-gate: 2 builds already running, waiting for a slot...
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running tests/neutron.rs (target/debug/deps/neutron-710560756f8b0cae)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s
```

