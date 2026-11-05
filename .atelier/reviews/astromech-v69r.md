---
schema: atelier.review
schema_version: 1
id: astromech-v69r
title: approval on astromech-nxz9 @326fd18e
target: astromech-nxz9
identity: reviewer-fable-nxz9
commit: 326fd18eebaf7994530bf4843f8fa9d107921807
created_at: 2026-11-05T08:46:21.372310387Z
event:
  event: approval
---

Reviewed 0669b643..326fd18e (nxz9) against its Done when, bi94, g8d5 items 4-7, ghyo, DESIGN.md (Places home, Lookup result row, States, Frame, Two Tab Rows Rule, Tabs, Buttons/Links), the PlacesHome and Finder mockups and design-system-places-home-after.png, AGENTS.md, CONTRIBUTING.md, ARCHITECTURE.md. Read App.svelte's moved() and hashchange rewrite, lib/places.ts, the five Places pages, MainInput, FinderPage, Home, SearchKit's onanswer effect and its ordering against the search effect, the deleted services/shipyard/outfitting pages, FindWant and lib/find.ts, the tests and the guide. Reran in app/: pnpm lint (0), pnpm check (452 files, 0 errors, 0 warnings), pnpm build, pnpm test (67 files, 432 passed); python3 scripts/check-guide.py (22 pages, 0 problems). In headless Chrome (scripts/ui on a Vite dev server, ?core=ready): #/services, #/services/black_market, #/outfitting/Pulse%20Laser/2F/Gimballed, #/outfitting/Frame%20Shift%20Drive?class=6&rating=A, #/outfitting, #/shipyard/Anaconda, #/shipyard each land on the right finder with the input filled in and one page-tab row (Places, Route); old links followed inside the window from the Places home, from #/route and from the Services finder are rewritten in place; after a Services search the home tile shows 'Black market · 0 jumps' and links to that search while the other tiles have no Lit text; Ctrl+K matches each finder, a service, a ship and a module to its Places route. git grep finds no other link to #/services, #/outfitting, #/shipyard, galaxy.md, FindWant or lib/find outside the tests, ARCHITECTURE's description and the two alpha release notes. Screenshots match the mockup minus the System tab and the unbuilt sections. Findings: two low defers (finder declaration lacks per-finder entries for 2hbw/kwoi; first-load pushState on an old link), two info no_actions (unknown finder keeps its address; the six decisions, branch matches). No fix_now.
