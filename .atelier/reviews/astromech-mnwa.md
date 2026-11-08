---
schema: atelier.review
schema_version: 2
id: astromech-mnwa
title: review on astromech-yis2 @910de95e
target: astromech-yis2
identity: claude-reviewer-8qsx
commit: 910de95ecd1c657460e6903294cae3f7e4a762db
created_at: 2026-11-07T23:25:32.503629589Z
event:
  event: review
---

8qsx review @910de95e (commits 1869aced..910de95e): Done when lines 1-5, 7 and 8 hold; line 6 (CI timing on the runner) is open until the PR's CI run. Not approved: R1 and R2 are fix_now. Proof rerun fresh: the suite 10/10 on Node 26.13 and 10/10 on Node 22.23, about 2.4 s each, with no core, home or temp world left; mutations rerun in a scratch copy (pauseAt removed fails 'a status call before the poll was due'; the first answer 200 fails the wait for the 503 alert), each leaving its trace, screenshot and core logs; pnpm check, lint, test (553), tsc -p journeys, build plus check-dist, bridge-check and committed_worlds all pass; no release.yml trigger path touched. The 503 comes from net.json's in_turn, the existing replay mechanism (worlds/preview/sol), with worlds/network's 503 body. Limits: not run in CI, no Windows run.
