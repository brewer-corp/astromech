---
schema: atelier.review
schema_version: 2
id: astromech-yzyz
title: finding on astromech-y6d6 @4fd16542
target: astromech-y6d6
identity: claude-reviewer-7i2k
commit: 4fd165420c02c19f3ef7e56ac925d164d100aa81
created_at: 2026-11-07T17:10:32.220301125Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: 2cb0a07f204f53192801157645f775956a039cb3
    recorded_at: 2026-11-07T17:14:41.329479727Z
    follow_up: astromech-v3u4
    reason: Noted on v3u4
---

low: crates/astromech-input/src/listen/source.rs:99-104. Hub::restart() returns Ok(()) without registering when started.get() is None, and that includes while the first subscribe() is still inside get_or_init, with backend.start registering on another thread. If, in v3u4, a session attaches while Builder::build runs, the source's registration can land before tao's. The restart() after build is then a silent no-op that reports Ok, and push-to-talk and the kill switch hear nothing. v3u4's Done when (start or restart after build, before the session can attach) avoids this if it holds, so it's latent today. Fix in v3u4: make restart() wait for a start that is in flight (OnceLock::wait, or a Mutex around start), or have v3u4's code make the order impossible and cover it in its smoke.
