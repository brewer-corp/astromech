---
created_at: "2026-10-17T00:27:09.429524459+00:00"
id: "astromech-uipo"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:27:04.442151361+00:00"
command: "cargo test -p astromech-galaxy --lib a_mount_none_of_the_page_sells"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-970a"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-970a"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R4: a page that sells the module only in other mounts says none of the N nearest stations sell that mount (failed before: 'its search filter may have changed')"
updated_at: "2026-10-17T00:27:09.430010687+00:00"
---

## Summary

R4: a page that sells the module only in other mounts says none of the N nearest stations sell that mount (failed before: 'its search filter may have changed')

## Command

```console
cargo test -p astromech-galaxy --lib a_mount_none_of_the_page_sells
```

Exit status: 0

## Stdout

Bytes: 209
Truncated: no

```text

running 1 test
test stations::tests::a_mount_none_of_the_page_sells_says_so_not_that_the_filter_changed ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.04s
```

## Stderr

Bytes: 281
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-970a/crates/astromech-galaxy)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.63s
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-a08adfc363780b79)
```

