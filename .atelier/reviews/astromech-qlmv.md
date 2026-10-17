---
schema: atelier.review
schema_version: 1
id: astromech-qlmv
title: 'app/src-tauri/src/main.rs make_api: the .map_err continuation lost its indent (rustfmt would indent it). The shell is outside the workspace and already not rustfmt-clean on main (with_api etc.), and CI''s cargo fmt doesn''t cover it. Cosmetic.'
target: astromech-7w5d
identity: review-agent (opus, independent)
commit: 8a46cb6d7e122d233226efd204a31f8aee0f1735
created_at: 2026-10-17T07:46:22.602261390Z
event:
  event: finding
  recommendation: no_action
---

app/src-tauri/src/main.rs make_api: the .map_err continuation lost its indent (rustfmt would indent it). The shell is outside the workspace and already not rustfmt-clean on main (with_api etc.), and CI's cargo fmt doesn't cover it. Cosmetic.
