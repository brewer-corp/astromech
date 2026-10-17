---
created_at: "2026-10-16T19:24:01.817300174+00:00"
id: "astromech-c9em"
issue_type: "task"
labels:
- "alpha"
- "size-s"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-lxma"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Onboarding follow-ups: shared config path, config error clears, fake-core contract test"
updated_at: "2026-10-17T17:01:42.146261371+00:00"
---

## Goal

Close three gaps review found in the onboarding slice (astromech-lxma):
- **R4:** in portable mode (or with `--config`) the window reads `%LOCALAPPDATA%\astromech\data\astromech.toml` while the core writes its own config path, so a Bindings folder pasted in onboarding is not picked up by the window. Take the config path from the core (`status.config`).
- **R6:** the startup `config_note` never clears after the user fixes the file, so `setup_needed` keeps reporting a config error until restart.
- **R7:** `app/src/lib/fakecore.ts` replies are hand-written, so wizard tests can stay green while the real core JSON changes (the no-mic message already differs). Add a contract test comparing the keys of the Rust `calibration::handle`, `setup::Found` and `mic_list` JSON with the fake.

## Done when

A pasted Bindings folder is used by the window in portable and `--config` layouts (test); fixing a broken config clears the config error without a restart (test); a contract test fails when the core and fake-core reply shapes diverge.
