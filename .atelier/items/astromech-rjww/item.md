---
schema: atelier.item
schema_version: 3
id: astromech-rjww
title: Type the remaining Carrier* and Colonisation* journal events
status: draft
mission: null
labels: []
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: null
created_at: 2026-10-16T18:46:17.939983095Z
---

## Goal

Type the Carrier* and Colonisation* events qgjp left as Other: CarrierBuy, CarrierFinance, CarrierBankTransfer, CarrierDepositFuel, CarrierCrewServices, CarrierTradeOrder, CarrierDockingPermission, CarrierNameChange, CarrierModulePack, CarrierShipPack, CarrierDecommission, CarrierCancelDecommission, CarrierLocation, ColonisationBeaconDeployed, ColonisationSystemClaim, ColonisationSystemClaimRelease.

## Done when

- [ ] A parser test per event on a synthetic fixture passes, and the original raw JSON is still kept.

## Links

Relationships of the old record that the new format has no field for.

- related: astromech-tpdb (Journal and companion-file coverage)
