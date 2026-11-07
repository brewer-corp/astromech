---
schema: atelier.review
schema_version: 1
id: astromech-9gl8
title: approval on astromech-ja5a @afbdc727
target: astromech-ja5a
identity: claude-reviewer-ja5a
commit: afbdc7271485becff798e783c6b8ff059c0737f4
created_at: 2026-11-07T00:09:07.264378729Z
event:
  event: approval
---

Checked 631532a5..afbdc727. Version 1.0.0-alpha.4 in Cargo.toml, both Cargo.locks (diffs touch only workspace crate versions), app/package.json, app/src-tauri/Cargo.toml, tauri.conf.json; README links the alpha.4 notes. Every What's new claim checked against code, docs/guide and git log v1.0.0-alpha.3..main (route strip, Find or plot and the neutron file's move to the plot panel, Places finders and origins, Add stop/Set destination, galaxy-map following, Spansh 166 jumps, voice origins and EVERYDAY, orange/Saira, two tab rows, setup strings, config path, journal, EDSM, neutron log, open_external). Test steps no longer reference removed UI; Known limits still true. Findings o2j0, 5bji, 0c5m, b3f4 fixed (44c0b46c, a87ccc67); dxuh and 6dci addressed too. check-release-notes.py: 41 open needs-human issues, 0 problems; check-guide.py: 0 problems. Tag and publish are post-merge and not yet done.
