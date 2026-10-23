---
created_at: "2026-10-23T00:51:16.444722324+00:00"
id: "astromech-r2t2"
evidence_type: "validation"
commit: "0f794799493b307a4e72cda7b9e8625817abea6a"
result: "blocked"
captured_at: "2026-10-23T00:51:16.444583726+00:00"
agent_identity: "validator-e8ux-2"
target:
  kind: "issue"
  id: "astromech-e8ux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e8ux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S6 (re-check 23 Oct, at 0f794799 = main be64422f): buildable part passes; live acceptance blocked. Claims: every live EDDN schema astromech should send is built from the live journal and validates; outfitting/2 now sent with commander-specific modules removed; EDSM upload builds. Schemas compared with EDDN's live list and EDMC: commodity/3, outfitting/2, shipyard/2, journal/1, fcmaterials_journal/1, navroute/1, approachsettlement, codexentry, dockinggranted, dockingdenied, fssallbodiesfound, fssbodysignals, fssdiscoveryscan, fsssignaldiscovered, navbeaconscan, scanbarycentre (all 16 have fixtures and builder tests); fcmaterials_capi is out of scope (cAPI) and blackmarket is retired. Ran fresh: cargo test -p astromech-eddn (149 unit incl. 34 EDSM, plus integration) and exploration/exobiology/intent/announce, 351 tests 0 fail; real_journals harness on a copy of ~/edjournal (32 files, 35955 lines): 2949 messages would be sent, 0 schema violations, 0 commander-name/FID leaks, 0 problems, nothing queued during catch-up; now includes outfitting-v2.0.json 1 (was 0 sent / 111 skipped on 22 Oct, astromech-t33g), plus commodity 1, shipyard 1, journal 2119, fsssignaldiscovered 414, dockinggranted 154, scanbarycentre 68, fssallbodiesfound 46, fssdiscoveryscan 41, approachsettlement 27, dockingdenied 27, navbeaconscan 25, fssbodysignals 24, codexentry 1; skipped: approachsettlement 2 (no location to cross-check), navroute 1 (file doesn't match its event). Independent scratch check (outside repo) of the real Outfitting.json (Jameson Memorial, 1084 items): the queued outfitting/2 carries 1008 modules = 1059 unique Hpt_/Int_/armour minus 51 commander-only by data/catalog/eddn-outfitting.json, 0 commander-only modules, no Int_PlanetApproachSuite. EDSM dry run queues 35929 events before the discard list, 18 held back. Limits: only the newest companion files exist, so outfitting/commodity/shipyard are proven on one real visit each; navroute and fcmaterials_journal validate only in unit tests (no matching real file); schema fixtures are the repo's copies, not re-fetched; the catalogue also drops paid-ship armour (e.g. Mandalay, Cobra Mk V) per EDDN's sku rule. Blocked: acceptance by the live EDDN gateway (test schemas, then a play session with no rejections) needs astromech-uxyq; a live EDSM upload needs astromech-shi5 (both needs-human). Nothing was sent to EDDN or EDSM."
updated_at: "2026-10-23T00:51:16.445416698+00:00"
---

S6 (re-check 23 Oct, at 0f794799 = main be64422f): buildable part passes; live acceptance blocked. Claims: every live EDDN schema astromech should send is built from the live journal and validates; outfitting/2 now sent with commander-specific modules removed; EDSM upload builds. Schemas compared with EDDN's live list and EDMC: commodity/3, outfitting/2, shipyard/2, journal/1, fcmaterials_journal/1, navroute/1, approachsettlement, codexentry, dockinggranted, dockingdenied, fssallbodiesfound, fssbodysignals, fssdiscoveryscan, fsssignaldiscovered, navbeaconscan, scanbarycentre (all 16 have fixtures and builder tests); fcmaterials_capi is out of scope (cAPI) and blackmarket is retired. Ran fresh: cargo test -p astromech-eddn (149 unit incl. 34 EDSM, plus integration) and exploration/exobiology/intent/announce, 351 tests 0 fail; real_journals harness on a copy of ~/edjournal (32 files, 35955 lines): 2949 messages would be sent, 0 schema violations, 0 commander-name/FID leaks, 0 problems, nothing queued during catch-up; now includes outfitting-v2.0.json 1 (was 0 sent / 111 skipped on 22 Oct, astromech-t33g), plus commodity 1, shipyard 1, journal 2119, fsssignaldiscovered 414, dockinggranted 154, scanbarycentre 68, fssallbodiesfound 46, fssdiscoveryscan 41, approachsettlement 27, dockingdenied 27, navbeaconscan 25, fssbodysignals 24, codexentry 1; skipped: approachsettlement 2 (no location to cross-check), navroute 1 (file doesn't match its event). Independent scratch check (outside repo) of the real Outfitting.json (Jameson Memorial, 1084 items): the queued outfitting/2 carries 1008 modules = 1059 unique Hpt_/Int_/armour minus 51 commander-only by data/catalog/eddn-outfitting.json, 0 commander-only modules, no Int_PlanetApproachSuite. EDSM dry run queues 35929 events before the discard list, 18 held back. Limits: only the newest companion files exist, so outfitting/commodity/shipyard are proven on one real visit each; navroute and fcmaterials_journal validate only in unit tests (no matching real file); schema fixtures are the repo's copies, not re-fetched; the catalogue also drops paid-ship armour (e.g. Mandalay, Cobra Mk V) per EDDN's sku rule. Blocked: acceptance by the live EDDN gateway (test schemas, then a play session with no rejections) needs astromech-uxyq; a live EDSM upload needs astromech-shi5 (both needs-human). Nothing was sent to EDDN or EDSM.
