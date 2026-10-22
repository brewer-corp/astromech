---
created_at: "2026-10-22T21:44:27.976799786+00:00"
id: "astromech-w94p"
evidence_type: "test"
commit: "b4e118b5345dafd978718e915eb4f40562203b41"
result: "pass"
captured_at: "2026-10-22T21:44:27.523286614+00:00"
command: "bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-status && python3 scripts/test_mvp_criteria.py'"
exit_status: "0"
agent_identity: "claude-implementer-0k88"
target:
  kind: "issue"
  id: "astromech-lnzj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-lnzj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "gui_focus_values_match_frontiers_manual: GUI_ constants equal Frontier's Journal Manual v38 p.97 table (1 right/InternalPanel, 2 left/ExternalPanel; failed on the old values); test_mvp_criteria checks the harness copy against the crate"
updated_at: "2026-10-22T21:44:27.977229988+00:00"
---

## Summary

gui_focus_values_match_frontiers_manual: GUI_ constants equal Frontier's Journal Manual v38 p.97 table (1 right/InternalPanel, 2 left/ExternalPanel; failed on the old values); test_mvp_criteria checks the harness copy against the crate

## Command

```console
bash -c 'CARGO_BUILD_JOBS=4 cargo test -p astromech-status && python3 scripts/test_mvp_criteria.py'
```

Exit status: 0

## Stdout

Bytes: 403
Truncated: no

```text

running 3 tests
test tests::gui_focus_values_match_frontiers_manual ... ok
test status_file::tests::position_on_a_body_is_read ... ok
test tests::parses_flags_and_rejects_partial_writes ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Stderr

Bytes: 313
Truncated: no

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/astromech_status-5a34b67e0cde254f)
   Doc-tests astromech_status
.......................
----------------------------------------------------------------------
Ran 23 tests in 0.003s

OK
```

