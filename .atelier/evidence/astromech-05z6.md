---
created_at: "2026-10-16T19:24:53.089423415+00:00"
id: "astromech-05z6"
evidence_type: "test"
commit: "cbcfc2475678431decf542e4d0775b99a895675e"
result: "pass"
captured_at: "2026-10-16T19:24:52.770547896+00:00"
command: "cargo test -p astromech-core --lib setup::"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-mmqs"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-mmqs"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Review fixes R1-R3: concurrent setup edits both survive in file and memory (fails 4/5 runs with the lock removed), PTT = kill switch refused by the core, upgrading users with push-to-talk skip onboarding"
updated_at: "2026-10-16T19:24:53.089628809+00:00"
---

## Summary

Review fixes R1-R3: concurrent setup edits both survive in file and memory (fails 4/5 runs with the lock removed), PTT = kill switch refused by the core, upgrading users with push-to-talk skip onboarding

## Command

```console
cargo test -p astromech-core --lib setup::
```

Exit status: 0

## Stdout

Bytes: 657
Truncated: no

```text

running 9 tests
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 32 filtered out; finished in 0.14s
```

## Stderr

Bytes: 159
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.11s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-8c448b21aa4d9de9)
```

