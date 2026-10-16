---
created_at: "2026-10-16T18:34:08.357586890+00:00"
id: "astromech-m0v6"
evidence_type: "test"
commit: "e5e955183283d226c7c75a32b20fc57e45f4774d"
result: "fail"
captured_at: "2026-10-16T18:34:05.800943326+00:00"
command: "bash -c 'cd app && pnpm test'"
exit_status: "1"
target:
  kind: "issue"
  id: "astromech-kbcm"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kbcm"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Before the fix: group-panel component test FAILS on the old code (filtered groups all start open together)"
updated_at: "2026-10-16T18:34:08.358113933+00:00"
---

## Summary

Before the fix: group-panel component test FAILS on the old code (filtered groups all start open together)

## Command

```console
bash -c 'cd app && pnpm test'
```

Exit status: 1

## Stdout

Bytes: 686
Truncated: no

```text

> astromech-app@0.1.5 test /home/supadava/projects/astromech-wt-feature-astromech-q318/app
> vitest run


 RUN  v5.0.5 /home/supadava/projects/astromech-wt-feature-astromech-q318/app

 ❯ src/modules/keybinds/KeybindsView.test.ts (4 tests | 2 failed) 423ms
   ❯ All keybinds group panels (4)
     × keeps panels independent when the SRV filter leaves three groups 87ms
     × keeps panels independent when the On foot filter leaves three groups 98ms

 Test Files  1 failed (1)
      Tests  2 failed | 2 passed (4)
   Start at  14:34:06
   Duration  1.41s (environment 44%, tests 33%, setup 11%, transform 11%, import 1%)

 ELIFECYCLE  Test failed. See above for more details.
```

## Stderr

Bytes: 1517
Truncated: no

```text

⎯⎯⎯⎯⎯⎯⎯ Failed Tests 2 ⎯⎯⎯⎯⎯⎯⎯

 FAIL  src/modules/keybinds/KeybindsView.test.ts > All keybinds group panels > keeps panels independent when the SRV filter leaves three groups
AssertionError: expected [ 'SRV driving', 'SRV turret', …(1) ] to deeply equal []

- Expected
+ Received

- []
+ [
+   "SRV driving",
+   "SRV turret",
+   "SRV systems",
+ ]

 ❯ src/modules/keybinds/KeybindsView.test.ts:41:36
     39|     expect(names()).toHaveLength(3);
     40|     // Nothing is open until you open it.
     41|     expect(names().filter(isOpen)).toEqual([]);
       |                                    ^
     42|
     43|     await fireEvent.click(header(first));

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[1/2]⎯

 FAIL  src/modules/keybinds/KeybindsView.test.ts > All keybinds group panels > keeps panels independent when the On foot filter leaves three groups
AssertionError: expected [ 'On foot movement', …(2) ] to deeply equal []

- Expected
+ Received

- []
+ [
+   "On foot movement",
+   "On foot combat",
+   "On foot tools and interface",
+ ]

 ❯ src/modules/keybinds/KeybindsView.test.ts:41:36
     39|     expect(names()).toHaveLength(3);
     40|     // Nothing is open until you open it.
     41|     expect(names().filter(isOpen)).toEqual([]);
       |                                    ^
     42|
     43|     await fireEvent.click(header(first));

⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯[2/2]⎯
```

