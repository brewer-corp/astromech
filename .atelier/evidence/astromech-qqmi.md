---
created_at: "2026-10-17T05:00:10.999870171+00:00"
id: "astromech-qqmi"
evidence_type: "test"
commit: "2a1f22ac13ac44849232e6410e61c1ee01006956"
result: "pass"
captured_at: "2026-10-17T05:00:10.999773109+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-7vpa"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-7vpa"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 2a1f22a: cargo test -p astromech-announce --test sampling (4 passed): replay of ScanOrganic Log/Sample/Analyse with Status.json positions on a 1000 km body; no cue at 495 m, 'Far enough for sample 2.' at 505 m for Stratum (500 m); once per sample; sample 3 cued only when 500 m from both earlier samples; Fumerola cues at 101 m, Electricae at 1001 m not 990 m; no cue without a known position; off via [callouts.sampling]. Loosening the range to 90% fails 2 tests. cargo test -p astromech-exobiology --test sampling (2 passed): counts, clearance distance/bearing, Sample without Log, jump reset. replay.rs catch-up test covers the rule (silent in catch-up, cues live); copy.rs checks its lines."
updated_at: "2026-10-17T05:00:11.000347453+00:00"
---

At 2a1f22a: cargo test -p astromech-announce --test sampling (4 passed): replay of ScanOrganic Log/Sample/Analyse with Status.json positions on a 1000 km body; no cue at 495 m, 'Far enough for sample 2.' at 505 m for Stratum (500 m); once per sample; sample 3 cued only when 500 m from both earlier samples; Fumerola cues at 101 m, Electricae at 1001 m not 990 m; no cue without a known position; off via [callouts.sampling]. Loosening the range to 90% fails 2 tests. cargo test -p astromech-exobiology --test sampling (2 passed): counts, clearance distance/bearing, Sample without Log, jump reset. replay.rs catch-up test covers the rule (silent in catch-up, cues live); copy.rs checks its lines.
