---
schema: atelier.review
schema_version: 2
id: astromech-7yw1
title: finding on astromech-yis2 @910de95e
target: astromech-yis2
identity: claude-reviewer-8qsx
commit: 910de95ecd1c657460e6903294cae3f7e4a762db
created_at: 2026-11-07T23:25:24.857888057Z
event:
  event: finding
  recommendation: fix_now
---

8qsx R2 (app/journeys/harness.ts:460, :473): every journey starts tracing, but only a failed one stops it. context.close() with tracing still running leaves an empty playwright-tracing-* folder in TMPDIR, one per journey per run: 40 after 20 suite runs, and an isolated repro leaves one on close and none after tracing.stop(). Residue on every passing local run. Fix: call 'await context?.tracing.stop()' (no path, so it is discarded) on the pass path.
