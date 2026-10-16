---
schema: atelier.review
schema_version: 1
id: astromech-0rqw
title: 'R1 medium: app/src/modules/odyssey/View.svelte:203-206 with crates/astromech-odyssey/src/shopping.rs:166. Picks persist in localStorage, and shopping() fails the whole list when any one pick is invalid (unknown gear or mod id, or a maker with no cost). The pick cards and their Remove buttons only render when shop is non-null, so after a failure the page shows only ''Couldn''t work out the list: unknown modification ...'' and offers no way to remove the bad pick. Clearing localStorage is the only way out. Ids are key(wiki title), so regenerating odyssey.json after a wiki rename (for example after the astromech-h9ch data check) strands every user who saved that pick. Fix: render the pick list from picks rather than shop so Remove always works, or have shopping report bad picks per pick instead of failing the whole list. Add a test with a stale saved pick.'
target: astromech-j4c9
identity: review-agent (j4c9 review)
commit: 7094a73373653b239990f7e278c845bcc2afc096
created_at: 2026-10-16T22:28:48.749390011Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium: app/src/modules/odyssey/View.svelte:203-206 with crates/astromech-odyssey/src/shopping.rs:166. Picks persist in localStorage, and shopping() fails the whole list when any one pick is invalid (unknown gear or mod id, or a maker with no cost). The pick cards and their Remove buttons only render when shop is non-null, so after a failure the page shows only 'Couldn't work out the list: unknown modification ...' and offers no way to remove the bad pick. Clearing localStorage is the only way out. Ids are key(wiki title), so regenerating odyssey.json after a wiki rename (for example after the astromech-h9ch data check) strands every user who saved that pick. Fix: render the pick list from picks rather than shop so Remove always works, or have shopping report bad picks per pick instead of failing the whole list. Add a test with a stale saved pick.
