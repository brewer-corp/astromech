---
created_at: "2026-11-07T03:00:11.045308872+00:00"
id: "astromech-woqv"
issue_type: "task"
labels:
- "size-m"
priority: "P1"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
status: "todo"
title: "Route data for the timeline"
updated_at: "2026-11-07T03:00:11.045308872+00:00"
---

## Goal

The core gives the Route page what the timeline shows: each jump's system and star (class, scoopable), the commander's place in the current leg, inhabited systems' allegiance, government, security, population and controlling faction state, and how the route was plotted (destination, plotter, engine, efficiency, range) so it can be replotted.

## Done when

- [ ] Waypoints carry their jumps' systems and star classes from the local neutron plotter and both Spansh plotters; imported routes carry what the CSV gives.
- [ ] The route view says the commander's position in the current leg, from the journal.
- [ ] Inhabited stops' details come from Spansh, fetched once per stop and cached; a failed lookup leaves the stop without them.
- [ ] The route stores its plot settings and route_replot plots again from the current system with them; tests cover it.
- [ ] fakecore has a route with all of it, for the preview.
