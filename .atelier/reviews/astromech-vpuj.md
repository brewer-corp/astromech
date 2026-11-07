---
schema: atelier.review
schema_version: 2
id: astromech-vpuj
title: approval on astromech-vucf @1417fc9a
target: astromech-vucf
identity: claude-reviewer-jkgv
commit: 1417fc9ad29309ae8c55fb48976c421b872332fb
created_at: 2026-11-07T22:44:45.482874199Z
event:
  event: approval
---

Review of the CI fixes 284015ba and 1417fc9a. bridge-check now listens on 127.0.0.1 with strictPort false and calls the port it got, which matches the bridge's loopback Host check. world_root() strips the config's last two path parts with either separator, and on_a_pc and in_the_world map the core's separator; the new Windows-style root test passes. One world core at a time keeps each world's parts in table order, as before, and quits each core. preview_fixtures 3/3 passed, and four parallel runs passed (inotify max_user_instances 128). bridge-check passed locally, with no world core left. PR #222 CI at 1417fc9a passed every check: Linux, WebKitGTK, WebView2, Windows MSVC build and tests, the installer, MVP vs mock and atelier doctor. scrub_strings in astromech-data replay.rs is live in production: Tape::record calls it, and that path is the Recorder transport world/net.rs uses for --record-net. A fresh-target build has no unused warning and 7/7 replay tests pass, and an external probe binary calling Tape::record redacted a JSON-escaped commander and key. The 'never used' warning, and a failing a_tape_redacts_secrets_an_answer_echoes_json_escaped, came from a stale astromech-data build in the shared plan-kjfs target, which cargo didn't rebuild. Running cargo clean -p astromech-data there fixes it.
