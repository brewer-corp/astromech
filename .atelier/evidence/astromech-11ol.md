---
created_at: "2026-10-17T08:31:43.690810645+00:00"
id: "astromech-11ol"
evidence_type: "test"
commit: "32367962ba84d93db2b222e4d6ecb62c56d60633"
result: "pass"
captured_at: "2026-10-17T08:31:43.690770881+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-w064"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-w064"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At 3236796: every_system_mapped_total_matches_edsm (14 systems, EDSM estimatedValueMapped, exact) and a_mapped_small_icy_body_is_floored_after_mapping (1 EM icy mapped = 1566 EDSM terms, 2121 with Odyssey, matching EDDI) failed before the fix (Ross 154 873228 vs 871509; 1667 vs 1566) and pass after; /tmp/landq/gates.sh passed in full (fmt, clippy, cargo test --workspace, check-deps, app lint/check/build/test)."
updated_at: "2026-10-17T08:31:43.691082082+00:00"
---

At 3236796: every_system_mapped_total_matches_edsm (14 systems, EDSM estimatedValueMapped, exact) and a_mapped_small_icy_body_is_floored_after_mapping (1 EM icy mapped = 1566 EDSM terms, 2121 with Odyssey, matching EDDI) failed before the fix (Ross 154 873228 vs 871509; 1667 vs 1566) and pass after; /tmp/landq/gates.sh passed in full (fmt, clippy, cargo test --workspace, check-deps, app lint/check/build/test).
