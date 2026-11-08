---
schema: atelier.review
schema_version: 2
id: astromech-npwm
title: approval on astromech-yis2 @6800a72e
target: astromech-yis2
identity: claude-reviewer-cwtv
commit: 6800a72eb904af40d3efca4633e2afd29316622c
created_at: 2026-11-08T00:05:12.608254361Z
event:
  event: approval
---

Approve astromech-yis2 at 6800a72e: harness 8qsx (approved xr1d), cwtv and 11vu. cwtv: bc60ea2f fixes blcv (line 1 now names worlds/journeys/plot/net.json, the 7 Nov --record-net recording); yzpn deferred to astromech-lltb, 0dmh no_action. 11vu: voice.journey.ts and worlds/journeys/kill-switch (worlds/kill-switch's header, game, bindings and controls, with only the kill and 1000 ms re-arm steps) meet Done when lines 1 and 2. The kill waits on the said line, checks the header is unchanged before the poll, then runFor(2000); the re-arm waits on the page first. The voice tester clicks Run live and waits for 'Pressed; the game confirmed it in N ms.', and asserts exactly Key_L down then up. Fresh: 10/10 runs of node --test 'journeys/*.journey.ts' (7 tests each) on Node 26.13. Mutations from scratch worlds: a 999 ms hold times out at the header wait (about 17 s); the Key_L reaction removed times out at the confirmation wait (about 16 s), with no quit or temp-world error, so the core quits cleanly while its verify is still waiting. The manual-clock verify that never ends is no CI hang: the page guard is 15 s, the journey guard 90 s and the CI step 5 min, and the core still exits 0. cargo test committed_worlds (2) and world_voice (4), pnpm lint, tsc -p journeys and atelier check astromech-yis2 pass. yis2 Done when line 1 holds: all five journeys are present and pass. Only line 2, the CI Linux job, is open. Not run: pnpm check and test, Node 22, cargo test --workspace, CI.
