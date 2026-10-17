---
schema: atelier.review
schema_version: 1
id: astromech-vnsb
title: 'Low: astromech_core::colonisation::Projects re-implements astromech_materials::journal_version (core already depends on astromech-materials), and no test changes the journal or Cargo.json between questions, so invalidation is unproven. The key looks right on inspection. During play the newest journal grows with every event, so almost every ''how is the build going'' or ''what is left'' replays the whole journal history, the same cold replay the app''s page notes can exceed 2 s. It runs on the spawned thread, so speech is not blocked, but those two asks say no ''Looking that up.'' first and can be silent for seconds on a long history. Suggest reusing journal_version, adding a re-read test, and an incremental fold later (astromech-0x68).'
target: astromech-3ez9
identity: review-agent (opus, independent)
commit: d850f53222d41a4a69ed5c90aa8306d7f60c2ed4
created_at: 2026-10-17T08:18:09.742590221Z
event:
  event: finding
  recommendation: defer
---

Low: astromech_core::colonisation::Projects re-implements astromech_materials::journal_version (core already depends on astromech-materials), and no test changes the journal or Cargo.json between questions, so invalidation is unproven. The key looks right on inspection. During play the newest journal grows with every event, so almost every 'how is the build going' or 'what is left' replays the whole journal history, the same cold replay the app's page notes can exceed 2 s. It runs on the spawned thread, so speech is not blocked, but those two asks say no 'Looking that up.' first and can be silent for seconds on a long history. Suggest reusing journal_version, adding a re-read test, and an incremental fold later (astromech-0x68).
