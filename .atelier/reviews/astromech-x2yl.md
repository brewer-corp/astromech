---
schema: atelier.review
schema_version: 1
id: astromech-x2yl
title: 'Review of d850f53: not approved, one FIX NOW (last-delivered rule untested). Checked and fine: progress, what is left and where to buy dispatch to the app''s functions (astromech_colonisation::from_journal_dir with Cargo.json, shopping_list for loads at the Loadout capacity, so the loads maths is identical to the page''s; astromech_market::list::plan_search with limit 12 like colonisation_buy). Filters come from search.filters.colonisation.buy (LOOKUP, the page''s SearchKit key); the test asserts maxDistance 40 reaches Ardent. The project rule (docked, else last delivered, else last depot update) intentionally differs from the page''s default selection (newest depot update first); the page lets the commander pick. New deps tools/core -> astromech-colonisation pass check-deps and follow the tools -> market/galaxy precedent. Work runs on a spawned thread. Copy test covers long names, large amounts, cargo and the no-answer lines; no contractions or ''I''. Docs updated (ARCHITECTURE crate map, V1-DESIGN Trade: Colonisation row). cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; merges cleanly with main, astromech-6sf7 and astromech-8joi. CI on PR #97: Linux green, Windows jobs pending at review time.'
target: astromech-3ez9
identity: review-agent (opus, independent)
commit: d850f53222d41a4a69ed5c90aa8306d7f60c2ed4
created_at: 2026-10-17T08:18:09.950142923Z
event:
  event: review
---

Review of d850f53: not approved, one FIX NOW (last-delivered rule untested). Checked and fine: progress, what is left and where to buy dispatch to the app's functions (astromech_colonisation::from_journal_dir with Cargo.json, shopping_list for loads at the Loadout capacity, so the loads maths is identical to the page's; astromech_market::list::plan_search with limit 12 like colonisation_buy). Filters come from search.filters.colonisation.buy (LOOKUP, the page's SearchKit key); the test asserts maxDistance 40 reaches Ardent. The project rule (docked, else last delivered, else last depot update) intentionally differs from the page's default selection (newest depot update first); the page lets the commander pick. New deps tools/core -> astromech-colonisation pass check-deps and follow the tools -> market/galaxy precedent. Work runs on a spawned thread. Copy test covers long names, large amounts, cargo and the no-answer lines; no contractions or 'I'. Docs updated (ARCHITECTURE crate map, V1-DESIGN Trade: Colonisation row). cargo test -p astromech-intent -p astromech-tools -p astromech-core passes; merges cleanly with main, astromech-6sf7 and astromech-8joi. CI on PR #97: Linux green, Windows jobs pending at review time.
