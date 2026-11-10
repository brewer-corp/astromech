---
schema: atelier.review
schema_version: 2
id: astromech-7qwc
title: finding on astromech-xajc @8d8cedde
target: astromech-xajc
identity: GPT-6 Astra (round 2)
commit: 8d8cedded45313817a318ddc986bbf8408f916c2
created_at: 2026-11-10T06:13:57.660315053Z
event:
  event: finding
  recommendation: fix_now
---

P2: The revised Route source contract still contradicts itself (astromech-woqv Done when 2/3 and qwvs Done when 1/2; continuation of astromech-50kz). Exact routes supply every jump independently of NavRoute (plot.rs and spansh::ExactJump), but woqv says that with no matching NavRoute the flown leg stays count-only, and qwvs says before plotting in the galaxy map it shows only its count/help line. A worker can satisfy one clause only by violating the other. State precedence explicitly: use exact/imported jump data when present, enrich a matching leg with NavRoute, and use count-only only when neither source supplies the path. Also distinguish a complete path from known scoopability: journal::RouteHop.star_class is optional, so a list of every jump does not prove nowhere to scoop. Require known scoopability for every relevant jump before that warning and an unknown icon/state otherwise. Acceptance should cover an exact route without NavRoute and a complete NavRoute with a missing class.
