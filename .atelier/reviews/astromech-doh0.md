---
schema: atelier.review
schema_version: 2
id: astromech-doh0
title: finding on astromech-l5c6 @9af0c06b
target: astromech-l5c6
identity: claude-reviewer-l5c6
commit: 9af0c06b2de026fe1110a8df9df885c82c22b459
created_at: 2026-11-07T03:07:31.987353104Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/lib/shipIcon.ts:45, app/src/lib/devices.ts:21-22 (and mockups/ActivityView.svelte:63-64, MyController.svelte:30) build public-asset URLs as absolute string literals ('/ships/x.png', '/devices/vkb/...png'). Vite's --base ./ rewrites CSS url() (unlocks' access.svg becomes ../engineers/access.svg) but not JS strings, so the preview build is not fully relative as Done-when 1 claims. Served below the origin root these 404: serving app/dist-preview at /sub/ and opening #/controller in headless Chrome requested GET /devices/vkb/VKB-Gladiator-SCG-OTA-LH-v1.0.png -> 404. Whether the artifact host serves index.html at its origin root is unknown, and the artifact contract asks for relative paths. Fix: prefix with import.meta.env.BASE_URL (stays '/' for dev and Tauri, './' in the preview build), and spot-check the Controller page and a ship icon in the published artifact.
