---
schema: atelier.review
schema_version: 1
id: astromech-aeub
title: finding on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:19.948013397Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/route/View.svelte:154 The Route page's .err is a white-on-orange filled block (pre-existing; this change kept it when it dropped the compact rail), which DESIGN.md States forbids ('Never a filled block'); the strip beside it now shows the same failure as a red '!' line, so one failure has two treatments on that page. Pre-existing, not worsened; fold into the States pass (as astromech-jgyk was) or fix with the Route page.
