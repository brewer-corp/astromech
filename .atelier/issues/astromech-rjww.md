---
created_at: "2026-10-16T18:46:17.939983095+00:00"
id: "astromech-rjww"
issue_type: "task"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-tpdb"
    type: "related"
schema: "atelier.issue"
schema_version: 2
status: "backlog"
title: "Type the remaining Carrier* and Colonisation* journal events"
updated_at: "2026-10-16T18:46:17.939983095+00:00"
---

## Goal

Type the Carrier* and Colonisation* events qgjp left as Other: CarrierBuy, CarrierFinance, CarrierBankTransfer, CarrierDepositFuel, CarrierCrewServices, CarrierTradeOrder, CarrierDockingPermission, CarrierNameChange, CarrierModulePack, CarrierShipPack, CarrierDecommission, CarrierCancelDecommission, CarrierLocation, ColonisationBeaconDeployed, ColonisationSystemClaim, ColonisationSystemClaimRelease.

## Done when

A parser test per event on a synthetic fixture passes, and the original raw JSON is still kept.
