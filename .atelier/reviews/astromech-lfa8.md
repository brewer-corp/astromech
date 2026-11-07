---
schema: atelier.review
schema_version: 2
id: astromech-lfa8
title: finding on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:20.271084424Z
event:
  event: finding
  recommendation: fix_now
  disposition:
    decision: fix_now
    identity: claude-orchestrator-kjfs
    commit: 1ece31b5ae4ad98c43c4e0ba5fa34922c08bf18e
    recorded_at: 2026-11-07T10:53:35.196394062Z
    resolution_commit: 1ece31b5ae4ad98c43c4e0ba5fa34922c08bf18e
    reason: Fixed in 1ece31b5
---

low: app/src/lib/odyssey.ts:61-63 and app/src/modules/odyssey/View.svelte:349-350 (96dv) The preview now fails odyssey_shopping instead of answering null, but loadShopping is still Promise<Shopping | null> with the doc '(else null)', and View keeps the null branch '{:else if !shopError}' that shows 'The shopping list is worked out in the app (the preview only has the sample picks).' That branch is now reached only while a list is loading or for stale picks, including in the real app, where the preview-only sentence is wrong. The report says the dead null branches were removed, but only Build.svelte and engineering View.svelte were. Fix: return Promise<Shopping>, fix the doc, and drop or reword the branch (e.g. nothing, or a loading line).
