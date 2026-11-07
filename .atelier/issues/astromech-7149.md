---
created_at: "2026-11-07T11:57:01.791744772+00:00"
id: "astromech-7149"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T12:49:56.270625319+00:00"
status: "done"
title: "One core assembly that takes its boundaries (app::assemble with Boundaries)"
updated_at: "2026-11-07T12:49:56.270625319+00:00"
---

## Goal

The core is built by one assembly that takes its boundaries as inputs. `app::run`, world mode and the core's own tests all use it. Today `app::run` builds every boundary inline (crates/astromech-core/src/app.rs:1490-1672), and the tests build a second copy of the core by hand (`core_with_state`, app.rs:1740-1780). This child is deliberately small so it can land early: astromech-y6d6's one-app child builds its startup on this API, and the world-mode child plugs fakes into it.

## Done when

- [x] crates/astromech-core/src/app.rs exposes this API. Final names are the implementer's, but the shape is this:
  ```rust
  pub struct Boundaries {
      pub paths: astromech_store::Paths,   // app, log, exe and journal-default folders
      pub processes: Arc<dyn Fn() -> Vec<String> + Send + Sync>,
      pub opener: Arc<dyn Opener>,          // open(target: &str), reveal(path: &Path)
      pub clipboard: Arc<dyn Fn(&str) -> Result<(), String> + Send + Sync>,
      pub notifier: Arc<dyn callouts::Notifier>,
      pub secrets: Arc<dyn astromech_store::secrets::Secrets>,
      pub autostart: Arc<dyn Autostart>,    // get() -> Option<String>, set(on, exe) -> Result<(), String>
      pub remote: Arc<astromech_data::Client>,
  }
  impl Boundaries { pub fn real(paths: Paths) -> Self }
  pub struct Assembled { pub core: Arc<Core>, pub ui: Receiver<Ui> }
  pub fn assemble(opts: &Opts, b: Boundaries) -> Assembled;  // loads config, builds Core; starts no thread
  pub struct Started { pub server: Option<control::Server> }  // threads are detached, as today
  pub fn start(core: &Arc<Core>) -> Started;  // shared by run and world mode
  pub fn run(opts: Opts) -> i32;  // Boundaries::real(Paths::current()), assemble, start, + installed-app steps
  ```
  `start` is what both `run` and world mode do after assembling:
  - serve the control API;
  - the retired-model check;
  - follow the journal history (under the config lock, as at app.rs:1623-1627);
  - the game-watch thread;
  - the idle reaper.
  `run` alone adds, in today's order:
  - `logging::init` and `crash::install` on the paths' folders;
  - the instance lock;
  - `autostart::apply_default`;
  - the update thread;
  - the window;
  - the tray (`ui_loop`).
- [x] Every `Paths::current()` the core reads after startup is replaced by the input `paths`: config.rs:327-356, app.rs:301, :325, :527, :566, :576. The input is also used by:
  - `open_path`, `reveal` and `open_config` (app.rs:1331-1374), and `open_external` (`State::with_opener`, crates/astromech-api/src/state.rs:390), through the opener;
  - `route::Desk`, for its clip writer, in place of `in_dir`'s hard-coded `clipboard::set_text` (route.rs:384-387);
  - the notifier (callouts.rs:360), the secrets (app.rs:1560), Start with Windows' get and set (autostart.rs:69-132), and gamewatch's process list (gamewatch.rs:87, read at app.rs:529 and :1445);
  - the remote-data client.
- [ ] `core_with_state` is gone, and the core's lib tests call `assemble` with test boundaries. `cargo test --workspace` passes on Linux and on Windows CI, and the every-window-command test still passes unchanged in what it asserts.
- [ ] The installed app is unchanged: `Boundaries::real` builds exactly what `run` builds today. Review checks the diff of `run` for that.

## Scope

The refactor only, with no world mode and no new fakes. The clock is added to `Boundaries` by the clock child, and the session's platform and voice by the devices child.

## Context

Decision astromech-6i8s: astromech-y6d6 moves this startup into the one Tauri executable, keeping `run`'s installed-app steps. `Paths` is astromech-store/src/paths.rs:12-90. Review finding t9w7 found `models_dir` escaping the test folder through `Paths::current()`.

## Notes

- 2026-11-07 [decision] Boundaries.remote is a factory, Fn(&Path) -> Arc<astromech_data::Client>, called by assemble with the data folder the config names
  Reason: Boundaries::real(paths) is built before the config is loaded, and the real client's cache lives in the configured data_dir (which --config or data_dir can move), so it can't be made inside real()
  Rejected: Arc<Client> as in the plan's sketch: real() would have to load the config or guess the data folder
- 2026-11-07 [decision] The EDDN and EDSM process-wide switches are set by start, not assemble
  Reason: assemble is what every lib test calls; setting process globals there would race the tests that hold CONFIG_EDITS to check those switches. Nothing reads them between assembly and start, so run behaves the same
  Rejected: Leave them in assemble and make every test core take CONFIG_EDITS (deadlocks the tests that already hold it)
- 2026-11-07 [decision] Opener has a third method, edit(path), for open_config
  Reason: open_config opens Notepad on Windows, not the default handler explorer.exe would pick; edit keeps that
- 2026-11-07 [decision] The platform notifier is the boundary; the session still wraps it in callouts::Background per session
  Reason: Wrapping at startup would start the notify thread at launch instead of with each session, and share its rate limit across sessions; both are behaviour changes
- 2026-11-07 [decision] Tests that need command-layer seams that are not boundaries yet (devices, neutron URL, Spansh polling) use pub(crate) assemble_with(opts, b, state), which assemble calls with the identity
  Reason: Arc::new_cyclic hands the state a Weak to the core, so the state can't be swapped after assembly; the seams belong to the devices and network children

## Report

The core is built by one assembly: `app::assemble(&Opts, Boundaries) -> Assembled { core, ui }` loads the config and builds the `Core` without starting a thread, `app::start(&core) -> Started { server }` starts what `run` and world mode share, and `app::run` is `Boundaries::real(Paths::current())`, `assemble`, `start` plus the installed app's steps. After startup the core reads only its input `Paths` and reaches processes, opener, clipboard, notifier, secrets, Start with Windows and the remote-data client only through `Boundaries`. `core_with_state` is gone; the lib tests assemble with test boundaries. Commit bf21e8b3 on astromech-vucf.

Proof (Linux, after the last code change):
- Done when 1 (API): `cargo check -p astromech-core --bins` and every test below build against it; the shapes are in app.rs (`Boundaries`, `Boundaries::real`, `Assembled`, `assemble`, `Started`, `start`, `run`).
- Done when 2 (no `Paths::current()` after startup): `grep -rn "Paths::current" crates/astromech-core/src` leaves only main.rs's CLI paths (`--quit`, `--apply-update`, console mode, each once at its start) and `run`'s first line. `Config::{journal_dir, models_dir, worker_exe, data_dir}` take `&Paths`; the opener serves `open_external` (`State::with_opener`), the saved-diagnostics reveal (window command and tray) and `open_config`; `Desk::in_dir` takes the clip writer; the session gets the notifier; status and `autostart_on/off` use `Autostart`; the game watch and `install_update` use the process list; secrets and the remote client come from `Boundaries`.
- Done when 3 (tests): `cargo test --workspace` exit 0, 134 test binaries ok, 0 failed. The every-window-command test passes with the same `NOT_RUN_FOR_REAL` list and the same opened-URL assertion; only its construction and three `not_called` reason strings changed. Windows CI not yet run.
- Done when 4 (installed app unchanged): live run of the old (HEAD before this change) and new debug `astromech-core` binaries, normal start, temp HOME, a config naming a journal folder of the history fixtures. Both: threads `control, gamewatch, history, notify-rs inotify, preview-idle, update` and the main thread; same `status` reply (app_dir, log_dir, config, autostart false, "voice not loaded", history 52 events ready); `astromech-core --quit` exit 0, core exit 0, core.json removed. The normalised logs differ in one line's position only (see Deviations). Review of `run`'s diff is outstanding.
- Also: `cargo clippy --workspace --all-targets -D warnings` clean; `cargo clippy -p astromech-shell --features window --all-targets -D warnings` clean (fake .pc files and empty sidecar placeholders, both removed); `cargo fmt --all --check`, `scripts/check-deps.py`, `scripts/check-guide.py`, `cargo deny check` pass.

### Deviations
- `Boundaries.remote` is `Arc<dyn Fn(&Path) -> Arc<Client>>`, not `Arc<Client>`: `real` runs before the config names the data folder (decision note).
- `start` also sets the EDDN and EDSM switches, which `run` set while building the core: in `assemble` they raced the lib tests (decision note).
- `Opener` has `edit(path)` besides `open` and `reveal`, to keep Notepad for the config (decision note).
- `autostart::apply_default` now runs before the config is loaded (it was after), so the log's `autostart:` line comes before the `config:` line. The two steps are independent. Its check reads the Run value once instead of twice.
- `Paths` is resolved once at startup. Before, each `Config` path call re-read it, so a portable `astromech.toml` created next to the exe while the app ran would have moved the models and data folders mid-run; now it takes a restart, as the config location already did.
- tests/fixtures/core-replies.json: `status` now shows the assembled core's `app_dir` (`<dir>/local/astromech/data`) and `voice: "voice not loaded"` (the hand-built test core had `""`).

### Decisions
Five decision notes on this record: the remote factory, switches in `start`, `Opener::edit`, the notifier wrapped per session, and `assemble_with` for the command layer's non-boundary seams.

### Surprises
The lib tests' cores on Windows CI fell back to the runner's real `USERPROFILE` for the journal folder through `Paths::current()`; test boundaries now give no user profile.

### Open items
- Windows CI: `cargo test --workspace`, clippy of the Windows-only code touched (session.rs `start_voice` takes the data folder, tray.rs reveal and import, autostart.rs's private `get`/`set`); it can't be built here (no MSVC C toolchain for the sqlite and ring build scripts).
- Independent review, including `run`'s diff against `Boundaries::real` (Done when 4).
- The every-window-command test's `not_called` entries (`autostart_on`, `autostart_off`, `open_config`) now have seams; running them is astromech-gllh's.
