---
created_at: "2026-11-07T06:33:38.678430735+00:00"
id: "astromech-ndzx"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-6siv"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Move the star-map, lookup, plotting and neutron commands into the command layer"
updated_at: "2026-11-07T06:33:38.678430735+00:00"
---

## Goal

The rest of the window's commands move into the command layer: star-map, lookup and route-plotting commands, the neutron star file, `open_external` and `shell_info`. The core process then has one remote-data client and one star map, shared by the window's commands and the session's voice lookups.

## Done when

- [ ] `State` also owns:
  - the `astromech_data::Client`;
  - the star map in a holder that opens the `Galaxy` lazily, on first use (window.rs:388-408), and retries a failed open on the next call;
  - the neutron `Kept` (window.rs:1020);
  - the two one-at-a-time flags (`ROUTE_BUSY` and `PLOT_BUSY`, window.rs:910-928) as fields, not statics.

  The data folder is read once at startup and fixed for the life of the process. `set_folders` changes only the Bindings and journal folders (crates/astromech-core/src/setup.rs:101-111), so none of these is reopened on a folder change.
- [ ] The session asks the holder for the star map each time a lookup needs it, instead of opening its own eagerly at session start (crates/astromech-core/src/session.rs:127-139, :180). It shares the one client, so astromech-data's rule of one client per process holds (crates/astromech-data/src/lib.rs:14-15). A voice lookup with no star map yet opens it. A failed open is the "site lookups are off" answer it gives now. crates/astromech-core/tests/trade_voice.rs and the other voice lookup tests pass.
- [ ] Registered `window` entries: `galaxy_nearby`, `galaxy_sites`, `galaxy_systems`, `stations_nearby`, `colony_candidates`, `trade_search`, `trade_anywhere`, `trade_cargo`, `trade_route`, `mining_hotspots`, `market_compare`, `colonisation_buy`, `search_filters`, `set_search_filters`, `powerplay_targets`, `engineering_trip`, `outfitting_search`, `route_plot_ship`, `route_plot`, `neutron_file`, `neutron_download`, `neutron_check`, `open_external`, `shell_info`. The shell keeps forwarders for them with no read timeout. window.rs keeps no command body and no `.manage` state.
- [ ] Each moved handler returns a typed reply struct of today's JSON shape instead of `json!`. The ones that build `json!` now are `outfitting_search`, `trade_cargo`, `trade_route`, `route_plot_ship`, `route_plot`, `neutron_file`'s status, `neutron_check` and `colonisation_buy` (window.rs:519, 536, 791, 950, 963-967, 1010, 1026-1027, 1090, 1110).
- [ ] `route_plot` hands its route to the follower by calling `route_set` through the registry, not over TCP (window.rs:1002). A test plots from recorded Spansh answers and sees the route in `route`'s view.
- [ ] `neutron_download` reports progress as progress lines, and the shell forwarder passes them to the page's `Channel`. A test over a local HTTP server checks that progress arrives before the reply.
- [ ] The star map preload, the bundled `starmap.db` (a Tauri resource at window.rs:401), is found by the core in its own exe's folder, where NSIS installs both.
  - The resolver is a function of the exe path, with a unit test: a `starmap.db` next to the exe is found, and none gives no preload, as the session opens today.
  - When the holder opens the star map it logs one line naming the database and the preload path, or "no preload". `Galaxy::open` (crates/astromech-galaxy/src/lib.rs:153-158) logs nothing now.
  - The installed-app check of that line is in the proxy child's onboarding smoke.
- [ ] `open_external` keeps its URL and folder rule (window.rs:1279-1298) behind a small opener the tests replace. A test shows a disallowed target gives "not allowed".
- [ ] A test in crates/astromech-core: a core built from a config whose `data_dir` is not the default serves `search_filters` and `logbook` from that folder (a saved filter and a `history.sqlite` placed there are what come back). It covers what `the_window_opens_its_cache_in_the_running_cores_data_folder` and `a_config_not_written_yet_gives_the_window_the_cores_folders` (app.rs:2068, :2092) guarded. Those tests, the astromech-c9em test from astromech-6siv and `astromech_store::window` are then deleted, since nothing uses them.
- [ ] Checks as before; Windows CI shell build and Release onboarding smoke pass.

## Scope

astromech-api, crates/astromech-core (session.rs, app.rs), astromech-galaxy only if the holder needs a hook, window.rs, astromech-store. The `input` listener (window.rs:1145-1195), the core start, the watchdog and single instance stay in the shell (astromech-y6d6).
