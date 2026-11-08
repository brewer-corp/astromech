---
schema: atelier.review
schema_version: 2
id: astromech-j4lx
title: approval on astromech-vucf @0aff5738
target: astromech-vucf
identity: claude-reviewer-xk0n
commit: 0aff5738bdc9c72759eb49d2535d13f8e12665bc
created_at: 2026-11-07T17:49:45.247861884Z
event:
  event: approval
---

Review of xk0n (16a82b2b, 1297e457), 86b4d3e8, and the fix 0aff5738 for 3dj9. 3dj9 is fixed: Http::proxy_from_env is true in Http::real() and State's default, false in world/net.rs, and it reaches all four ureq agents that took a proxy on main (update http_get and http_download, data download agent, voice download agent; git grep try_proxy_from_env main vs HEAD: the same four sites). The installed app's callers all pass true: fetch_and_verify, stage, download_from, neutron download/newer_available through State, and models::download/ensure_in. WorldCore removes the six variables. Run inside bwrap --unshare-net: cargo test -p astromech-data -p astromech-galaxy -p astromech-voice -p astromech-api -p astromech-core, 585 passed, 0 failed, including downloads_ignore_the_proxy_variables (which my pre-fix run with the variables set failed). Workspace clippy -D warnings and fmt are clean. Earlier pass: every HTTP stack checked against world mode, installed-app agents, timeouts, limiter and URLs unchanged from main, deviations 1-3 accepted, and EDDN/EDSM in world mode accepted. ur4v and 9rru are deferred by the orchestrator.
