---
created_at: "2026-11-05T06:46:56.373938359+00:00"
id: "astromech-ghyo"
issue_type: "decision"
labels: []
priority: "P2"
relationships:
  blocks: []
  children: []
  attachments: []
  relates:
  - kind: "issue"
    id: "astromech-pey5"
    type: "discovered_from"
schema: "atelier.issue"
schema_version: 2
closed_at: "2026-11-05T06:47:00.564739273+00:00"
status: "done"
title: "Shared controls: nine open design choices from the pey5 review"
updated_at: "2026-11-05T06:47:00.564739273+00:00"
---

## Question

Nine choices DESIGN.md and PRODUCT.md left open while building the shared controls (astromech-pey5). Under Sumeet's direction that agents settle design choices DESIGN.md leaves open, taking the reversible option, they were decided by the independent reviewer reviewer-fable-pey5 (Fable 5.1), 5 Nov 2026.

## Options

1. Dropdown: native select with `appearance: base-select`, or a custom listbox.
2. Clear ×: a new `clear` glyph added to DESIGN, or × as text.
3. Older glyphs: 1 px stroke like the set, or keep 1.25 px.
4. On/off settings: make Start with Windows (On/Off choice) and EDDN/EDSM (status + TURN ON/OFF) consistent, or leave them different.
5. Controller rows: keep the word "Bind this", or add a bind icon.
6. Powerplay "Weak spot" badge: outlined, emphasis by order or wording, or something else.
7. Which buttons get the one primary fill per view.
8. Plot panel shadow and orange top border: fix in pey5, or leave for 2hbw.
9. 1100 px minimum width vs a one-row header that needs about 1513–1689 px: (a) two header rows below a breakpoint, (b) collapse Find to a Ctrl+K button, (c) move status and Help to the route strip, (d) raise the minimum width, (e) fewer top tabs.

## Decision

1. Native select with base-select: no own keyboard or a11y code; WebView2 draws the DESIGN popover, WebKitGTK falls back to the system list on the dev host only.
2. Keep the `clear` glyph and add it to DESIGN's icon list: an icon button gives the 28 px target, tooltip and label a text × lacks.
3. 1 px everywhere, as DESIGN's "one set, 1 px stroke" says.
4. Leave them different, by DESIGN's two patterns: a plain boolean preference is an On/Off choice; a status whose words carry more than on/off (EDSM "On, but no API key", EDDN "On (test schemas)") stays status + action word.
5. Keep the word: it is the row's one action, the list is short, and a bind icon would be a new glyph; revisit if the list grows.
6. Leave it outlined; emphasis by order (weak spot is shown first).
7. Find route, Plot and follow, Follow this route, Odyssey Add, Save diagnostics and the bind dialog's Save each are the only fill in their view. Calibration's Record is primary only in Settings' dialog, secondary in onboarding where Next is the fill. Voice tester has no primary ("at most one").
8. Fix in pey5: 2hbw keeps that panel, so it would never get fixed there.
9. (a): below the width where the one-row header fits (about 1600 px; remeasure when 2hbw lands the 320 px box and Ctrl+K hint) the top tabs sit on a 44 px second row; above it one 64 px row. (b) and (c) need mechanisms 2hbw and 1d92 own, (d) asks for a 1600 px side window, (e) is an information-architecture change for Sumeet. pey5 changes DESIGN.md's text; astromech-2hbw implements the breakpoint with the one-row header. How it looks on a real second monitor and at Windows DPI scaling stays Sumeet's check (astromech-cxgq).

## Close Reason

Decided by the independent reviewer reviewer-fable-pey5 (Fable 5.1) in the astromech-pey5 review, 5 Nov 2026, under Sumeet's direction that agents settle design choices DESIGN.md leaves open. See Decision.
