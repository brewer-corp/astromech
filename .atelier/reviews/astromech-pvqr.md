---
schema: atelier.review
schema_version: 1
id: astromech-pvqr
title: '[low] crates/astromech-core/src/lookups.rs:47: any Error::Rejected (every 4xx, or a 200 body with an error field) from a by-name search is spoken as ''Spansh doesn''t know <system> yet'', even when the rejection was something else, such as a filter shape Spansh stopped accepting. This only happens when StarPos is unknown (rare after a Location/FSDJump). Matching the 400 message would be stricter.'
target: astromech-970a
identity: review-agent (970a review)
commit: c672283ee1d91440319cda080c675a3610873109
created_at: 2026-10-17T00:33:50.576561652Z
event:
  event: finding
  recommendation: no_action
---

[low] crates/astromech-core/src/lookups.rs:47: any Error::Rejected (every 4xx, or a 200 body with an error field) from a by-name search is spoken as 'Spansh doesn't know <system> yet', even when the rejection was something else, such as a filter shape Spansh stopped accepting. This only happens when StarPos is unknown (rare after a Location/FSDJump). Matching the 400 message would be stricter.
