---
schema: atelier.review
schema_version: 1
id: astromech-42xh
title: 'Newest-per-body partitions by (cmdr, kind, body_name, body_id) with coalesce; a body scanned once with and once without BodyID would be read twice. Harmless: fold keys by name, newest still applied last. Honks are all read (one per visit), fold keeps the last; all_found is sticky by design.'
target: astromech-lelp
identity: review-agent (sonnet, independent)
commit: ee185ff1f9f94135a46db0e554218e19d0cfa189
created_at: 2026-10-17T08:48:05.864084990Z
event:
  event: finding
  recommendation: no_action
---

Newest-per-body partitions by (cmdr, kind, body_name, body_id) with coalesce; a body scanned once with and once without BodyID would be read twice. Harmless: fold keys by name, newest still applied last. Honks are all read (one per visit), fold keeps the last; all_found is sticky by design.
