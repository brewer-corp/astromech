---
schema: atelier.review
schema_version: 1
id: astromech-9mvu
title: 'Low: the voice answer can differ from the app''s. The app''s galaxy_nearby (app/src-tauri/src/main.rs:677-680) calls g.record_visits(journal visits) before Galaxy::nearby. lookups::faction_answer does not, so a state the commander saw on arrival is used only if the app has searched since. Otherwise voice says ''Reported N days ago'' with Spansh''s older state where the app says ''Seen on your visit''. Same search function and filters otherwise.'
target: astromech-6sf7
identity: review-agent (opus, independent)
commit: cd662e2540a539a7030be7854e9b5e5ba1ea4ab7
created_at: 2026-10-17T08:17:36.645323721Z
event:
  event: finding
  recommendation: defer
---

Low: the voice answer can differ from the app's. The app's galaxy_nearby (app/src-tauri/src/main.rs:677-680) calls g.record_visits(journal visits) before Galaxy::nearby. lookups::faction_answer does not, so a state the commander saw on arrival is used only if the app has searched since. Otherwise voice says 'Reported N days ago' with Spansh's older state where the app says 'Seen on your visit'. Same search function and filters otherwise.
