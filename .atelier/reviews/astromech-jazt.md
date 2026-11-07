---
schema: atelier.review
schema_version: 2
id: astromech-jazt
title: finding on astromech-kjfs @5d19a292
target: astromech-kjfs
identity: claude-reviewer-kjfs
commit: 5d19a29220e8c49fd861ed2ad6541ef56f24eeb8
created_at: 2026-11-07T04:54:25.333651910Z
event:
  event: finding
  recommendation: fix_now
---

high: astromech-xil2 Done when (main.ts dynamic import vs the dist check) A dynamic import of app/src/preview/transport.ts outside Tauri still emits a chunk into app/dist on pnpm build, so the issue's own build check (no fixture string such as 'Arietis Sector' in app/dist) fails by design. scripts/preview-publish builds with the same vite build and needs the fake. Fix: gate the import on a build-time constant (import.meta.env.DEV, or a VITE_PREVIEW flag that preview-publish sets alongside VITE_PREVIEW_QUERY) so Vite drops it from the release build, and say so in the line.
