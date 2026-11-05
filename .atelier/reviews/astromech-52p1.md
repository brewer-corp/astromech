---
schema: atelier.review
schema_version: 1
id: astromech-52p1
title: finding on astromech-t34c @60dc9bf4
target: astromech-t34c
identity: review-agent-t34c-1
commit: 60dc9bf4bac92cb3eb296e3749f57bdc6cfc4761
created_at: 2026-11-04T23:39:49.956029183Z
event:
  event: finding
  recommendation: defer
---

medium: crates/astromech-core/src/pipeline.rs:733-736 (lpsk) The being-written rule only holds the game's journal while its last line is within 120 s of now. After a clock correction (J1 headed by the fast clock, J2 the game's), once the game goes quiet for two minutes (docked, AFK, in menus: common, since Status.json changes write no journal line), a tool touching J1 satisfies !begins_before(J1, J2) && !being_written(J2) and J1 takes the session. State becomes J1's (ship, system) until the game next writes J2, which takes it back, so it flaps once per touch, replaying both journals from 0 each time. J1 is also latest by name and was never retired (J2 took the session as Gated and set_game_session retired nothing), so classify(J1) is Class::Game, and any lines a tool appends to J1 past its read point go live whatever their stamps. EDDN drops them by its 30-min age limit, but callouts and EDSM do not. This falls short of the issue's 'a touched or appended older journal is still never followed'. Not a regression: before, J1 was followed from the start. Suggested fix: make 'being written' sticky, e.g. remember the journal last seen being written and do not leave it for one that only wins on header order. Also retire the followed session's stamp when a being-written journal takes over against header order. Add a test like after_a_clock_correction_and_a_restart_... with the touch after the game has been quiet for 3 min (stamps ts(200)).
