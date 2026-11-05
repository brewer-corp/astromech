---
schema: atelier.review
schema_version: 1
id: astromech-4srf
title: approval on astromech-dfvp @c30c40ec
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: c30c40ecec014b51a4aff90c72c3a45a0fd4505a
created_at: 2026-11-05T17:22:50.409104894Z
event:
  event: approval
---

Re-review of ea0a8f8b (fixes for astromech-hdht, vnrp, va35) at head c30c40ec; the commits after it touch only .atelier records (git diff ea0a8f8b..HEAD excluding .atelier is empty). Read the 3-file diff: .cell.empty:hover (Row Hover) and .cell.empty:focus-visible (1 px orange, 2 px offset) added after the Seam outline rule; the access marks scoped to '.access > :is(.badge, .outline, .barred, .mail)' and per-mark rules; 'Speed (top / boost)'. Reran in app/: pnpm lint, pnpm check (460 files, 0 errors 0 warnings), pnpm build, pnpm test (465 passed). Re-probed on the dev build with scripts/ui: a focused empty material tile now matches :focus-visible with computed outline rgb(250,110,0) 1 px at 2 px offset, and the '.cell.empty:hover { var(--row-hover) }' rule is in the compiled stylesheet; a card given the 'barred' class stays 382x219 px on hull with no border while the access marks still measure 12x14 px with their mask. No fix_now finding remains; the deferred ones are filed on astromech-0132 and astromech-rpdc. Limits unchanged: no Windows window, hover judged from computed styles.
