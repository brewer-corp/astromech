---
schema: atelier.review
schema_version: 1
id: astromech-akom
title: finding on astromech-jp36 @e2a7b6bb
target: astromech-jp36
identity: reviewer-sonnet-jp36
commit: e2a7b6bb7dfdbdef096bb343d095fa592db019c8
created_at: 2026-11-05T08:24:34.444975337Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/lib/search.ts:71 and app/src/lib/ui/SearchKit.svelte:188 'hidden_stale' is a field nothing sets (docs/ARCHITECTURE.md describes it as set 'when a source can say'), and the Done when 'hidden stale rows are counted in the summary' cannot be met by counting on the backend: the age filter is pushed into the Spansh query (Filters::updated_since) and into market requests (max_days_ago), so the source never sees the rows it excludes; a count needs a second request per search (request budget) for a minor summary clause. Decision: option (c). The summary already names the limit ('reports up to 30 days old') and the Empty state offers 'Allow 90 days', which is what the count was for. Fix: drop Results.hidden_stale and its use in SearchKit (keep the client-side re-check and the 'N older reports hidden' clause only for rows the kit itself removes, or drop it too if no source can deliver such rows), reword DESIGN.md States ('...they are hidden and the summary line names the limit'), amend jp36's Done when, fix the ARCHITECTURE.md search-kit sentence.
