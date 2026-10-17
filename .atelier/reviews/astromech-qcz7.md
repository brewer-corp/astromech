---
schema: atelier.review
schema_version: 1
id: astromech-qcz7
title: 'crates/astromech-core/src/speech.rs:482: if the session stops while the tts-start thread loads Kokoro, the thread still finishes, speaks any queued lines after the session ended, then drops the worker; a session restarted in that window loads a second Kokoro briefly. Narrow window, self-cleaning; note only.'
target: astromech-mcdt
identity: review-agent (mcdt review)
commit: abf17ec4992104d88eff0a9ddfbc1e724bc1023d
created_at: 2026-10-17T00:02:18.595834613Z
event:
  event: finding
  recommendation: no_action
---

crates/astromech-core/src/speech.rs:482: if the session stops while the tts-start thread loads Kokoro, the thread still finishes, speaks any queued lines after the session ended, then drops the worker; a session restarted in that window loads a second Kokoro briefly. Narrow window, self-cleaning; note only.
