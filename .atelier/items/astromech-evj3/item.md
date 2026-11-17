---
schema: atelier.item
schema_version: 3
id: astromech-evj3
title: Exploration > Biology as a card per body
status: ready
mission: astromech-xajc
labels:
- size-m
- feature
priority: medium
blocked_by:
- astromech-8e22
implementers: []
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

- [ ] Exploration > Biology is a Board page on the type roles: one card per body with biology, in the order the reply gives (most valuable first), in a Tiles grid. The head: the system as the Title, White, with "biology on 4 bodies" and the system's value range in Label.
- [ ] **Sampling**, while `sampling` is set, is the page's leading block (Hull Raised, the one leading block DESIGN.md allows): the species in Name, Options Grey; the samples taken as three short meters, one lit per sample (Prose Grey lit, Mark Off unlit), with "1 of 3" as its accessible name and in Label; then, until the third sample, the distance to the nearest earlier sample against the colony range as the page's main value in Value XL Orange Lit ("140 m of 250 m"), with "Far enough" in Orange once `clearance.far_enough`. Analysed says so in place of the distance. The body it is on in Label.
- [ ] **A body card**: a Seam outline on Void. Its short name in Name, Options Grey; its value range as the card's one value in Value L, Options Grey, except that with nothing being sampled the first card's range is the page's main value in Orange Lit; the signals count in Label; then each genus on a line: the genus in Prose Grey, the species it could be in Label, its value range in Figures; a genus known from the DSS or a sample in Prose Grey, a predicted one in Label Grey with "predicted" in its Tip; an analysed genus marked with three lit marks; the genus being sampled with its marks lit to the count. A body with no genera yet says "Scan it to predict its genera" in Label Grey.
- [ ] **Reported by other players** (`known`) merges into the same cards by `body_id`: on a card the commander has, a genus or species only others reported is added in Label with "reported" in its Tip; a body only others reported gets its own card after the commander's, marked "Reported" as a badge. Nothing it shows today is lost (genera, species with their values, the value range).
- [ ] States: no system yet, loading, a failed poll (the shared Stale line, or the failure in words: the page re-reads on its own) and no biology in the system keep today's words. `known` null (not asked or failed) shows the commander's cards only.
- [ ] The brightest things: while sampling, the distance to the nearest sample against the colony range (main value) and, until it is far enough, that same distance still to go (what needs the commander); with no sampling, the first card's value range (main value) and nothing needing the commander. Nothing on the page can be changed. Orange only for the main value and "Far enough". No colour outside DESIGN.md's list; if the sample marks need colour beyond Prose Grey and Mark Off, the item adds them to DESIGN.md's list with their meaning first.
- [ ] No printed explanations: the value rule (`VALUE` Tip) and the colony range rule go in Tips.
- [ ] Checked in the browser preview at 1100 x 700 and 1920 x 1080 at 100 percent Text size and at 2560 x 1440 at 145 percent, with the recorded `exobiology` fixture (sampling in progress, bodies and reports): with a route followed the sidebar is docked at 1920 and 2560 and is the strip at 1100, where the cards fall to one or two columns. Before-and-after captures at 1100 x 700 and 2560 x 1440 in `docs/design/biology/`. Each size is measured with `node scripts/browser/audit.mjs` (astromech-8e22's tool) from a plan of this page's states saved beside the captures (`plan.json`), and lists nothing under `sideways`, `clipped`, `overlaps`, `small`, `far` or `contrast` (outside its exempt list), or the Report says why a listing is not a fault; its `bright` list for the page, the sidebar apart, leads with the main value, what can be changed and what needs the commander.
- [ ] Both parts of the page-sizing check are on for `app/src/modules/exobiology/View.svelte` in `PAGE_SIZING`, and the `exobiology` entry leaves `TOKENS_EXEMPT`.
- [ ] Tests: `exobiology/View.browser.test.ts`'s four cases keep their behaviour on the new markup ("lists what other players reported" reads the merged cards). The merge of reports into cards is a pure function with a table test. No test of layout or copy.
- [ ] `docs/guide/exploration.md`'s Biology section describes the cards and the sampling block.
- [ ] `pnpm check`, `pnpm lint` and `pnpm test` pass, then the journeys (`node --test 'journeys/*.journey.ts'`, as CONTRIBUTING.md says). One full test run at a time: each agent here has 4 GB.

## Scope

Exploration > Biology. No core change. The System tree's OPEN IN BIOLOGY link (astromech-dpaz) lands here.

## Context

There is no mockup; the cards follow DESIGN.md's Board and Facts, stats and meters. Replies: `Biology.ts` (`bodies: BodyBio[]`, `known: KnownView[] | null`, `sampling: SamplingView | null`), `BodyBio.ts`, `GenusBio.ts` (`known`, `species`, `value`, `colony_range_m`, `analysed`), `SamplingView.ts` (`species`, `count`, `complete`, `clearance`), `Clearance.ts` (`next`, `nearest_m`, `colony_range_m`, `far_enough`), `KnownView.ts`. Per-genus sample counts exist only for the species being sampled (`SamplingView.count`); other genera are analysed or not (`GenusBio.analysed`), which is all the marks show.
