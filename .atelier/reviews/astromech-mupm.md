---
schema: atelier.review
schema_version: 2
id: astromech-mupm
title: review on astromech-y6d6 @a2e6c32d
target: astromech-y6d6
identity: claude-reviewer-v3u4
commit: a2e6c32d500e28b07a45bbcdf6f0aeda3c2e4c2f
created_at: 2026-11-07T19:22:37.917440115Z
event:
  event: review
---

Reviewed f47c529b (fix for 43sv) and a2e6c32d (opre disposition, records only). read_status_whole retries only on a read error or incomplete JSON; complete JSON is recorded (status_seen) and applied only when parse_status finds Flags, so a main-menu file is read once per change. Behaviour otherwise unchanged: the notification path now stops retrying on main-menu JSON (previously five fruitless tries, same outcome); a half-written or empty file still fails serde and is retried next idle pass. The new test a_main_menu_status_json_is_not_read_again_while_unchanged covers both the record and a later Flags write. Ran cargo test -p astromech-core --lib pipeline:: (68/68) and cargo clippy -p astromech-core --all-targets -D warnings (clean). CI on HEAD not yet seen.
