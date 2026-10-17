---
created_at: "2026-10-17T16:54:23.622988405+00:00"
id: "astromech-yin0"
issue_type: "task"
labels:
- "alpha"
- "size-m"
workflow_branch:
  branch_base: main
  work_branch: task/astromech-yin0
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "in_progress"
title: "Replay real journals through every module"
updated_at: "2026-10-17T21:32:24.296931911+00:00"
---

## Goal

Run Sumeet's real journal history (years of events the fixtures don't cover) through the core and every module, and fix crashes, panics, parse failures and implausible results.

## Done when

A replay of the full folder runs clean (no panics, every unknown event counted and reviewed), each fixed problem has a fixture test, and the import time is recorded on astromech-tqdh.

## Context

Needs a copy of Sumeet's `Saved Games\Frontier Developments\Elite Dangerous` folder on the dev machine, kept outside the repo (it holds his commander name and history); only small, anonymised excerpts become fixtures. Also gives astromech-tqdh its import timing.

## Notes

- 2026-10-17 [note] Raw-event review (real logs, 35,955 lines, 137 event types, 0 malformed): 78 types typed, 59 kept as raw JSON (5,864 lines); ShipyardBuy without a trade-in and non-material EngineerContribution stay raw by design. Of the 59, read by name elsewhere: Music, UnderAttack, LaunchDrone, CollectCargo, EjectCargo, Commander, Repair, ShipyardSwap, Shutdown, Backpack, SuitLoadout, BuySuit, BuyWeapon, CreateSuitLoadout, SwitchSuitLoadout, ModuleInfo, ShipyardNew, CommunityGoalReward, SearchAndRescue, Died. Not needed by any toolbox module: costs (RefuelAll, RepairAll, BuyAmmo, BuyDrones, SellDrones, RestockVehicle, PayFines, BuyTradeData, BuyMicroResources: the next Cargo/ShipLocker snapshot carries the result), HUD-only state (ShieldState, HeatWarning, HullDamage, ReservoirReplenished), flight and SRV/on-foot transitions (SupercruiseDestinationDrop, USSDrop, DockingRequested, Liftoff, LaunchSRV, DockSRV, Embark, BookTaxi, GameModeChange, Resurrect), crime and combat (CommitCrime, CrimeVictim, FactionKillBond, EscapeInterdiction), and DatalinkScan, DataScanned, MaterialDiscovered, AsteroidCracked, Promotion, ModuleSwap, ModuleSell, LoadoutEquipModule (a Loadout follows every outfitting change). Matter but covered: CommunityGoal and CommunityGoalJoin (astromech-yfvv, Community goals near me), ShipRedeemed/ShipyardRedeem (a ship acquired without ShipyardNew: folded into astromech-vett). No new event needed a typed form.
