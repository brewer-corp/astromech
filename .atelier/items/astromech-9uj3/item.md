---
schema: atelier.item
schema_version: 3
id: astromech-9uj3
title: CI on main has failed on every push since 8 Nov, and nothing stops a merge
status: in_review
mission: null
labels:
- bug
- size-m
priority: critical
blocked_by: []
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3Add68300b-007f-4c7a-b70c-754733a70bea
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

- [x] The Report names the first failing run of each workflow and the commit or change that caused it, one cause per failure, from the logs.
- [x] Each cause is fixed at the cause: a failing test is fixed or the product bug behind it is; a workflow that runs the wrong Atelier runs the version the records need. No test is skipped or retried to get a green run; a real quarantine names its item, as CONTRIBUTING.md's Tests says.
- [x] `ci.yml` and `atelier.yml` both pass on main on the commit this item merges, and on the next push after it. Where a fix needs a push to see CI's answer, the item says how many pushes it took.
- [x] astromech-7o1g's change to `app/src-tauri/src/host.rs` (the window's minimum size, with screen fitting that was never compiled before it merged) builds on the Windows job; if it does not, it is fixed here.
- [x] `caused_by` is set, or the body says why there is more than one cause.

## Scope

The two workflows and whatever they are right to fail on. Making merges run the checks is astromech-r198. Heavy builds do not run on this machine: use CI's own runs, or boat.

## Report

Both workflows pass on this branch after one push: ci.yml run 44093067446 (both jobs, including the Windows Tauri build and its clippy) and atelier.yml run 44093072409. There were four independent causes, so `caused_by` stays empty.

| Workflow | First failing run | Cause | Fix |
|---|---|---|---|
| atelier.yml | 44027233143 on e4a16611 | The workflow installed Atelier at 7f499285, older than the 0.7.0 that writes the records. It rejects `status: open` in astromech-xajc's mission.md ("unknown variant `open`"), and "names no mission" on every item of the mission follows from that. | 36f515f7: `ATELIER_REV` is the v0.7.0 tag's commit, 24a00e83. |
| ci.yml, Linux cargo-deny | 43196334048 on 010b068c (12 Nov) | RustSec published RUSTSEC-2026-0459 against the SQLite that libsqlite3-sys 0.38.2 bundles. No commit caused it. | d5276fdd: rusqlite 0.43.0, libsqlite3-sys 0.40.0. |
| ci.yml, Windows frontend build | 43200713843 on d33b34a2. The first run that contained it, 2f5e2a80, was cancelled. | f82b686b (astromech-mwh5) added `listDetail.svelte.ts` beside `ListDetail.svelte`. Windows ignores case, so `import "./listDetail.svelte"` opened the component itself: "provideDetail is not exported". Linux added `.ts` and built. | acc6b3a2: the module is `detail.svelte.ts`. The new lint rule `imports/no-case-clash` (app/eslint-rules/imports.js, with a test) fails such an import on Linux. |
| ci.yml, tests on both jobs | 44083188995 on 9f2a0cec | `every_preview_fixture_is_the_cores_reply_or_says_why_not` found three `gravity_g` values in exploration.json one bit off. jsonschema turns on serde_json's `float_roundtrip`, so `cargo test --workspace` (CI) parses floats exactly, while `cargo test -p astromech-app` (the refresh command the test prints, and the app's own build) did not. 4c8824b0 refreshed the fixture under `-p`. | 6bba0777: the workspace asks for `float_roundtrip` itself, and the fixture holds the exact values. The shipped app now parses JSON floats exactly too. |

The Windows "world mode: couldn't remove ...astromech-world-..." lines are noise. They appear in run 43200713843 too, where the Windows tests step passed.

The red cargo-deny step hid three more advisories:
- RUSTSEC-2026-0548 (zip's `ZipStreamReader`): fixed by moving to zip 9.6.1. astromech-core reads only through `ZipArchive`.
- RUSTSEC-2026-0543 and -0555 (chrono and chrono-tz unmaintained): no patched version and no known vulnerability. `deny.toml` ignores them, naming astromech-y374, which replaces chrono (filed and merged to main as a draft; it needs readying). This is a stopgap.

astromech-ov96's Done when is met here: libsqlite3-sys 0.40.0, and `cargo deny check`, `cargo test --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` pass. Its "no new ignore" holds for SQLite, but the chrono ignore is new, so the lead decides whether to close it.

Local checks (Linux): fmt, clippy, `cargo test --workspace` (143 test binaries), cargo deny, pnpm lint, check, journeys tsc, build, check-dist, pnpm test (916 tests), bridge-check and the 7 journeys all pass.

Still to do after review: CI on main for this item's merge commit and for the next push. ci.yml cancels a run when a newer push lands, so main must stay quiet for those two runs. The implementer records both run ids here after `atelier merge`.

