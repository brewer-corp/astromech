---
created_at: "2026-10-17T19:44:45.793786450+00:00"
id: "astromech-yrxa"
evidence_type: "test"
commit: "88fda883691c5fe0c634a943740c744e015d3a07"
result: "pass"
captured_at: "2026-10-17T19:44:45.373946336+00:00"
command: "cargo test -p astromech-core --lib update"
exit_status: "0"
agent_identity: "implementer"
target:
  kind: "issue"
  id: "astromech-naa3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-naa3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-core --lib update"
updated_at: "2026-10-17T19:44:45.796823053+00:00"
---

## Summary

cargo test -p astromech-core --lib update

## Command

```console
cargo test -p astromech-core --lib update
```

Exit status: 0

## Stdout

Bytes: 510
Truncated: no

```text

running 7 tests
test update::tests::the_tray_check_item_follows_the_update_status ... ok
test update::tests::version_order ... ok
test update::tests::baked_public_key_parses ... ok
test route::tests::with_following_off_jumps_still_update_where_you_are ... ok
test update::tests::copy_skips_data_dir ... ok
test update::tests::zip_roundtrip_and_hash ... ok
test update::tests::signs_and_verifies_manifest ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 120 filtered out; finished in 0.01s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-70016420cf9e37ea)
```

