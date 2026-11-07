---
schema: atelier.review
schema_version: 2
id: astromech-gmop
title: finding on astromech-vucf @a4f2b03e
target: astromech-vucf
identity: claude-reviewer-7149
commit: a4f2b03eeefddbbf2a2ec862d58ed06300f0da90
created_at: 2026-11-07T12:59:09.185205173Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-core/src/app.rs:1580-1582 assemble's doc comment is garbled ('load the config (...), set / make the core with ...'): a sentence was half-edited. And crates/astromech-core/src/app/every_window_command.rs:186-188 save_diagnostics' reason still says the zip is revealed 'with no opener seam', which this change added (Boundaries.opener). Fix both in the next commit on this branch (d2nq/gllh touch these files).
