---
created_at: "2026-10-17T05:10:05.005159290+00:00"
id: "astromech-d05l"
evidence_type: "test"
commit: "d6db7cbb87f06c0a5ecc21bbabab18ace43ef3d8"
result: "pass"
captured_at: "2026-10-17T05:10:05.005054124+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-ce4s"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-ce4s"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "At d6db7cb: astromech-history exploration tests: History fixture with two flown touchdowns on Explo A 2 (plus a walk, an SRV trip, a recalled-ship touchdown and a station pad, none counted) and a dropship to A 3 gives A 2 = 2, A 3 = 1; the system view filled by add_landings shows 2 and 1 with 'first footfall likely' on the undiscovered landable bodies of the unpopulated system, and no hint for the same body in a populated system. astromech-announce tests/arrival.rs: 'Landed on A 2 twice before.' on arrival, 'Landed on 3 bodies here before.', nothing by carrier or with no landings; core visits.rs reads landings from a real imported history; copy.rs keeps the lines within 8 words. App vitest shows 'Landed 2 times'; screenshot /tmp/p4ls/exploration-landings.png. Full gates (/tmp/landq/gates.sh) exit 0 at d6db7cb."
updated_at: "2026-10-17T05:10:05.005900605+00:00"
---

At d6db7cb: astromech-history exploration tests: History fixture with two flown touchdowns on Explo A 2 (plus a walk, an SRV trip, a recalled-ship touchdown and a station pad, none counted) and a dropship to A 3 gives A 2 = 2, A 3 = 1; the system view filled by add_landings shows 2 and 1 with 'first footfall likely' on the undiscovered landable bodies of the unpopulated system, and no hint for the same body in a populated system. astromech-announce tests/arrival.rs: 'Landed on A 2 twice before.' on arrival, 'Landed on 3 bodies here before.', nothing by carrier or with no landings; core visits.rs reads landings from a real imported history; copy.rs keeps the lines within 8 words. App vitest shows 'Landed 2 times'; screenshot /tmp/p4ls/exploration-landings.png. Full gates (/tmp/landq/gates.sh) exit 0 at d6db7cb.
