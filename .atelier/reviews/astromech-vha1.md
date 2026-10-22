---
schema: atelier.review
schema_version: 1
id: astromech-vha1
title: 'R2 low, ak62 header stacks, unstacks and restacks as the window narrows: App.svelte:142 header gap is clamp(16px,2.5vw,40px) (28 px at 1120-1180) but the @media (max-width: 1100px) block (App.svelte:165) drops it to 16 px and the nav gap to 14 px, so the measured one-row width falls at 1100. Measured sweep (?core=ready, Keybinds): stacked at 1120-1180, one row at 1080-1100, stacked again at 1060 and below, the same both directions. Dragging the window edge flips the header twice; the record asks for a deliberate wrap. Also ''ResizeObserver loop completed with undelivered notifications'' fires on window during the sweep (benign, no loop). Fix: make the narrow gaps apply above the stacking point (e.g. move the header/nav gap rule to the stacking width or to header.stacked) so stacking is monotonic.'
target: astromech-jo3l
identity: review-agent-jo3l-1
commit: 80085727d888bdaa7d42f5c57a54b04d47b706d1
created_at: 2026-10-22T21:33:58.079510660Z
event:
  event: finding
  recommendation: fix_now
---

R2 low, ak62 header stacks, unstacks and restacks as the window narrows: App.svelte:142 header gap is clamp(16px,2.5vw,40px) (28 px at 1120-1180) but the @media (max-width: 1100px) block (App.svelte:165) drops it to 16 px and the nav gap to 14 px, so the measured one-row width falls at 1100. Measured sweep (?core=ready, Keybinds): stacked at 1120-1180, one row at 1080-1100, stacked again at 1060 and below, the same both directions. Dragging the window edge flips the header twice; the record asks for a deliberate wrap. Also 'ResizeObserver loop completed with undelivered notifications' fires on window during the sweep (benign, no loop). Fix: make the narrow gaps apply above the stacking point (e.g. move the header/nav gap rule to the stacking width or to header.stacked) so stacking is monotonic.
