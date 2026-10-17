---
schema: atelier.review
schema_version: 1
id: astromech-lopq
title: '[medium] The unsold cartographic total is unvalidated against real sales. Two likely overcounts: the arrival star''s AutoScan is re-added on every revisit to a system already sold (crates/astromech-exploration/src/unsold.rs Scan arm), and NavBeaconDetail scans count at full value. Neither is confirmed by a game source. Suggested follow-up: on Sumeet''s logs, compare the fold''s estimate for each sold system with the BaseValue+Bonus of SellExplorationData/MultiSellExplorationData, then adjust the rules.'
target: astromech-tzc2
identity: 'review-agent (opus, independent, part A: values and data)'
commit: 8bfc4e716e943e3ce43a826fbe938883ac639b0a
created_at: 2026-10-17T05:18:01.573325136Z
event:
  event: finding
  recommendation: defer
---

[medium] The unsold cartographic total is unvalidated against real sales. Two likely overcounts: the arrival star's AutoScan is re-added on every revisit to a system already sold (crates/astromech-exploration/src/unsold.rs Scan arm), and NavBeaconDetail scans count at full value. Neither is confirmed by a game source. Suggested follow-up: on Sumeet's logs, compare the fold's estimate for each sold system with the BaseValue+Bonus of SellExplorationData/MultiSellExplorationData, then adjust the rules.
