---
created_at: "2026-10-22T23:48:52.430236019+00:00"
id: "astromech-beyd"
evidence_type: "validation"
commit: "327c059301ee2d1a145df8472e7f626f461ac933"
result: "pass"
captured_at: "2026-10-22T23:48:52.430198870+00:00"
agent_identity: "validator-e8ux"
target:
  kind: "issue"
  id: "astromech-e8ux"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-e8ux"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "S1 Outfitting: pass. At 327c059 (main 8866d1f): fresh CARGO_BUILD_JOBS=4 cargo test --workspace --no-fail-fast = 1124 passed, 0 failed, 1 ignored (needs the speech model); galaxy stations pad tests (has_large_pad set for a large ship, not for a Krait) included. Live, through the core Assistant with real HTTP (scratch harness outside the repo, game state folded from a copy of the real journals): find.module 'where's the nearest 5A frame shift drive', find.ship 'nearest Krait Mk II', find.service 'nearest interstellar factors' each answered with station, system, distance and age from Spansh from the journal's current system; from Sol, 5A FSD -> Abraham Lincoln (Sol), Type-9 -> Galileo (Sol). Pad filter follows the ship: from Sol 'nearest black market' gives Rescue Ship - Li Qing Jao (Sol) for a Sidewinder and Bakshi Manufacturing Complex (Alpha Centauri) for a Type-9. App (browser preview, ?core=ready): #/outfitting, #/shipyard, #/services render the search kit with a typed FROM origin, pad S/M/L, data age and results; pnpm test 380/380. Limits: the Tauri shell can't build on Linux (CI on main 8866d1f green); app checked on preview fixtures, not live; voice searches from the current system only (typed origin is app-only; saying 'near Sol' misroutes, filed astromech-j75v). In-game purchase check astromech-4su3 is still open but is not the criterion's stated proof."
updated_at: "2026-10-22T23:48:52.430505191+00:00"
---

S1 Outfitting: pass. At 327c059 (main 8866d1f): fresh CARGO_BUILD_JOBS=4 cargo test --workspace --no-fail-fast = 1124 passed, 0 failed, 1 ignored (needs the speech model); galaxy stations pad tests (has_large_pad set for a large ship, not for a Krait) included. Live, through the core Assistant with real HTTP (scratch harness outside the repo, game state folded from a copy of the real journals): find.module 'where's the nearest 5A frame shift drive', find.ship 'nearest Krait Mk II', find.service 'nearest interstellar factors' each answered with station, system, distance and age from Spansh from the journal's current system; from Sol, 5A FSD -> Abraham Lincoln (Sol), Type-9 -> Galileo (Sol). Pad filter follows the ship: from Sol 'nearest black market' gives Rescue Ship - Li Qing Jao (Sol) for a Sidewinder and Bakshi Manufacturing Complex (Alpha Centauri) for a Type-9. App (browser preview, ?core=ready): #/outfitting, #/shipyard, #/services render the search kit with a typed FROM origin, pad S/M/L, data age and results; pnpm test 380/380. Limits: the Tauri shell can't build on Linux (CI on main 8866d1f green); app checked on preview fixtures, not live; voice searches from the current system only (typed origin is app-only; saying 'near Sol' misroutes, filed astromech-j75v). In-game purchase check astromech-4su3 is still open but is not the criterion's stated proof.
