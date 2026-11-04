---
created_at: "2026-10-22T21:50:43.743780411+00:00"
id: "astromech-f2k1"
issue_type: "bug"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: bug/astromech-f2k1
priority: "P3"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-04T21:48:57.250676713+00:00"
status: "done"
title: "Window's config path at startup: --config runs and a not-yet-written config file"
updated_at: "2026-11-04T21:48:57.250676713+00:00"
---

## Goal

The app window uses the same config file, data folder and Bindings folder as the core it talks to, from the moment it opens.

## Done when

Tests cover both gaps: (1) the window sets up its remote-data cache from the config the running tray core published, not from a stale config.path left by an earlier --config run whose file still exists; (2) a tray core started with --config pointing at a file that doesn't exist yet and the window agree on the folders until the core writes that file.

## Context

Review findings R2 (astromech-nhfz) and R3 (astromech-5ij6) on feature/astromech-0k88: crates/astromech-store/src/paths.rs:92 (fallback to the default file), app/src-tauri/src/main.rs:1229 (remote-data cache set up before the tray core publishes), crates/astromech-core/src/app.rs:1238.

## Close Reason

Implemented on bug/astromech-f2k1 @c4cce78b. Paths::publish_config now returns a Published guard; the tray core (app::run via choose_config) holds it until exit, dropped before the instance lock, and its Drop removes config.path only if it still names this core's file. Paths::config_in_use takes a published path as-is (no is_file fallback), else the default file. So (1) once a --config core quits, a window opened later sets up its remote-data cache in the default data folder, which the core it starts uses, though the old file still exists; (2) a --config file not written yet gives the window defaults, as the core has, until the core writes it. New astromech_store::window::data_dir is what the shell's data_dir() now calls (one-line change in app/src-tauri/src/main.rs). Tests: core app::tests::the_window_opens_its_cache_in_the_running_cores_data_folder and a_config_not_written_yet_gives_the_window_the_cores_folders; store paths::tests::a_published_config_that_does_not_exist_yet_is_still_the_one_in_use, dropping_a_publication_leaves_a_newer_one, and the revised the_window_reads_the_config_the_core_published. All failed with the fix reverted (evidence astromech-2zbg). Deviation: this replaces astromech-6a7c's fallback to the default file when the published file is missing (the R2 fix). Not compiled here: app/src-tauri (no webkit2gtk); Windows CI builds it. Open items: a core that crashes or is killed leaves config.path behind, as it leaves core.json, so a window opened before a new tray core starts still follows the dead core's --config (the watchdog's restart runs without --config). Independent review outstanding.
