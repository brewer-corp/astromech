---
created_at: "2026-10-16T20:01:14.102975671+00:00"
id: "astromech-lo14"
evidence_type: "validation"
commit: "8069aac300c0a77cfa5f3a69a4f0b58c02e916a4"
result: "pass"
captured_at: "2026-10-16T20:01:14.102942810+00:00"
target:
  kind: "issue"
  id: "astromech-u3f4"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-u3f4"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Release timing at 8069aac (examples/import-history, Linux, Ryzen 5 PRO 6650H) on a regenerated 400.3 MB synthetic set (80 files, 888,854 lines, one commander): clean import 9.10 s, database 654.9 MB (1.64x). Out-of-order case: 79 newer files imported first (11.6 s), then the oldest file added: that import (11,105 lines) took 8.59 s including the full rebuild of the commander's ledgers in game-time order, and ended with the same ledger counts as the clean import (visits 74,679, earnings 25,218, ships 12, scans 341,378). Synthetic data deleted afterwards."
updated_at: "2026-10-16T20:01:14.103183198+00:00"
---

Release timing at 8069aac (examples/import-history, Linux, Ryzen 5 PRO 6650H) on a regenerated 400.3 MB synthetic set (80 files, 888,854 lines, one commander): clean import 9.10 s, database 654.9 MB (1.64x). Out-of-order case: 79 newer files imported first (11.6 s), then the oldest file added: that import (11,105 lines) took 8.59 s including the full rebuild of the commander's ledgers in game-time order, and ended with the same ledger counts as the clean import (visits 74,679, earnings 25,218, ships 12, scans 341,378). Synthetic data deleted afterwards.
