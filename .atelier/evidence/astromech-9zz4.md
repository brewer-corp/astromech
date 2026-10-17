---
created_at: "2026-10-17T17:28:48.889354945+00:00"
id: "astromech-9zz4"
evidence_type: "validation"
commit: "eb6346ea4377eb9b4e16a8695e633d5b15bd25bd"
result: "pass"
captured_at: "2026-10-17T17:28:48.889291707+00:00"
agent_identity: "docs-agent"
target:
  kind: "issue"
  id: "astromech-basj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-basj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Negative check: pointing one manifest at trade-x.md and another at galaxy.md#nope made help.test.ts fail naming both ('no docs/guide/trade-x.md', 'no heading #nope in galaxy.md'); reverted. Help button click calls window.open with the guide URL for powerplay.md and trade.md (App tests). Screenshots (docs/guide/img) were taken from scripts' preview build with the example preset; no Frontier assets. Not run in the Tauri shell on Windows: open_external already allows https://github.com/spadaval/astromech* so the link should open, but that is untested here."
updated_at: "2026-10-17T17:28:48.889673098+00:00"
---

Negative check: pointing one manifest at trade-x.md and another at galaxy.md#nope made help.test.ts fail naming both ('no docs/guide/trade-x.md', 'no heading #nope in galaxy.md'); reverted. Help button click calls window.open with the guide URL for powerplay.md and trade.md (App tests). Screenshots (docs/guide/img) were taken from scripts' preview build with the example preset; no Frontier assets. Not run in the Tauri shell on Windows: open_external already allows https://github.com/spadaval/astromech* so the link should open, but that is untested here.
