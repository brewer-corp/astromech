---
schema: atelier.review
schema_version: 1
id: astromech-g26m
title: 'Approved at 6c75c7e. Read the full diff against main and the issue. Checked: StoredShip.market_id parsing (ShipMarketID, shipyard MarketID for ShipsHere, None in transit), FleetBuilder carrying the last dock''s market id to a ship left by a swap (Docked, and Location only when docked) and clearing it on ShipTransfer; the new ExtLinks component (Spansh station by market id or name, Inara by station name, system links otherwise; Inara hidden when only a market id is known) used in the Station docked header and the Build stored-ship line, replacing the old inline buttons and their CSS; tests for the Station header links (4 URLs), the stored-ship station link, the fleet market ids and the 40-module URL-length test. Ran cargo test -p astromech-engineering (25 + 9 pass; URL lengths 6374/5899), -p astromech-journal (29 pass) and vitest for station, engineering and links (30 pass). Full gates taken from the author''s report, not rerun. Findings R1 (defer: Windows URL-length path unverified), R2 (no action), R3 (defer: no undocked current-system links) do not block.'
target: astromech-qiey
identity: review-agent (opus, independent)
commit: 6c75c7e172df1b5811ae53b2b6f54fb737016ba5
created_at: 2026-10-17T04:54:56.794587061Z
event:
  event: approval
---

Approved at 6c75c7e. Read the full diff against main and the issue. Checked: StoredShip.market_id parsing (ShipMarketID, shipyard MarketID for ShipsHere, None in transit), FleetBuilder carrying the last dock's market id to a ship left by a swap (Docked, and Location only when docked) and clearing it on ShipTransfer; the new ExtLinks component (Spansh station by market id or name, Inara by station name, system links otherwise; Inara hidden when only a market id is known) used in the Station docked header and the Build stored-ship line, replacing the old inline buttons and their CSS; tests for the Station header links (4 URLs), the stored-ship station link, the fleet market ids and the 40-module URL-length test. Ran cargo test -p astromech-engineering (25 + 9 pass; URL lengths 6374/5899), -p astromech-journal (29 pass) and vitest for station, engineering and links (30 pass). Full gates taken from the author's report, not rerun. Findings R1 (defer: Windows URL-length path unverified), R2 (no action), R3 (defer: no undocked current-system links) do not block.
