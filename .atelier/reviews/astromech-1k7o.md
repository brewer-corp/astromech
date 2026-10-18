---
schema: atelier.review
schema_version: 1
id: astromech-1k7o
title: 'R1: liveness used journal_logs().last(), which ranks a name that isn''t a journal stamp (sync conflict copy) by mtime, so a copy can sort latest and make the game''s journal non-live for the session (probe: 0 EDDN sends after, 2 before).'
target: astromech-0d7l
identity: reviewer-a66807ef
commit: a802eeb10f21b33ccce9e110c507a5d34b2e3d45
created_at: 2026-10-17T20:14:47.450658262Z
event:
  event: finding
  recommendation: fix_now
---

R1: liveness used journal_logs().last(), which ranks a name that isn't a journal stamp (sync conflict copy) by mtime, so a copy can sort latest and make the game's journal non-live for the session (probe: 0 EDDN sends after, 2 before).
