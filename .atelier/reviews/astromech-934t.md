---
schema: atelier.review
schema_version: 2
id: astromech-934t
title: finding on astromech-xajc @7b1eb202
target: astromech-xajc
identity: Grok 4.6
commit: 7b1eb20293d9255df8c2a77a59f656e5c44b8421
created_at: 2026-11-10T06:01:14.505254625Z
event:
  event: finding
  recommendation: fix_now
---

astromech-q79q is Ready Work that a worker cannot finish from its body against the code. It wants ship name, jump range, fuel and cargo from the latest Loadout and Status. App.svelte polls coreStatus(); CoreStatus (app/src/lib/api/types/CoreStatus.ts) has no ship, fuel or cargo. Fuel is in the core (crates/astromech-status fuel_main) and is not on a window command. Ship name and max_jump_range are on engineering_fleet Ship. The issue names no component path (strategy has app/src/lib/ui/StatusBar.svelte) and no command. astromech-k6pl stores Text size with the other window settings; there is no window-settings store (settings go through core commands; UI prefs are ad-hoc localStorage), so persist format is left to the implementer. Name StatusBar.svelte, engineering_fleet plus the status/fuel command to add (or drop fuel/cargo until that core task exists), and Text size persistence (localStorage key vs core config).
