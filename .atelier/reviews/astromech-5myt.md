---
schema: atelier.review
schema_version: 1
id: astromech-5myt
title: 'LazySpeaker meets the Done-when: with the TTS model missing at session start, the first line said after the download marker appears starts Kokoro once (state transition under one mutex, so concurrent say starts it exactly once), queues lines during the load and replays them in order under the lock, applies a voice chosen while waiting or loading, and falls back to printing on a failed start. find_tts per line is two small marker reads (plus a directory walk only once installed) and cannot pass on a partial download since the marker is written after extraction. speaks_aloud is true once the model is ready. The unit test checks no start before ready, order, voice and one start, and would fail if any broke. fmt, clippy -D warnings, workspace tests, check-deps and the app''s locked metadata pass. Two low findings deferred (interrupt during load drops unprinted lines; session wiring untested), one note.'
target: astromech-mcdt
identity: review-agent (mcdt review)
commit: abf17ec4992104d88eff0a9ddfbc1e724bc1023d
created_at: 2026-10-17T00:02:24.690639471Z
event:
  event: approval
---

LazySpeaker meets the Done-when: with the TTS model missing at session start, the first line said after the download marker appears starts Kokoro once (state transition under one mutex, so concurrent say starts it exactly once), queues lines during the load and replays them in order under the lock, applies a voice chosen while waiting or loading, and falls back to printing on a failed start. find_tts per line is two small marker reads (plus a directory walk only once installed) and cannot pass on a partial download since the marker is written after extraction. speaks_aloud is true once the model is ready. The unit test checks no start before ready, order, voice and one start, and would fail if any broke. fmt, clippy -D warnings, workspace tests, check-deps and the app's locked metadata pass. Two low findings deferred (interrupt during load drops unprinted lines; session wiring untested), one note.
