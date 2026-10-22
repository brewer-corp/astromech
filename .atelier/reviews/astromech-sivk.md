---
schema: atelier.review
schema_version: 1
id: astromech-sivk
title: 'R4 low crates/astromech-powerplay/src/standing.rs:103 (xvov judgement): the pledge clears at a run''s first login without Powerplay, and a same-run relog writes no Powerplay. With two commanders in one journal folder: first login as unpledged B clears the state, then a relog from the menu to pledged A in the same run leaves the page ''Not pledged'' until the next game start. Main showed A''s pledge for B instead, so not a regression, but a new wrong state. Evidence for the rule itself is good (every pledged first login in ~/edjournal writes Powerplay before Missions). Sumeet to accept; a fix would key Standing per commander.'
target: astromech-fdh3
identity: review-agent-fdh3-1
commit: 63470471264d143f5c0e854f4641ee757634d5a8
created_at: 2026-10-22T21:45:44.559677216Z
event:
  event: finding
  recommendation: defer
---

R4 low crates/astromech-powerplay/src/standing.rs:103 (xvov judgement): the pledge clears at a run's first login without Powerplay, and a same-run relog writes no Powerplay. With two commanders in one journal folder: first login as unpledged B clears the state, then a relog from the menu to pledged A in the same run leaves the page 'Not pledged' until the next game start. Main showed A's pledge for B instead, so not a regression, but a new wrong state. Evidence for the rule itself is good (every pledged first login in ~/edjournal writes Powerplay before Missions). Sumeet to accept; a fix would key Standing per commander.
