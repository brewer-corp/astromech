---
created_at: "2026-10-15T20:36:13.651279078+00:00"
id: "astromech-qgjp"
issue_type: "task"
labels:
- "size-m"
priority: "P0"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-16T18:35:59.069050092+00:00"
status: "done"
title: "Type the journal events the toolbox work needs"
updated_at: "2026-10-16T18:35:59.069050092+00:00"
---

## Goal

Today `astromech-journal` types about 15 events (FileHeader, LoadGame, Loadout, StoredShips, Location, FSDJump, Docked, Materials, EngineerCraft, EngineerProgress, …) and everything else is `Other`. Type the events below, and keep each event's raw JSON next to the typed form so the EDDN sender can forward the original keys.

## Done when

A parser test per event on a synthetic fixture passes, and a test shows the raw JSON of a parsed event is byte-for-byte what was read.

## Context

Events: `Market`, `Outfitting`, `Shipyard`, `Cargo`, `MarketBuy`/`MarketSell`, `CarrierJump`,
`FSDTarget`, `StartJump`, `NavRoute`, `NavRouteClear`, `DockingGranted`/`DockingDenied`,
`SupercruiseExit`/`SupercruiseEntry`, `FSSDiscoveryScan`, `FSSAllBodiesFound`,
`FSSSignalDiscovered`, `FSSBodySignals`, `SAASignalsFound`, `SAAScanComplete`, `Scan`,
`ScanBaryCentre`, `ScanOrganic`, `SellOrganicData`, `SellExplorationData`/
`MultiSellExplorationData`, `CodexEntry`, `ApproachSettlement`, `NavBeaconScan`,
`Missions`/`Mission*`, `Bounty`, `RedeemVoucher`, `ProspectedAsteroid`, `MiningRefined`,
`Powerplay*`, `Colonisation*`, `Carrier*`, `ShipLocker`, `Statistics`,
`Rank`/`Progress`/`Reputation`, `Interdicted`, `ShipTargeted`, `Scanned`, `FuelScoop`,
`ReceiveText`, `Touchdown`, `Disembark`.

Journal manual v35 (July 2022) is the last published version; check newer fields against
real logs. Partly built: the 15 events above.

## Close Reason

Typed the toolbox events as JournalEvent::Extra(ExtraEvent); JournalLine.raw_json keeps the original line; per-event and byte-for-byte tests pass.
