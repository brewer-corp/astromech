---
created_at: "2026-10-23T00:50:35.324863688+00:00"
id: "astromech-1pxi"
evidence_type: "test"
commit: "9f376ee030beefc92ba87f5f0d99b4ce03c7cc78"
result: "pass"
captured_at: "2026-10-23T00:50:35.177893099+00:00"
command: "grep -n brewer-corp/astromech/releases crates/astromech-core/src/update.rs scripts/pack-update.py"
exit_status: "0"
agent_identity: "claude-implementer-rcui"
target:
  kind: "issue"
  id: "astromech-rcui"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-rcui"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Updater feed and download URLs point at brewer-corp (update.rs DEFAULT_FEED and download URL, pack-update.py latest.json url)"
updated_at: "2026-10-23T00:50:35.325179096+00:00"
---

## Summary

Updater feed and download URLs point at brewer-corp (update.rs DEFAULT_FEED and download URL, pack-update.py latest.json url)

## Command

```console
grep -n brewer-corp/astromech/releases crates/astromech-core/src/update.rs scripts/pack-update.py
```

Exit status: 0

## Stdout

Bytes: 388
Truncated: no

```text
crates/astromech-core/src/update.rs:23:pub const DEFAULT_FEED: &str = "https://github.com/brewer-corp/astromech/releases/latest/download/latest.json";
crates/astromech-core/src/update.rs:228:        "https://github.com/brewer-corp/astromech/releases/download/v{}/{}",
scripts/pack-update.py:66:    url = f"https://github.com/brewer-corp/astromech/releases/download/v{version}/{zip_name}"
```

## Stderr

Bytes: 0
Truncated: no

```text
```

