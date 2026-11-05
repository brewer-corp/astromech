---
schema: atelier.review
schema_version: 1
id: astromech-9e5p
title: finding on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:20.171351271Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-1d92.md:1 The Done when's off-route rule ('the commander's current system is not the previous or next waypoint') is narrower in Strip.svelte:63, which warns only when the next waypoint is one jump on (exact and galaxy-map routes), so neutron and engineer-trip legs don't cry wolf. The reviewer settles that open choice for the one-jump rule (see the 1d92 review report); the record should match the code: say so in the close reason or amend the Done when line when closing.
