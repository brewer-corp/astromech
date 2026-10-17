---
schema: atelier.review
schema_version: 1
id: astromech-ulyt
title: 'R4 [low] crates/astromech-announce/src/sampling.rs:38-42: the once-per-sample key is (system, body, species, next). Re-logging the same species on the same body after an abandoned or completed run reproduces an earlier key (e.g. (sys,body,sp,2)), so ''Far enough for sample 2'' is silent the second time. Realistic only when a sample is restarted at the same spot''s body; the key should include something that changes on each Log (a Log counter in Progress) or clear on a Log. Also after an app restart mid-sample the first live Status re-cues once (cued starts empty); harmless.'
target: astromech-7vpa
identity: 'review-agent (sonnet, independent, part B: callouts, state, UI)'
commit: dcd988bc1ffbf9a6c4171078e4b65c1ee60fa48f
created_at: 2026-10-17T05:53:47.094144415Z
event:
  event: finding
  recommendation: defer
---

R4 [low] crates/astromech-announce/src/sampling.rs:38-42: the once-per-sample key is (system, body, species, next). Re-logging the same species on the same body after an abandoned or completed run reproduces an earlier key (e.g. (sys,body,sp,2)), so 'Far enough for sample 2' is silent the second time. Realistic only when a sample is restarted at the same spot's body; the key should include something that changes on each Log (a Log counter in Progress) or clear on a Log. Also after an app restart mid-sample the first live Status re-cues once (cued starts empty); harmless.
