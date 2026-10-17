---
schema: atelier.review
schema_version: 1
id: astromech-nam2
title: 'crates/astromech-announce/src/docking.rs:47: STARPORTS leaves out AsteroidBase (and the newer Dodec), which the LandingPad plugin this table follows treats as having the same 45-pad starport layout. Failure: DockingGranted at an asteroid base gets no pad line, a silent gap rather than a wrong line. Fix: confirm the layout in the game check astromech-505u and add the types.'
target: astromech-77a1
identity: review-agent (77a1 review)
commit: 94747f219dd1884a56d2a091af42f68fc031ff7e
created_at: 2026-10-17T02:05:40.348027931Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-announce/src/docking.rs:47: STARPORTS leaves out AsteroidBase (and the newer Dodec), which the LandingPad plugin this table follows treats as having the same 45-pad starport layout. Failure: DockingGranted at an asteroid base gets no pad line, a silent gap rather than a wrong line. Fix: confirm the layout in the game check astromech-505u and add the types.
