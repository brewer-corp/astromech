---
schema: atelier.review
schema_version: 3
round: 1
commit: 2da2e2368ad3e77090488bd50372457a5926ea93
reviewers:
- thread:delegated-task:command%3Amcp%3Ac6ad92d8-d76e-4058-bbd3-0d44677a0cb3%3Adelegate-task%3Aa3512de4-16b7-4075-a8e9-ea40b1c39c83
model: gpt-6.1-sol
verdict: changes
created_at: 2026-11-17T11:55:34.815804759Z
---

## Summary

Complexity: medium. The change joins resource and shopping data, introduces a second segment in the shared Meter, and puts the existing search into ListDetail.

Value: high. Delivered, aboard and still-to-buy amounts together give the commander the progress view the item asks for.

Risk of merging: medium. Incorrect states or names reach the commander, but the affected UI is visible and the change is easy to revert.

The value justifies the size. I found no substantially smaller change that delivers the required progress rows and station details.

I reviewed `git diff main...2da2e2368ad3e77090488bd50372457a5926ea93` against every Done-when line, read the Rust shopping-list producer and the SearchKit/ListDetail contracts, and inspected the committed before-and-after captures and both audit reports. The reported audit counts agree: only the documented Stock ellipses appear outside the exemptions. The layout, type roles, delivered/aboard segments, ordering, site dropdown, search origin, lint activation and guide match the item. Keeping Colonies outside ListDetail avoids its second search taking over the station panel. Its pre-existing paragraph is a reasonable scope exception because the item explicitly prohibits changing Colonies; the lead still needs the follow-up described in the Report.

Independent checks passed: all nine Colonisation browser cases; `pnpm check` (zero errors or warnings); `pnpm lint`; `pnpm test` (92 files, 788 tests); all seven journeys; `node --check scripts/browser/audit.mjs`; and `python3 scripts/check-guide.py` (22 pages, no problems). The journeys used the existing `/home/supadava/projects/astromech/target/debug/astromech-core` via ASTROMECH_CORE. I did not rebuild or run the Rust suite; this item changes no Rust.

I also exercised the real page in headless Chromium with transport answers based on the committed colonisation fixture, changing only failure/absent-data states. That reproduced the two findings below. An all-aboard answer correctly removed the purchase search and showed the delivery message. I inspected the supplied captures at the required sizes rather than regenerating them; I did not check the Windows WebView or live journals.

## Finding 1: Suppress the purchase search when a construction project has failed
Recommendation: fix now

`app/src/modules/colonisation/View.svelte:132` mounts SearchKit whenever `needs.length` is nonzero, even when `project.failed` is true. A failed build can still have undelivered resources: `crates/astromech-colonisation/src/lib.rs:313-328` preserves both the failure flag and the resource amounts, and `shopping.rs:140-162` builds shopping lines from remaining amounts without filtering failed projects.

I reproduced this by setting `failed: true` on the first fixture project, leaving its resources and shopping list intact, then remounting Colonisation. The head read “Failed”, but the page still called `colonisation_buy` once, displayed Efremov Plant, and said “9,554 t to buy · 13 loads at 784 t”. The terminal-state message at line 147 is unreachable in this case. The same guard existed before this change, but the item's explicit finished-project/no-search requirement remains unmet, and this rewrite retains the erroneous purchase advice under its new Failed head. Gate the search on the project's terminal state as well as its shopping list, and cover a failed project with outstanding resources in the existing finished-project case.

## Finding 2: Preserve a readable commodity fallback when localised names are absent
Recommendation: fix now

`app/src/lib/colonisation.ts:58` falls back to `r.name`, which is the journal token (`$steel_name;`, `$CMMComposite_name;`), rather than the market symbol previously used by the page (`l.name_localised ?? l.symbol`). Both resource and shopping localised names are nullable; the committed raw colonisation fixture also has a completed Steel resource without Name_Localised (`crates/astromech-colonisation/tests/fixtures/colonisation.jsonl:10`). Newly displayed completed rows have no shopping line to supply a fallback.

With only `name_localised` cleared on the fixture resources and shopping lines, the real page displayed `$steel_name;`, `$CMMComposite_name;`, `$CeramicComposites_name;`, `$InsulatingMembrane_name;` and `$aluminium_name;` as its commodity names. This regresses the previous symbol fallback for incomplete rows and exposes journal syntax on the new completed rows. Keep the raw resource name for joining, but use the shopping symbol, or the same normalised commodity id for a resource without a shopping line, as the final display fallback. Cover this absent-name case at the data-mapping layer.
