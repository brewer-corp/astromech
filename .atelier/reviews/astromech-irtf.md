---
schema: atelier.review
schema_version: 1
id: astromech-irtf
title: 'app/src/modules/engineering/Build.svelte:84: the shopping list reaches the core only when the Build page works out a plan. Materials collected in game stay on the core''s list until the commander reopens that page, so the arrival briefing keeps saying ''Boom here. Proto Heat Radiators.'' after the need is met. Deleting a ship''s plan also never sends the empty list that drops it. Suggest recomputing remaining needs in the core from the materials ledger, or sending an empty list when a plan is cleared.'
target: astromech-77a1
identity: review-agent (77a1 review)
commit: 94747f219dd1884a56d2a091af42f68fc031ff7e
created_at: 2026-10-17T02:05:40.495685828Z
event:
  event: finding
  recommendation: defer
---

app/src/modules/engineering/Build.svelte:84: the shopping list reaches the core only when the Build page works out a plan. Materials collected in game stay on the core's list until the commander reopens that page, so the arrival briefing keeps saying 'Boom here. Proto Heat Radiators.' after the need is met. Deleting a ship's plan also never sends the empty list that drops it. Suggest recomputing remaining needs in the core from the materials ledger, or sending an empty list when a plan is cleared.
