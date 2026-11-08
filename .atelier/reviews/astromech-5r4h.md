---
schema: atelier.review
schema_version: 2
id: astromech-5r4h
title: approval on astromech-vucf @a43fb01a
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: a43fb01a8e58abe7e80d9805f29557c3ae0182c9
created_at: 2026-11-07T14:01:44.675211218Z
event:
  event: approval
---

Re-review of d2nq's fixes (83cee7e3, notes 24220b3b, dispositions a43fb01a). 4zdn fixed: WorldCore owns an empty home, sets all eight home variables, quit() asserts it empty; world_mode passes. q0bw fixed: write_config forces [eddn]/[edsm] enabled=false; a_world_never_switches_on_the_uploads passes (local set_eddn/set_edsm can still turn them on, noted). 3tli fixed: WORLD_CONFLICTS covers all 18 other Args fields, leaving only dry_run/no_voice/debug_input; world_mode_refuses_every_other_option passes. nzgh fixed: Boundaries::window; open_shell is the old Core::open_window body verbatim, over the same Paths exe_dir, ASTROMECH_SHELL still read per call; tray, second start and the open_window command all go through Core::open_window, so the installed app is unchanged; world mode records window {route}. zmth fixed: stdin piped and held, world mode quits on EOF; the kill test passed, and afterwards no astromech-world-* or astromech-test-home-* dirs and no stray processes remained. Done when 6 (no-spawn proof) holds against World::boundaries: opener, Start with Windows, notifier and now the window are recorded fakes, RunKey/SystemOpener/platform_notifier/open_shell appear only in Boundaries::real. Ran: cargo test -p astromech-core --lib (226 ok), --bins, --test world_mode (3 ok, 1 ignored); clippy -p astromech-core --all-targets -D warnings; fmt --check; check-guide. Real ~/.local/share/astromech absent. Not run: Windows CI (skt1, vucf's PR).
