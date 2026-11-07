---
schema: atelier.review
schema_version: 2
id: astromech-hzp5
title: finding on astromech-7rcn @387084e6
target: astromech-7rcn
identity: claude-reviewer-xil2
commit: 387084e6a623284f73e619d64e1f13c38016305c
created_at: 2026-11-07T05:27:45.068992229Z
event:
  event: finding
  recommendation: fix_now
---

medium: scripts/app-shots.sh:8 builds with a plain 'pnpm -s build' and then screenshots 'every app screen in the browser preview (example fixture, no Tauri)' from vite preview. Since this branch a plain build drops app/src/preview and picks the Tauri transport, so outside Tauri every call throws: serving a fresh 'pnpm build' and opening #/keybinds shows '! TypeError: Cannot read properties of undefined (reading invoke)' and 'Could not read the route' (checked with scripts/ui). All 13 shots would be error pages; CONTRIBUTING.md:96, docs/V1-DESIGN.md:866 and issues astromech-mf5e/j42m/5qh0 still point at the script. Fix: build with VITE_PREVIEW=1 in app-shots.sh (into a separate outDir so app/dist stays a release build), and check its shots once.
