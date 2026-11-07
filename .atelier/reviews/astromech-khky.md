---
schema: atelier.review
schema_version: 2
id: astromech-khky
title: approval on astromech-vucf @8dc8a026
target: astromech-vucf
identity: claude-reviewer-gllh
commit: 8dc8a02695e9bb1249c39aa371c6c98078d7d592
created_at: 2026-11-07T19:19:17.307375942Z
event:
  event: approval
---

gllh (699a6b5d..8dc8a026) approved: the every-window-command test now spawns astromech-core --world worlds/every-command and calls all 102 window commands with call_window, failing for a missing, non-window or duplicate entry; Done when 2's effects (signed feed verified then zip 503, models 503 logged and still missing, autostart on/off, open_config edit, diagnostics reveal, mic clip levels, voice_preview said, EDSM 203 with the request redacted, live-run keys, exobiology from the session) are awaited in world_observed or checked in the reply; NOT_RUN_FOR_REAL is install_update only; example args round-trip through their types. With ro7a and 96dv this meets one-command-list. Reviewer reran: the test 15/15 in a row and 24/24 six at a time (0.35-0.9 s); strace: 2 execve (test, astromech-core --world), every connect to 127.0.0.1; cargo test -p astromech-core -p astromech-api -p astromech-data all pass, bindings test passes with no app/ change; clippy -D warnings on the touched crates and fmt clean. Recordings: placeholder commander Test/Jameson, key not-a-real-key, EDSM URL redacted, public latest.json kept CRLF (-text, i/crlf). Production parsing unchanged (Serialize derives only; Spansh cap only at every=0, never in Polling::default). Not rerun: the author's code mutations (code read-only here; read the paths they exercise) and Windows CI, whose Done when line stays unticked and gates the merge. One defer finding: n5cr.
