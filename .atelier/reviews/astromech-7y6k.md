---
schema: atelier.review
schema_version: 1
id: astromech-7y6k
title: The plotter (astromech-6bjl) owns how the file is held while loaded; it must keep the handle or re-check the header before reading names.
target: astromech-1842
identity: orchestrator-claude
commit: cd4e3322fb78a5aee9e3cc858fad1f8df480d37e
created_at: 2026-10-22T21:11:35.403818264Z
event:
  event: disposition
  finding: astromech-4h9h
  decision: defer
  supersedes: null
  resolution_commit: null
  follow_up: astromech-6bjl
---

The plotter (astromech-6bjl) owns how the file is held while loaded; it must keep the handle or re-check the header before reading names.
