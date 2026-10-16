---
created_at: "2026-10-16T18:57:50.719368280+00:00"
id: "astromech-aifo"
evidence_type: "test"
commit: "7b5ab437d7437ddaa9234eedc07915c8d200077b"
result: "fail"
captured_at: "2026-10-16T18:57:50.719322455+00:00"
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
title: "Before the fix (commit 7b5ab43): KeybindsView.test.ts 'ignores a mouse click that arrives as a Mouse_1 press' fails; after clicking Flight and injecting Mouse_1, all 18 groups are open instead of only Flight."
updated_at: "2026-10-16T18:57:50.720126165+00:00"
---

Before the fix (commit 7b5ab43): KeybindsView.test.ts 'ignores a mouse click that arrives as a Mouse_1 press' fails; after clicking Flight and injecting Mouse_1, all 18 groups are open instead of only Flight.
