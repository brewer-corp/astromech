---
schema: atelier.review
schema_version: 2
id: astromech-7si9
title: approval on astromech-vucf @d0ed51dc
target: astromech-vucf
identity: claude-reviewer-gllh
commit: d0ed51dc5b8ef84d6e19ea477981f72110dae4d5
created_at: 2026-11-07T19:49:40.536127005Z
event:
  event: approval
---

gllh fixes approved through d0ed51dc. 6iev: set_voice now sets bm_george, 1.25 and harsh (all differ from Config::default's bf_emma, 1.0 and cockpit; harsh is a real Colour) and reads all three back. The first status entry pins the world's start (ptt and kill_switch null, mic empty, onboarded false, voice_paused and autostart false, voice bf_emma/1.0/cockpit, EDSM commander Test), and the other setters' start values are already asserted (scan enabled, threshold 20, exclude_carriers false), so no read-back can pass unchanged. n5cr (2d3ab0e7): the read-backs are real, the four exceptions are justified in the gllh note, and spansh-material-traders.json is an exact copy of spansh-services.json's material_trader page. Reviewer reran: every_window_command 12/12 six at a time; cargo test -p astromech-core -p astromech-data pass (at 2d3ab0e7); clippy -D warnings and fmt clean. The author's mutations were not rerun (code is read-only here). Windows CI is still pending on gllh's Done when 6.
