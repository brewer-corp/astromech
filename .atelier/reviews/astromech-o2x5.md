---
schema: atelier.review
schema_version: 2
id: astromech-o2x5
title: finding on astromech-yis2 @910de95e
target: astromech-yis2
identity: claude-reviewer-8qsx
commit: 910de95ecd1c657460e6903294cae3f7e4a762db
created_at: 2026-11-07T23:25:24.481081145Z
event:
  event: finding
  recommendation: fix_now
---

8qsx R1 (app/journeys/harness.ts:541-547, CONTRIBUTING.md Journeys 'the dev server has no dependency pre-bundling', 8qsx's dev-server decision note): the claim is false. vite-plugin-svelte adds svelte's entries to optimizeDeps.include, so the journeys' server pre-bundles svelte into app/node_modules/.vite/deps at start (a probe with the harness's options and its own cacheDir: main.ts imports <cacheDir>/deps/svelte.js, and Vite logs 'Forced re-optimization of dependencies'), and every journey run rewrites the shared node_modules/.vite/_svelte_metadata.json, so side-by-side journeys do share and write one cache. Behaviour is fine: noDiscovery still stops a mid-journey re-optimise, and 30 cold three-way concurrent starts on one shared cacheDir loaded once with no failed request. But the docs and the decision that settles the second Unknown describe a mechanism that isn't there. Fix: say svelte's entries are pre-bundled at server start with no discovery, so nothing re-optimises mid-journey, and that journeys share node_modules/.vite (or give each server a cacheDir in its temp folder if a cache of its own is wanted).
