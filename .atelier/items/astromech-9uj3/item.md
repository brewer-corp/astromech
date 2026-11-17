---
schema: atelier.item
schema_version: 3
id: astromech-9uj3
title: CI on main has failed on every push since 8 Nov, and nothing stops a merge
status: ready
mission: null
labels:
- bug
- size-m
priority: critical
blocked_by: []
implementers: []
ready:
  by: the lead of astromech-xajc
  at: 2026-11-17T13:18:23.923857946Z
trivial: null
caused_by: null
created_at: 2026-11-17T13:18:23.854260250Z
---

## Goal

Both CI workflows pass on main, so a red run means something. Found by the lead of astromech-xajc on 19 Nov 2026 while checking whether astromech-7o1g's `host.rs` change, which was never compiled locally, built on Windows:

- `ci.yml` last passed on main at 9402d4b9 on 8 Nov 2026 (`gh run list --workflow ci.yml --branch main --status success`). Every completed run since has failed, about 40 mission merges among them. On 9170f0a1 both jobs fail in their `tests` step: "Linux (fmt, clippy, tests, deny, deps, frontend)" and "Windows (MSVC build + tests, the app with its window)". The Windows log shows "world mode: couldn't remove ...astromech-world-...: The process cannot access the file because it is being used by another process"; whether that is the failure or noise is not known.
- `atelier.yml` last passed at dce420ca on 12 Nov 2026. Since then `atelier check` in CI reports, for every item of the mission, "mission: `astromech-xajc` names no mission", while `atelier check` with the installed Atelier 0.7.0 says every record is valid. The workflow's Atelier is probably older than the records' layout.
- astromech-ov96 (cargo-deny fails on RUSTSEC-2026-0459) is a known part of it and stays its own item unless it is the only thing left.

Nobody saw this because merges run no checks (astromech-r198) and no agent looks at CI after a push.

## Done when

- [ ] The Report names the first failing run of each workflow and the commit or change that caused it, one cause per failure, from the logs.
- [ ] Each cause is fixed at the cause: a failing test is fixed or the product bug behind it is; a workflow that runs the wrong Atelier runs the version the records need. No test is skipped or retried to get a green run; a real quarantine names its item, as CONTRIBUTING.md's Tests says.
- [ ] `ci.yml` and `atelier.yml` both pass on main on the commit this item merges, and on the next push after it. Where a fix needs a push to see CI's answer, the item says how many pushes it took.
- [ ] astromech-7o1g's change to `app/src-tauri/src/host.rs` (the window's minimum size, with screen fitting that was never compiled before it merged) builds on the Windows job; if it does not, it is fixed here.
- [ ] `caused_by` is set, or the body says why there is more than one cause.

## Scope

The two workflows and whatever they are right to fail on. Making merges run the checks is astromech-r198. Heavy builds do not run on this machine: use CI's own runs, or boat.
