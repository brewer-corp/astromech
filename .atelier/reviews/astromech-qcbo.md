---
schema: atelier.review
schema_version: 2
id: astromech-qcbo
title: finding on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:21.063839147Z
event:
  event: finding
  recommendation: defer
---

low: app/src/preview/fixtures/plot.json:41 (96dv) The trimmed Spansh neutron sample says jumps 128 while its recorded legs now give 0/10/1, so a preview neutron plot shows 128 jumps and the route strip 11 left. The Close Reason lists it as an open item but no issue is filed. Fix later: file an issue to re-record or re-trim the sample so its total matches its legs.
