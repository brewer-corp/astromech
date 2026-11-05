---
schema: atelier.review
schema_version: 1
id: astromech-hdht
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:01.520225689Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/modules/materials/View.svelte:223-224 An empty tile has no visible keyboard focus and no hover: '.cell.empty { background: var(--bg); outline: 1px solid var(--line); outline-offset: -1px }' (0,2,0) beats both the global ':focus-visible' orange ring (0,1,0) and the earlier '.cell:hover { background-color: var(--row-hover) }' (same specificity, declared first). Proof on the dev build: focusing an empty tile gives matches(':focus-visible') true with computed outline rgb(58,39,22) at -1 px (Seam), i.e. the ring is invisible; on main empty tiles had no outline at rest so the ring showed, and every tile hovered to an Orange Dim outline. Most tiles in a grid are empty, so a keyboard user tabbing the Materials grid sees nothing move. Fix: add '.cell.empty:focus-visible { outline-color: var(--accent); outline-offset: 2px; }' (DESIGN.md Focus) and '.cell.empty:hover { background: var(--row-hover); }' (or keep Void and lift the outline to Orange Mid).
