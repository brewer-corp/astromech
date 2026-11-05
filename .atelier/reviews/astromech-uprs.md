---
schema: atelier.review
schema_version: 1
id: astromech-uprs
title: finding on astromech-dfvp @84b18f26
target: astromech-dfvp
identity: reviewer-fable-dfvp
commit: 84b18f26398d85ca5984acce5abcc1a316f7281d
created_at: 2026-11-05T17:20:02.548152809Z
event:
  event: finding
  recommendation: defer
---

low: app/src/modules/datasharing/Edsm.svelte:103 A keyring failure (st.key_error, a failed read or write of the API key) is drawn as an orange '!' warning row with no recovery action; DESIGN.md's hard error is a red '!' plus words plus the action that recovers. Pre-existing; a follow-up moves it to <Failure> with 'Enter the key again' (or the row's existing action).
