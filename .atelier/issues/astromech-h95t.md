---
created_at: "2026-11-08T15:52:46.472286357+00:00"
id: "astromech-h95t"
issue_type: "task"
labels: []
workflow_branch:
  branch_base: main
  work_branch: astromech-h95t
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Release notes for v1.0.0-alpha.5, the first one-app release"
updated_at: "2026-11-08T15:52:51.454075353+00:00"
---

## Goal

Players updating from v1.0.0-alpha.4 learn that astromech is now one app, what else changed, and what still needs checking in the game.

## Done when

- [x] `docs/releases/v1.0.0-alpha.5.md` follows alpha.4's layout (intro, Installing, What's new since alpha.4, What has not been checked in the game, Known limits) and covers every user-visible change on main since the v1.0.0-alpha.4 tag.
- [x] README's release-notes link points at it.
- [x] `python3 scripts/check-release-notes.py docs/releases/v1.0.0-alpha.5.md` passes.

## Report

`docs/releases/v1.0.0-alpha.5.md` follows alpha.4's layout. What's new leads with the one app (astromech-y6d6's Report, V1-DESIGN §7.1 to §7.7): one `astromech-core.exe`, the window freeing WebView2 when closed (7 MB windowless on CI), tray-only Start with Windows, the `astromech-shell.exe` forwarder, both update routes from alpha.4, and no crash restart (decision astromech-lkwj). It also covers the window-crash recovery (astromech-v3u4), caught command panics (astromech-96ci), the Status.json re-read (107e5c5d, f47c529b), the idle neutron-file drop (astromech-k6da) and one line on the journeys (astromech-yis2).

The test script carries alpha.4's sessions forward; no listed issue had closed. Added astromech-k5s5 (a step before Session 1), astromech-3fpi and astromech-278z (features not in this alpha), and astromech-7ck3's step now measures tray-only, open and closed again. Known limits gain no crash restart, astromech-y2m1 (interactive setup.exe and Start with Windows), astromech-vzwk (45 s update timeout) and astromech-dkvi (Trade, Route on Ardent's null rows); the neutron file's memory line follows k6da. README links the new notes. `scripts/check-release-notes.py` (44 open needs-human, 0 problems) and `scripts/check-guide.py` pass.
