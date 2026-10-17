---
created_at: "2026-10-17T04:56:59.677256098+00:00"
id: "astromech-mahn"
evidence_type: "test"
commit: "1d75d0070657e1bd23f585f616d78c905a12a772"
result: "pass"
captured_at: "2026-10-17T04:56:59.677209772+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-8e8c"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-8e8c"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 1d75d00: cargo test -p astromech-exobiology --test predict (4 passed): 48 real bodies from a trimmed Spansh search (fixtures/spansh-bio-bodies.json) covering 70 species; every sampled species is among the predicted candidates at its catalogue value and inside its genus's predicted value range (Bacterium Nebulus and Tussock Ventusa: Spansh's codex value differs from the wiki's, asserted explicitly). Breaking the atmosphere gate fails 2 tests. Over 1000 Spansh bio bodies the prediction found 99.8% of reported species (misses: Bacterium Acies on a Neon world round a G star, Bacterium Vesicula on an ammonia HMC), averaging 7 candidates per body. --test system (3 passed): journal replay of FSDJump, star and planet Scan, FSSBodySignals, SAASignalsFound genus list, ScanOrganic: counts, predicted then known genera, value bounds, sample pins the species, jump resets. Data licences: values/colony ranges are wiki facts; conditions EDMC-ExplorerLite MIT (licence text in NOTICE.md); genus symbols per Elite Observatory (MIT)."
updated_at: "2026-10-17T04:56:59.677590352+00:00"
---

At 1d75d00: cargo test -p astromech-exobiology --test predict (4 passed): 48 real bodies from a trimmed Spansh search (fixtures/spansh-bio-bodies.json) covering 70 species; every sampled species is among the predicted candidates at its catalogue value and inside its genus's predicted value range (Bacterium Nebulus and Tussock Ventusa: Spansh's codex value differs from the wiki's, asserted explicitly). Breaking the atmosphere gate fails 2 tests. Over 1000 Spansh bio bodies the prediction found 99.8% of reported species (misses: Bacterium Acies on a Neon world round a G star, Bacterium Vesicula on an ammonia HMC), averaging 7 candidates per body. --test system (3 passed): journal replay of FSDJump, star and planet Scan, FSSBodySignals, SAASignalsFound genus list, ScanOrganic: counts, predicted then known genera, value bounds, sample pins the species, jump resets. Data licences: values/colony ranges are wiki facts; conditions EDMC-ExplorerLite MIT (licence text in NOTICE.md); genus symbols per Elite Observatory (MIT).
