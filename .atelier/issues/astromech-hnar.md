---
created_at: "2026-10-16T18:58:34.478833170+00:00"
id: "astromech-hnar"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-610m"
    type: "advances"
  - kind: "issue"
    id: "astromech-z4jb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Keep stored modules and ship locations current between visits"
updated_at: "2026-10-17T16:54:21.763824799+00:00"
---

## Goal

Keep the fleet's stored-module list and ship locations current between shipyard and outfitting visits. Today `StoredModules` is applied only when outfitting opens, so it goes stale after ModuleStore, ModuleRetrieve, ModuleSellRemote and FetchRemoteModule. The parser also drops TransferCost, TransferTime and Hot. A ShipyardTransfer marks a ship in transit to the current system, but never marks it arrived (the next StoredShips corrects it); use TransferTime to settle that.

## Done when

Journal fixtures with each of those events show the Ships page's stored-module list and ship locations changing as the game would, and the parser keeps TransferCost, TransferTime and Hot.

## Notes

- 2026-10-16 [note] Also: the remembered docked system is never reset at a session boundary, so a Loadout for another ship after a journal gap stores the old ship at a stale system. Consider triggering on ShipyardSwap/ShipyardBuy StoreShipID with the docked system instead (review of mycc round 2, N2).
