---
schema: atelier.review
schema_version: 1
id: astromech-s25j
title: 'R1/R2: showRow() and the device switch do not clear unknown, so an unknown-press panel can outlive a click on an action row (not a regression from this PR).'
target: astromech-54hd
identity: review-subagent-10
commit: ff32bfec92411960a3368009ddb40393ad6d854e
created_at: 2026-10-17T19:33:32.804852121Z
event:
  event: finding
  recommendation: defer
---

R1/R2: showRow() and the device switch do not clear unknown, so an unknown-press panel can outlive a click on an action row (not a regression from this PR).
