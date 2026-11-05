---
schema: atelier.review
schema_version: 1
id: astromech-ux31
title: finding on astromech-nxz9 @326fd18e
target: astromech-nxz9
identity: reviewer-fable-nxz9
commit: 326fd18eebaf7994530bf4843f8fa9d107921807
created_at: 2026-11-05T08:46:20.578951236Z
event:
  event: finding
  recommendation: defer
---

low: app/src/App.svelte:54 The first-load normalisation of a moved link still assigns location.hash (a pushState) while the in-window path uses history.replaceState, so opening the window on #/services leaves #/services in history under #/places/services/...: one Back press rewrites it in place and the page stays, a second leaves. Pre-existing pattern (activities -> controller) now reached by every old Galaxy bookmark; by inspection, not reproduced in the browser (an in-window old link followed from #/route goes Back to #/route correctly). Fix: history.replaceState(null, '', '#/' + start) there.
