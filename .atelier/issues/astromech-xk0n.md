---
created_at: "2026-11-07T11:57:01.793670822+00:00"
id: "astromech-xk0n"
issue_type: "task"
labels:
- "size-l"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-i1ev"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Recorded network: every HTTP request in world mode answered from net.json, with faults and a redacting record mode"
updated_at: "2026-11-07T17:01:39.631047129+00:00"
---

## Goal

In world mode every HTTP request the core makes is answered from the world's `net.json` (decision: world format), and none reaches the internet. Lookups can be answered with a 503, a timeout, a connection failure or garbage. A record mode captures real answers into the same format, so recordings stay raw inputs (decision astromech-1zhg). Today these modules build `ureq` agents of their own:
- the lookups' `astromech_data::Transport` (crates/astromech-data/src/transport.rs:80-131);
- `astromech_data::download` (crates/astromech-data/src/download.rs:36-49), which the neutron check and download use (crates/astromech-galaxy/src/neutron.rs:446, :527, base from `State::with_neutron_url`, crates/astromech-api/src/state.rs:397);
- EDDN's upload transport (crates/astromech-eddn/src/transport.rs:46-71, built at crates/astromech-core/src/pipeline.rs:1024);
- EDSM's `HttpApi` (crates/astromech-eddn/src/edsm/api.rs:37-72, built inline for `edsm_flight_log` at crates/astromech-core/src/app.rs:916-928);
- the updater's `http_get` and `http_download` (crates/astromech-core/src/update.rs:265-300);
- the speech-model download (crates/astromech-voice/src/download.rs:121-131).

## Done when

- [x] `astromech_data::replay` holds the recording, its matching rules and an in-process replay `Transport`, moved from the private `Recorded` in crates/astromech-data/tests/recorded.rs, which now uses it. Its tests cover each answer:
  - a 503 gives `Error::Unavailable`;
  - `timeout` gives `Error::Timeout` at once;
  - `offline` gives `Error::Offline`;
  - a non-JSON body gives `Error::Schema`;
  - answers in turn give 503 and then the result;
  - an unmatched request is refused and reported as unrecorded.
- [x] World mode answers every module above from the one recording:
  - in-process: the remote-data client, EDDN's transport, and EDSM's `Api` (its trait, api.rs:29);
  - a replay server on 127.0.0.1: the update feed and zip, `astromech_data::download` and the speech models, with base URLs rewritten by the composition root. The rewrite uses `[update] feed`, `State::with_neutron_url`, and new overrides for the model catalog's base (crates/astromech-voice/src/catalog.rs:5) and the update zip (update.rs:229).
  Every request lands in `world_observed`, unrecorded ones marked. A lookup must not wait on the world clock, so world mode's client has no request spacing (`Limiter::none()`) and Spansh polling every 0 (`NO_WAIT`, app.rs:1735).
- [x] A world test, spawning the binary, runs a lookup the window makes (`galaxy_systems` or similar) over the control API. It is answered first from a recorded Spansh body, then, from a `net` step on, with the handler's own error for a recorded 503. It observes no unrecorded request.
- [x] Record mode: `--record-net <file>`, with `--world`, records the remote-data client's and EDSM `Api`'s requests and their real answers into `net.json` form. Values of `apiKey` and `commanderName` (api.rs:140), and any form or query field named like a key, are replaced by `<redacted>`, and replay matching ignores those fields. A test against a local server records a flight-log request with key `k3y` and commander `Jameson`: the file contains neither string, and it replays to the same reply. Downloads (update, models, neutron) are not recorded; their worlds use statuses and faults.
- [x] `clippy.toml` disallows `ureq::AgentBuilder::new`, `ureq::agent`, `ureq::builder`, `ureq::get`, `ureq::post`, `ureq::put`, `ureq::delete` and `ureq::request`. The six modules above carry `#[allow]` with "HTTP stack; world mode rewrites or replaces it", so a new HTTP path fails clippy.

## Scope

The replay library, the replay server, endpoint overrides passed through `Boundaries`, record mode and the lint. The installed app's URLs and timeouts are unchanged. This child also fills in the `net` step kind the script child reserved.

## Context

ro7a's 503 server (crates/astromech-core/src/app/every_window_command.rs, `unavailable_server`) is the replay server's precursor, and the every child retires it. A timeout is in-process only: the server path can't make a client time out without a real wait. Recorded bodies already in the repository are in crates/astromech-data/tests/fixtures.

## Notes

- 2026-11-07 [decision] EDDN and EDSM come back on in world mode: the world's config decides (default off), and their uploads go to the recording in process
  Reason: With the replay none of their requests reach the internet, and gllh's set_eddn/set_edsm need the real switches. Caveat: the pipeline stamps EDDN/EDSM lines with wall_now() (pipeline.rs:361, :364, :404, :430, :973), so in a world EDDN treats the world's lines as too old and posts nothing until astromech-zgoo moves those reads to the clock
  Rejected: keep forcing them off (d2nq's rule): leaves world_mode testing nothing about them
- 2026-11-07 [decision] One Http boundary (eddn and edsm factories, plus downloads: a full-URL rewrite) instead of per-download base overrides
  Reason: Real manifests carry an absolute url (pack-update.py), so a prefix override at update.rs:229 alone would leak; rewriting each full URL at its call site (feed, zip, each model asset, the neutron base passed through with_neutron_url) covers base and absolute URLs alike, and Http::real() is the identity with today's agents
  Rejected: an override per base: catalog BASE, update zip prefix
- 2026-11-07 [decision] Record mode records the remote-data client and EDSM's GETs only; EDSM POSTs (the journal upload) and EDDN stay on the replay; the recording client runs on SystemClock with the default limiter and polling
  Reason: Recording must never publish a world's journal as a commander; reads are safe. The manual clock would hang the limiter and NO_WAIT would hammer Spansh's job polling
- 2026-11-07 [decision] Matching: url is a substring of the full URL including the query (keys and values percent-encoded like path segments, secrets <redacted>); optional sent is a substring of the body; an exhausted in_turn stops matching; a repeated request in record mode is written as in_turn
  Reason: One rule for in-process and server requests; record mode's exact URLs then replay to themselves and distinct POST bodies stay distinct
- 2026-11-07 [decision] Over the replay server, timeout and offline both close the connection unanswered, and so does an unrecorded request
  Reason: A real client timeout needs a real wait (issue Context); the in-process path gives a real Timeout at once
- 2026-11-07 [decision] trade_route polls with State::plot_polling (default unchanged) so world mode's NO_WAIT covers it
  Reason: It used Polling::default() directly and would sleep on the world's clock

## Report

**Outcome.** Every HTTP request the core makes in world mode is answered from the world's `net.json`, and none reaches the internet. Code commit 16a82b2b on `astromech-vucf`. Not pushed. All five Done when lines are ticked, with two deviations for the orchestrator to judge (below).

Where things are:
- crates/astromech-data/src/replay.rs: the format, the matching rules and redaction (in its doc comment), `Replay` (the in-process `Transport` and `answer`), `Server`, `Tape` and `Recorder`.
- crates/astromech-core/src/world/net.rs: the world's `Net` (the remote-data client, the EDDN and EDSM adapters, record mode).
- `app::Http`: the new boundary (`eddn`, `edsm`, `downloads`), with `Http::real()` for the installed app.
- the `net` step: world/script.rs and world/steps.rs.
- worlds/network, crates/astromech-core/tests/world_net.rs, and the `ureq` list in clippy.toml.

**Installed app.**
- `Http::real()` builds exactly today's agents: EDDN's `HttpTransport::new(UPLOAD_URL, "astromech/<v>")`, and EDSM's `HttpApi::with_timeout(ua, REQUEST_TIMEOUT)` for the uploader and `FLIGHT_LOG_TIMEOUT` for the flight log. `downloads` is the identity.
- So the URLs are unchanged:
  - the feed is `feed_url(settings)`;
  - the zip is `zip_url(m)`;
  - each model is `a.url()`;
  - the neutron base is `RELEASE_URL`.
- The data client's `HttpTransport`, the limiter and the timeouts are untouched.
- `trade_route` now reads `State::plot_polling`, whose default is `Polling::default()`, the value it used before.
- Verified by inspection only: no live installed-app run was made.

**Proof** (Linux, after the last code change):
1. **The replay library.** recorded.rs's `Recorded` is now a thin logger over `replay::Replay`, and all 22 of its earlier tests pass on it. New tests, through `Client` on a manual clock that never moves:
   - `each_recorded_fault_is_the_lookups_own_error` (a table): 503 gives `Unavailable{503}`; `timeout` gives `Timeout`, at once (the clock unchanged, one request); `offline` gives `Offline`; `garbage` (a non-JSON body) gives `Schema`.
   - `answers_in_turn_give_a_503_and_then_the_recorded_result`.
   - `a_request_the_recording_doesnt_hold_is_refused_and_reported` (Offline, "has no answer for …", observed `unrecorded`).
   - Unit tests in replay.rs: last match wins, an exhausted `in_turn` falls through, the `sent` matcher, redaction, load errors, `from` bodies, the server, and the tape.
   - Mutations, each failing the named tests (`--no-fail-fast`):
     - timeout→offline: `each_recorded_fault…`, `no_network…`;
     - offline→timeout: `each_recorded_fault…`, `offline_still_serves…`;
     - garbage→offline: `each_recorded_fault…`, the server test;
     - status forced to 200: `answers_in_turn…` and four more;
     - in_turn never advancing: `answers_in_turn…` and the polling tests, and `spansh_polls_on_the_clock` hung, so I killed it;
     - unrecorded answered as an empty 200: the refusal test and four unit tests;
     - unrecorded not reported: the refusal test and the server test;
     - the server answering a refused request with a 404: the server test.
2. **Every module from the one recording.**
   - `world::tests::every_http_path_is_answered_from_the_one_recording`, over world mode's `Boundaries`: the remote client, `(http.eddn)()`, `(http.edsm)(t)`, and through the server the neutron checksum (`download::text`), the update feed (`update::http_get`) and the STT model (`models::download_from`). Each is answered and observed with method and URL, and an unrecorded one is marked.
   - Through the binary: `downloads_and_the_flight_log_are_answered_from_the_recording`, where `check_update`, `download_models`, `neutron_check` and `edsm_flight_log` are answered with the recorded 503s and none is unrecorded.
   - Mutations: with `downloads` set to the identity, both tests fail (that mutant also sent its GETs to the real public GitHub URLs until the 30 s wait gave up); with the `net` step ignored, the lookup world test fails.
   - World mode uses `Limiter::none()` and `assemble_with(.., plot_polling(NO_WAIT))`, with every 0.
3. **The world test.** `world_net::a_window_lookup_is_answered_from_the_recording_then_from_a_net_step` runs over `worlds/network`, whose `net.json` points at the recorded fixture `spansh-system-names.json`.
   - `galaxy_systems{Shin}` gives the four names.
   - Then `world_step` to `spansh-down`, and `galaxy_systems{Sol}` gives "Spansh is having trouble (HTTP 503). Try again later."
   - The two observations are both `unrecorded: false`.
4. **Record mode.**
   - `world::net::tests::a_recorded_flight_log_holds_no_key_or_commander_and_replays_the_same`: `flight_log` runs through the record-mode EDSM API against a local stand-in, which is really asked with `commanderName=Jameson&apiKey=k3y`. The file holds neither string, and its URL is `…get-logs?commanderName=<redacted>&apiKey=<redacted>`. Replayed from that file with another commander and key, it gives the same visits and the stand-in is asked nothing more. The observations hold neither string either.
   - Mutation with `secret()` always false: that test and two replay unit tests fail.
   - Clap: `--record-net` needs `--world` (main.rs test).
   - Live run: `astromech-core --world worlds/minimal --record-net <scratch>` and one `galaxy_systems{Shinr}` made one public, credential-free Spansh GET. The answer was written verbatim as `json`, observed, exit 0, and nothing was left in the home. The scratch file was deleted, not committed.
5. **The lint.** clippy.toml lists the eight named paths plus `ureq::Agent::new`, `ureq::request_url`, `ureq::head` and `ureq::patch`. The `#[allow(clippy::disallowed_methods, reason = "HTTP stack; world mode rewrites or replaces it")]` sits on the agent-building function in each of the six modules: data `HttpTransport::new` and `download::agent`, eddn `HttpTransport::new`, edsm `HttpApi::with_timeout`, update `http_get` and `http_download`, and voice `download::agent`. Planted `ureq::AgentBuilder::new()`, `ureq::agent()`, `ureq::Agent::new()` and `ureq::get()` in astromech-core's lookups.rs: clippy gave 4 errors, one for each. Reverted.

**Checks.**
- `cargo test --workspace --no-fail-fast`: exit 0, 137 binaries, 1364 passed, 0 failed. The baseline was 578 in the four crates I touched.
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: clean.
- `cargo fmt --all --check`, `scripts/check-deps.py`, `scripts/check-guide.py` and `cargo deny check`: pass.
- Loops: world_net, world_mode, world_steps and the core lib's `world::` tests together ran 50 times from the built binaries, 50 passed and no temp worlds were left. The data `recorded` and `replay` tests also ran 50 times, 0 failures. The real HOME has no astromech folders.

### Decisions
Six decision notes are above:
- EDDN and EDSM come back on;
- one `Http` boundary with a full-URL rewrite;
- what record mode records;
- the matching rules;
- the server's faults;
- `trade_route`'s polling.

### Deviations
- **Done when 2.** The rewrite covers each full URL, not just a base. The feed is rewritten where `check_update` reads `[update] feed`; the zip is `downloads(zip_url(m))`; the models are `downloads(a.url())` through the new `models::download_from`; the neutron base still goes through `with_neutron_url`. Real manifests carry an absolute `url`, so an override of update.rs:229's prefix alone would have reached GitHub.
- **Done when 4.** Record mode records EDSM's reads (GETs: the flight log and the discard list), not its POSTs. The journal upload stays on the replay so recording never publishes a world's journal as the commander. The orchestrator should confirm this reading.
- **The lint list** is a superset of the issue's eight paths.
- **The `http` observation** is now `{method, url, unrecorded}`. d2nq's `path` field is gone: the URL holds the query, redacted.
- **d2nq's world_mode test** `a_world_never_switches_on_the_uploads` is now `a_worlds_sharing_switches_are_its_own`.
- **`World::load`** takes the record path.

### Surprises
- In a world, EDDN and EDSM judge the world's journal lines by `wall_now()` (pipeline.rs:361, :364, :404, :430, :973), so EDDN treats them as replays and posts nothing. An end-to-end EDDN world test therefore isn't possible until astromech-zgoo moves those reads to the clock. EDDN's transport is proven at the boundary (Done when 2).

### Open items
- The issue couldn't be started or closed in the tracker: `start` is blocked while astromech-i1ev is in progress. Its Done when lines are ticked here; the orchestrator transitions it after i1ev closes.
- Windows: none of this has run there. The replay server binds 127.0.0.1 with std only.
- For astromech-zgoo: the five `wall_now()` sites above.
- For astromech-gllh: real manifests are signed, so `check_update` "against the recorded feed" needs a recording of the real latest.json and .sig. ro7a's `unavailable_server` and `offline()` are still there for gllh to retire.
- Independent review is outstanding.
