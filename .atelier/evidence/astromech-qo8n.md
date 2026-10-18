---
created_at: "2026-10-18T08:04:18.150016836+00:00"
id: "astromech-qo8n"
evidence_type: "validation"
commit: "5f1ff9fd9745a87910e2ee68940c394ef3ce5674"
result: "pass"
captured_at: "2026-10-18T08:04:18.149889548+00:00"
path: "docs/evidence/astromech-vkzj/round5-main-vs-head.txt"
agent_identity: "claude (author agent)"
target:
  kind: "issue"
  id: "astromech-vkzj"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-vkzj"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "After the round-5 fix (5f1ff9fd): main -> branch has 0 Ask/NoMatch -> Execute transitions and no Execute -> Execute command or slot change on every set, including the round-5 slot probe (299,614) and two-edit set (232,479). Eval transcripts: the same 81 clips execute -> ask; 0 wrong executes, 64/64 rejects (re-scored; the fresh audio eval was not re-run because the transcripts are deterministic and the re-score covers the grammar change)."
updated_at: "2026-10-18T08:04:18.150808982+00:00"
---

After the round-5 fix (5f1ff9fd): main -> branch has 0 Ask/NoMatch -> Execute transitions and no Execute -> Execute command or slot change on every set, including the round-5 slot probe (299,614) and two-edit set (232,479). Eval transcripts: the same 81 clips execute -> ask; 0 wrong executes, 64/64 rejects (re-scored; the fresh audio eval was not re-run because the transcripts are deterministic and the re-score covers the grammar change).
