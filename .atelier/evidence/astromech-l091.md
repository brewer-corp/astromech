---
created_at: "2026-10-17T01:03:06.803732515+00:00"
id: "astromech-l091"
evidence_type: "test"
commit: "1ffbef839da4ec7ab4d15a962d9340306798d37a"
result: "pass"
captured_at: "2026-10-17T01:02:47.722315168+00:00"
command: "bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q > $HOME/.cache/6xyo-test.log 2>&1 && grep -cE '\"'\"'test result: ok'\"'\"' $HOME/.cache/6xyo-test.log && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo all-gates-ok'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-6xyo"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-6xyo"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After merging main (5t34 follow_history): fmt, clippy -D warnings, workspace tests, check-deps, app metadata"
updated_at: "2026-10-17T01:03:06.804166235+00:00"
---

## Summary

After merging main (5t34 follow_history): fmt, clippy -D warnings, workspace tests, check-deps, app metadata

## Command

```console
bash -c 'cargo fmt --all --check && cargo clippy --workspace --all-targets -q -- -D warnings && cargo test --workspace -q > $HOME/.cache/6xyo-test.log 2>&1 && grep -cE '"'"'test result: ok'"'"' $HOME/.cache/6xyo-test.log && python3 scripts/check-deps.py && cd app/src-tauri && cargo metadata --locked --offline --format-version 1 >/dev/null && echo all-gates-ok'
```

Exit status: 0

## Stdout

Bytes: 58
Truncated: no

```text
76
dependency rules OK (26 workspace crates)
all-gates-ok
```

## Stderr

Bytes: 0
Truncated: no

```text
```

