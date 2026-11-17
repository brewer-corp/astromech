---
schema: atelier.item
schema_version: 3
id: astromech-evj3
title: Exploration > Biology as a card per body
status: approved
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers:
- thread:delegated-task:command%3Amcp%3A7af80d85-cde0-4a8e-9929-43c0997f41ce%3Adelegate-task%3A317f63c7-f990-4fe6-99c6-d992007bddb0
ready:
  by: Sumeet
  at: 2026-11-17T10:51:58.242665710Z
trivial: null
caused_by: null
created_at: 2026-11-10T05:55:40.418245755Z
---

## Goal

Each body with biology is a card: what grows there, what it is worth, and how far the sampling has got, with the species being sampled and the distance to the next sample at the top while the commander is on foot. Today it is a row of stat boxes and two tables, the commander's bodies and what other players reported, each with a column per field (`app/src/modules/exobiology/View.svelte`).

## Done when

- [x] Exploration > Biology is a Board page on the type roles: one card per body with biology, in the order the reply gives (most valuable first), in a Tiles grid. The head: the system as the Title, White, with "biology on 4 bodies" and the system's value range in Label.
- [x] **Sampling**, while `sampling` is set, is the page's leading block (Hull Raised, the one leading block DESIGN.md allows): the species in Name, Options Grey; the samples taken as three short meters, one lit per sample (Prose Grey lit, Mark Off unlit), with "1 of 3" as its accessible name and in Label; then, until the third sample, the distance to the nearest earlier sample against the colony range as the page's main value in Value XL Orange Lit ("140 m of 250 m"), with "Far enough" in Orange once `clearance.far_enough`. Analysed says so in place of the distance. The body it is on in Label.
- [x] **A body card**: a Seam outline on Void. Its short name in Name, Options Grey; its value range as the card's one value in Value L, Options Grey, except that with nothing being sampled the first card's range is the page's main value in Orange Lit; the signals count in Label; then each genus on a line: the genus in Prose Grey, the species it could be in Label, its value range in Figures; a genus known from the DSS or a sample in Prose Grey, a predicted one in Label Grey with "predicted" in its Tip; an analysed genus marked with three lit marks; the genus being sampled with its marks lit to the count. A body with no genera yet says "Scan it to predict its genera" in Label Grey.
- [x] **Reported by other players** (`known`) merges into the same cards by `body_id`: on a card the commander has, a genus or species only others reported is added in Label with "reported" in its Tip; a body only others reported gets its own card after the commander's, marked "Reported" as a badge. Nothing it shows today is lost (genera, species with their values, the value range).
- [x] States: no system yet, loading, a failed poll (the shared Stale line, or the failure in words: the page re-reads on its own) and no biology in the system keep today's words. `known` null (not asked or failed) shows the commander's cards only.
- [x] The brightest things: while sampling, the distance to the nearest sample against the colony range (main value) and, until it is far enough, that same distance still to go (what needs the commander); with no sampling, the first card's value range (main value) and nothing needing the commander. Nothing on the page can be changed. Orange only for the main value and "Far enough". No colour outside DESIGN.md's list; if the sample marks need colour beyond Prose Grey and Mark Off, the item adds them to DESIGN.md's list with their meaning first.
- [x] No printed explanations: the value rule (`VALUE` Tip) and the colony range rule go in Tips.
- [x] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `exobiology` fixture (sampling in progress, bodies and reports): with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the cards fall to one or two columns. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/biology/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [x] Both parts of the page-sizing check are on for `app/src/modules/exobiology/View.svelte` in `PAGE_SIZING`, and the `exobiology` entry leaves `TOKENS_EXEMPT`.
- [x] Tests: `exobiology/View.browser.test.ts`'s four cases keep their behaviour on the new markup ("lists what other players reported" reads the merged cards). The merge of reports into cards is a pure function with a table test. No test of layout or copy.
- [x] `docs/guide/exploration.md`'s Biology section describes the cards and the sampling block.
- [x] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Exploration > Biology. No core change. The System tree's OPEN IN BIOLOGY link (astromech-dpaz) lands here.

## Context

There is no mockup; the cards follow DESIGN.md's Board and Facts, stats and meters. Replies: `Biology.ts` (`bodies: BodyBio[]`, `known: KnownView[] | null`, `sampling: SamplingView | null`), `BodyBio.ts`, `GenusBio.ts` (`known`, `species`, `value`, `colony_range_m`, `analysed`), `SamplingView.ts` (`species`, `count`, `complete`, `clearance`), `Clearance.ts` (`next`, `nearest_m`, `colony_range_m`, `far_enough`), `KnownView.ts`. Per-genus sample counts exist only for the species being sampled (`SamplingView.count`); other genera are analysed or not (`GenusBio.analysed`), which is all the marks show.

## Report

**A call the lead can overturn: the cards are a wide Tiles card.** The item asks for a Tiles grid
and for one or two columns at 1100. Tiles' 200–240 px columns give four or five at 1100, and a
genus line (genus, species, value, marks) does not fit 240 px. So `Tiles` gained `wide`: a card
two tracks and the gap wide (408–488 px, `minmax(25.5rem, 30.5rem)`). DESIGN.md's Page layouts
names it beside the 200–240 px grids. `Blocks` was the other choice; it stretches to `1fr` and
is Now's answer row, so I kept Tiles. Measured: two columns at 1100 x 700, and at 1920 and 2560
both cards sit side by side with room for a third.

**What changed.**

- `app/src/modules/exobiology/View.svelte` is on the Board. The head is the system as the Title
  with "biology on N bodies · <range> · <source>" in Label. The sampling block (Hull Raised, the
  one leading block) holds the species in Name, "on A 2" in Label, three marks with "1 of 3" as
  their accessible name and in Label, then "209 m of 500 m" with 209 in Value XL Orange Lit and
  "291 m to go" (the metres in Name, Options Grey). Once `far_enough` it says "Far enough" in
  Orange. Complete, it says "Analysed". With no clearance yet it shows the colony range in Label.
  The colony range rule is the Tip on the distance.
- A body card is a Seam outline on Void: the short name, the range in Value L (the `VALUE` Tip),
  the signals in Label, then a line per genus (genus, species in Label, range in Figures,
  marks). A predicted genus is Label Grey with a "Predicted" Tip, which also says when other
  players reported it. An analysed genus shows three lit marks, and the sampled genus shows its
  count. The first card's range is Orange Lit only when no distance leads the page (no sampling,
  sampling complete, or no clearance yet).
- `bodyCards` in `app/src/lib/exobiology.ts` merges `known` into the cards: by `body_id` when
  both have one, else by full name, and a null id never matches another null. On a commander's
  card it adds a genus or species only others reported (in Label, with a "Reported" Tip). It
  marks a species others reported that the commander already lists, so that species prints
  its reported value as the old table did ("Tectonicas 19 M"). It also adds their value range
  as "reported 20–28 M" when that differs from the commander's. A reported species finds its
  genus among the body's genus names, not by its first word. The catalogue has Brain Tree,
  Anemone and Sinuous Tuber species named before their genus ("Roseum Brain Tree"), and
  Amphora Plant, Bark Mound and Crystalline Shard species that share their genus's name. A body
  only others reported gets its own card after the commander's, with a "Reported" badge. The
  attribution line "Spansh, from other players' EDDN reports" moved into the Reported Tip.
- `app/src/lib/ui/Samples.svelte` draws the marks (DESIGN.md Meter: Prose Grey lit, Mark Off
  unlit), so no colour was added to DESIGN.md's list. It lives in `lib/ui`, since a page file
  may not set widths and the marks need one.
- `eslint.config.js`: `exobiology/View.svelte` is in `PAGE_SIZING` with both parts, and the
  `exobiology` entry is gone from `TOKENS_EXEMPT`.
- `docs/guide/exploration.md`'s Biology section describes the cards and the sampling block.

**Tests.** `exobiology.test.ts` is the table test for `bodyCards`: `known` null and empty, a
report that adds a genus, a species and a range, a report of what the commander already has,
a reported species among the commander's predicted candidates, species named before their
genus, a body only others reported, no commander bodies, and null ids. The recorded fixture has no reported-only body or
genus, so the table test is the only place those show. `View.browser.test.ts` keeps its four
cases on the new markup. The first reads the cards and checks the predicted genus's Tip in place
of the old "From the DSS" / "Predicted from the scan" line. The sampling case reads the block
and the marks' accessible name. "Lists what other players reported" reads the reported ranges on
the merged cards, which are what `known` adds in the fixture.

**Captures and audit** (`docs/design/biology/`). `before/` holds today's page at 1100 x 700 and
2560 x 1440 (`plan-before.json`), taken before the change. `plan.json` covers sampling at 1100,
1920 and 2560 at 145 percent, plus loading and failed at 1100 and 1920. Every capture lists
nothing under `sideways`, `clipped`, `overlaps`, `small`, `contrast` or `far`. At 1100 the
route is the strip; at 1920 and 2560 the sidebar is docked. `report.json` was written from the
working tree just before the commit, so its `commit` field is the parent.

- `bright`, the sidebar apart, ranks by luminance, as dpaz's and rj6i's Reports note. White comes
  first (the wordmark and the system title), then Options Grey (the species, "291 m", the card
  names and ranges), then Prose Grey genera and figures. Orange Lit on the page holds only
  "209", the main value. Nothing on the page can be changed, so no orange action appears; the
  orange in the list is the frame's (Help, Ctrl+K, Waiting for Elite).
- Not captured: the no-sampling state, where the first card's range is the main value, and an
  empty system. The recorded fixture is always sampling, and `?core=empty` answers this call with
  an error. I did not hand-edit the recorded fixture or add a preview switch that invents a reply.
- With no bodies of the commander's, the page keeps today's "No biological signals found in this
  system yet. The FSS shows them.", with any reported-only cards under it.
- Units are drawn as units (Label, Label Grey): "20 M" on a card, "19 M" on a genus line and
  "291 m to go" keep their figure in the value's role and the unit after it.

**Other changes.**

- A separate commit fixes `eslint-rules/pageSizing.test.js`, which failed on main. It used
  Station as its page not yet enrolled, and astromech-7vxi enrolled Station. It now uses Stored
  modules.
- The journeys ran with `ASTROMECH_CORE` pointed at the main checkout's debug core. This
  worktree has no core build, and the item changes no core code.

**Checks.** `pnpm check`, `pnpm lint` and `pnpm test` pass in one full run (901 tests). The
journeys pass (7 of 7). The audit was retaken after the last markup change.

**After round 1's approval: main merged in.** `app/eslint.config.js` conflicted with
astromech-fzhp, which enrolled Session and Logbook in `PAGE_SIZING` and took their entry off
`TOKENS_EXEMPT` in the same places. Both enrolments are kept, and neither exemption remains.
After the merge, `pnpm lint`, `pnpm check` and `pnpm test` pass (906 tests), and so do the
journeys (7 of 7).
