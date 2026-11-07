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
updated_at: "2026-11-07T11:57:01.793670822+00:00"
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

- [ ] `astromech_data::replay` holds the recording, its matching rules and an in-process replay `Transport`, moved from the private `Recorded` in crates/astromech-data/tests/recorded.rs, which now uses it. Its tests cover each answer:
  - a 503 gives `Error::Unavailable`;
  - `timeout` gives `Error::Timeout` at once;
  - `offline` gives `Error::Offline`;
  - a non-JSON body gives `Error::Schema`;
  - answers in turn give 503 and then the result;
  - an unmatched request is refused and reported as unrecorded.
- [ ] World mode answers every module above from the one recording:
  - in-process: the remote-data client, EDDN's transport, and EDSM's `Api` (its trait, api.rs:29);
  - a replay server on 127.0.0.1: the update feed and zip, `astromech_data::download` and the speech models, with base URLs rewritten by the composition root. The rewrite uses `[update] feed`, `State::with_neutron_url`, and new overrides for the model catalog's base (crates/astromech-voice/src/catalog.rs:5) and the update zip (update.rs:229).
  Every request lands in `world_observed`, unrecorded ones marked. A lookup must not wait on the world clock, so world mode's client has no request spacing (`Limiter::none()`) and Spansh polling every 0 (`NO_WAIT`, app.rs:1735).
- [ ] A world test, spawning the binary, runs a lookup the window makes (`galaxy_systems` or similar) over the control API. It is answered first from a recorded Spansh body, then, from a `net` step on, with the handler's own error for a recorded 503. It observes no unrecorded request.
- [ ] Record mode: `--record-net <file>`, with `--world`, records the remote-data client's and EDSM `Api`'s requests and their real answers into `net.json` form. Values of `apiKey` and `commanderName` (api.rs:140), and any form or query field named like a key, are replaced by `<redacted>`, and replay matching ignores those fields. A test against a local server records a flight-log request with key `k3y` and commander `Jameson`: the file contains neither string, and it replays to the same reply. Downloads (update, models, neutron) are not recorded; their worlds use statuses and faults.
- [ ] `clippy.toml` disallows `ureq::AgentBuilder::new`, `ureq::agent`, `ureq::builder`, `ureq::get`, `ureq::post`, `ureq::put`, `ureq::delete` and `ureq::request`. The six modules above carry `#[allow]` with "HTTP stack; world mode rewrites or replaces it", so a new HTTP path fails clippy.

## Scope

The replay library, the replay server, endpoint overrides passed through `Boundaries`, record mode and the lint. The installed app's URLs and timeouts are unchanged. This child also fills in the `net` step kind the script child reserved.

## Context

ro7a's 503 server (crates/astromech-core/src/app/every_window_command.rs, `unavailable_server`) is the replay server's precursor, and the every child retires it. A timeout is in-process only: the server path can't make a client time out without a real wait. Recorded bodies already in the repository are in crates/astromech-data/tests/fixtures.
