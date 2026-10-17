---
schema: atelier.review
schema_version: 1
id: astromech-f9mc
title: 'R4 [medium] Most of the prediction''s conditions are untested. Mutations of predict.rs that leave every test in astromech-exobiology, -exploration, -announce and -core green: removing the body-type gate, making volcanism always fit, removing the star-class gate, and ignoring pressure entirely. Only the atmosphere gate and the temperature taper are exercised (the ''rules out'' test''s four excluded species are all excluded by atmosphere alone). The fixture test measures recall only; nothing bounds precision, and star_class is None for 38 of 48 fixture bodies (Spansh parents trimmed to barycentres/planets), so star gates are barely reached. Fix: add small targeted cases, one per gate, each failing under its mutation (e.g. Electricae Pluma needs an A/N/D/H/AeBe parent; an icy-body-only species on a rocky body with the right atmosphere; a ''none'' volcanism species on a volcanic body), and a precision ceiling on the fixture (mean candidates per body) so a loosened gate shows up.'
target: astromech-8e8c
identity: 'review-agent (opus, independent, part A: data and prediction)'
commit: e5f94c22467a3514e2f7f88d5e0f7a58780e5631
created_at: 2026-10-17T05:57:53.417362597Z
event:
  event: finding
  recommendation: fix_now
---

R4 [medium] Most of the prediction's conditions are untested. Mutations of predict.rs that leave every test in astromech-exobiology, -exploration, -announce and -core green: removing the body-type gate, making volcanism always fit, removing the star-class gate, and ignoring pressure entirely. Only the atmosphere gate and the temperature taper are exercised (the 'rules out' test's four excluded species are all excluded by atmosphere alone). The fixture test measures recall only; nothing bounds precision, and star_class is None for 38 of 48 fixture bodies (Spansh parents trimmed to barycentres/planets), so star gates are barely reached. Fix: add small targeted cases, one per gate, each failing under its mutation (e.g. Electricae Pluma needs an A/N/D/H/AeBe parent; an icy-body-only species on a rocky body with the right atmosphere; a 'none' volcanism species on a volcanic body), and a precision ceiling on the fixture (mean candidates per body) so a loosened gate shows up.
