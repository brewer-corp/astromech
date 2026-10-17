---
schema: atelier.review
schema_version: 1
id: astromech-8os1
title: '[low] crates/astromech-announce/src/lib.rs:415,421 (Engine::fire filters slots by enabled for both replayed() and react()): with [callouts.route_follow] enabled = false, Desk.here is never updated any more (the removed follow_route thread updated it on every system change). Effects: the Route page''s ''you are here'' marker (app/src/modules/route/View.svelte:71) goes stale, and Desk::set''s start-after-the-first-stop rule (route.rs set) uses a stale system. Either document that the toggle also stops tracking the current system, or let the Desk take here from the game state independently of the toggle.'
target: astromech-1135
identity: claude-reviewer-2
commit: 642cafe0a3bc7073503113bcc662608b12300bec
created_at: 2026-10-17T17:39:52.814795384Z
event:
  event: finding
  recommendation: defer
---

[low] crates/astromech-announce/src/lib.rs:415,421 (Engine::fire filters slots by enabled for both replayed() and react()): with [callouts.route_follow] enabled = false, Desk.here is never updated any more (the removed follow_route thread updated it on every system change). Effects: the Route page's 'you are here' marker (app/src/modules/route/View.svelte:71) goes stale, and Desk::set's start-after-the-first-stop rule (route.rs set) uses a stale system. Either document that the toggle also stops tracking the current system, or let the Desk take here from the game state independently of the toggle.
