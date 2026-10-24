---
created_at: "2026-10-24T23:07:07.622893259+00:00"
id: "astromech-yn6y"
issue_type: "spike"
labels:
- "size-s"
workflow_branch:
  branch_base: main
  work_branch: spike/astromech-yn6y
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-24T23:26:05.839742273+00:00"
status: "done"
title: "Compound place queries: test two linked-place recipes on real data"
updated_at: "2026-10-24T23:26:05.839742273+00:00"
---

## Goal

Find out whether a query engine for linked places (place A with conditions, within some distance of place B with conditions, ranked by one goal) is workable on the data astromech can reach, before the Places UI is designed around it.

## Done when

Two hard-coded recipes run end to end from a throwaway prototype under `spikes/`, against the live services through the existing rate-limited client or equivalent:

1. An HGE-likely system (Boom, Expansion or War/Civil War, at least 1 bn people, by allegiance) within N ly of a material trader of a chosen type, nearest to a given origin.
2. A ring with a hotspot for a chosen mineral within N ly of a station buying it above a price, ranked by price against extra jumps.

The findings record, for each: requests made, wall time at 1 request/s per service, the candidate-set sizes on each side, answer quality spot-checked against Inara/Spansh, and what failed. Also: which facts could come from a local static file built from the public Spansh dumps (stations, services, traders, rings, hotspots), its estimated size, and which must stay live (prices, faction states). Conclude with a go / no-go and the engine shape to build.

## Unknowns

- Whether Spansh's body/station search can filter rings by hotspot, and how complete hotspot data is.
- How fresh faction-state data is for HGE prediction.
- Whether searching from the rarer side (traders, hotspots) keeps candidate sets small enough.

## Close Reason

Go. Linked-place queries work on the data astromech can reach. Each recipe is answered by fetching each side once as a set around the origin and joining locally: 2 to 7 requests, 2 to 7 s at 1 request/s per service. One search per candidate costs 8 to 164 requests for the same answer. The prototype is spikes/q1-linked-places/: the real astromech_data::Client over a transport that records every answer to cache/ (gitignored). `sh spikes/q1-linked-places/replay.sh` reruns every query below from that cache with --offline.

Requests for the whole spike: Spansh 123, Ardent 21, EDSM 4 (61 s waiting on the network, cap 400), plus 9 HEAD requests and two 48 MB range downloads from downloads.spansh.co.uk.

Recipe 1 finds an HGE-likely system (a faction of the chosen allegiance in Boom, Expansion, War or Civil War, at least 1 bn people) within 20 ly of a Manufactured trader, within 80 ly of the origin, nearest first. Each query is a systems search for A (one page of 500), a systems search for permit systems, the stations search for B (pages of 50), and an autocomplete.
- Sol, Federation: A 70 (274 with no state filter), B 198 traders, 51 trips, 7 requests, 27 MB.
- Shinrarta Dezhra, Empire: A 8, B 165, 5 trips, 7 requests.
- Achenar, Empire: A 74, B 102, 39 trips, 6 requests.
- Maia, Empire: A 1, B 0, no trips, 4 requests.
- Fan-out, Sol at radius 30 and link 15: the set join took 3 requests, traders first 16, HGE systems first 12, all with the same 9 trips. Shinrarta: HGE systems first took 8, and traders first would need 164. The rarer side depends on the origin.
- Faction data age (median / p90 / max, in hours): Sol 3.5 / 30 / 65, Achenar 7.9 / 51 / 200.
- EDSM disagreed with Spansh on 2 of 4 top answers: in Epsilon Indi EDSM showed the civil war over, and in Wolf 1453 Spansh had Boom where EDSM had Election.

Recipe 2 finds a ring with a hotspot for the mineral within 60 ly of the origin and a buyer within 30 ly, ranked by credits per hour with a list of price against extra jumps. Each query takes 3 requests (Spansh bodies, Ardent nearby imports, autocomplete), about 3 s and 1 MB.
- Platinum at 250k or more: Sol 327 rings in 234 systems against 71 buyers, with 287 rings having a buyer. Shinrarta 283 / 64 / 223. Achenar 171 / 15 / 86. Maia 111 / 1 / 46.
- At any price: Painite 328 rings against 719 buyers, LTD 706 rings against 709 buyers.
- Fan-out, Sol at radius 25 and link 15: the set join took 2 requests, hotspots first 14, buyers first 16.
- Ardent and Spansh prices match exactly where both have the same market update. Ardent's buyers had a median age of 4 to 9 h. Spansh's market search came back with a median age of 55 days.

Hotspots: Spansh's bodies search filters `ring_signals` by ore name and by count range (verified: 91 of 91 hits), per body rather than per ring. Ring type can't be filtered, but `reserve_level` can. Within 50 ly, 83 to 88% of known ringed bodies in the bubble have a hotspot recorded, and 60% around Colonia. That is a lower bound, and hotspots never change.

Local or live: systems, stations (type, pads, arrival distance, services, trader kind), rings and hotspots, and the permit flags could come from a file built from galaxy_stations.json.gz (4.56 GB): about 142k systems, 290k stations and 270k hotspot rings, roughly 90 MB as SQLite and 30 MB gzipped (±2x, sampled from the start of the dump). Hotspot rings galaxy-wide would be about 4M rows and 500 MB, so it isn't worth it. Prices (Ardent) and faction states (Spansh, with the journal on arrival) must stay live.

Engine to build: a query is a plan of legs {kind, conditions, near: (leg, max_ly)} with an origin, a radius and a goal (ordered score terms). Recipes are saved plans with named slots. "Find near this" pins the first leg to a result. Each leg becomes one set within radius plus link of the origin, joined and re-checked locally. Fan-out is only a fallback when one side has no bound, chosen with Spansh's cheap `count` probe. Station, ring and permit legs are local; faction and price legs are live. The budget is at most 2 live legs and about 4 requests per query, or 1 per recipe once stations are local. Along a route, the A side is the route's corridor, ranked by detour.

### Deviations
- The findings are in this record, not in a FINDINGS.md in the spike folder: this agent's harness refuses to write report .md files. spikes/README.md points here.
- Inara answered HTTP 503, so spot checks used EDSM (faction states), the raw Spansh records (hotspots) and Spansh against Ardent (prices).
- The dumps were sampled from their first 48 MB using HTTP Range requests. No dump was downloaded whole: the smallest galaxy dump is 1.26 GB.

### Decisions
- Prices come from Ardent, not Spansh's market search, which is weeks stale and hits its count cap.
- Permit-locked systems are a third set (`needs_permit: {"value": true}`, verified). Station records don't carry the flag, and neither do the dumps.
- Credits per hour scales mining time by the ring's hotspot count and reserve level. That model is illustrative, not tuned.

### Surprises
- Ardent's nearby imports ignore `minLandingPadSize`: 22 to 29% of the rows were below the pad asked for. They also round distance to whole light years, and they leave out the searched system's own markets.
- A Spansh station record is about 94 kB, 80% of it module lists, so recipe 1 moved 14 to 27 MB per query. That is the main reason to keep stations local.
- Traders 179k to 387k ls from the arrival star make it into the top answers unless arrival distance is scored.
- Spansh's faction `state` filter missed nothing: no faction matched only through `active_states`.
- The dumps have faction `pendingStates`, which the API lacks, and they have no material trader kind: it has to be derived from the station's economy.

### Open items
- The galaxy rework (astromech-51ue) should build the local station, ring and hotspot file in CI from galaxy_stations.json.gz, like the star map preload. Check its real size on a full build.
- Score arrival distance and data age in every goal. When an answer is empty, widen the search once and then name the nearest match.
- Ask Spansh's maintainer before the engine ships, if live searches stay at more than a few per query.
