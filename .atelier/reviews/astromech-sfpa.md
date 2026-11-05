---
schema: atelier.review
schema_version: 1
id: astromech-sfpa
title: finding on astromech-7c7c @33091c33
target: astromech-7c7c
identity: review-agent-7c7c-1
commit: 33091c3344d9a804fccc7a4721b844857be946d5
created_at: 2026-11-04T21:09:44.082403915Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/lib/ui/announce.ts:8-12 announce() empties the live region, awaits Svelte's tick() (a microtask DOM flush, not a render) and refills it, so the empty and refilled text land in the same task. Chromium/WebView2 builds its accessibility tree at the next lifecycle update and raises live-region events only when the serialized node changed, so a repeated 'Saved' or error most likely nets no change and is still not re-announced: the aznm goal. announce.test.ts only watches DOM mutations in jsdom, so it cannot fail for this. Fix: let a frame pass between empty and fill (a setTimeout of about 100 ms, or two requestAnimationFrame), or key the region's content so a new text node is inserted ({#key n}<span>{status}</span>{/key}); keep the test asserting the empty-then-text sequence. The comment 'once the page has updated' should then be true. Not reproduced with a screen reader here (no Windows/AT on this host); based on how Blink defers accessibility events.
