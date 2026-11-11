---
schema: atelier.mission
schema_version: 3
id: astromech-kjfs
title: One tested process
status: ready
labels: []
lead: null
ready:
  by: conversion
  at: 2026-11-07T22:55:39.284832217Z
created_at: 2026-11-07T04:50:34.669875805Z
---

## Why

astromech is two processes that each do half the work. The core runs the live pipeline, voice,
input, the route follower and the tray. The Tauri shell, meant to be thin, registers 52
commands, 48 of them its own: journal scans, keybind edits, Spansh lookups and route plotting. So every
cache and the 184 MB neutron star file have two owners, and the "one API for every client"
rule (ARCHITECTURE §18) is broken by the shell itself. Testing matches the split. The window
is tested against hand-written fixtures behind an `inTauri` check in about 26 modules. The
fixtures drift from the real core, and the fake ships in the release bundle. No test drives a
user journey from the journal to the page. Every UI change waits on a Windows build or a hand
check at Sumeet's PC.

## Outcome

astromech is one Tauri process. A single command layer serves the window, the control API and
a headless mode. Agents and CI exercise the real core: from a scripted game world (game files,
recorded network, a controlled clock) through the pipeline to the page in Chrome, with faults
as ordinary inputs. The window reaches its backend through one transport. The fake transport
lives apart from the app, never ships, and serves only UI behaviour tests and the static
preview link. A small automated suite of user journeys runs on every push. Worth shipping on
its own: the transport seam and the command layer, even before the single-process merge.

## Success criteria

- one-process: The installed app is one app process plus its supervised speech worker
  (`astromech-voice-worker`, which stays separate), and with the window closed no WebView2
  process is left. Shown on Windows CI. Memory is reported, not bounded (decision
  astromech-01ft): next to Elite and a browser it is small.
- one-command-list: Every command the window uses is served by the command layer and reachable
  over the control API and the headless mode. The shell holds only the window, the tray and
  startup. Shown by a test that lists the window's commands and calls each one through the
  control API of a headless core.
- journeys-in-ci: The journey suite (at least plot and follow, jump to the next stop and copy
  it, and a lookup failing with its error shown) runs in CI on every push against the real
  headless core and a scripted world, with no sleeps, no wall-clock reads and no retries.
  Shown by CI runs on the mission's last merged commit.
- no-fake-in-release: The release bundle contains no preview transport or fixtures. Shown by a
  build check that fails if a fixture string appears in `app/dist`.

## Scope

In: the transport seam and the preview's move out of the app; the command layer; the
single-process merge, with tray, updater, installer and Start with Windows; the headless mode;
the world seam (scripted game folder, HTTP record and replay, clock, fault inputs) and a dev
bridge so the browser preview can run the real core; the journey suite in CI; a Windows smoke
test of the packaged app at release; the testing rules in AGENTS.md and CONTRIBUTING.md;
moving UI tests to browser mode; ARCHITECTURE.md updated to the new model.

Out: compiling the core to WASM for the static preview (its own backlog item once the command
layer exists); Electron or any other shell; the Route redesign (astromech-ok29), which
continues alongside and uses the transport seam; new user features.

Environment: Windows 10 and 11 with WebView2, next to Elite Dangerous and VR, for the shipped
app. Linux (this host and GitHub's ubuntu runners) for the headless core, the preview and CI.

## Authority

Sumeet set the outcome and the two decisions in conversation on 6 Nov 2026, and delegated plan
approval to an independent agent review. An orchestrator may change the increment breakdown,
sequencing, crate and module names, the record-and-replay format, and which journeys the
suite holds beyond the three named above. These must come back to Sumeet: changing the outcome
or success criteria; dropping the journey suite or the single-process merge; anything that
changes what the shipped app does while playing;.
