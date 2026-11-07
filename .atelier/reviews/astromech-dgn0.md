---
schema: atelier.review
schema_version: 2
id: astromech-dgn0
title: approval on astromech-l5c6 @ead814bf
target: astromech-l5c6
identity: claude-reviewer-l5c6
commit: ead814bff913ea564ac5b231e851de3bc68bfc99
created_at: 2026-11-07T03:10:23.518976921Z
event:
  event: approval
---

Re-reviewed 12b9d909..ead814bf. R1 (doh0): ship, device and mockup image URLs now use import.meta.env.BASE_URL; no absolute public paths left in JS (CSS url() is rewritten by Vite). Served app/dist-preview at /sub/ and loaded the default page plus #/controller, #/engineering, #/unlocks and #/keybinds in headless Chrome: every request stayed under /sub/ and returned 200 (device PNG, ships/*.png, engineers/access.svg); only the browser's own /favicon.ico 404'd; the route=exact fake route rendered. R2 (1qa8): Keybinds q/bind and Controller device/activity go through previewQuery, which now merges the page query over the baked one key by key (new test). R3 (pnzj): --core validated, missing flag values exit 2 with a message, title rename asserted, DOCTYPE regex handles internal subsets. pnpm check/lint clean, test 512/512. Dev and Tauri unchanged (BASE_URL '/', env unset). Not verified by me: the artifact viewer check (Done-when 3) needs Sumeet to open the published link.
