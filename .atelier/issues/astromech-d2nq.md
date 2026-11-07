---
created_at: "2026-11-07T11:57:01.794229152+00:00"
id: "astromech-d2nq"
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
    id: "astromech-7149"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-e1a5"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T13:44:21.013851856+00:00"
status: "done"
title: "Headless world mode: astromech-core --world, fakes for the assembly's boundaries, observations"
updated_at: "2026-11-07T15:17:31.945727681+00:00"
---

## Goal

`astromech-core --world <dir>` runs the real core with no window or tray, on a temp copy of a world (decision: world format), with fakes for the boundaries the assembly takes. It prints how to reach it on stdout. Tests, the dev bridge and yis2's driver start this binary and talk to it over the control API.

## Done when

- [x] `--world <dir>` copies the world to a fresh temp directory and builds `Boundaries` there:
  - `Paths` with app, log and exe folders under the temp dir;
  - a config naming every folder (data, models, journal, Bindings, game) under it;
  - fakes for the process list, opener, clipboard, notifier, Start with Windows, and secrets in memory seeded from the world.
  It then calls `assemble` and `start`, the same start function `run` uses (game watch, idle reaper, history, serving). It doesn't call what only `run` adds: the instance lock, `autostart::apply_default`, the update thread, the window and the tray.
- [x] Once the control API is up, the binary prints one JSON line on stdout: `{"control":{"port":…,"token":…},"app_dir":…}`. This is the only endpoint source the tests and the bridge use. It runs until `quit`.
- [x] The fakes record what reaches them, in order, as observations. `world_observed {since}` and `world_wait {match}` are registered only in world mode, as `local` commands (decision astromech-unh9: refused with the window's origin). `world_wait` is woken by the observation itself, not by polling, and has a hang guard that fails the call.
- [x] A test helper in crates/astromech-core/tests/support/ spawns `CARGO_BIN_EXE_astromech-core --world <dir>`, reads the stdout line, and calls commands with `astromech_api::control`.
- [ ] A test over `worlds/minimal` runs with HOME, XDG_DATA_HOME, LOCALAPPDATA, APPDATA and USERPROFILE all pointing at one empty folder. It checks that:
  - `status` answers;
  - `open_config` appears in `world_observed` with the world's config path;
  - `autostart_on` appears as Start with Windows set;
  - after `quit` the process exits 0, and the empty folder is still empty.
  It runs in `cargo test --workspace` on Linux and on Windows CI.
- [x] The no-spawn proof for world mode: every opener, Start with Windows and notifier effect appears in `world_observed`, and the real ones are not reachable from `Boundaries` in world mode. This is checked in review against the world's `Boundaries` construction. The speaker and platform are added by the devices child.

## Scope

The flag, the world directory loader (seed folders and config; steps come with the script child), the fakes named above, the two observation commands and the test helper. Don't change `run`.

## Context

The assembly API comes from the assembly child. On Linux `ui_loop` already runs without a tray (app.rs:1685-1698). The journal folder falls back to `%USERPROFILE%` (astromech-store/src/paths.rs:150-160), hence USERPROFILE in the empty-folder test. The flag moves to the one executable in astromech-y6d6.

## Notes

- 2026-11-07 [note] From finding astromech-gmop (7149 review): fix assemble's half-edited doc comment (app.rs:1580-1582) and save_diagnostics' stale 'with no opener seam' reason (every_window_command.rs:186-188) in this child's commits.
- 2026-11-07 [decision] Mode-only commands come through Boundaries::more_commands, Fn(&Arc<Core>, &mut Registry), which Core::commands calls last; None for the installed app and the lib tests
  Reason: start serves core.commands(), so world mode registers world_* without a second start function or a change to run; the siblings' world_step and gate hook can register the same way, and the closure gets the core for them
  Rejected: start_with(core, extra) beside start: world mode would then not call the start run uses
  Rejected: A world field on Core: the core would know about worlds
- 2026-11-07 [decision] A world's secrets are seeded from an optional secrets.json, {name: value} by the secret's name (edsm-api-key)
  Reason: The Done when needs secrets seeded from the world and decision astromech-e1a5 names no place; a seed file sits with the other seed files, and world.jsonl's header belongs to the script child
  Rejected: A secrets field in world.jsonl's header: that loader is astromech-i1ev's
- 2026-11-07 [decision] In world mode the remote-data client refuses every request and records it as http {url, path, unrecorded: true}, in memory with no request spacing
  Reason: World mode must reach no real service; the replay of net.json is astromech-xk0n's, which replaces this transport
  Rejected: The real client until xk0n lands: lookups from a world would reach Spansh, Ardent and EDSM
- 2026-11-07 [decision] control::call_to and call_window_to take a Contact {port, token} (Endpoint::contact), and the stdout line's control object is a Contact
  Reason: The Done when makes the stdout line the only endpoint source, and control::call reads core.json; call is now read_endpoint then call_to
  Rejected: Calling with app_dir and core.json: a second endpoint source
- 2026-11-07 [decision] On quit, world mode drops the control server and removes its temp folder; a failure is printed on stderr and the exit is still 0
  Reason: Each test run would otherwise leave a world in the temp folder; on Windows the core's open databases and log can keep it, which only leaks a temp folder
- 2026-11-07 [decision] The app window is a boundary, Boundaries::window (Fn(Option<&str>) -> Result<(), String>): Boundaries::real starts the shell exactly as Core::open_window did (open_shell over the paths' exe folder), and world mode records window {route}
  Reason: Finding astromech-nzgh: open_window spawned the real shell in world mode when ASTROMECH_SHELL was set; a boundary keeps run unchanged and lets world mode record it like the opener
  Rejected: Refusing open_window in world mode: the tray and second-start path would then be untestable in a world
- 2026-11-07 [result] Review fixes in 83cee7e3: 4zdn (the helper owns an empty home for all eight home variables; quit() checks it), q0bw (world config forces [eddn] and [edsm] enabled = false), 3tli (--world conflicts with --quit, --apply-update and its options, --config, --background, --route and every console option), nzgh (Boundaries::window, recorded in world mode), zmth (the helper pipes stdin; world mode quits on stdin EOF). Still open: a local client's set_eddn or set_edsm with enabled true switches uploading back on in a world; no committed world or test does that.
- 2026-11-07 [result] Teardown flakes fixed in 04cf0853. (1) quit lost its reply: the handler sent Ui::Quit before the reply was written and teardown ended the process; now Call::after_reply runs it after the control API sends the reply (an ordering fix the installed app shares). (2) 'Directory not empty': the history follower's first History::open created history.sqlite-shm during remove_dir_all; Following::stop joins the follower and world mode calls it before removing the temp world. world_mode passed 200 of 200 runs after the fix (8 of 300 killed-parent failures before).

## Report

`astromech-core --world <dir>` runs the real core headless against a temp copy of a world. `world::World::load` copies the world into `<tmp>/world/` and rewrites its `astromech.toml` so data, models, journal (`world/game`), Bindings (`world/bindings`) and the game install folder (`elite/`) all sit under the temp folder. `World::boundaries` builds the `Paths` (exe `<tmp>/exe`, LOCALAPPDATA `<tmp>/local`, no user profile) and these fakes:
- the world's process list;
- an opener, clipboard, notifier and Start with Windows that record what reaches them;
- in-memory secrets seeded from `secrets.json`;
- a remote-data client that refuses every request and records it as unrecorded.

`world::run` then calls `app::assemble` and `app::start`, prints `{"control":{"port","token"},"app_dir"}` on stdout, waits for `quit`, deactivates, and removes the temp folder. `Boundaries::more_commands` registers the `local` commands `world_observed {since?}` and `world_wait {match, since?}` in world mode only. `world_wait` waits on a condvar that each recording notifies, and fails at `WAIT_GUARD` (30 s). `control::call_to` and `call_window_to` reach a core through a `Contact {port, token}`. `tests/support/` has `WorldCore`, and `tests/world_mode.rs` runs `worlds/minimal`. Commit 8b38e85d on astromech-vucf.

Proof (Linux, after the last code change):
- Done when 1 (temp copy, Boundaries, assemble and start): `world::tests::a_loaded_world_names_only_its_temp_folders_and_leaves_the_world_alone` checks every config folder, app and log dir, exe dir and seeded secret against the temp root. It also checks that the source world is unchanged and that the temp folder goes with the `World`. `the_fakes_record_what_reaches_them` checks each fake's observation. `world::serve` calls `app::assemble` and `app::start` and nothing else from `run` (instance lock, `apply_default`, update thread, window, tray). Inspection: the `run` diff is empty apart from `Boundaries::real` gaining `more_commands: None`.
- Done when 2 (stdout line): live run `./target/debug/astromech-core --world worlds/minimal` printed `{"app_dir":"/tmp/astromech-world-Vywmx8/local/astromech/data","control":{"port":37231,"token":"…"}}`. Raw TCP calls with that port and token gave:
  - `status`: ok, config `/tmp/astromech-world-Vywmx8/world/astromech.toml`;
  - `open_config` and `autostart_on` (origin app_window): `{"ok":true}`;
  - `world_observed`: the `edit` (config path) and `autostart` (on, command line) observations, next 2;
  - `world_wait {"match":{"kind":"edit"}}`: the edit;
  - `world_observed` from app_window: "not allowed: world_observed is for local control clients only";
  - `quit`: ok. The process exited and the temp folder was gone.
  The test helper reads only this line (`control::call_to`).
- Done when 3 (observations, local, woken, guard): unit tests `observations_are_numbered_in_order_and_read_from_a_seq`, `a_wait_finds_a_past_match_from_its_seq_and_skips_earlier_ones`, `a_wait_is_woken_by_the_observation` and `a_wait_with_no_match_fails_at_its_guard`. In the wake test the waiter reports that it is parking while it still holds the lock, so each recording lands while it is parked. Mutation: with `notify_all` removed, that test hangs (killed by `timeout 20`, exit 143). The world test checks that both commands refuse the window's origin.
- Done when 4 (helper): crates/astromech-core/tests/support/mod.rs `WorldCore::start` spawns `CARGO_BIN_EXE_astromech-core --world`, reads the line and calls through `control::call_to` / `call_window_to`. Drop kills a child that is still running.
- Done when 5 (minimal world, empty folders), Linux only: `the_minimal_world_runs_headless_and_leaves_the_users_folders_alone` passes in `cargo test --workspace`. Mutation: with world mode's `Paths` set to `Paths::current()`, it fails at the empty-folder assertion (world_mode.rs:51). Real HOME: after the live run, `find ~ -maxdepth 3 -newer <marker> -iname '*astromech*'` (excluding .t3 and .claude) found nothing, and `~/.local/share/astromech` does not exist.
- Also: `cargo test --workspace` exit 0 (135 test binaries, 1309 passed, 0 failed); `cargo clippy --workspace --all-targets -- -D warnings` clean; `cargo fmt --all --check`, `scripts/check-deps.py`, `scripts/check-guide.py` and `cargo deny check` pass.

### Deviations
- `Boundaries` has a new field, `more_commands` (decision note). The plan named only fakes.
- `world_wait` and `world_observed` also take an optional `since`, so a wait can skip earlier matches.
- World mode starts logging, into the temp folder's log dir, but no crash marker.
- Finding astromech-gmop: assemble's doc comment and save_diagnostics' reason are fixed (the reason now says the test's opener doesn't record a reveal, and names astromech-gllh).

### Decisions
Five decision notes on this record: `more_commands`, `secrets.json`, the refusing remote client, `Contact` and `call_to`, and removing the temp folder on quit.

### Surprises
- My first live run used a stale binary, the `Paths::current()` mutant from Done when 5's mutation (`cargo test --lib --no-run` doesn't rebuild the bin). It created `~/.local/share/astromech/data` with its log and `core.json`. That folder didn't exist before the run, and I removed it along with that run's temp world. I then rebuilt the binary and repeated the run cleanly.
- The core starts in milliseconds, so the world test takes about 0.01 s.

### Open items
- Done when 5's Windows half: the world test on Windows CI. It hasn't run there. On Windows the temp folder may survive quit, because the core's open files hold it; the test asserts its removal only off Windows.
- Done when 6, no-spawn proof: left for review against `World::boundaries`.
- Still real in world mode, for the later children: the update, models, neutron, EDSM flight-log and EDDN HTTP paths (astromech-xk0n). The mic on Windows, and a session's platform, speech worker and speaker (astromech-1hh9). `open_window` looks for the window in the world's empty exe folder, so it starts one only if `ASTROMECH_SHELL` names one.
- Independent review is outstanding.
