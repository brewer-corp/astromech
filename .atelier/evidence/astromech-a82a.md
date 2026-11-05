---
created_at: "2026-11-05T16:01:41.592963581+00:00"
id: "astromech-a82a"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:41.592921502+00:00"
path: "docs/design/validation/page-powerplay-1600.png"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Claim 4: the Powerplay, Exploration and Colonisation lookups stay on their pages with the result row and actions; Exploration's embedded nearest-services lookup becomes a link into the Services finder; Colonisation stays as it is otherwise. Observed with ?core=ready&route=exact at 1600 px: Powerplay's 'Where to work' side panel lists 5 compact rows (Jumps | System, then Add stop and Set destination icons, 5 of each); Colonisation lists 22 rows under heads 'Sells | Station | Stock | Distance | Jumps | Pad | Arrival | Updated' with 22 Add stop and 22 Set destination buttons and its page otherwise unchanged (tab under Trade); Exploration's System page has no lookup and links 'Nearest Universal Cartographics -> #/places/services/universal_cartographics' and 'Nearest Vista Genomics -> #/places/services/vista_genomics'; Trade's Commodities page links Sell, Buy and Sell my cargo into Places (#/places/sell/painite, #/places/buy/painite, #/places/sell-cargo) and keeps trade routes; Materials links the five raw-site kinds into #/places/raw-sites/<kind> and, with Pharmaceutical Isolators selected, 'Find the nearest systems -> #/places/faction-states/pharma'. Screenshots docs/design/validation/page-{powerplay,colonisation,exploration,trade,materials}-1600.png, page-materials-faction-link-1600.png. Tests: Targets.test.ts, Colonies.test.ts, trade/View.test.ts link test, App.test.ts 'links each page a lookup moved from into its finder' pass. Proof limit: the Engineering build plan's material-source lookup (lib/ui/Nearby.svelte, the same SearchKit) was not reached in the preview because the sample ships carry no plan with sources; it rests on SearchKit.test.ts."
updated_at: "2026-11-05T16:01:41.593427065+00:00"
---

Claim 4: the Powerplay, Exploration and Colonisation lookups stay on their pages with the result row and actions; Exploration's embedded nearest-services lookup becomes a link into the Services finder; Colonisation stays as it is otherwise. Observed with ?core=ready&route=exact at 1600 px: Powerplay's 'Where to work' side panel lists 5 compact rows (Jumps | System, then Add stop and Set destination icons, 5 of each); Colonisation lists 22 rows under heads 'Sells | Station | Stock | Distance | Jumps | Pad | Arrival | Updated' with 22 Add stop and 22 Set destination buttons and its page otherwise unchanged (tab under Trade); Exploration's System page has no lookup and links 'Nearest Universal Cartographics -> #/places/services/universal_cartographics' and 'Nearest Vista Genomics -> #/places/services/vista_genomics'; Trade's Commodities page links Sell, Buy and Sell my cargo into Places (#/places/sell/painite, #/places/buy/painite, #/places/sell-cargo) and keeps trade routes; Materials links the five raw-site kinds into #/places/raw-sites/<kind> and, with Pharmaceutical Isolators selected, 'Find the nearest systems -> #/places/faction-states/pharma'. Screenshots docs/design/validation/page-{powerplay,colonisation,exploration,trade,materials}-1600.png, page-materials-faction-link-1600.png. Tests: Targets.test.ts, Colonies.test.ts, trade/View.test.ts link test, App.test.ts 'links each page a lookup moved from into its finder' pass. Proof limit: the Engineering build plan's material-source lookup (lib/ui/Nearby.svelte, the same SearchKit) was not reached in the preview because the sample ships carry no plan with sources; it rests on SearchKit.test.ts.
