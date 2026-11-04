---
schema: atelier.review
schema_version: 1
id: astromech-m2np
title: finding on astromech-58a7 @4657397b
target: astromech-58a7
identity: review-agent-58a7-1
commit: 4657397bda73c11b22d0216501b72bed0d493a75
created_at: 2026-11-04T23:11:26.657025520Z
event:
  event: finding
  recommendation: defer
---

low: CONTRIBUTING.md:103 says when the weekly check fails, regenerate, commit and ship. Regeneration sends any new symbol with a blank FDevIDs entitlement as station stock; if FDevIDs adds a module before its sku is known (or a new free-looking entitlement appears), that is bad data, against 'No data is better than bad data'. Suggest one sentence: before committing, look at the new station-side symbols and any new entitlement values against the EDDN Elisions rule and astromech-b8js.
