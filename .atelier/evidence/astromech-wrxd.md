---
created_at: "2026-11-05T00:49:04.353991513+00:00"
id: "astromech-wrxd"
evidence_type: "validation"
commit: "2cc2655a98fd37fe0460e1901236f814d7f2fcdd"
result: "pass"
captured_at: "2026-11-05T00:49:04.353919630+00:00"
agent_identity: "orchestrator (scripts/voice-eval.sh on main dc7ea2a0)"
target:
  kind: "issue"
  id: "astromech-xi2i"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-xi2i"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Synthetic voice eval rerun on main dc7ea2a0, after the voice changes in PRs #190, #192, #198 and #201: 3200 clips, execute-or-ask 3122/3136, 0 wrong executes, 64/64 rejects, grammar-only 196/196. Every family is at 95% or above (lowest: status 98.1%, srv 98.4%). The Oct 22 baseline (docs/evidence/astromech-xi2i/report.md) was 3123/3136 with 0 wrong executes, so there is no regression. Per-clip transcripts under noise vary between runs (e.g. 'She ultimately' vs 'She alta.' for the same clip), so single-clip changes are noise. Commands below 95%: status.credits 93.8% ('rebuy' heard as 'Raban' or 'Goodbye.' under noise, the known xi2i case), status.ship 93.8% ('Current Scheerburg', known; 'Car and ship.' NoMatch), srv_handbrake.off 93.8% (was 87.5%). All are NoMatch or Ask, never a wrong action."
updated_at: "2026-11-05T00:49:04.355322473+00:00"
---

Synthetic voice eval rerun on main dc7ea2a0, after the voice changes in PRs #190, #192, #198 and #201: 3200 clips, execute-or-ask 3122/3136, 0 wrong executes, 64/64 rejects, grammar-only 196/196. Every family is at 95% or above (lowest: status 98.1%, srv 98.4%). The Oct 22 baseline (docs/evidence/astromech-xi2i/report.md) was 3123/3136 with 0 wrong executes, so there is no regression. Per-clip transcripts under noise vary between runs (e.g. 'She ultimately' vs 'She alta.' for the same clip), so single-clip changes are noise. Commands below 95%: status.credits 93.8% ('rebuy' heard as 'Raban' or 'Goodbye.' under noise, the known xi2i case), status.ship 93.8% ('Current Scheerburg', known; 'Car and ship.' NoMatch), srv_handbrake.off 93.8% (was 87.5%). All are NoMatch or Ask, never a wrong action.
