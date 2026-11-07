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
status: "todo"
title: "One core assembly that takes its boundaries (app::assemble with Boundaries)"
updated_at: "2026-11-07T11:57:01.791744772+00:00"
---

## Goal

The core is built by one assembly that takes its boundaries as inputs. `app::run`, world mode and the core's own tests all use it. Today `app::run` builds every boundary inline (crates/astromech-core/src/app.rs:1490-1672), and the tests build a second copy of the core by hand (`core_with_state`, app.rs:1740-1780). This child is deliberately small so it can land early: astromech-y6d6's one-app child builds its startup on this API, and the world-mode child plugs fakes into it.

## Done when

- [ ] crates/astromech-core/src/app.rs exposes this API. Final names are the implementer's, but the shape is this:
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
- [ ] Every `Paths::current()` the core reads after startup is replaced by the input `paths`: config.rs:327-356, app.rs:301, :325, :527, :566, :576. The input is also used by:
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
