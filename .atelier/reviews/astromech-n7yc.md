---
schema: atelier.review
schema_version: 1
id: astromech-n7yc
title: 'R6 [low] Docs and residue: docs/V1-DESIGN.md and ARCHITECTURE.md say the Biology page bodies ''come from the same fold as the System page''. It is the same type, but the System page folds newest journals via history while the Biology page reads GameState.system_bodies; the live System page is not fed by GameState (nothing in astromech-core uses SystemFold besides biology), so the reconciliation change (''a line from another system starts it afresh'', SystemFold::here) affects no landing/System page path today, and exploration tests pass. Also StatusSnap''s new GUI_SAA constant is unused. Reword the doc sentence; drop or use GUI_SAA.'
target: astromech-znhx
identity: 'review-agent (sonnet, independent, part B: callouts, state, UI)'
commit: dcd988bc1ffbf9a6c4171078e4b65c1ee60fa48f
created_at: 2026-10-17T05:53:47.277312497Z
event:
  event: finding
  recommendation: defer
---

R6 [low] Docs and residue: docs/V1-DESIGN.md and ARCHITECTURE.md say the Biology page bodies 'come from the same fold as the System page'. It is the same type, but the System page folds newest journals via history while the Biology page reads GameState.system_bodies; the live System page is not fed by GameState (nothing in astromech-core uses SystemFold besides biology), so the reconciliation change ('a line from another system starts it afresh', SystemFold::here) affects no landing/System page path today, and exploration tests pass. Also StatusSnap's new GUI_SAA constant is unused. Reword the doc sentence; drop or use GUI_SAA.
