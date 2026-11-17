---
schema: atelier.item
schema_version: 3
id: astromech-lf3a
title: Powerplay cuts system names short beside the docked route at 1920 x 1080
status: dropped
mission: astromech-xajc
labels:
- bug
priority: medium
blocked_by: []
implementers: []
ready: null
trivial: null
caused_by: astromech-uc7i
created_at: 2026-11-17T10:08:06.508726574Z
---

## Goal

Powerplay shows whole system names and states beside the docked route sidebar. Found by astromech-8e22 on 17 Nov 2026: at 1920 x 1080 at 100 percent, with the sidebar docked, the four-column board leaves each column about 350 px. This system's name reads "Piscium Sector EB-X ...", and every Where to work row cuts its system ("Piscium Secto...") and its state ("Unoccupied, acquisition ..."). At 3840 x 2160 at 180 percent, Where to work is a fourth column again and cuts one system ("Piscium Sector GW-W b...") and two states ("... also ..."). At 2560 x 1440 it sits under My standing and nothing is cut. Capture: docs/design/increment2/powerplay-1920x1080.png; the cut texts are under `clipped` in docs/design/increment2/report.json.

## Done when

- [ ] At 1920 x 1080 at 100 percent and 3840 x 2160 at 180 percent with the route docked, no system name or state on Powerplay ends in an ellipsis: `node scripts/browser/audit.mjs --base <dev server> --plan docs/design/increment2/plan.json --only powerplay` lists nothing under `clipped`.
- [ ] A browser test fails first where a layout rule causes the cut, if one does; a pure layout change is shown with before-and-after captures instead.

## Scope

The Powerplay page's layout at widths beside the docked sidebar. astromech-rj6i rebuilds this page and carries this case as a Done when line: astromech-8e22 handed it there and closed (its second Done when line names the handoff).

## Report

Dropped: Another item covers it: astromech-rj6i rebuilds the page and carries this case as a Done when line.
