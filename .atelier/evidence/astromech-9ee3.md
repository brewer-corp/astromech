---
created_at: "2026-10-17T05:00:10.139363963+00:00"
id: "astromech-9ee3"
evidence_type: "test"
commit: "ecb37abcc6dd4453aebe2ea417e20b7341d4390c"
result: "pass"
captured_at: "2026-10-17T05:00:10.139210225+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-gpee"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-gpee"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At ecb37ab: cargo test -p astromech-announce --test discovery passes (4 tests): a replayed ELW scan gets exactly one line ('Earth-like world. 4.4 million mapped.'), a repeat scan of it, FSSDiscoveryScan and FSSAllBodiesFound get none; terraformable HMC, water and ammonia worlds get lines, icy/gas giant/plain HMC/star don't; nav beacon scans are silent; [callouts.discovery] enabled=false turns it off. replay.rs catch-up test (14 rules, silent in catch-up, speaks live) and astromech-core tests/copy.rs (every planet class x terraformable x mass x first-discovery within 8 words) pass."
updated_at: "2026-10-17T05:00:10.140823141+00:00"
---

At ecb37ab: cargo test -p astromech-announce --test discovery passes (4 tests): a replayed ELW scan gets exactly one line ('Earth-like world. 4.4 million mapped.'), a repeat scan of it, FSSDiscoveryScan and FSSAllBodiesFound get none; terraformable HMC, water and ammonia worlds get lines, icy/gas giant/plain HMC/star don't; nav beacon scans are silent; [callouts.discovery] enabled=false turns it off. replay.rs catch-up test (14 rules, silent in catch-up, speaks live) and astromech-core tests/copy.rs (every planet class x terraformable x mass x first-discovery within 8 words) pass.
