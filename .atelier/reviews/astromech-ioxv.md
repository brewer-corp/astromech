---
schema: atelier.review
schema_version: 1
id: astromech-ioxv
title: 'R1 [low] crates/astromech-intent/src/lib.rs:164-169: the new status.merits_here phrases pull an existing merit query to the wrong command. ''what are my merits'' was NoMatch on main and is now Ask ''what earns merits here'' (0.600, runner ''where are we'' 0.522), so a commander asking for their merit count is offered the activity list; ''powerplay activities'' also moved from Ask status.powerplay to Ask merits_here. Probed with the rank example on this branch and on main''s intent crate. Fix: add ''what are my merits'' (and ''how many merits do i have'', NoMatch on both) to status.powerplay and pin them in v1_t1_and_status_phrases_execute.'
target: astromech-w62j
identity: review-agent (w62j review)
commit: 6ffb066bc9dc8ff1509916c30dc9fd799acbbb41
created_at: 2026-10-17T01:10:15.397418396Z
event:
  event: finding
  recommendation: fix_now
---

R1 [low] crates/astromech-intent/src/lib.rs:164-169: the new status.merits_here phrases pull an existing merit query to the wrong command. 'what are my merits' was NoMatch on main and is now Ask 'what earns merits here' (0.600, runner 'where are we' 0.522), so a commander asking for their merit count is offered the activity list; 'powerplay activities' also moved from Ask status.powerplay to Ask merits_here. Probed with the rank example on this branch and on main's intent crate. Fix: add 'what are my merits' (and 'how many merits do i have', NoMatch on both) to status.powerplay and pin them in v1_t1_and_status_phrases_execute.
