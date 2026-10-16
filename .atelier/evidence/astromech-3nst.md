---
created_at: "2026-10-16T19:22:32.838574135+00:00"
id: "astromech-3nst"
evidence_type: "validation"
commit: "318f1bf8937e3c832a2cc9a86e36311141ff6ae1"
result: "pass"
captured_at: "2026-10-16T19:22:32.838510697+00:00"
target:
  kind: "issue"
  id: "astromech-3njr"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-3njr"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "The component tests can fail: with Stats.svelte's Plan column changed to show the current value, pnpm test failed the distributor, FSD and thermal booster tests (3 failed, 3 passed); restored, 6 passed. The planned_modules check in check_plan asserts the plan gives exactly the modules Coriolis was run on; a no-op plan would fail every plan_* stat check because the expected values differ from the base."
updated_at: "2026-10-16T19:22:32.838966919+00:00"
---

The component tests can fail: with Stats.svelte's Plan column changed to show the current value, pnpm test failed the distributor, FSD and thermal booster tests (3 failed, 3 passed); restored, 6 passed. The planned_modules check in check_plan asserts the plan gives exactly the modules Coriolis was run on; a no-op plan would fail every plan_* stat check because the expected values differ from the base.
