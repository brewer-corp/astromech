---
schema: atelier.review
schema_version: 1
id: astromech-f5zt
title: 'R2 low, for astromech-3x0x: NeutronStars::names refuses a replaced file with ''Plot the route again'' (crates/astromech-galaxy/src/neutron.rs:295), which only helps if the shell then loads the new file; 3x0x''s Done-when doesn''t say the loaded file is reloaded after a download or on that error, so a shell that caches NeutronStars would repeat the error until restart. plot() also drops Route::fewest, so the page and docs can''t say a long route is near-fewest unless 3x0x states it. Fix in 3x0x: reload on download completion or on the ''changed'' error (with a test), and list ''long routes are near the fewest jumps, not proven'' among the limits in routes.md/PLOTTING.md.'
target: astromech-1842
identity: review-agent-6bjl-1
commit: 730a39ec6ed356aba4bd173975f5c46745944c48
created_at: 2026-10-22T23:18:34.560509361Z
event:
  event: finding
  recommendation: defer
---

R2 low, for astromech-3x0x: NeutronStars::names refuses a replaced file with 'Plot the route again' (crates/astromech-galaxy/src/neutron.rs:295), which only helps if the shell then loads the new file; 3x0x's Done-when doesn't say the loaded file is reloaded after a download or on that error, so a shell that caches NeutronStars would repeat the error until restart. plot() also drops Route::fewest, so the page and docs can't say a long route is near-fewest unless 3x0x states it. Fix in 3x0x: reload on download completion or on the 'changed' error (with a test), and list 'long routes are near the fewest jumps, not proven' among the limits in routes.md/PLOTTING.md.
