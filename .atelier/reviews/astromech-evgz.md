---
schema: atelier.review
schema_version: 1
id: astromech-evgz
title: '[low] crates/astromech-announce/src/biology.rs react_all, FsdJump|Location arm: on a login the known-biology lines are spoken whenever ask() already has a Done answer, even if the system''s own FSS/DSS results are already in the fold (biology::ranked(sys) non-empty), which only clears waiting_until. Reachable when astromech was started mid-session in the system (arrival in the silent catch-up, so said.arrived is false), the Biology page has already asked (SpanshBiology holds Done), and the commander relogs: ''Known biology on N bodies.'' after the system was scanned, against the spec ''unless the system''s own FSS or DSS results came first''. Fix: on the Location arm, say nothing when biology::ranked(sys) is non-empty (test: login in a scanned system with a ready answer is silent).'
target: astromech-h5gs
identity: review-agent (opus, independent)
commit: 86da05d28e3e1223fe70d6e4e3b6e11acbeadf22
created_at: 2026-10-17T08:11:14.975795991Z
event:
  event: finding
  recommendation: defer
---

[low] crates/astromech-announce/src/biology.rs react_all, FsdJump|Location arm: on a login the known-biology lines are spoken whenever ask() already has a Done answer, even if the system's own FSS/DSS results are already in the fold (biology::ranked(sys) non-empty), which only clears waiting_until. Reachable when astromech was started mid-session in the system (arrival in the silent catch-up, so said.arrived is false), the Biology page has already asked (SpanshBiology holds Done), and the commander relogs: 'Known biology on N bodies.' after the system was scanned, against the spec 'unless the system's own FSS or DSS results came first'. Fix: on the Location arm, say nothing when biology::ranked(sys) is non-empty (test: login in a scanned system with a ready answer is silent).
