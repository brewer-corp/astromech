---
created_at: "2026-11-07T11:57:01.793391661+00:00"
id: "astromech-zgoo"
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
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Clippy rejects direct time reads and sleeps; the remaining reads move to the clock"
updated_at: "2026-11-07T21:35:27.000794365+00:00"
---

## Goal

Lint keeps the core on its clock: a new direct time read or sleep in production code fails clippy. The clock child moves the behaviour sites. This child moves the rest and adds the rule, so the rule holds for the whole workspace and not just the sites the worlds need today.

## Done when

- [x] `clippy.toml` disallows these methods, so `cargo clippy --workspace --all-targets --locked -- -D warnings` fails on a new use:
  - `std::time::Instant::now`;
  - `std::time::SystemTime::now`;
  - `chrono::Utc::now`;
  - `chrono::Local::now`;
  - `std::thread::sleep`;
  - `std::sync::Condvar::wait_timeout`;
  - `std::sync::mpsc::Receiver::recv_timeout`.
- [x] The remaining production reads go through the clock: diagnostics file names (diagnostics.rs), crash marker (crash.rs), health `checked_at` (health.rs:103), keybinds backup and doctor stamps (crates/astromech-keybinds/src/edit.rs, doctor.rs), powerplay and galaxy dates, EDDN feed stamps, voice download progress, and `control.rs`'s one read. A read that must stay real carries `#[allow(clippy::disallowed_methods)]` with its reason. Those are:
  - `SystemClock`'s impl;
  - astromech-log's line stamps;
  - the speech worker binary;
  - Windows raw-input, XInput and mic sample timing;
  - the pacing waits the clock child listed.
- [x] Test code is exempt for now, by `#![allow(clippy::disallowed_methods)]` at the top of each crates/*/tests/*.rs file and `#[allow(clippy::disallowed_methods)]` on each `#[cfg(test)] mod tests`. astromech-mockgame, examples and spikes are exempt the same way. A follow-up issue (`--discovered-from` this one) holds removing the test exemptions file by file, under decision astromech-1zhg.
- [x] Mutation: an `Instant::now()` added to pipeline.rs's production code fails clippy, and the same line inside its test module doesn't.
- [ ] `cargo test --workspace` passes on Linux and Windows CI, and Windows clippy (`cfg(windows)` paths) is clean.

## Scope

clippy.toml, the remaining reads, the allow attributes and the follow-up issue. No behaviour change. `controls::start` and `Mapper::new` stay untouched apart from lint allows.

## Context

The site counts at planning: about 116 production reads and 40 waits across the core process's crates. CONTRIBUTING.md's Tests section ("Lint rejects `setTimeout` and wall-clock reads in test files") gets a matching Rust line.

## Notes

- 2026-11-07 [note] From finding astromech-4k6h (d2nq review): world_wait's 30 s guard (world.rs:437-458) uses Instant::now and Condvar::wait_timeout on real time; add it to the list of waits that stay real, with an #[allow] and the reason (it only fails a stuck call and decides nothing).
- 2026-11-07 [note] From finding astromech-vwbs (1hh9 review): the voice tester is armed and read with Instant::now (app.rs:1185, :1200, :418) while the push-to-talk loop reads TESTER.armed(clk.now()); move these three sites to the clock rather than giving them an allow.
- 2026-11-07 [decision] clippy.toml also disallows Instant::elapsed and SystemTime::elapsed, beyond the seven methods Done when 1 names
  Reason: elapsed() is Instant::now() minus the instant: an Instant taken from a ManualClock and measured with elapsed() would mix the world's time with real time, silently. The sites it found (verify ms, session start, STT timings, the confirm window, download progress) moved to clock.now().saturating_duration_since(t), which is what elapsed does
  Rejected: The seven methods only: the confirm window (assistant.rs) would have taken its stamp from the clock and its check from real time
- 2026-11-07 [decision] Sites that stay on real time beyond the spec's list, each with an allow and its reason: the TTS speaker's PlaySoundW line end (speech.rs Activity::playing and TtsSpeaker::busy), ConsoleSpeaker's printed stamp, the installed app's std ControlRx (controls.rs), calibration's record_mic, the mic's callback stamp and stall check, the window process's watchdog, input listener and cold-start line (app/src-tauri window.rs), world_wait's and the settle and advance hang guards, ManualClock's origin Instant and wait_others_parked's guard
  Reason: Each is device timing (audio plays and is recorded in real time), a printed line's stamp (the log's category), the window process rather than the core, or a hang guard that decides nothing; none runs as behaviour under a world's clock. A test fake (input's platform::mock) and a cfg(test) helper are exempt as tests are
  Rejected: Threading a clock into TtsSpeaker and Speech::speaker: under a manual clock a line's end would follow advance rather than the audio device
- 2026-11-07 [decision] Clock parameters, not new defaults: keybinds' Api holds a clock (Api::discover takes it) and edit's backup/restore/set_slot/set_active, doctor::Inputs, health::run_check(_with), tester::dry_run_idle, diagnostics::Inputs, crash::install, control::serve, apply_edsm_switch, models::download_from, ensure_in_from, PreviewSlot::say, neutron Kept::get and powerplay overview take one; Toasts and Background take theirs at construction. The installed app passes SystemClock or Core::clock, the CLI paths (main.rs) SystemClock
  Reason: One owner per clock (Boundaries::clock); the pieces read time only where they did before, now on the clock they are given, so under SystemClock each read is the same call at the same point. Keybinds and voice gain an astromech-proto dependency (allowed: proto is a leaf)
- 2026-11-07 [decision] eddn's wall_now() is removed: its callers (the pipeline's EDDN and EDSM stamps, the outbox discards) read clock.now_s(), and feed.rs's tests have a local helper
  Reason: Clock::now_s's default is wall_now's body (SystemTime::now since UNIX_EPOCH in whole seconds, 0 before it), so the stamps are identical under SystemClock; one function for it instead of two
- 2026-11-07 [decision] vucf pre-merge (uerh): one watcher constructor, astromech_core::watch::watcher(tx, item), applies may_change; clippy.toml disallows notify::recommended_watcher and notify::Watcher::new elsewhere
  Reason: A new watcher can't skip the filter without an allow a reviewer sees; the three watchers (pipeline, history, health) use it, with what each sends unchanged
- 2026-11-07 [decision] vucf pre-merge (jkgv): removed the dumper examples the drift test covers (odyssey dump-odyssey-fixture, ship dump-ships-fixture, market dump-trade, galaxy dump-sites and dump-nearby, engineering dump-fixture and dump-unlocks, materials dump-fixture, keybinds fixture) and materials' sample_inventory, which only the materials dumper used; kept galaxy's dump-local-route
  Reason: Each of those files is now a Core part of preview_fixtures.rs. plot.json's /local is a Not part that names dump-local-route as its seed (a world doesn't install the neutron file), so it still has a job

## Report

**Outcome.** Done when 1-4 hold; line 5 holds on Linux (tests and Windows-target clippy) and waits for the PR's Windows CI, so it stays unticked. Branch `astromech-vucf`, commits f6180249 (code, docs, records) and 835020c8 (spikes), after "Start astromech-zgoo" 995688d4. Not pushed; review outstanding.

1. **The rule.** `clippy.toml` disallows the seven methods plus `Instant::elapsed` and `SystemTime::elapsed` (decision note). Mutation: a function in pipeline.rs's production code calling `Instant::now`, `SystemTime::now`, `thread::sleep`, `mpsc::Receiver::recv_timeout`, `Condvar::wait_timeout`, `chrono::Utc::now`, `chrono::Local::now`, `Instant::elapsed` and `notify::recommended_watcher` failed `cargo clippy -p astromech-core --all-targets --locked -- -D warnings` with one "use of a disallowed method" error each (9 errors). A second run with only `<RecommendedWatcher as Watcher>::new` in production and `Instant::now()` in pipeline.rs's `mod tests` failed on the watcher alone: the test-module line passed. Both reverted.
2. **The remaining reads, on the clock.** Under `SystemClock` each is the same call at the same point, so the installed app's behaviour is unchanged. `Clock::now()` is `Instant::now()`, `utc()` is `SystemTime::now()`, and `now_s()` is `wall_now()`'s body. `DateTime::<Utc>::from(utc())` is `Utc::now()`, and `DateTime::<Local>::from(utc())` is `Local::now()`, checked in chrono 0.4.45's source: `Local::now` is `Utc::now().with_timezone(&Local)`, the same as `From<SystemTime>`; the two differ only before 1970, where `Utc::now` panics. `clock.now().saturating_duration_since(t)` is `t.elapsed()`. Sites, each compared with the diff against the branch base:
   - **EDDN/EDSM uploads** (xk0n): the pipeline's `wall_now()` at pipeline.rs's journal, NavRoute, companion-file and flush stamps is now `self.clock.now_s()`. The EDDN and EDSM outbox discards in `apply_eddn_switch` and `apply_edsm_switch` read `clock.now_s()` from `Core::clock`, or `SystemClock` in main.rs's console path. `wall_now` is removed (decision note).
   - **Voice tester** (vwbs): `TESTER.arm` and `armed` in `voice_test_poll` and `voice_test_stop`, and `status_line`, read `Core::clock.now()`, the same clock the push-to-talk loop reads.
   - **Voice and actions:**
     - the confirm window (assistant.rs `Pending` `at` and its check) runs on the policy's clock;
     - the verify's "confirmed in N ms" uses the hub's clock;
     - the STT thread's "heard … (N ms)" and the saved clip's file stamp use the session clock;
     - the voice preview's idle stamp (`PreviewSlot::say`) uses `Core::clock`, as its reaper does.
   - **Stamps:**
     - the diagnostics zip name and `system.txt` time (diagnostics.rs);
     - the crash marker (crash.rs: `run` passes `SystemClock`);
     - health `checked_at` and the doctor report's `generated_at`;
     - keybinds backup names, read at each try as before, from `Api::clock`, which is the command layer's clock;
     - toast times (`Toasts::new(clock)`);
     - the update check's `last_check`;
     - "active in N ms".
   - **Command layer:**
     - powerplay `overview(now)` from `State::clock()`;
     - the neutron file's last use (`Kept::get(clock)`), which `release_idle` compares with the same clock;
     - the known-bodies and known-biology lookups' times (`Background::new(thread, client clock)`);
     - control.rs's token time (`serve(.., clock)`).
   - **Voice download progress** (download.rs): `elapsed` and the 200 ms report throttle, on the clock passed to `download_from` and `ensure_in_from`. The other entry points pass `SystemClock`.
   - **Stay real, with an allow and its reason:**
     - `SystemClock`'s impl and its std waits;
     - `ManualClock`'s origin and `wait_others_parked`'s guard;
     - astromech-log's line, roll and prune stamps;
     - the speech worker binary (crate-level);
     - raw-input `handle_input`, the mic's callback stamp and stall check, and calibration's `record_mic`;
     - the installed app's std `ControlRx`;
     - the pacing waits u8ge listed;
     - world_wait's guard (4k6h), the settle `wait_until` guard and `world_step`'s advance guard.
     Beyond the spec's list, these stay real too (decision note): the TTS speaker's PlaySoundW line end and `busy`, `ConsoleSpeaker`'s printed stamp, and the window process's watchdog, input listener and cold-start line.
3. **Exemptions:**
   - `#![allow]` at the top of every crates/*/tests/*.rs file (77), every examples/*.rs file (7), the mock game and every spike crate root (21);
   - `#[allow]` on every `#[cfg(test)]` module (154) and on two `cfg(test)` helpers (galaxy's `now_s`, `Background::settle`);
   - input's `platform::mock` test fake.
   Follow-up astromech-c7lc (backlog, `--discovered-from astromech-zgoo`) removes the test ones file by file.
4. **Checks, after the last code change** (spikes changed after the suite ran; they are outside the workspace):
   - `cargo test --workspace --no-fail-fast` exited 0: 141 binaries, 1,377 passed, 0 failed.
   - `cargo clippy --workspace --all-targets --locked -- -D warnings` is clean, and so is the Windows-target run (`--target x86_64-pc-windows-msvc`, fake MSVC tools).
   - `cargo fmt --all --check`, `scripts/check-deps.py` ("32 workspace crates") and `cargo deny check` pass.
   - The world tests ran 30 times from the built binaries: world_steps, world_voice, world_mode, world_net, every_window_command, preview_fixtures and the lib's `world::` and `watch::` tests. All 210 runs passed.
   - In app/, `pnpm -s check` (0 errors), `pnpm -s lint` and `pnpm -s test` (77 files, 544 tests) pass. app/ changed only comments in four test files.

### vucf pre-merge

- **Finding uerh.** `astromech_core::watch::watcher(tx, item)` is the one constructor, and it applies `may_change`, which moved there from pipeline.rs. The pipeline, history and health watchers use it. clippy.toml disallows `notify::recommended_watcher` and `notify::Watcher::new` elsewhere; the mutation in 1 shows both rules fire. Test `watch::tests::only_a_real_write_reaches_the_clock_channel` (Linux) reads a file in a watched folder, then writes another, and asserts the first item on the clock channel is the write's. With the filter removed it failed: the first event was `Access(Open(Any))` on the read file.
- **Finding 9rru.**
  - crates/astromech-core/tests/committed_worlds.rs runs in `cargo test --workspace`. It fails for a `worlds/**/secrets.json` value that doesn't say `not-a-real`, `fake` or `placeholder` (or is empty), and for a `net.json` get-logs answer holding logs: `json`, a JSON `text`, an `in_turn` answer or a `from` file.
  - `the_check_finds_a_real_key_and_a_flight_log` covers each case. With a 40-hex key planted in worlds/every-command/secrets.json, the committed-worlds test failed naming it; the file was restored.
  - Record mode now also redacts secrets that an answer echoes JSON-escaped (`\uXXXX`, `\/`), by replacing them in the parsed strings. Test `replay::tests::a_tape_redacts_secrets_an_answer_echoes_json_escaped` failed before the fix and passes after.
  - CONTRIBUTING.md states both rules.
- **jkgv's dumpers.** Removed the nine examples that the drift test covers, and materials' `sample_inventory`, which only its dumper used. Kept galaxy's `dump-local-route`: plot.json's `/local` is a `Not` part that names it as the seed. The comments in docs/V1-DESIGN.md §7.11 and four app test files now point at the worlds instead.

### Deviations

- The lint also covers `elapsed` and `notify`'s watcher constructors. Some sites stay real beyond the spec's list; see the decision notes.
- CONTRIBUTING.md's Tests section has a "The core's clock" paragraph, and ARCHITECTURE's Time paragraph names the rule.

### Open items

- **Windows CI.** Done when 5 waits for the PR's Windows test and clippy jobs. Not built locally:
  - the shell's `window` feature (CI's "app shell clippy"): its build script fails here, so the three allows in app/src-tauri/src/window.rs were only read;
  - the speech worker binary (feature `sherpa`).
- **astromech-2ebj** (backlog): in a world, the session's assistant judges market data age on `SystemClock` (`Assistant::new`'s default). It was like that before this change; the installed app is unaffected.
- **astromech-5i80** (backlog): preview_fixtures' reason for not dumping eddn-log.json says the EDDN outbox runs on the wall clock, which no longer holds as written.
- Under a world's manual clock the control token mixes in the world's time. Its hashers are still OS-seeded, so the token stays random.
