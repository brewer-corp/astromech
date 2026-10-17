---
schema: atelier.review
schema_version: 1
id: astromech-un8d
title: 'R2 [low] crates/astromech-journal/src/lib.rs stored_ships: market_id falls back to the event''s MarketID for ShipsRemote entries too, so a remote ship missing ShipMarketID would link to the local shipyard. Real journals write ShipMarketID for every non-transit remote ship, and StarSystem already uses the same fallback, so no current failure; restricting the fallback to ShipsHere would make it exact. Recommendation: NO ACTION (optional tidy).'
target: astromech-qiey
identity: review-agent (opus, independent)
commit: 6c75c7e172df1b5811ae53b2b6f54fb737016ba5
created_at: 2026-10-17T04:54:50.530581277Z
event:
  event: finding
  recommendation: no_action
---

R2 [low] crates/astromech-journal/src/lib.rs stored_ships: market_id falls back to the event's MarketID for ShipsRemote entries too, so a remote ship missing ShipMarketID would link to the local shipyard. Real journals write ShipMarketID for every non-transit remote ship, and StarSystem already uses the same fallback, so no current failure; restricting the fallback to ShipsHere would make it exact. Recommendation: NO ACTION (optional tidy).
