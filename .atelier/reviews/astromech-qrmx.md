---
schema: atelier.review
schema_version: 2
id: astromech-qrmx
title: finding on astromech-vucf @2348c8e9
target: astromech-vucf
identity: claude-reviewer-i1ev
commit: 2348c8e90388cdd90cf1e69e800cfa34ccafd91d
created_at: 2026-11-07T16:22:24.887231362Z
event:
  event: finding
  recommendation: no_action
---

info: crates/astromech-core/src/session.rs:373-382. On Windows, world-mode sessions still run the real start_voice, so bus::attach starts the real Raw Input and XInput listener until astromech-1hh9 lands. Judged not a CI hazard. In worlds/jump nothing is configured for push-to-talk, the kill switch or paste. The worker exe is absent from the world's exe folder, so there is no mic and no speech-to-text, and the speaker is ConsoleSpeaker. The listener only reads input, and the ptt thread is unregistered, so it doesn't affect quiescence. start_buttons only fails if a thread spawn fails. It does contradict e1a5's 'reach no real device' until 1hh9, which already owns that. No action here; Windows CI is the first real run.
