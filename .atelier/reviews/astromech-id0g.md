---
schema: atelier.review
schema_version: 1
id: astromech-id0g
title: 'low: gamestate MissionRedirected sets destination_system to new_destination_system even when None (a station-only redirect), erasing a known system. Keep the old value when None.'
target: astromech-rpsb
identity: review-agent (sonnet, independent)
commit: 5f02b645b125ba32a44502aa4c84b61edfa8acfc
created_at: 2026-10-17T03:14:52.132495344Z
event:
  event: finding
  recommendation: defer
---

low: gamestate MissionRedirected sets destination_system to new_destination_system even when None (a station-only redirect), erasing a known system. Keep the old value when None.
