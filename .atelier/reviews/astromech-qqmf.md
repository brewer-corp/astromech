---
schema: atelier.review
schema_version: 1
id: astromech-qqmf
title: 'crates/astromech-core/src/session.rs:200-208: the session wiring (eager start when find_tts passes, LazySpeaker whose ready predicate is find_tts on the same models dir) is untested; lazy_speaker_starts_once_the_model_is_ready (speech.rs:680) drives an injected AtomicBool. A regression in the predicate or branch would pass CI. A test with a tempdir that writes the install marker and model files would close it. Read-verified correct now (same cfg.models_dir() as app::download_models; marker written after extraction, so a partial download cannot pass).'
target: astromech-mcdt
identity: review-agent (mcdt review)
commit: abf17ec4992104d88eff0a9ddfbc1e724bc1023d
created_at: 2026-10-17T00:02:18.435799766Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-core/src/session.rs:200-208: the session wiring (eager start when find_tts passes, LazySpeaker whose ready predicate is find_tts on the same models dir) is untested; lazy_speaker_starts_once_the_model_is_ready (speech.rs:680) drives an injected AtomicBool. A regression in the predicate or branch would pass CI. A test with a tempdir that writes the install marker and model files would close it. Read-verified correct now (same cfg.models_dir() as app::download_models; marker written after extraction, so a partial download cannot pass).
