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
closed_at: "2026-11-07T07:32:57.622869685+00:00"
status: "done"
title: "Move the star-map, lookup, plotting and neutron commands into the command layer"
updated_at: "2026-11-07T07:32:57.622869685+00:00"
---

## Goal

The rest of the window's commands move into the command layer: star-map, lookup and route-plotting commands, the neutron star file, `open_external` and `shell_info`. The core process then has one remote-data client and one star map, shared by the window's commands and the session's voice lookups.

## Done when

- [x] `State` also owns:
  - the `astromech_data::Client`;
  - the star map in a holder that opens the `Galaxy` lazily, on first use (window.rs:388-408), and retries a failed open on the next call;
  - the neutron `Kept` (window.rs:1020);
  - the two one-at-a-time flags (`ROUTE_BUSY` and `PLOT_BUSY`, window.rs:910-928) as fields, not statics.

  The data folder is read once at startup and fixed for the life of the process. `set_folders` changes only the Bindings and journal folders (crates/astromech-core/src/setup.rs:101-111), so none of these is reopened on a folder change.
- [x] The session asks the holder for the star map each time a lookup needs it, instead of opening its own eagerly at session start (crates/astromech-core/src/session.rs:127-139, :180). It shares the one client, so astromech-data's rule of one client per process holds (crates/astromech-data/src/lib.rs:14-15). A voice lookup with no star map yet opens it. A failed open is the "site lookups are off" answer it gives now. crates/astromech-core/tests/trade_voice.rs and the other voice lookup tests pass.
- [x] Registered `window` entries: `galaxy_nearby`, `galaxy_sites`, `galaxy_systems`, `stations_nearby`, `colony_candidates`, `trade_search`, `trade_anywhere`, `trade_cargo`, `trade_route`, `mining_hotspots`, `market_compare`, `colonisation_buy`, `search_filters`, `set_search_filters`, `powerplay_targets`, `engineering_trip`, `outfitting_search`, `route_plot_ship`, `route_plot`, `neutron_file`, `neutron_download`, `neutron_check`, `open_external`, `shell_info`. The shell keeps forwarders for them with no read timeout. window.rs keeps no command body and no `.manage` state.
- [x] Each moved handler returns a typed reply struct of today's JSON shape instead of `json!`. The ones that build `json!` now are `outfitting_search`, `trade_cargo`, `trade_route`, `route_plot_ship`, `route_plot`, `neutron_file`'s status, `neutron_check` and `colonisation_buy` (window.rs:519, 536, 791, 950, 963-967, 1010, 1026-1027, 1090, 1110).
- [x] `route_plot` hands its route to the follower by calling `route_set` through the registry, not over TCP (window.rs:1002). A test plots from recorded Spansh answers and sees the route in `route`'s view.
- [x] `neutron_download` reports progress as progress lines, and the shell forwarder passes them to the page's `Channel`. A test over a local HTTP server checks that progress arrives before the reply.
- [x] The star map preload, the bundled `starmap.db` (a Tauri resource at window.rs:401), is found by the core in its own exe's folder, where NSIS installs both.
  - The resolver is a function of the exe path, with a unit test: a `starmap.db` next to the exe is found, and none gives no preload, as the session opens today.
  - When the holder opens the star map it logs one line naming the database and the preload path, or "no preload". `Galaxy::open` (crates/astromech-galaxy/src/lib.rs:153-158) logs nothing now.
  - The installed-app check of that line is in the proxy child's onboarding smoke.
- [x] `open_external` keeps its URL and folder rule (window.rs:1279-1298) behind a small opener the tests replace. A test shows a disallowed target gives "not allowed".
- [x] A test in crates/astromech-core: a core built from a config whose `data_dir` is not the default serves `search_filters` and `logbook` from that folder (a saved filter and a `history.sqlite` placed there are what come back). It covers what `the_window_opens_its_cache_in_the_running_cores_data_folder` and `a_config_not_written_yet_gives_the_window_the_cores_folders` (app.rs:2068, :2092) guarded. Those tests, the astromech-c9em test from astromech-6siv and `astromech_store::window` are then deleted, since nothing uses them.
- [x] Checks as before; Windows CI shell build and Release onboarding smoke pass.

## Scope

astromech-api, crates/astromech-core (session.rs, app.rs), astromech-galaxy only if the holder needs a hook, window.rs, astromech-store. The `input` listener (window.rs:1145-1195), the core start, the watchdog and single instance stay in the shell (astromech-y6d6).

## Notes

- 2026-11-07 [note] From finding astromech-9h8f (6siv review): the folders for moved commands now come from the core's in-memory config, while the shell's remaining config()/journal_dir() still re-read astromech.toml on each call, so after a hand edit the two halves can disagree. When ndzx removes the shell's config()/journal_dir(), make sure no reader of astromech.toml is left in the shell; one owner (the core's config) is the end state.

## Close Reason

Implementer report (commit 93356b8a on feature/astromech-bp23). Outcome: astromech_api::lookups registers the 24 commands (galaxy_nearby, galaxy_sites, galaxy_systems, stations_nearby, colony_candidates, trade_search, trade_anywhere, trade_cargo, trade_route, mining_hotspots, market_compare, colonisation_buy, search_filters, set_search_filters, powerplay_targets, engineering_trip, outfitting_search, route_plot_ship, route_plot, neutron_file, neutron_download, neutron_check, open_external, shell_info) as window entries with snake_case arg structs; the eight json! replies are structs (Outfitting, CargoSale, TradeRoute, PlotShip, RoutePlotted, NeutronFile, NeutronCheck, ShoppingPlan) of the same shape. State::new(data_dir, remote, preload, folders) now owns the one astromech_data::Client, a GalaxyHolder (galaxy.db opened on first use, a failed open retried next call, one log line 'star map: opened <db>, preload <p>' or ', no preload', or why it failed), the neutron Kept, route_busy/plot_busy AtomicBool fields (Busy guard), the plot polling, open_external's folders (data dir plus app and log dir from the core) and its opener. preload_beside(exe) finds starmap.db next to the exe. The handler's second argument is now a Call (was Progress): call.progress(v) and call.run(cmd, args), which dispatches through the same registry with the caller's origin; route_plot uses it for route_set. Session::start takes &astromech_api::State and uses its client and holder (Runner.galaxy: Option<Galaxy> became Runner.stars: Arc<GalaxyHolder> with Runner::galaxy(); a failed open gives NO_STAR_MAP as before); session::remote_data is gone; the console mode builds a State over its fixed config. The shell: one-line forwarders taking Value for domain args, neutron_download forwards progress lines to its Channel<Value>, no .manage, no config()/data_dir()/journal_dir(), EngineeringCache and PowerplayCache deleted, deps on journal/engineering/data/galaxy/market/station/powerplay/anyhow/serde dropped. No reader of astromech.toml is left in the shell (Paths::current only for core.json's app dir and the log dir). Deleted: astromech_store::window, the c9em test and the two f2k1 tests in app.rs; the store paths test that used WindowConfig keeps only its config_in_use assertion. ARCHITECTURE.md (crate tree, §5.8 client, Paths) updated. Proof: cargo test -p astromech-api 23/23, new: neutron_download over the control API from a local HTTP server (progress line first, reply last, file loaded, neutron_check newer true then false), open_external (a folder outside, a ../ escape, an exe name and an unknown URL are 'not allowed' with nothing opened; data subfolder, log folder and a GitHub link go to the test opener), preload_beside, holder retries after a failed open, holder log line with and without preload, Call::run refuses a local command for app_window. Mutations seen failing: progress not sent, the not-allowed check removed. cargo test -p astromech-core all pass, new: the_window_reads_the_data_folder_the_config_names (non-default data_dir: saved filter and history.sqlite there come back via search_filters/logbook over the control API), a_plotted_route_is_the_one_the_follower_shows (recorded Spansh exact plot, route_plot over the control API, route's view equals the plot's), site_lookup a_star_map_that_would_not_open_is_tried_again_on_the_next_lookup (first voice lookup NO_STAR_MAP, next opens it and answers); trade_voice's shared-client test now builds the State the session is given; all voice tests pass. cargo test --workspace 1280 passed 0 failed; cargo clippy --workspace --all-targets -D warnings clean; cargo fmt --check clean; check-deps OK; check-guide OK; cargo deny ok. Shell window feature on Linux: cargo clippy -p astromech-shell --features window --all-targets -D warnings clean with fake .pc files (PKG_CONFIG_PATH/LIBDIR under /tmp/claude-1000/ndzx-work), placeholder externalBin files and app/dist (removed); a planted type error proved window.rs is compiled. Live run: the real astromech-core --background under a temp LOCALAPPDATA with the history fixtures' journals, a copy of the exe beside an empty starmap.db: as app_window shell_info, route_plot_ship, search_filters/set_search_filters/search_filters (33 saved and read back), neutron_file, outfitting_search, market_compare (named error), engineering_trip, trade_cargo, open_external (/etc not allowed, logs folder ok), galaxy_systems, galaxy_sites, stations_nearby and trade_search (live Spansh/Ardent answers), powerplay_targets (not pledged), neutron_check (GitHub, newer true), route_plot (no loadout error), quit refused; log line 'star map: opened %LOCALAPPDATA%/astromech/data/galaxy.db, preload .../bin/starmap.db'; quit without origin exited. No app code changed, so no pnpm checks. Open items: (1) Windows CI only: the shell's window build and clippy on Windows, NSIS putting starmap.db beside astromech-core.exe (stand-in: preload_beside unit test and the live Linux run with the exe beside a starmap.db), the installed-app preload log line (in the proxy child's onboarding smoke, astromech-i8vr), and the Release onboarding smoke. (2) Paths::publish_config/config_in_use (config.path, its Note, pid liveness and windows-sys use) now has no reader; left in place and its doc comments say so; deleting it is a follow-up. (3) open_external now runs explorer.exe/xdg-open from the core process (detached on Windows), not the shell; untested on Windows. (4) shell_info's version is astromech-api's crate version (same workspace version) and needs a running core, as every forwarded command does. (5) The known-biology lookup still makes its own in-memory Client (session.rs), as before; not changed. (6) The first voice lookup after start now opens (and seeds once from the preload) the star map on the speech thread instead of at session start. (7) trade_route keeps Spansh's default polling; only route_plot's is configurable (State::plot_polling) for tests. Review outstanding.
