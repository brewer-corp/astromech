---
schema: atelier.review
schema_version: 2
id: astromech-6ktq
title: approval on astromech-bp23 @36ac2e58
target: astromech-bp23
identity: claude-reviewer-ndzx
commit: 36ac2e58e02be587b011992816354b14f2ca84fd
created_at: 2026-11-07T07:42:04.376666790Z
event:
  event: approval
---

Review of astromech-ndzx (b42b7f92..36ac2e58) against its Done when (with the 9h8f note), bp23, unh9/1zhg/6i8s. Read lookups.rs, state.rs (GalaxyHolder, Busy, open_external, settings), lib.rs (Call, nested dispatch keeps origin), the core wiring (api_state, remote_client, Session::start, Runner::galaxy, console), and window.rs forwarders vs the old bodies: arg names match, and the 8 typed replies (and null replies of market_compare/set_search_filters/open_external) keep the JSON shape; neutron progress {done,total} reaches Channel<Value> as onProgress, matching main's transport CHANNELS. No astromech.toml reader, cache or .manage is left in the shell; tauri.conf installs starmap.db and astromech-core in INSTDIR. Reran: api 23/23, core all ok, workspace 1280/0, clippy -D warnings, fmt, check-deps, check-guide, deny; shell --features window clippy clean on an exported copy with fake .pc files (planted type error in window.rs caught); live headless core as app_window (shell_info, route_plot_ship, filters round trip, neutron_file/check, outfitting, trade_cargo, logbook, open_external refuses /etc and an unknown URL, quit refused for the window and accepted locally), log line 'star map: opened .../galaxy.db, preload .../bin/starmap.db'. Four deferred findings, the medium one (shell_info/Diagnostics) to i8vr before bp23 merges.
