---
created_at: "2026-11-07T06:33:38.679175382+00:00"
id: "astromech-ro7a"
issue_type: "task"
labels:
- "size-s"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-ndzx"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Test: every window command answers over a running core's control API"
updated_at: "2026-11-07T11:08:37.493744436+00:00"
---

## Goal

A test lists the window's commands and calls each one through a running core's control API, so a command the window uses can't be missing from the command layer (success criterion one-command-list).

## Done when

- [ ] An integration test in crates/astromech-core builds a core over a temp app, data and journal folder, serves its registry on the real control API (TCP, token, `core.json`), and calls every `window`-exposure command with example args from a table in the test. The test fails when a `window` command has no entry in the table.
- [ ] Each reply is `ok`, or an error that is the handler's own (for example "journal folder not found"). It is never "unknown command", bad args, "not allowed" or an internal error (a caught panic).
- [ ] No network and no external program: the remote-data client runs over a transport that refuses, and the opener is the test's. Commands whose effect has no seam yet (speech-model and neutron downloads, the update check and install, start with Windows, `open_config`, `save_diagnostics`' Explorer reveal, the microphone and voice preview, and EDSM's flight log) are called with args or state that make them refuse before the effect, or are listed by name with the missing seam. The scripted world in astromech-vucf replaces that list with real calls.
- [ ] The test runs in `cargo test --workspace` on Linux, with no sleeps and no wall-clock waits.

## Scope

A test, plus the example args table. Do not build a headless binary or a world script here: that is astromech-vucf.

## Context

"The window's commands" is the registry's `window` entries. astromech-96dv makes `call()` accept only those names, so the window can't call a command outside them. Together the two children prove one-command-list: everything the window can call is in this list, and everything in this list answers over the control API. The headless half of that criterion comes with astromech-vucf (its Done when runs this test against the headless core).

## Notes

- 2026-11-07 [note] Decision (implementer): the test is a module of astromech-core's lib tests (crates/astromech-core/src/app/every_window_command.rs), not a file under crates/astromech-core/tests/. Reason: a Core with the test's seams (in-memory secrets, refusing remote client, test opener and devices, no clipboard) is built only by app.rs's test helper over the private api_state; a public constructor belongs with astromech-vucf's headless mode, which will start the core this test then talks to. The test still goes through the real control API (serve, core.json, token, TCP, control::call_window). Also (implementer): the neutron check and download run for real against a local server answering 503 (State::with_neutron_url made public), so they are off the not-run-for-real list; astromech-vucf's Done when line now names the exact list. For the plan owner to accept or change.
