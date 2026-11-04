---
schema: atelier.review
schema_version: 1
id: astromech-aj2w
title: finding on astromech-58a7 @4657397b
target: astromech-58a7
identity: review-agent-58a7-1
commit: 4657397bda73c11b22d0216501b72bed0d493a75
created_at: 2026-11-04T23:11:26.404764577Z
event:
  event: finding
  recommendation: defer
---

low: .atelier/issues/astromech-b8js.md:28 (Done when) lists the Arx-store ships whose armour EDCD must rule on but not the Anaconda Mk II, whose six armour symbols 9f08b20f now puts in the commander list (FDevIDs@c7c29f7 entitlement ELITE_V_ANACONDA_MKII, same pattern as CobraMkV). Dropping them is correct under the README sku rule and matches b8js's cautious choice, but when EDCD answers, the Anaconda Mk II could be missed. Add it to b8js's ship list.
