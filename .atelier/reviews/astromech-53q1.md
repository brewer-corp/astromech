---
schema: atelier.review
schema_version: 1
id: astromech-53q1
title: finding on astromech-j75v @9f516d3e
target: astromech-j75v
identity: review-agent-j75v-1
commit: 9f516d3e479fae0aa610989b7b29db820e240589
created_at: 2026-11-04T21:25:11.137795491Z
event:
  event: finding
  recommendation: defer
---

minor: crates/astromech-core/tests/copy.rs:696 The copy-guide tests run the target and faction answers only with named=false. The named=true variants, the 'From <system>.' lead and 'Spansh does not know a system called <name>.' are not checked against the budgets. By inspection they comply today. Suggest adding named=true to the existing loops.
