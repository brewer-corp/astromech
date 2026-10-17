---
schema: atelier.review
schema_version: 1
id: astromech-n6fd
title: 'R1: after a renderer panic a late say() can leave busy() stuck.'
target: astromech-cmhv
identity: review-agent (cmhv merge of onboarding)
commit: 68689d5314bab261ab51a74aedaf080f4dde4d2b
created_at: 2026-10-16T20:47:21.660041595Z
event:
  event: finding
  recommendation: defer
---

R1: after a renderer panic a late say() can leave busy() stuck.
