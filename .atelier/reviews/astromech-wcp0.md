---
schema: atelier.review
schema_version: 1
id: astromech-wcp0
title: finding on astromech-1n9b @1a25cada
target: astromech-1n9b
identity: review-agent-1n9b-1
commit: 1a25cada2df231c244763fd65bbc7293ddc93667
created_at: 2026-11-04T23:26:37.028386267Z
event:
  event: finding
  recommendation: defer
---

minor: crates/astromech-intent/src/lib.rs:504 With 'in' now an origin word, a lookup ending in an ordinary phrase is read as a system name. 'where can I sell gold in bulk' gives origin 'bulk' and 'where can I sell painite in large quantities' gives 'large quantities'. Both execute and answer 'bulk. Spansh does not know that system.' (the name is kept as heard, so the sentence starts in lower case), at the cost of a Spansh request. This is plain and searches nothing wrong, so it is within Done when's 'or say plainly what was heard'. Suggest, when it next matters: capitalise the first letter of the heard name in lookup::unknown_system, and add common quantity words ('bulk', 'large', 'quantity', 'quantities') to the no-name list, so these search from here.
