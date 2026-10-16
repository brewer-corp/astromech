---
created_at: "2026-10-16T19:28:25.232601221+00:00"
id: "astromech-8z7g"
evidence_type: "test"
commit: "75aaae2fbb5e6f8d9b8a2a4185445b58c1c9e398"
result: "pass"
captured_at: "2026-10-16T19:28:25.084659067+00:00"
command: "env FILTERSTATE=/home/supadava/projects/astromech-wt-feature-astromech-qz9u/app/src/lib/filterstate.ts node --test /tmp/qz9u-tests/filterstate.test.mjs"
exit_status: "0"
agent_identity: "claude-opus-5.5"
target:
  kind: "issue"
  id: "astromech-qz9u"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-qz9u"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "R1/R2: lib/filterstate.ts unit tests under node --test (script kept outside the repo at /tmp/qz9u-tests/filterstate.test.mjs, since this branch has no pnpm test setup): failed load falls back to defaults and logs; failed save keeps the previous filters with a 'Couldn't save the filter' error; save reloads; failed reload keeps the new filters."
updated_at: "2026-10-16T19:28:25.232842010+00:00"
---

## Summary

R1/R2: lib/filterstate.ts unit tests under node --test (script kept outside the repo at /tmp/qz9u-tests/filterstate.test.mjs, since this branch has no pnpm test setup): failed load falls back to defaults and logs; failed save keeps the previous filters with a 'Couldn't save the filter' error; save reloads; failed reload keeps the new filters.

## Command

```console
env FILTERSTATE=/home/supadava/projects/astromech-wt-feature-astromech-qz9u/app/src/lib/filterstate.ts node --test /tmp/qz9u-tests/filterstate.test.mjs
```

Exit status: 0

## Stdout

Bytes: 414
Truncated: no

```text
✔ a failed load falls back to the defaults so the search still runs (1.191817ms)
✔ a good load returns the saved filters (0.150902ms)
✔ a failed save keeps the old filters and says why (0.285374ms)
✔ a save reloads (pad back to the ship's); a failed reload keeps the new filters (0.233386ms)
ℹ tests 4
ℹ suites 0
ℹ pass 4
ℹ fail 0
ℹ cancelled 0
ℹ skipped 0
ℹ todo 0
ℹ duration_ms 91.913171
```

## Stderr

Bytes: 0
Truncated: no

```text
```

