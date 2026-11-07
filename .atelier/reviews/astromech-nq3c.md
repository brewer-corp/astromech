---
schema: atelier.review
schema_version: 2
id: astromech-nq3c
title: finding on astromech-bp23 @36531f8c
target: astromech-bp23
identity: claude-reviewer-ro7a
commit: 36531f8c99c5623824057f12726be11a1e8f58ce
created_at: 2026-11-07T11:16:06.649805620Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/app/every_window_command.rs:177-185,222,301 The example args are hand-written JSON that is never checked against the arg types, and no arg struct denies unknown fields. For all-optional arg structs (set_folders' Folders, set_voice, set_edsm, set_callout's enabled/cooldown_s, binds_set_override's joy, set_kill_switch null), a renamed or misspelt field parses to None and the call answers ok as a no-op. Want::Ok also reads only the envelope, never the reply, so a handler emptied to Done::empty() still passes. That meets ro7a's Done when (ok or the handler's own error) and the window's args are typed by ts-rs, so the window itself can't send the wrong names. But the table's 'example args' prove less than they look like, and vucf reuses them. Fix in vucf (defer there): for each setter, read back one effect (callouts after set_callout, voice_settings after set_voice, find_elite or the config after set_folders), or round-trip each example through its arg type and compare.
