---
status: active
mission: astromech-7q0z
last_changed: "2026-10-15: first version, from the toolbox research (Sumeet's direction)"
---

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

# Risks and Unknowns

- Spansh's search and route APIs are undocumented and can change without notice. Each item
  that uses one checks the request shape first.
- The Inara whitelist and the cAPI client-id approval have lead time we don't control.
- Powerplay and colonisation rules change often. Keep their rules tables as data.
- Community data (exobiology prediction tables, body value formulas) needs a licence check per
  source before it's bundled.

# Appendix: what the toolbox does

The jobs players use each tool for (researched 15 Oct 2026). S9's validation checks against
this list. More detail in [REPORT.md](../../../docs/REPORT.md) §4 and §7–9 and
[COMPETITORS.md](../../../docs/COMPETITORS.md).

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
