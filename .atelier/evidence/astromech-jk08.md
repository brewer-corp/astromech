---
created_at: "2026-10-22T23:33:01.132783906+00:00"
id: "astromech-jk08"
evidence_type: "test"
commit: "ed861d2534e76accb1888784bc7d47d6273fedb7"
result: "pass"
captured_at: "2026-10-22T23:32:59.452316920+00:00"
command: "sh -c 'cargo fmt --all --check && CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings && python3 scripts/check-deps.py && python3 scripts/check-guide.py'"
exit_status: "0"
agent_identity: "claude-implementer-3x0x"
target:
  kind: "issue"
  id: "astromech-3x0x"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3x0x"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Rust gates: cargo fmt --all --check, clippy -D warnings over the workspace, check-deps, check-guide"
updated_at: "2026-10-22T23:33:01.133078124+00:00"
---

## Summary

Rust gates: cargo fmt --all --check, clippy -D warnings over the workspace, check-deps, check-guide

## Command

```console
sh -c 'cargo fmt --all --check && CARGO_BUILD_JOBS=4 cargo clippy --workspace --all-targets -- -D warnings && python3 scripts/check-deps.py && python3 scripts/check-guide.py'
```

Exit status: 0

## Stdout

Bytes: 79
Truncated: no

```text
dependency rules OK (29 workspace crates)
checked 22 guide pages: 0 problem(s)
```

## Stderr

Bytes: 72
Truncated: no

```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
```

