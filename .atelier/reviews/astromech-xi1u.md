---
schema: atelier.review
schema_version: 1
id: astromech-xi1u
title: 'R1 medium, ak62 Powerplay gap persists: app/src/modules/powerplay/View.svelte:136 ''@container (min-width: 1136px) and (max-width: 1527px)'' assumes 1rem = 16px, but tokens.css:45 sets html font to var(--fs) = 15px, so the grid (minmax(22rem,1fr), 40px gap) has three columns from a 1070 px container and four from 1440. For containers 1070-1135 px (windows of about 1150-1225 px) there are three columns with no span rule: at a 1200 px window (container 1118) Where to work sits at y=1338 under the 1209 px merits list, leaving an empty band of about 900 px under My standing and This system (measured in headless Chrome, ?core=ready). Fix: (min-width: 1070px) and (max-width: 1439px), or derive from rem, and add 1200 px to the sweep.'
target: astromech-jo3l
identity: review-agent-jo3l-1
commit: 80085727d888bdaa7d42f5c57a54b04d47b706d1
created_at: 2026-10-22T21:33:57.824632691Z
event:
  event: finding
  recommendation: fix_now
---

R1 medium, ak62 Powerplay gap persists: app/src/modules/powerplay/View.svelte:136 '@container (min-width: 1136px) and (max-width: 1527px)' assumes 1rem = 16px, but tokens.css:45 sets html font to var(--fs) = 15px, so the grid (minmax(22rem,1fr), 40px gap) has three columns from a 1070 px container and four from 1440. For containers 1070-1135 px (windows of about 1150-1225 px) there are three columns with no span rule: at a 1200 px window (container 1118) Where to work sits at y=1338 under the 1209 px merits list, leaving an empty band of about 900 px under My standing and This system (measured in headless Chrome, ?core=ready). Fix: (min-width: 1070px) and (max-width: 1439px), or derive from rem, and add 1200 px to the sweep.
