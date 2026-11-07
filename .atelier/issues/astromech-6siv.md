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
closed_at: "2026-11-07T07:07:04.159948954+00:00"
status: "done"
title: "Move the window's journal and local-file commands into the command layer"
updated_at: "2026-11-07T07:07:04.159948954+00:00"
---

## Goal

The window's journal and local-file commands are handlers in astromech-api, served by the core. Their state then has one owner, the core's command-layer state, and the shell keeps one-line forwarders so the window doesn't change.

## Done when

- [x] A `State` in astromech-api owns, once per process: the keybinds `Api` (window.rs:102-139), `MaterialsCache`, `OdysseyCache`, `PowerplayCache`, `EngineeringCache`, `ExplorationCache` and `ColonisationCache` (window.rs:202-906).
- [x] `State` reads its folders through a folder provider: a trait or closure in astromech-api that returns the journal, Bindings and game folders on each call, and the data folder, which is fixed at startup. The core implements it over `Core::cfg()`. `Config` lives in astromech-core (config.rs:134), which astromech-api must not depend on, and a startup snapshot would miss `set_folders`.
  - The keybinds `Api` is built once from its folders (window.rs:117-139). It is dropped and rebuilt when the Bindings or game folder it was built from changes.
  - Each cache is dropped when its journal folder changes.
  - Today the shell re-reads the config file on every call, so this keeps that behaviour.
- [x] These commands are registered `window` entries backed by that state: `binds_*` (9), `materials_inventory`, `engineering_fleet`, `engineering_plan`, `engineering_stats`, `engineering_headlines`, `engineering_engineers`, `engineering_unlocks`, `odyssey_view`, `odyssey_shopping`, `powerplay_overview`, `colonisation_projects`, `exploration_system`, `exploration_unsold`, `logbook`, `session_summary`, `docked_here`, `trade_route_defaults`. Each shell command of the same name forwards to the core through `astromech_api::control` with `origin: "app_window"` and no read timeout. Its body and state are deleted from window.rs.
- [x] Each moved handler returns a typed reply struct, not `json!`. `engineering_engineers` (window.rs:589) is one example. The JSON is unchanged. Existing domain types are returned as they are.
- [x] Args keep the window's names. Tauri maps the camelCase keys to the forwarders' snake_case parameters, so the handlers' arg structs use snake_case.
- [x] Tests in astromech-api over a temp journal folder:
  - one per cache shows it is rebuilt after a journal append and kept when nothing changed;
  - `binds_set_slot` and `binds_reset` from two callers in turn see each other's edits.
- [x] A test in crates/astromech-core: `set_folders` over the control API, then `binds_state` reads the new Bindings folder and `materials_inventory` the new journal folder, with no restart. It replaces the window-side tests for astromech-c9em (`a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout`, app.rs:2006-2050), deleted in the next child. Those checked that the window process found the core's config file in every layout; with one reader of `Core::cfg` that question is gone.
- [ ] Checks as in the registry child; Windows CI's shell build and the Release onboarding smoke pass.

## Scope

astromech-api, app.rs startup, window.rs. These commands use no galaxy and no network. The galaxy and network group, `engineering_trip`, `outfitting_search` and `powerplay_targets` are the next child.

## Context

The `blocking` wrapper (window.rs:415-418) is no longer needed in the core: the control API's thread is the worker.

## Close Reason

Implementer report (commit b325d559 on feature/astromech-bp23). Outcome: astromech_api::state::State owns, once per process, the keybinds Api and the materials, Odyssey, Powerplay, engineering (JournalState, moved from window.rs), exploration (view + unsold) and colonisation caches. Folder provider: State::new(data_dir, impl Fn() -> Folders {journal, bindings, game}) read on every call; the core implements it in app.rs (api_state/api_folders over Core::cfg(), through a Weak<Core> from Arc::new_cyclic, so no cycle). The Api is rebuilt when the (bindings, game) pair it was built from differs, or on binds_reset; each journal cache sits in a per-folder holder (ForDir) that is replaced when the journal folder differs. The unsold cache is keyed by history.sqlite in the fixed data folder, so it is never dropped. The data folder is fixed at startup, and device-overrides.json is carried over once at core startup (it used to be on every shell make_api). astromech_api::commands::register adds the 26 commands as `window` entries with snake_case arg structs (path, edit, src/name, name/contexts, backup, device/control/joy, plan, picks, query/cmdr). Registry::add_typed now takes a typed reply R: Serialize. Replies are domain types (State, View, EditResult, PathBuf, Overrides, Arc<Inventory>, Fleet, PlanResult, Compared, headlines map, Tree, OdysseyView, Shopping, Overview, Arc<Vec<ProjectView>>, SystemView, Option<UnsoldData>, Logbook, Option<Session>, DockedHere, Defaults) and one new struct, state::Engineers, for engineering_engineers. serde's rc feature is on in astromech-api so the Arc caches serialize. The shell has one-line async forwarders (forward() = call_with with origin app_window, no read timeout) and their bodies and caches are deleted. Deviation: window.rs keeps an EngineeringCache over api's JournalState::cache/get, and its PowerplayCache, because ndzx's commands still read them (search_origin, engineering_trip, powerplay_targets, route_plot*, search_filters, outfitting_search). astromech-ndzx should delete both. The shell dropped its now-unused materials, ship, colonisation, history, odyssey and exploration deps.
Proof: cargo test -p astromech-api 15/15, 8 new: materials, Odyssey and colonisation (also Cargo.json) kept by Arc identity and rebuilt after an append; engineering kept for an unwatched Music line and rebuilt for FSDJump; exploration parses counter; Powerplay follower picks up an appended Powerplay line and is unchanged otherwise; a cache dropped when the journal folder switches; two callers over the control API (app_window and local) in turn see each other's binds_set_slot, also across binds_reset. Core test the_window_reads_the_folders_set_folders_saved: set_folders over the control API, then binds_state shows the new preset and materials_inventory the new journal, with no restart. Breaking the binds rebuild and the ForDir folder check makes it fail. cargo test --workspace 1272 passed, 0 failed; cargo clippy --workspace --all-targets -D warnings clean; fmt, check-deps, check-guide and cargo deny clean. Shell window feature on Linux: cargo check and cargo clippy -p astromech-shell --features window --all-targets -D warnings pass, using fake .pc files for the gtk/webkit stack (PKG_CONFIG_PATH; check only, no link), placeholder externalBin files and app/dist (removed afterwards). A planted type error proved the check compiles window.rs. Windows-only cfg code isn't covered by this. Live run: the real core under a temp LOCALAPPDATA with the history fixtures' journals and a Bindings folder answered all 26 commands with origin app_window: values as expected, and bad args or a missing ship give the named errors.
Open items: (1) Windows CI only: the shell's window build and clippy on Windows, and the Release onboarding smoke. (2) The c9em window tests (a_pasted_bindings_folder_is_what_the_window_reads_in_every_layout) are still there, to be deleted in the next child as planned. (3) ndzx removes the shell's EngineeringCache, PowerplayCache, config() and journal_dir(). (4) The engineering_fleet and engineering_unlocks replies are a clone of the cached Fleet and Tree on each call; the shell serialized them by reference. (5) The forwarders take plan, picks and edit as Value, so their deserialization errors now come from the core ('bad args for …') and not from Tauri. Review outstanding.
