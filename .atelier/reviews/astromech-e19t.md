---
schema: atelier.review
schema_version: 1
id: astromech-e19t
title: 'medium: target-here only works for missions accepted in the current journal. The catch-up replays only the current journal, and the login Missions event carries no destination or target faction, so multi-day missions accepted in an earlier session have no destination and never trigger ''Mission target in this system''. Expiry warnings are unaffected. Persist or backfill (scan earlier journals) in a follow-up, or document the limit.'
target: astromech-rpsb
identity: review-agent (sonnet, independent)
commit: 5f02b645b125ba32a44502aa4c84b61edfa8acfc
created_at: 2026-10-17T03:14:51.859456138Z
event:
  event: finding
  recommendation: defer
---

medium: target-here only works for missions accepted in the current journal. The catch-up replays only the current journal, and the login Missions event carries no destination or target faction, so multi-day missions accepted in an earlier session have no destination and never trigger 'Mission target in this system'. Expiry warnings are unaffected. Persist or backfill (scan earlier journals) in a follow-up, or document the limit.
