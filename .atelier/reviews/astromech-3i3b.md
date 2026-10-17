---
schema: atelier.review
schema_version: 1
id: astromech-3i3b
title: 'app/src/modules/engineering/Build.svelte:91-92: dropShoppingList fires only when the ship being viewed fails to evaluate with an error containing ''not in the fleet'' (a string match on main.rs:532''s message). A ship sold or stored away while the app is closed, or never opened again in Build, keeps its list in the core (callouts.arrival.shopping) indefinitely. Fix later: on fleet load, send an empty list for each saved plan whose ship is no longer in the fleet (or have the core prune lists by the fleet), and match an error code rather than text.'
target: astromech-77a1
identity: review-agent (77a1 fix review, sonnet)
commit: 6256a28557b84d63e9e7158bc0d6f516b24e2647
created_at: 2026-10-17T02:18:08.663556165Z
event:
  event: finding
  recommendation: defer
---

app/src/modules/engineering/Build.svelte:91-92: dropShoppingList fires only when the ship being viewed fails to evaluate with an error containing 'not in the fleet' (a string match on main.rs:532's message). A ship sold or stored away while the app is closed, or never opened again in Build, keeps its list in the core (callouts.arrival.shopping) indefinitely. Fix later: on fleet load, send an empty list for each saved plan whose ship is no longer in the fleet (or have the core prune lists by the fleet), and match an error code rather than text.
