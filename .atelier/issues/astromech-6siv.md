---
created_at: "2026-11-07T06:33:38.679516538+00:00"
id: "astromech-6siv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-8s6k"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Move the window's journal and local-file commands into the command layer"
updated_at: "2026-11-07T06:54:25.255405627+00:00"
---

## Goal

The window's journal and local-file commands are handlers in astromech-api, served by the core. Their state then has one owner, the core's command-layer state, and the shell keeps one-line forwarders so the window doesn't change.

## Done when

- [ ] A `State` in astromech-api owns, once per process: the keybinds `Api` (window.rs:102-139), `MaterialsCache`, `OdysseyCache`, `PowerplayCache`, `EngineeringCache`, `ExplorationCache` and `ColonisationCache` (window.rs:202-906).
- [ ] `State` reads its folders through a folder provider: a trait or closure in astromech-api that returns the journal, Bindings and game folders on each call, and the data folder, which is fixed at startup. The core implements it over `Core::cfg()`. `Config` lives in astromech-core (config.rs:134), which astromech-api must not depend on, and a startup snapshot would miss `set_folders`.
  - The keybinds `Api` is built once from its folders (window.rs:117-139). It is dropped and rebuilt when the Bindings or game folder it was built from changes.
  - Each cache is dropped when its journal folder changes.
  - Today the shell re-reads the config file on every call, so this keeps that behaviour.
- [ ] These commands are registered `window` entries backed by that state: `binds_*` (9), `materials_inventory`, `engineering_fleet`, `engineering_plan`, `engineering_stats`, `engineering_headlines`, `engineering_engineers`, `engineering_unlocks`, `odyssey_view`, `odyssey_shopping`, `powerplay_overview`, `colonisation_projects`, `exploration_system`, `exploration_unsold`, `logbook`, `session_summary`, `docked_here`, `trade_route_defaults`. Each shell command of the same name forwards to the core through `astromech_api::control` with `origin: "app_window"` and no read timeout. Its body and state are deleted from window.rs.
- [ ] Each moved handler returns a typed reply struct, not `json!`. `engineering_engineers` (window.rs:589) is one example. The JSON is unchanged. Existing domain types are returned as they are.
- [ ] Args keep the window's names. Tauri maps the camelCase keys to the forwarders' snake_case parameters, so the handlers' arg structs use snake_case.
- [ ] Tests in astromech-api over a temp journal folder:
  - one per cache shows it is rebuilt after a journal append and kept when nothing changed;
  - `binds_set_slot` and `binds_reset` from two callers in turn see each other's edits.
- [ ] A test in crates/astromech-core: `set_folders` over the control API, then `binds_state` reads the new Bindings folder and `materials_inventory` the new journal folder, with no restart. It replaces the window-side tests for astromech-c9em (`a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout`, app.rs:2006-2050), deleted in the next child. Those checked that the window process found the core's config file in every layout; with one reader of `Core::cfg` that question is gone.
- [ ] Checks as in the registry child; Windows CI's shell build and the Release onboarding smoke pass.

## Scope

astromech-api, app.rs startup, window.rs. These commands use no galaxy and no network. The galaxy and network group, `engineering_trip`, `outfitting_search` and `powerplay_targets` are the next child.

## Context

The `blocking` wrapper (window.rs:415-418) is no longer needed in the core: the control API's thread is the worker.
