---
created_at: "2026-10-17T20:06:15.139147058+00:00"
id: "astromech-hjt1"
evidence_type: "test"
commit: "33bdbfda02fb10964e7435021df7a1c5519ed0f6"
result: "pass"
captured_at: "2026-10-17T20:06:13.530405283+00:00"
command: "cargo test -q -p astromech-core --lib an_older_journal_written_after_we_follow_it_is_still_a_replay"
exit_status: "0"
agent_identity: "claude-diagnose"
target:
  kind: "issue"
  id: "astromech-0d7l"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-0d7l"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "pipeline unit test an_older_journal_written_after_we_follow_it_is_still_a_replay passes (failed before the fix: 2 EDDN messages queued from an older journal)"
updated_at: "2026-10-17T20:06:15.139429515+00:00"
---

## Summary

pipeline unit test an_older_journal_written_after_we_follow_it_is_still_a_replay passes (failed before the fix: 2 EDDN messages queued from an older journal)

## Command

```console
cargo test -q -p astromech-core --lib an_older_journal_written_after_we_follow_it_is_still_a_replay
```

Exit status: 0

## Stdout

Bytes: 115
Truncated: no

```text

running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 129 filtered out; finished in 0.05s
```

## Stderr

Bytes: 0
Truncated: no

```text
```

