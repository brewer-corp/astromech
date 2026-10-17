---
schema: atelier.review
schema_version: 1
id: astromech-9gjv
title: 'Pre-existing, same class as R3: the shell still opens galaxy.db (main.rs:424), history.sqlite (446, 462) and the outfitting book (480, 492), and reads astromech.toml for keybinds (129, 201), from app_dir(), so in portable mode or with a data_dir override these are not the core''s files. docs/ARCHITECTURE.md''s ''the core, which opens the same galaxy.db'' holds only in the default layout. Follow-up: route these through the same data-dir/config-path rule.'
target: astromech-n1ij
identity: review-agent (opus, independent)
commit: df2c4fb438a4f201a3a05fc02770c5536db43657
created_at: 2026-10-17T05:05:44.061350257Z
event:
  event: finding
  recommendation: defer
---

Pre-existing, same class as R3: the shell still opens galaxy.db (main.rs:424), history.sqlite (446, 462) and the outfitting book (480, 492), and reads astromech.toml for keybinds (129, 201), from app_dir(), so in portable mode or with a data_dir override these are not the core's files. docs/ARCHITECTURE.md's 'the core, which opens the same galaxy.db' holds only in the default layout. Follow-up: route these through the same data-dir/config-path rule.
