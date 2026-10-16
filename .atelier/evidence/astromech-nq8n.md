---
created_at: "2026-10-16T20:45:58.925350165+00:00"
id: "astromech-nq8n"
evidence_type: "test"
commit: "5ec872891199c78c841ce74dc64ee7bb15cfa311"
result: "pass"
captured_at: "2026-10-16T20:45:55.780876611+00:00"
command: "cargo test -p astromech-core --lib -- setup eddn_log"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-09s3"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-09s3"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 5ec8728, astromech-core: the_eddn_switch_is_saved_and_keeps_its_comment (set_eddn through setup::apply), eddn_log_reads_the_outbox_without_creating_it (last_error carries the failed send's detail; skips listed apart)"
updated_at: "2026-10-16T20:45:58.925787542+00:00"
---

## Summary

At 5ec8728, astromech-core: the_eddn_switch_is_saved_and_keeps_its_comment (set_eddn through setup::apply), eddn_log_reads_the_outbox_without_creating_it (last_error carries the failed send's detail; skips listed apart)

## Command

```console
cargo test -p astromech-core --lib -- setup eddn_log
```

Exit status: 0

## Stdout

Bytes: 996
Truncated: no

```text

running 14 tests
test setup::tests::finds_elite_from_its_folders ... ok
test setup::tests::requests_become_edits ... ok
test setup::tests::a_broken_config_is_not_overwritten ... ok
test setup::tests::edits_keep_comments_and_other_settings ... ok
test setup::tests::voice_settings_persist ... ok
test setup::tests::ptt_and_kill_switch_must_differ ... ok
test setup::tests::writes_a_new_file_and_clears_a_button ... ok
test setup::tests::ptt_written_by_the_wizard_then_restart_is_still_not_onboarded ... ok
test setup::tests::onboarding_flag ... ok
test setup::tests::the_eddn_switch_is_saved_and_keeps_its_comment ... ok
test setup::tests::upgrading_users_with_push_to_talk_skip_onboarding ... ok
test setup::tests::fresh_installs_need_onboarding ... ok
test app::tests::eddn_log_reads_the_outbox_without_creating_it ... ok
test setup::tests::concurrent_edits_all_survive_in_file_and_memory ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.26s
```

## Stderr

Bytes: 397
Truncated: no

```text
   Compiling astromech-eddn v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-eddn)
   Compiling astromech-core v0.1.5 (/home/supadava/projects/astromech-wt-feature-astromech-tyjz/crates/astromech-core)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
     Running unittests src/lib.rs (target/debug/deps/astromech_core-d5290a02597a1840)
```

