---
schema: atelier.mission
schema_version: 3
id: astromech-7q0z
title: Replace the daily toolbox
status: ready
labels: []
lead: null
ready:
  by: conversion
  at: 2026-10-15T20:36:18.885951529Z
created_at: 2026-10-15T20:36:13.666399940Z
---

## Why

Playing Elite day to day means keeping several tools open next to the game. **Inara** and
**Spansh** answer "where's the nearest X" for modules, ships, commodities and services, plus
Powerplay, faction state and trade routes. **EDMC** uploads market and journal data to EDDN,
EDSM and Inara. **EDCoPilot** and **OD Explorer** react to the journal: arrival briefings,
prospector results, exobiology tracking. **Coriolis** and **EDSY** plan builds, and **EDSM**
keeps the flight log. Around them sit Ardent Insight, Raven Colonial, BGS Tally, SrvSurvey,
BioScan and ED Odyssey Materials Helper. astromech already knows the commander's state from the
journal and can answer by voice. If it answers these questions too, the browser tabs and the
second companion app go away.

## Outcome

One app covers the lookups and reactions players use those tools for every day. It doesn't
match each tool feature for feature; it covers the 90% case well enough that they aren't needed.
The five daily lookups plus EDDN sharing would be worth shipping on their own.

## Success criteria

Every lookup works from the current system or a typed one, in the app and as a voice query.
A module isn't done until its `voice_exposed` queries have phrase templates (ARCHITECTURE
§18.7).

- S1: **Outfitting:** nearest station selling a given module, ship or service, filtered by the
  current ship's pad size. Shown by lookups against known stations.
- S2: **Trade:** where to sell or buy a commodity near me, and the best station for the cargo
  I'm carrying. Shown by lookups against known markets.
- S3: **Engineering materials:** High Grade Emissions signals called out with the likely
  material pool, Boom/Famine/War systems for the shopping list, and the nearest raw-material
  sites (crystalline shards and the like). Shown by journal replay and lookups.
- S4: **Powerplay:** my standing and merits, the system's Powerplay state on arrival, and nearby
  systems for my power to work in. Shown by journal replay and lookups.
- S5: **Colonisation:** depot progress, what's left to deliver (in loads for the current ship),
  and where to buy each commodity. Shown by journal replay and lookups.
- S6: **Data sharing replaces EDMC:** commodity, outfitting, shipyard, journal and the other live
  EDDN schemas are sent from the live game and accepted (test schemas first, then a play session
  with no rejections). EDSM journal upload works.
- S7: **Reactions:** prospector results, being scanned, arrival briefing, docking pad position
  and fuel along the route are spoken. Nothing already prominent on the HUD is spoken. Shown by
  journal replay tests and a play session.
- S8: **Exploration and exobiology:** a live system view with body values, a system biology
  summary, and the exobiology sampling helper. Shown by journal replay.
- S9: **Acceptance:** Sumeet plays for a week without opening the replaced tools for the jobs in
  S1–S8. Checked by an independent validation item.

## Scope

In: lookups and reactions for the jobs in S1–S8, EDDN and EDSM sending, links out to Inara,
Spansh, Coriolis and EDSY.

Out, with what we do instead:

| Thing | Why | Instead |
|---|---|---|
| Our own galaxy-wide market or outfitting database | Needs a server fed by EDDN, like Inara, Spansh and Ardent run | Query Spansh/Ardent; contribute via EDDN. An optional local EDDN cache stays in the backlog |
| Inara writes without approval | Inara's author must whitelist the app name | Ask early; link out until then |
| Fleet-carrier details and `fcmaterials_capi` | Only via cAPI, which needs a Frontier client-id approval | Journal `Carrier*` events; cAPI stays in the backlog |
| EDMC plugin compatibility | Plugins are Python against EDMC's API; EDMC is GPL-2.0 | Build the popular plugins' features natively (LandingPad, SpanshRouter, BGS Tally, hourly income, massacre tracker) |
| Auto-plotting each hop | EULA grey area (PLOTTING.md, REPORT §5) | One voice "next" or button press per hop |
| First-footfall detection | Not in the journal (manual v35); screen capture is fragile | Skip; a "first footfall likely" hint from discovery data |
| Squadrons, forums, commander social pages | Community features belong to Inara | Link out |
| Exact BGS influence history | EliteBGS is gone; EDSM history is patchy | Current state with its data age, plus our own visit history |
| Coriolis/EDSY parity soon | Large, licence-sensitive, fast-moving | Open in Coriolis/EDSY, plus core stats from the journal loadout |

Environment: Windows, next to a running Elite (desktop or VR), one local user.

## Authority

The orchestrator may change how the work is broken down, sequencing, and which data source answers a
lookup. Changes to S1–S9, adding a service that needs an account or sends data on the user's
behalf, and anything touching the EULA line come back to Sumeet.
