---
created_at: "2026-10-16T23:23:44.925699245+00:00"
id: "astromech-shqj"
evidence_type: "test"
commit: "2f214f574681234fd0de00b760b6c2f74030528c"
result: "pass"
captured_at: "2026-10-16T23:23:22.674470305+00:00"
command: "bash -c 'set -e; cd app; pnpm install --frozen-lockfile >/dev/null; pnpm lint; pnpm check | tail -1; pnpm build | tail -1; pnpm test 2>&1 | grep -E \"Test Files|Tests \"'"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-05bi"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-05bi"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "App gates on 2f214f5 (review fix x81v, in-flight poll guard): pnpm install --frozen-lockfile, lint, check, build, test (19 files, 91 tests). The Rust side is unchanged since f219f91 (evidence ebt8)."
updated_at: "2026-10-16T23:23:44.926079845+00:00"
---

## Summary

App gates on 2f214f5 (review fix x81v, in-flight poll guard): pnpm install --frozen-lockfile, lint, check, build, test (19 files, 91 tests). The Rust side is unchanged since f219f91 (evidence ebt8).

## Command

```console
bash -c 'set -e; cd app; pnpm install --frozen-lockfile >/dev/null; pnpm lint; pnpm check | tail -1; pnpm build | tail -1; pnpm test 2>&1 | grep -E "Test Files|Tests "'
```

Exit status: 0

## Stdout

Bytes: 255
Truncated: no

```text

> astromech-app@0.1.5 lint /home/supadava/projects/astromech-wt-feature-astromech-k3wp/app
> eslint .

1792193015599 COMPLETED 302 FILES 0 ERRORS 0 WARNINGS 0 FILES_WITH_PROBLEMS
✓ built in 1.60s
 Test Files  19 passed (19)
      Tests  91 passed (91)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

