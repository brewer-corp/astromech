---
created_at: "2026-10-22T21:44:04.005666837+00:00"
id: "astromech-vqr8"
evidence_type: "test"
commit: "b4e118b5345dafd978718e915eb4f40562203b41"
result: "pass"
captured_at: "2026-10-22T21:44:03.780950739+00:00"
command: "python3 scripts/test_webview_cost.py"
exit_status: "0"
agent_identity: "claude-implementer-0k88"
target:
  kind: "issue"
  id: "astromech-k8yk"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-k8yk"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Harness judge now fails cold_start when the after-crash start is >= 2 s (test_a_start_after_a_core_crash_counts). The shell fix (core check off the setup path) is inferred from code; the Windows measurement comes from the App window cost workflow on the PR, not run here"
updated_at: "2026-10-22T21:44:04.006174814+00:00"
---

## Summary

Harness judge now fails cold_start when the after-crash start is >= 2 s (test_a_start_after_a_core_crash_counts). The shell fix (core check off the setup path) is inferred from code; the Windows measurement comes from the App window cost workflow on the PR, not run here

## Command

```console
python3 scripts/test_webview_cost.py
```

Exit status: 0

## Stdout

Bytes: 0
Truncated: no

```text
```

## Stderr

Bytes: 106
Truncated: no

```text
........
----------------------------------------------------------------------
Ran 8 tests in 0.001s

OK
```

