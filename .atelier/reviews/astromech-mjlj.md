---
schema: atelier.review
schema_version: 1
id: astromech-mjlj
title: '[low] crates/astromech-galaxy/src/stations.rs:249: none_offered counts every result on the page (''None of the N nearest stations selling X sell it turreted''), including stations that don''t sell X in any mount. So if Spansh ignored the name filter and only some stations sell X, N overstates the count. Counting results where any.offer(s) is Some would be exact. Cosmetic.'
target: astromech-970a
identity: review-agent (970a review)
commit: c672283ee1d91440319cda080c675a3610873109
created_at: 2026-10-17T00:33:50.510434599Z
event:
  event: finding
  recommendation: no_action
---

[low] crates/astromech-galaxy/src/stations.rs:249: none_offered counts every result on the page ('None of the N nearest stations selling X sell it turreted'), including stations that don't sell X in any mount. So if Spansh ignored the name filter and only some stations sell X, N overstates the count. Counting results where any.offer(s) is Some would be exact. Cosmetic.
