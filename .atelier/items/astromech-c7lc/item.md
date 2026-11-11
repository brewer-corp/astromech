---
schema: atelier.item
schema_version: 3
id: astromech-c7lc
title: 'Rust tests read time through the clock: remove the disallowed_methods test exemptions file by file'
status: draft
mission: null
labels: []
priority: low
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-11-07T21:16:15.194493423Z
---

## Goal

Rust tests follow decision astromech-1zhg as the frontend tests do: they wait on events, never on time, and read the time only through a clock they control. astromech-zgoo made clippy reject direct time reads and sleeps (`clippy.toml` disallowed-methods) in production code, and exempted test code for now with `#![allow(clippy::disallowed_methods)]` at the top of each crates/*/tests/*.rs file and `#[allow(clippy::disallowed_methods)]` on each `#[cfg(test)]` module. This removes those exemptions one file at a time.

## Done when

- [ ] No crates/*/tests/*.rs file and no `#[cfg(test)]` module carries the zgoo exemption: each one either waits on events and reads time through a `ManualClock`, or carries a narrower `#[allow]` with its reason (a 30 s hang guard that decides nothing).
- [ ] `cargo clippy --workspace --all-targets --locked -- -D warnings` is clean on Linux and on the Windows target.

## Scope

Test code only, a file or a few per PR. Examples, astromech-mockgame and spikes keep their exemption: they are tools, not the core.

## Context

`grep -rn "tests are exempt for now (astromech-c7lc)" crates` lists what is left.

## Links

Relationships of the old record that the new format has no field for.

- discovered_from: astromech-zgoo (Clippy rejects direct time reads and sleeps; the remaining reads move to the clock)
