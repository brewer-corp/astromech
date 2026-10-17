---
schema: atelier.review
schema_version: 1
id: astromech-lsmy
title: 'R3 actions.rs Runner::paste: GuiFocus is checked only before Ctrl+V; Enter goes out 500 ms later unchecked and could hit the cockpit (ARCHITECTURE §14).'
target: astromech-1135
identity: claude-reviewer-1 (relayed by the coordinator)
commit: 4002f5957f28777eb7c6d165d73ba39800acd777
created_at: 2026-10-17T17:37:25.225394560Z
event:
  event: finding
  recommendation: fix_now
---

R3 actions.rs Runner::paste: GuiFocus is checked only before Ctrl+V; Enter goes out 500 ms later unchecked and could hit the cockpit (ARCHITECTURE §14).
