---
schema: atelier.review
schema_version: 2
id: astromech-35vx
title: finding on astromech-yis2 @910de95e
target: astromech-yis2
identity: claude-reviewer-8qsx
commit: 910de95ecd1c657460e6903294cae3f7e4a762db
created_at: 2026-11-07T23:25:25.597032311Z
event:
  event: finding
  recommendation: no_action
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 0b53d5ae43689e501c7a8670d78cc369517ebe1d
    recorded_at: 2026-11-07T23:37:43.105478655Z
    resolution_commit: f093bf71fc24cc71c842e73a8c4455ab1a8bee5a
    reason: Taken now; fixed in f093bf71
---

8qsx R4 (.github/workflows/ci.yml journeys step, Done when line 1): confirmed 'node --test' on a glob matching nothing exits 0 on Node 22.23 and 26.13. The CI 'ls' makes a run of zero fail in CI, which is what journeys-in-ci needs, and harness.journey.ts covers journeys renamed away. A local run from the wrong folder still passes as zero, so line 1's 'a run of zero can't pass' holds in CI only. Optional: one glob in place of two, 'files=$(ls journeys/*.journey.ts)' then 'node --test $files'. A runner script would add a file for no more protection in CI.
