---
created_at: "2026-10-16T19:22:13.403533588+00:00"
id: "astromech-ej9n"
evidence_type: "test"
commit: "80f3f0fee3e26fc2784defc9b5a53f3dde2bcb74"
result: "pass"
captured_at: "2026-10-16T19:22:05.390714909+00:00"
command: "pnpm test"
exit_status: "0"
target:
  kind: "issue"
  id: "astromech-rhfn"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-rhfn"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Re-review fix (N4): Mouse_1 click still ignored; a Mouse_4 bound in the game still opens its group and names the binding (the new test failed before the fix)"
updated_at: "2026-10-16T19:22:13.403941350+00:00"
---

## Summary

Re-review fix (N4): Mouse_1 click still ignored; a Mouse_4 bound in the game still opens its group and names the binding (the new test failed before the fix)

## Command

```console
pnpm test
```

Exit status: 0

## Stdout

Bytes: 347
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-q318/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-q318/app


 Test Files  4 passed (4)
      Tests  16 passed (16)
   Start at  15:22:06
   Duration  6.69s (tests 48%, environment 29%, transform 13%, setup 9%, import 1%)
```

## Stderr

Bytes: 0
Truncated: no

```text
```

