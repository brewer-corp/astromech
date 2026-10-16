---
schema: atelier.review
schema_version: 1
id: astromech-tr4g
title: '[medium] crates/astromech-galaxy/src/lib.rs:261 with remote.rs:106-121 (HGE ranking, astromech-bwmz): hge_score only re-ranks Spansh''s 50 nearest systems matching the faction filter, with no population filter. Checked live on Spansh, 16 Oct 2026: those 50 Federation systems lie within 15-20 ly in the bubble. From Deciat only 1 of them has over a billion people (HIP 38129, 17 ly). The same search with population {comparison ''<=>'', value [1e9, 1e11]} returns 50 such systems within 67 ly (2117 in total). So the Done-when case, a 2-billion system 30 ly away, is outside the candidate set in real use, and the ranked list is mostly systems under a billion. The fixed-position test passes only because its cache holds a hand-written three-system answer. The evidence (astromech-0nml) notes this limit but the issue closed anyway. Fix: for by_population queries, also fetch a page with Spansh''s population filter (at least HGE_POPULATION) and merge it with the nearest page before scoring. Merging keeps sub-billion state systems for Boom, War and Outbreak. Test the request shape against a recorded answer.'
target: astromech-sfys
identity: review-agent (sfys review)
commit: 8c44864ffb8b65cc78872de4fd3c564004816b0a
created_at: 2026-10-16T22:31:57.937370224Z
event:
  event: finding
  recommendation: fix_now
---

[medium] crates/astromech-galaxy/src/lib.rs:261 with remote.rs:106-121 (HGE ranking, astromech-bwmz): hge_score only re-ranks Spansh's 50 nearest systems matching the faction filter, with no population filter. Checked live on Spansh, 16 Oct 2026: those 50 Federation systems lie within 15-20 ly in the bubble. From Deciat only 1 of them has over a billion people (HIP 38129, 17 ly). The same search with population {comparison '<=>', value [1e9, 1e11]} returns 50 such systems within 67 ly (2117 in total). So the Done-when case, a 2-billion system 30 ly away, is outside the candidate set in real use, and the ranked list is mostly systems under a billion. The fixed-position test passes only because its cache holds a hand-written three-system answer. The evidence (astromech-0nml) notes this limit but the issue closed anyway. Fix: for by_population queries, also fetch a page with Spansh's population filter (at least HGE_POPULATION) and merge it with the nearest page before scoring. Merging keeps sub-billion state systems for Boom, War and Outbreak. Test the request shape against a recorded answer.
