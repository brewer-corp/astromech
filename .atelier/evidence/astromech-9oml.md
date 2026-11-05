---
created_at: "2026-11-05T16:01:40.021933451+00:00"
id: "astromech-9oml"
evidence_type: "validation"
commit: "f46105564cdb8644dfb788492e83ee1cb6708270"
result: "pass"
captured_at: "2026-11-05T16:01:40.021818957+00:00"
path: "docs/design/validation/strip-materials-route-1100.png"
agent_identity: "validator-fable-kyqw"
target:
  kind: "issue"
  id: "astromech-kyqw"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-kyqw"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "Claim 1: every page except onboarding shows the route strip; the Galaxy-only rail is gone. Vite dev server with the fake core (?core=ready&route=exact), scripts/ui headless Chrome at 1100 and 1600 px: all 24 routed pages (controller, keybinds, presets, keytester, engineering, stored, materials, odyssey, unlocks, places, route, trade, colonisation, exploration, exobiology, powerplay, logbook, session, mining, station, settings, datasharing, diagnostics, voicetester) render a 36 px .strip reading 'Next | Arietis Sector LD-H a11-1 | 4 of 10 | To | Maia | 7 jumps left'; with no route it reads 'No route | FIND OR PLOT' and that button focuses the header combobox; the copy icon works. #/setup and ?core=new (not onboarded) render onboarding with no strip. No aside[aria-label=Route] on any page and modules/route/Rail.svelte is gone from the tree (git grep). Header is one 64 px row from 1580 px and two rows (64+44) at 1560 and 1100. Screenshots docs/design/validation/strip-*.png, onboarding-setup-no-strip-1100.png. Tests: Strip.test.ts (9) and App.test.ts 'is under the header on every page, Galaxy's included, and gone in onboarding' pass. Proof limit: Off route, Refuel, clipboard-failure, Route finished and Paused+Resume strip states cannot be produced in the preview (the fixture's commander sits on-route at waypoint 3 with no fuel flag); they rest on Strip.test.ts only. Under ?fail=all the strip shows the red '! Could not read the route … TRY AGAIN' hard error."
updated_at: "2026-11-05T16:01:40.022506470+00:00"
---

Claim 1: every page except onboarding shows the route strip; the Galaxy-only rail is gone. Vite dev server with the fake core (?core=ready&route=exact), scripts/ui headless Chrome at 1100 and 1600 px: all 24 routed pages (controller, keybinds, presets, keytester, engineering, stored, materials, odyssey, unlocks, places, route, trade, colonisation, exploration, exobiology, powerplay, logbook, session, mining, station, settings, datasharing, diagnostics, voicetester) render a 36 px .strip reading 'Next | Arietis Sector LD-H a11-1 | 4 of 10 | To | Maia | 7 jumps left'; with no route it reads 'No route | FIND OR PLOT' and that button focuses the header combobox; the copy icon works. #/setup and ?core=new (not onboarded) render onboarding with no strip. No aside[aria-label=Route] on any page and modules/route/Rail.svelte is gone from the tree (git grep). Header is one 64 px row from 1580 px and two rows (64+44) at 1560 and 1100. Screenshots docs/design/validation/strip-*.png, onboarding-setup-no-strip-1100.png. Tests: Strip.test.ts (9) and App.test.ts 'is under the header on every page, Galaxy's included, and gone in onboarding' pass. Proof limit: Off route, Refuel, clipboard-failure, Route finished and Paused+Resume strip states cannot be produced in the preview (the fixture's commander sits on-route at waypoint 3 with no fuel flag); they rest on Strip.test.ts only. Under ?fail=all the strip shows the red '! Could not read the route … TRY AGAIN' hard error.
