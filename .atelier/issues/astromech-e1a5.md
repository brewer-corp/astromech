---
created_at: "2026-11-07T11:57:01.794495098+00:00"
id: "astromech-e1a5"
issue_type: "decision"
labels: []
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-kjfs"
    type: "advances"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-07T11:57:14.958244574+00:00"
status: "done"
title: "World format: a directory with a JSONL step timeline, seed files and a network recording, settled by generation and clock quiescence"
updated_at: "2026-11-07T11:57:14.958244574+00:00"
---

## Question

astromech-vucf runs the real core against a scripted world, and astromech-yis2's journeys, the dev bridge and the preview-fixture dumps all read worlds committed to the repository. What is a world on disk, how does a test drive it, and how does it decide that the core has caught up with a step? This is a persisted format other increments build on, so it is settled before vucf's children start. The strategy's Authority section gives the record-and-replay format to the orchestrator.

## Options

- **Reuse astromech-mockgame's `mock.json`** (crates/astromech-mockgame/src/main.rs:12-20). It is a config of key rules (scancode to a Status flag, GuiFocus or vehicle swap) with wall-clock delays, for a Windows-only window. It has no timeline, no journal, no network and no clock.
- **Reuse each replay test's format as it is.** Input replay is a JSON file of `DeviceMeta` and raw `Sample`s (crates/astromech-input/src/listen/replay.rs:10-20). The data tests' recorded network is a Rust struct matching URL substrings, with answers in turn and faults (crates/astromech-data/tests/recorded.rs:21-70). The reaction-engine tests write synthetic journal lines in code (crates/astromech-announce/tests/replay.rs:1-3). Each covers one boundary, two of them only as Rust code, and they share no timeline.
- **Worlds as Rust code** (a builder DSL). This suits Rust tests, but the dev bridge, `pnpm dev` and a Playwright journey can't load a world without compiling it.
- **One world directory that embeds the existing formats.** A JSONL timeline of steps, plus seed files and a network recording. Each boundary keeps the format it already has: journal lines and companion files verbatim, input as listen::model `Sample`s and `DeviceMeta`, and network exchanges using recorded.rs's matching rules, moved into a library. This is close to the `astromech-rec` JSONL that docs/ARCHITECTURE.md §13 (Ingest row) planned.

## Decision

The fourth option. Decided by the orchestrator under docs/strategies/one-tested-process.md, Authority ("the record-and-replay format").

**Layout.** A world is a directory `worlds/<name>/`:
- `astromech.toml`: the config. World mode points every folder at its own temp copy.
- `game/`: the journal folder's seed.
- `bindings/`: `.binds` files.
- `net.json`: the network recording.
- `world.jsonl`: the steps.
- Payloads (bodies, WAVs, files), referenced by relative path. Recorded bodies may point at existing fixtures such as crates/astromech-data/tests/fixtures.

**Steps.** `world.jsonl` holds one JSON object per line. The first line is a header: the start time, devices, key reactions, and the foreground window. Every later line is a step, one of these kinds:
- `journal` (append a line, or start a new file);
- `file` (a companion file, optionally a partial write);
- `game` (running, launcher or exited);
- `advance`;
- `focus`;
- `press` and `release` (listen::model `Sample`s);
- `audio` and `say`;
- `net`;
- `mark`.

**Time.** The world clock is a manual `astromech_proto::Clock`. It moves only on `advance`, or forward to a journal line's own timestamp when that line is appended. Recorded journal lines keep their timestamps and replay as live. The manual clock owns every timed wait in the core: sleeps, condvar waits and channel waits with a timeout. The long-lived timer threads register with it. Short pacing waits toward a device, a file or a process stay on real time and decide nothing (listed in the clock child).

**Clock waits.** The manual clock wakes its waits itself. Channel waits use a channel the clock owns, condvar waits hand the clock an `Arc<Condvar>`, and untimed clock waits count as parked. Registered threads block only in clock waits. A thread inherits its registration from its parent before it is spawned. Whatever stops a clock-waiting thread wakes that wait.

**Settling.** Nothing runs on its own. A test or driver calls these commands, which are `local`, registered only in world mode, and refused for the window:
- `world_step` runs steps up to a mark. It returns once the core has consumed them:
  - a file step, once the pipeline has handled that write's generation (a counter the world bumps and the pipeline echoes after handling the file, whether or not it parsed); a file the pipeline doesn't watch settles at once;
  - an `advance` step, once every registered thread is parked in a clock wait again;
  - a later-stamped journal line, by advancing the clock to its stamp and settling before it is appended;
  - `game running` or `launcher`, on the session-started-or-failed event;
  - `game exited`, once the session is dropped and the clock is quiescent.
  Threads that aren't registered (speech-to-text, command handler threads, the verify wait under `voice_test_live`) can still be running after `advance` or `world_step` returns. Their effects are awaited with `world_wait`, never assumed once `world_step` has returned.
- `world_wait` blocks until an observation matches.
- `world_observed` lists what reached the world: lines said, clipboard writes, keys sent, files opened, notifications, Start with Windows, HTTP requests, and requests the recording did not hold.

**Network.** `net.json` exchanges are matched on a URL substring and optionally the method. The last matching rule wins, as in recorded.rs. An answer is a status with a body (inline or a file), a list answered in turn, or a fault: `timeout`, `offline`, or garbage. A request no rule matches is refused and observed as unrecorded.

**Running.** World mode is the `--world <dir>` flag of the core's one executable (`astromech-core` until astromech-y6d6 merges the processes). It ships in the release binary with no cargo feature: its fakes write only under the world's temp directory and reach no real device, program or service. The one exception is `--record-net <file>`, which deliberately sends lookups to the real services and writes the answers, with keys and commander names redacted, to the named file. It also ships in the release binary, since it runs only when both flags are given. Gating either behind a feature would split `cargo test --workspace` from the release build for no current need.

**Tests.** World tests spawn the built binary rather than starting a core in the test process, because the core keeps process globals (the controls bus, EDDN_SHARING, the EDSM switch, the voice tester, VOICE_PAUSED, VOICE_TURN).

**Consequences.** astromech-yis2's journeys are world directories plus a page script. A new boundary fake adds a step kind here and updates the format reference in the world module's doc comment.

## Report

A world is a directory (`astromech.toml`, `game/`, `bindings/`, `net.json`, `world.jsonl`, and payloads) whose JSONL steps embed the existing boundary formats. A manual clock owns every timed wait in the core and settles `advance` by quiescence of its registered threads. File steps settle by a generation the pipeline echoes, `game` steps by the session's own events, and unregistered threads' effects are awaited with `world_wait`. `--world` and `--record-net` ship in the release binary. Decided by the orchestrator under docs/strategies/one-tested-process.md, Authority ("the record-and-replay format").
