---
schema: atelier.review
schema_version: 1
id: astromech-hnzq
title: finding on astromech-51ue @bfb7f1dd
target: astromech-51ue
identity: reviewer-fable-51ue-plan
commit: bfb7f1ddd9bc83022431ffd734d9a7782c43e41b
created_at: 2026-11-05T06:26:17.896943938Z
event:
  event: finding
  recommendation: defer
---

defer: astromech-51ue. bi94 counted lookups on Powerplay, Exploration and Colonisation among the 13, and jp36 and rz0f apply to every SearchKit, but the parent never says what happens to them. Exploration's System page embeds a nearest-services lookup (app/src/modules/exploration/View.svelte:165, stations.service), Powerplay Targets runs powerplay.<target>. Fix: one line in 51ue: Powerplay, Exploration and Colonisation lookups stay where they are and get the row and the actions; Exploration's embedded services lookup becomes a link into the Services finder like the Trade and Materials embeds.
