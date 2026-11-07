---
schema: atelier.review
schema_version: 2
id: astromech-90pi
title: finding on astromech-vucf @a43fb01a
target: astromech-vucf
identity: claude-reviewer-d2nq
commit: a43fb01a8e58abe7e80d9805f29557c3ae0182c9
created_at: 2026-11-07T14:01:44.276431547Z
event:
  event: finding
  recommendation: defer
  disposition:
    decision: defer
    identity: claude-orchestrator-kjfs
    commit: b5137f3317f220968c5d4c2f60fa54d46eb1f45a
    recorded_at: 2026-11-07T14:06:44.511807175Z
    follow_up: astromech-vucf
    reason: Noted on vucf
---

low: docs/ARCHITECTURE.md:848 the Assembly paragraph's list of what Boundaries holds still ends at the remote-data client; since 83cee7e3 it also holds the app window (Boundaries::window), and the World mode paragraph below names it. Fix: add 'the app window' to that list. Defer to astromech-vucf.
