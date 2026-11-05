---
created_at: "2026-10-24T23:07:07.848384897+00:00"
id: "astromech-51ue"
issue_type: "feature"
labels:
- "size-l"
workflow_branch:
  branch_base: main
  work_branch: feature/astromech-51ue
priority: "P1"
relationships:
  blocks: []
  children:
  - kind: "issue"
    id: "astromech-1d92"
  - kind: "issue"
    id: "astromech-2hbw"
  - kind: "issue"
    id: "astromech-f7i0"
  - kind: "issue"
    id: "astromech-g8d5"
  - kind: "issue"
    id: "astromech-jp36"
  - kind: "issue"
    id: "astromech-kwoi"
  - kind: "issue"
    id: "astromech-kyqw"
  - kind: "issue"
    id: "astromech-nxz9"
  - kind: "issue"
    id: "astromech-rz0f"
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-7q0z"
    type: "advances"
  - kind: "issue"
    id: "astromech-bi94"
    type: "derived_from"
  - kind: "issue"
    id: "astromech-c4i0"
    type: "blocked_by"
  - kind: "issue"
    id: "astromech-yn6y"
    type: "blocked_by"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T16:11:06.126608106+00:00"
status: "done"
title: "Galaxy rework: route strip, Places finders, one search box"
updated_at: "2026-11-05T16:11:06.126608106+00:00"
---

## Goal

Replace the scattered place-finding pages and the header plot box with the model decided in the galaxy direction decision (astromech-bi94): a persistent route shown on every page, Places as a family of finders with Add stop and Set destination on every result, and one "Find or plot" box shared with Ctrl+K. The linked-place engine, recipes, "Find near this" and Find along route are the follow-on increment astromech-os70.

## Done when

On the Linux build, shown by the children's tests and `scripts/ui` screenshots, an independent review of the branch, and the validation child's receipts:

- Every page except onboarding shows the route strip; the Galaxy-only rail is gone.
- Every lookup result offers Add stop and Set destination, and no action replaces the followed route without saying so and offering Resume.
- Galaxy opens on Places: a home page of finder tiles, one page and URL per finder, covering today's Services, Outfitting, Shipyard, material traders, Sell, Buy, Sell my cargo, raw sites and faction states, plus Mining hotspots; old URLs and palette entries still open them.
- The Powerplay, Exploration and Colonisation lookups stay on their pages and get the result row and the actions; Exploration's embedded nearest-services lookup (System page) becomes a link into the Services finder, like the Trade and Materials embeds. Colonisation stays as it is otherwise.
- Every finder shows the DESIGN.md Lookup result row and States.
- The header has one Find or plot box, also reached with Ctrl+K, that plots or adds a system and opens a finder from a phrase.
- Each new finder has a voice query, and the guide describes what shipped (`scripts/check-guide.py` passes).

The play check on Windows next to the game is astromech-cxgq (needs-human), not part of this Done when.

## Background

Decision astromech-bi94 sets the direction and decision astromech-g8d5 settles the design choices DESIGN.md left open; DESIGN.md (Route strip, Lookup result row, Find or plot, Places home, States) and the `#/mockup/ds-*` mockups set the look. Shared controls and icons come from astromech-pey5. Children share this issue's branch.

## Close Reason

Galaxy rework delivered on the Linux build. Children: rz0f (Add stop and Set destination replace Go on every lookup result; core route_add; Set destination parks the replaced route, Resume brings it back), 1d92 (36 px route strip on every page but onboarding with next system, n of N, flags, jumps or stops left, warnings; the Galaxy-only rail is gone), jp36 (DESIGN.md Lookup result row and States in the shared search kit, shared Failure line, compact row in narrow panels), nxz9 (Places home and one page and URL per finder; Galaxy tabs Places and Route; old URLs redirect), f7i0 (Mining hotspots finder ranked by hotspots over the whole radius in at most 4 Spansh requests, with its voice query), kwoi (Sell, Buy, Sell my cargo, Raw sites and Faction states moved into Places; Trade, Materials and Exploration link in), 2hbw (one Find or plot header box with Ctrl+K: systems with Plot/Add stop/Set destination, finder phrases from each finder's declared inputs; header takes a second tab row below about 1580 px), and decision records g8d5 and ghyo. Validation kyqw (validator-fable-kyqw): all seven Done when claims pass with receipts; Windows, the game, a real second monitor and DPI are astromech-cxgq (needs-human). Every child was independently reviewed (Fable 5.1 and Sonnet 5.5 reviewers settled open design choices under Sumeet's direction); the final whole-branch review approved (astromech-lus6). History: a deleted append-only review record (astromech-5gtt) was removed from history with Sumeet's approval (main and this branch rewritten; the old branch is archive/feature-51ue-pre-rewrite). The linked-place engine, recipes, Find near this and Find along route continue in astromech-os70. Follow-ups: astromech-orha, nh0g, rfn8, vaz9, 5s9h, z9rd, k8t4, y14z, 4odm, uk11, fci3, 6trg, 0wmn.
