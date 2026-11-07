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
updated_at: "2026-11-07T21:13:06.972883127+00:00"
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
