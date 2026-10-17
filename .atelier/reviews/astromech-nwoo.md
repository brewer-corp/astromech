---
schema: atelier.review
schema_version: 1
id: astromech-nwoo
title: 'crates/astromech-announce/src/fuel.rs:108-122: scoopable has no cooldown and speaks on every arrival at a non-scoopable star while on a route. On a neutron-highway route that is every other jump (''Next star is scoopable.''), and in the same session the route rule''s ''Fuel too low to reach a scoopable star.'' and fuel_range''s identical line can be said seconds apart. That is a borderline case of ''nothing spams on every jump''. Suggest: say ''Next star is scoopable.'' only when fuel is short, or suppress the line on neutron stars, and check this in astromech-505u.'
target: astromech-77a1
identity: review-agent (77a1 review)
commit: 94747f219dd1884a56d2a091af42f68fc031ff7e
created_at: 2026-10-17T02:05:40.421501587Z
event:
  event: finding
  recommendation: defer
---

crates/astromech-announce/src/fuel.rs:108-122: scoopable has no cooldown and speaks on every arrival at a non-scoopable star while on a route. On a neutron-highway route that is every other jump ('Next star is scoopable.'), and in the same session the route rule's 'Fuel too low to reach a scoopable star.' and fuel_range's identical line can be said seconds apart. That is a borderline case of 'nothing spams on every jump'. Suggest: say 'Next star is scoopable.' only when fuel is short, or suppress the line on neutron stars, and check this in astromech-505u.
