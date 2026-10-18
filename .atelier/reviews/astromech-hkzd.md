---
schema: atelier.review
schema_version: 1
id: astromech-hkzd
title: 'LATER: the audio tail after a release is now post_roll_ms (about 200 ms) instead of about 260 ms, since the late PttUp delivery gave slack; if debounce >= post_roll the tail is 0 (session.rs:570-577)'
target: astromech-e0zz
identity: e0zz-reviewer-1
commit: e9984b578f3ce5a638ebd7155c41d2742abe2b0d
created_at: 2026-10-17T23:40:48.349495324Z
event:
  event: finding
  recommendation: defer
---

LATER: the audio tail after a release is now post_roll_ms (about 200 ms) instead of about 260 ms, since the late PttUp delivery gave slack; if debounce >= post_roll the tail is 0 (session.rs:570-577)
