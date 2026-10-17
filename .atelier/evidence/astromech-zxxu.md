---
created_at: "2026-10-17T00:27:24.178410648+00:00"
id: "astromech-zxxu"
evidence_type: "test"
commit: "467ae488f07bd2be3d1861f435d8e3cf0e166a86"
result: "pass"
captured_at: "2026-10-17T00:27:09.718235497+00:00"
command: "cargo test -p astromech-intent -p astromech-galaxy -p astromech-core --lib -- class_and_rating_said_apart spoken_words_give_class module_phrases_resolve"
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
title: "R5: 'nearest A rated FSD' / 'A-rated' / 'A grade' / 'grade A' keep the rating (normalise to 'arated', ModuleWant rating A); failed before: rating dropped as filler"
updated_at: "2026-10-17T00:27:24.237669103+00:00"
---

## Summary

R5: 'nearest A rated FSD' / 'A-rated' / 'A grade' / 'grade A' keep the rating (normalise to 'arated', ModuleWant rating A); failed before: rating dropped as filler

## Command

```console
cargo test -p astromech-intent -p astromech-galaxy -p astromech-core --lib -- class_and_rating_said_apart spoken_words_give_class module_phrases_resolve
```

Exit status: 0

## Stdout

Bytes: 534
Truncated: no

```text

running 1 test
test lookups::tests::module_phrases_resolve_to_lookups ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 71 filtered out; finished in 4.45s


running 1 test
test outfitting::tests::spoken_words_give_class_rating_and_mount ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s


running 1 test
test tests::class_and_rating_said_apart_stay_together ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.00s
```

## Stderr

Bytes: 809
Truncated: no

```text
   Compiling astromech-galaxy v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-970a/crates/astromech-galaxy)
   Compiling astromech-market v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-970a/crates/astromech-market)
   Compiling astromech-tools v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-970a/crates/astromech-tools)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-task-astromech-970a/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 9.53s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-094d1cbbf733166e)
     Running unittests src/lib.rs (target/debug/deps/astromech_galaxy-5995832b88110b0b)
     Running unittests src/lib.rs (target/debug/deps/astromech_intent-49143e7857eb60de)
```

