---
created_at: "2026-10-23T01:00:07.388889336+00:00"
id: "astromech-5k55"
issue_type: "feature"
labels:
- "size-m"
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates: []
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-10-23T01:23:59.639209924+00:00"
status: "done"
title: "Top-bar system search that plots from the current system"
updated_at: "2026-10-23T01:23:59.639209924+00:00"
---

## Goal

A system search in the top bar, on every page: type a name (Spansh autocomplete), pick it, and plot a neutron or exact route to it from the current system, or open it on Spansh or Inara.

## Done when

- The top bar has the search on every page outside onboarding; Enter or a click on a suggestion opens the plot options and plots.
- The Plot sub-panel is gone from the route rail and the Route page, so the closing bug cannot recur; the rail points to the top bar.
- No `From` field: plotting is from the current system only, and the core's `from` input is removed.
- Frontend tests cover search, plot and the rail; `npm run check` and the tests pass.

## Close Reason

Top-bar Plot to a system on every page; Plot panel gone from the rail and Route page; no From; reviewed and approved (astromech-x9pk, astromech-0dmb)
