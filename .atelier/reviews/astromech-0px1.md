---
schema: atelier.review
schema_version: 1
id: astromech-0px1
title: 'R1 low materials_cap.rs remember(): its list of material-changing events duplicates Ledger::apply''s match; a new ledger event would leave the rule''s full set stale. Recompute on every journal line instead.'
target: astromech-fg9l
identity: reviewer agent acd63c21 (independent)
commit: 2f261a25d8c164ef457450aed969b4e6500be299
created_at: 2026-10-17T21:55:58.984389049Z
event:
  event: finding
  recommendation: defer
---

R1 low materials_cap.rs remember(): its list of material-changing events duplicates Ledger::apply's match; a new ledger event would leave the rule's full set stale. Recompute on every journal line instead.
