---
created_at: "2026-10-17T17:10:57.338590522+00:00"
id: "astromech-wowc"
evidence_type: "validation"
commit: "f9d5dc286dde126b220f7c03cc3c26bc4f1effa9"
result: "pass"
captured_at: "2026-10-17T17:10:57.338518778+00:00"
agent_identity: "docs-agent"
target:
  kind: "issue"
  id: "astromech-stxg"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-stxg"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "README has no M0/gear-down-slice framing (grep for 'M0 preview' and 'gear-down' returns nothing); it links every guide page via docs/guide/index.md; CONTRIBUTING.md 'Before a PR' says a user-visible change updates its guide page in the same PR. Static check only; page bodies land in the next children."
updated_at: "2026-10-17T17:10:57.338946185+00:00"
---

README has no M0/gear-down-slice framing (grep for 'M0 preview' and 'gear-down' returns nothing); it links every guide page via docs/guide/index.md; CONTRIBUTING.md 'Before a PR' says a user-visible change updates its guide page in the same PR. Static check only; page bodies land in the next children.
