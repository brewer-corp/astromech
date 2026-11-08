---
schema: atelier.review
schema_version: 2
id: astromech-0fyh
title: finding on astromech-h95t @f19cd827
target: astromech-h95t
identity: claude-reviewer-h95t
commit: f19cd827b023ba7d64a0e93f3cf1bc7c8ce070f8
created_at: 2026-11-08T16:01:07.817924908Z
event:
  event: finding
  recommendation: fix_now
---

low: docs/releases/v1.0.0-alpha.5.md:12 'With the window closed and Elite not running, astromech used about 7 MB': receipt astromech-osyy's 7.2 MB is windowless before the window was ever opened; 30 s after each close it was 10.8 to 13.1 MB (astromech-xz7i report), on a CI runner, speech worker not running. Fix: 'about 7 MB at login before you open the window, and 11 to 13 MB after you close it'.
