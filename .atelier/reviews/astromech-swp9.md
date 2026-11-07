---
schema: atelier.review
schema_version: 1
id: astromech-swp9
title: finding on astromech-jdks @f8fc601a
target: astromech-jdks
identity: claude-reviewer-jdks
commit: f8fc601a97c50c538f8fdc4a44d42a163063544d
created_at: 2026-11-07T00:47:13.266122363Z
event:
  event: finding
  recommendation: defer
---

low (pre-existing, but the biggest lever on this issue's goal): .github/workflows/release.yml:84 The rust-cache key 'release' is only ever saved by PR runs (scoped to refs/pull/N) and tag runs; GitHub lets a tag run restore only its own ref's or the default branch's caches, and release.yml never runs on main, so every tag build compiles cold, as in the 17-minute baseline. Fix: save the release cache from main (e.g. run the build steps on push to main with the same key, or a scheduled warm-up), then measure the tag build again.
