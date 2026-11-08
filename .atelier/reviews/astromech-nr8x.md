---
schema: atelier.review
schema_version: 2
id: astromech-nr8x
title: review on astromech-yis2 @2c03629d
target: astromech-yis2
identity: claude-reviewer-cwtv
commit: 2c03629db80aa723b18791e44f39b941a32fa4be
created_at: 2026-11-07T23:54:56.531535452Z
event:
  event: review
---

Independent review of astromech-cwtv (commits 7d53fcec, d1535ac8, 2c03629d) at 2c03629d. review request refuses because Done when line 3 is unticked (CI half), so I reviewed from issue show --full plus the diff. Fresh proof: cargo build -p astromech-core, then cargo test committed_worlds (2) and world_steps (8) pass. node --test 'journeys/*.journey.ts' passed 10/10 on Node 26.13 (about 4 s a run), with HOME in scratch. pnpm lint and tsc -p journeys are clean. Mutations, run from scratch copies of the worlds and journey: (m1) an off-route jump (Alpha Centauri on the plot route, Luhman 16 on the next-stop route) fails both journeys at the clipboard world_wait (30 s guard). (m2) the same jumps with the core waits and the pre-poll asserts removed fail at the page wait for the new next stop (15 s), so the page alone also keeps the old stop. (m0) the page-only variant on the real worlds passes. The recordings hold Spansh public answers only (the system-names query, the route POST with 202, seven polls with one real job id, and bodies searches): 38.7 KB and 17.6 KB, no commander, FID, key or discoverer fields. The game folders are byte copies of worlds/preview/sol-nov and worlds/jump, as the harness world already does. EDSM is refused: the core logs 'Cannot reach EDSM' and the arrival briefing leaves out the known-bodies count, which neither journey asserts. Not run: pnpm check, pnpm test (563), Node 22, CI. Finding: one fix_now (line 1's text), one defer, one no_action. No approval until the fix_now is fixed.
