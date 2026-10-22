---
schema: atelier.review
schema_version: 1
id: astromech-axga
title: 'R5 low, nhyb live region does not re-announce repeats: Callouts.svelte:66 renders {error || note}; after the first save the text stays ''Saved. It applies the next time Elite starts.'', so a second switch or cooldown change makes no DOM change and a screen reader announces nothing; the same for a repeated identical error here and in ProspectorThresholds.svelte:56. Meets the Done when literally (aria-live on the lines). Fix later by clearing the line and setting it after tick().'
target: astromech-jo3l
identity: review-agent-jo3l-1
commit: 80085727d888bdaa7d42f5c57a54b04d47b706d1
created_at: 2026-10-22T21:33:58.734774159Z
event:
  event: finding
  recommendation: defer
---

R5 low, nhyb live region does not re-announce repeats: Callouts.svelte:66 renders {error || note}; after the first save the text stays 'Saved. It applies the next time Elite starts.', so a second switch or cooldown change makes no DOM change and a screen reader announces nothing; the same for a repeated identical error here and in ProspectorThresholds.svelte:56. Meets the Done when literally (aria-live on the lines). Fix later by clearing the line and setting it after tick().
