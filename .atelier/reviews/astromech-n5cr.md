---
schema: atelier.review
schema_version: 2
id: astromech-n5cr
title: finding on astromech-vucf @8dc8a026
target: astromech-vucf
identity: claude-reviewer-gllh
commit: 8dc8a02695e9bb1249c39aa371c6c98078d7d592
created_at: 2026-11-07T19:19:06.001297008Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/tests/every_window_command.rs:142-288 (gllh) About 85 of the 102 window commands are checked only for an ok envelope and reply (Want::Ok, no sees or check): the setters (set_folders, set_voice, set_edsm, set_callout, set_mic, set_eddn, set_search_filters), the binds_* edits, the route_* edits and every lookup. A handler emptied to Done::empty(), or a lookup that answers ok without making its recorded request, still passes; nothing checks that each net.json exchange was used. This is the second half of finding nq3c (read back an effect), which gllh closed by the round trip, and it meets gllh's Done when and criterion one-command-list (each command reachable and called), since Done when 2 names the commands whose effects are checked. Suggested follow-up: read back one effect per setter (callouts after set_callout, voice_settings after set_voice, the config after set_folders, binds_view after binds_set_slot, route after route_set/route_add) and assert at the end that every net.json exchange was requested at least once.
