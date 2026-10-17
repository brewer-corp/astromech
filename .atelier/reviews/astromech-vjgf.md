---
schema: atelier.review
schema_version: 1
id: astromech-vjgf
title: Nothing tests that StoredShips clears the arrival of a ship it lists at a station; the .filter(|s| s.in_transit) can be dropped with all tests passing.
target: astromech-hnar
identity: review-agent
commit: 03cae6ee3cf1304e8251b029a0e372680b140bf0
created_at: 2026-10-17T17:30:46.254485135Z
event:
  event: finding
  recommendation: fix_now
---

Nothing tests that StoredShips clears the arrival of a ship it lists at a station; the .filter(|s| s.in_transit) can be dropped with all tests passing.
