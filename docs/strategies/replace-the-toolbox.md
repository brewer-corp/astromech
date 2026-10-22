---
status: active
mission: astromech-7q0z
last_changed: "2026-10-15: first version, from the toolbox research (Sumeet's direction)"
---

# Why

Playing Elite day to day means keeping several tools open next to the game. **Inara** and
**Spansh** answer "where's the nearest X" for modules, ships, commodities and services, plus
Powerplay, faction state and trade routes. **EDMC** uploads market and journal data to EDDN,
EDSM and Inara. **EDCoPilot** and **OD Explorer** react to the journal: arrival briefings,
prospector results, exobiology tracking. **Coriolis** and **EDSY** plan builds, and **EDSM**
keeps the flight log. Around them sit Ardent Insight, Raven Colonial, BGS Tally, SrvSurvey,
BioScan and ED Odyssey Materials Helper. astromech already knows the commander's state from the
journal and can answer by voice. If it answers these questions too, the browser tabs and the
second companion app go away.

# Outcome

One app covers the lookups and reactions players use those tools for every day. It doesn't
match each tool feature for feature; it covers the 90% case well enough that they aren't needed.
The five daily lookups plus EDDN sharing would be worth shipping on their own.

# Success Criteria

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

# Shape

- **Data sources.** The user's own files cover everything about *me* (journal, `Status.json`
  and the companion files). Spansh covers "anywhere" searches and the route plotters. Its APIs
  are undocumented (REPORT §4), so wrap each call in a schema check and cache it. Ardent Insight
  has a documented REST API and is the best fit for commodity and nearest-service questions.
  EDSM covers star positions, traffic and the flight log. EDDN is where we give data back.
  Inara is commander sync only. The Frontier cAPI is only for fleet-carrier detail.
- **One remote-data layer** with a shared User-Agent, a TTL cache in SQLite, rate limits and
  offline errors in plain words. Every lookup uses it, and one "search near me" UI kit.
- **Voice only for what the HUD doesn't show** (Sumeet, 15 Oct). A reaction speaks only what is
  off screen, buried, or easy to miss mid-action. Being scanned and prospector results are
  wanted for that reason. Star class, security, interdiction, hull, shields, heat, fuel warnings
  and scoop progress are not spoken. Callouts use the COVAS register (V1-DESIGN §6.4).
- **No data is better than bad data** for EDDN: follow the EDCD sender rules exactly, and write
  the sender from the spec, not from EDMC's GPL-2.0 code.
- **Player input per action stays.** One voice "next" or one button press per hop. No
  auto-plotting and no automation that acts without a request (V1-DESIGN §1.4, PLOTTING.md).

# Scope

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

# Authority

The orchestrator may change how the work is broken down, sequencing, and which data source answers a
lookup. Changes to S1–S9, adding a service that needs an account or sends data on the user's
behalf, and anything touching the EULA line come back to Sumeet.

# Risks and Unknowns

- Spansh's search and route APIs are undocumented and can change without notice. Each item
  that uses one checks the request shape first.
- The Inara whitelist and the cAPI client-id approval have lead time we don't control.
- Powerplay and colonisation rules change often. Keep their rules tables as data.
- Community data (exobiology prediction tables, body value formulas) needs a licence check per
  source before it's bundled.

# Appendix: what the toolbox does

The jobs players use each tool for (researched 15 Oct 2026). S9's validation checks against
this list. More detail in [REPORT.md](../REPORT.md) §4 and §7–9 and
[COMPETITORS.md](../COMPETITORS.md).

| Tool | Jobs players actually use it for | Where its data comes from |
|---|---|---|
| **Inara** | Nearest module/ship/commodity/service search; best buy/sell prices; commodity trade routes; system/station/faction pages; Powerplay overview; CG list; engineer and blueprint reference; commander profile (fleet, engineering progress, ranks, reputation); fleet-carrier pages; squadrons | EDDN (galaxy and markets), its own API (commander uploads via EDMC and others) |
| **Spansh** | Neutron, exact (galaxy) plotter with refuel, road to riches, ELW/WW/AW and exomastery routes, fleet-carrier router, trade router, tourist; station, system and body search with any filter (modules, ships, commodities, economy, states, Powerplay, colonisation); nightly galaxy dumps | EDDN plus dumps; undocumented JSON APIs |
| **Coriolis / EDSY** | Ship build planner: full stats (jump range, shields, DPS, heat, power), engineering effects, compare builds, share by link | Static game data (coriolis-data is Frontier IP; EDSY is CC BY-NC) |
| **EDMC** | Uploads markets, outfitting, shipyard and journal events to EDDN; syncs EDSM flight log and Inara profile; cAPI client; plugin host (SpanshRouter, BGS Tally, Canonn, overlays, LandingPad, hourly income, massacre tracker, Discord presence, …) | Journal, companion files, cAPI |
| **EDSM** | Flight log and map of where you've been; system/body database; traffic and deaths; nightly dumps; sphere/cube search | Uploaded journals (via EDMC and others) |
| **EDCoPilot** | Event announcements (arrival, scoopable stars, pad location, mission expiry, materials full, HGE candidates, pirate scans, …); voice control; Spansh routes with an auto-next-hop; location finder; bookmarks; exobiology tracking; mining and session stats; colonisation tracker; missions; surface guidance | Journal, Status, binds, Spansh, EDSM, Inara |
| **OD Explorer** | Exploration and exobiology companion: system bodies with values as you scan, biology checklist, Spansh route import and the next hop on the clipboard, notifications, EDAstro points of interest | Journal, Spansh CSV, EDSM, EDAstro GEC |
| *Also in the toolbox* | **Ardent Insight** (open REST API: nearest service, commodity imports/exports near a system), **Raven Colonial / Architect Tracker** (colonisation planning and contribution tracking), **BGS Tally** (BGS work per faction), **SrvSurvey** (surface guidance, Guardian and bio overlays), **BioScan** (species prediction), **EDEngineer / ED Odyssey Materials Helper** (blueprints and micro-resources) | |

No record covers Discord presence yet; Squadrons and the rest of the Scope table are out.
