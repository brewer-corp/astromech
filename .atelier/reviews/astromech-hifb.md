---
schema: atelier.review
schema_version: 1
id: astromech-hifb
title: review on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:02.951290926Z
event:
  event: review
---

Independent review of task/astromech-dfvp (d1f77174..84b18f26, dfvp and 0k63). Reran in app/: pnpm lint, pnpm check (460 files, 0 errors 0 warnings), pnpm build, pnpm test (70 files, 465 passed); python3 scripts/check-guide.py (22 pages, 0 problems). Greps: --cell-opt gone from app/src and tokens.css (only the first-draft mockups define it locally), no 'until astromech-dfvp' left, var(--cell*) only on Input, Dropdown, Choice, Keybinds/BindDialog/onboarding binding slots, presets choice cells, Settings/Data sharing/Setup .value rows and the disabled primary button. Recomputed contrast: Orange Mid on hull 4.79, Alarm on hull 4.81, Orange Dim on hull 3.08 (state only), Orange Mid on Row Hover 4.59. Viewed the ten before/after PNGs and the materials mockup; rendered Materials (Iron selected), Engineers, Settings, Ships, Keybinds and Setup/Microphone on the dev build with scripts/ui: every editable value is a cell, no editable control lost its cell, read-only tiles, cards, bands and facts are hull or bare. Probed an empty material tile's focus (finding: ring hidden) and a barred engineer card (finding: pre-existing collapse). Limits: no real window on Windows; hover states judged from the cascade and computed styles, not a pointer; siblings dzwy and sllo untouched by the branch.
