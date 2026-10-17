---
schema: atelier.review
schema_version: 1
id: astromech-qn18
title: 'R3 The R7 rescope is sound. Ardent cuts at 1000 rows; on the sell side placeholders carry the high price, so they sort first and the rows cut are the worst-priced real markets. Not counting placeholders would clear truncated exactly when real markets were cut. The pinning test (crates/astromech-market/tests/recorded.rs placeholders_use_up_ardents_rows_so_a_full_answer_is_truncated) is right: 1000 rows with 600 placeholders are truncated, only the 400 real rows are offered, and 999 rows are not truncated. It fails under ''> NEARBY_LIMIT'' or under counting only real rows. It pins existing behaviour rather than a change, as the rescope intends.'
target: astromech-243w
identity: review-agent (opus, independent)
commit: 6a3910d6d36c5ba561d282fa63c537001b31e7cd
created_at: 2026-10-17T06:09:48.133099565Z
event:
  event: finding
  recommendation: no_action
---

R3 The R7 rescope is sound. Ardent cuts at 1000 rows; on the sell side placeholders carry the high price, so they sort first and the rows cut are the worst-priced real markets. Not counting placeholders would clear truncated exactly when real markets were cut. The pinning test (crates/astromech-market/tests/recorded.rs placeholders_use_up_ardents_rows_so_a_full_answer_is_truncated) is right: 1000 rows with 600 placeholders are truncated, only the 400 real rows are offered, and 999 rows are not truncated. It fails under '> NEARBY_LIMIT' or under counting only real rows. It pins existing behaviour rather than a change, as the rescope intends.
