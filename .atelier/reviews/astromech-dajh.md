---
schema: atelier.review
schema_version: 2
id: astromech-dajh
title: finding on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:20.666193770Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 1ece31b5ae4ad98c43c4e0ba5fa34922c08bf18e
    recorded_at: 2026-11-07T10:53:35.600016953Z
    resolution_commit: 1ece31b5ae4ad98c43c4e0ba5fa34922c08bf18e
    reason: Fixed in 1ece31b5
---

low: docs/ARCHITECTURE.md:1745 (96dv, Types) 'so a misspelt command, wrong args or a fixture that drifted from the core fails pnpm check' overstates the check: recorded<T>() widens string literals and tuples and does not see a JSON import's extra fields (recorded.ts says so), and tests' answer()/previewAnswer() replies are unchecked (report open item 2). Verified: a fixture missing NearbyHit.exact or with range:'far' fails; a non-fresh object with an extra field passes; a wrong enum value passes. Fix: say 'a fixture missing a field the core sends or holding one of the wrong type', as CONTRIBUTING does.
