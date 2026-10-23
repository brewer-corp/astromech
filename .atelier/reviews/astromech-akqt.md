---
schema: atelier.review
schema_version: 1
id: astromech-akqt
title: 'R1 [low] scripts/gen-eddn-outfitting.py:38 (policy b): paid-ship armour (ELITE_V_COBRA_MKV, TYPE8, MANDALAY, CORSAIR, PYTHON_NX, PANTHER_MKII, LAKON_MINER, EXPLORER_NX, SMALL_COMBAT_01, and also MEDIUM_TRANSPORT_01 / Lynx Highliner, which the issue note omits; sku.csv: ''was free from day one'') is dropped. This matches the README''s literal rule and EDMC''s CAPI path, which would drop the same items when the CAPI carries those skus, so it is EDDN-conformant. But FDevIDs sku.csv says most of these requirements are ''not required anymore'', so astromech''s lists for every station (Jameson Memorial lists 42 such armour items) omit armour anyone can buy, and they disagree with EDMC journal-path senders for the same station. The real file lists no Panther or Lynx armour, so whether the armour list depends on the commander''s entitlements can''t be ruled out, and keeping the conservative drop is reasonable. Recommendation: defer. Ask EDCD (EDDN issue) whether ARX early-access ship skus count as commander status, and amend the note to include the Lynx.'
target: astromech-dlrb
identity: review-agent-dlrb-1
commit: de5a0a0e0d5870a35e9349a5a46fae2cce0517f7
created_at: 2026-10-23T00:05:52.549651281Z
event:
  event: finding
  recommendation: defer
---

R1 [low] scripts/gen-eddn-outfitting.py:38 (policy b): paid-ship armour (ELITE_V_COBRA_MKV, TYPE8, MANDALAY, CORSAIR, PYTHON_NX, PANTHER_MKII, LAKON_MINER, EXPLORER_NX, SMALL_COMBAT_01, and also MEDIUM_TRANSPORT_01 / Lynx Highliner, which the issue note omits; sku.csv: 'was free from day one') is dropped. This matches the README's literal rule and EDMC's CAPI path, which would drop the same items when the CAPI carries those skus, so it is EDDN-conformant. But FDevIDs sku.csv says most of these requirements are 'not required anymore', so astromech's lists for every station (Jameson Memorial lists 42 such armour items) omit armour anyone can buy, and they disagree with EDMC journal-path senders for the same station. The real file lists no Panther or Lynx armour, so whether the armour list depends on the commander's entitlements can't be ruled out, and keeping the conservative drop is reasonable. Recommendation: defer. Ask EDCD (EDDN issue) whether ARX early-access ship skus count as commander status, and amend the note to include the Lynx.
