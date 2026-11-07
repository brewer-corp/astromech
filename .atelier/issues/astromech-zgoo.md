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

- [ ] `clippy.toml` disallows these methods, so `cargo clippy --workspace --all-targets --locked -- -D warnings` fails on a new use:
  - `std::time::Instant::now`;
  - `std::time::SystemTime::now`;
  - `chrono::Utc::now`;
  - `chrono::Local::now`;
  - `std::thread::sleep`;
  - `std::sync::Condvar::wait_timeout`;
  - `std::sync::mpsc::Receiver::recv_timeout`.
- [ ] The remaining production reads go through the clock: diagnostics file names (diagnostics.rs), crash marker (crash.rs), health `checked_at` (health.rs:103), keybinds backup and doctor stamps (crates/astromech-keybinds/src/edit.rs, doctor.rs), powerplay and galaxy dates, EDDN feed stamps, voice download progress, and `control.rs`'s one read. A read that must stay real carries `#[allow(clippy::disallowed_methods)]` with its reason. Those are:
  - `SystemClock`'s impl;
  - astromech-log's line stamps;
  - the speech worker binary;
  - Windows raw-input, XInput and mic sample timing;
  - the pacing waits the clock child listed.
- [ ] Test code is exempt for now, by `#![allow(clippy::disallowed_methods)]` at the top of each crates/*/tests/*.rs file and `#[allow(clippy::disallowed_methods)]` on each `#[cfg(test)] mod tests`. astromech-mockgame, examples and spikes are exempt the same way. A follow-up issue (`--discovered-from` this one) holds removing the test exemptions file by file, under decision astromech-1zhg.
- [ ] Mutation: an `Instant::now()` added to pipeline.rs's production code fails clippy, and the same line inside its test module doesn't.
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
