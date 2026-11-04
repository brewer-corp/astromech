---
schema: atelier.review
schema_version: 1
id: astromech-zp7i
title: finding on astromech-7c7c @1280be47
target: astromech-7c7c
identity: review-agent-7c7c-1
commit: 1280be475222359ddb3202f9e06ed83636882ed4
created_at: 2026-11-04T21:13:04.052583559Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/lib/ui/announce.ts:6,14-17 'latest' is one module-wide counter shared by every live region, so a newer announce() in any region cancels a pending one in another. Settings mounts Callouts and ProspectorThresholds together (modules/settings/View.svelte:207,211); with the core down both loads fail at once, and the first region is emptied and never refilled, so its error disappears (it showed before this branch). Reproduced with the function's logic in node: two calls 0 ms apart on two setters leave the first ''. The 'a newer call wins' test (lib/ui/announce.test.ts) uses one setter, so it cannot catch this. Fix: keep the counter per region, e.g. export announcer(set) returning (text) => Promise<void> with its own counter, made once per component; add a test with two regions announcing within 100 ms that both end filled.
