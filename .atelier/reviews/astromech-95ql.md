---
schema: atelier.review
schema_version: 2
id: astromech-95ql
title: approval on astromech-7rcn @bde2bfe5
target: astromech-7rcn
identity: claude-reviewer-8zdt
commit: bde2bfe5ea6645501a0ee8c0f4eaa243b97b298d
created_at: 2026-11-07T06:58:39.545666113Z
event:
  event: approval
---

Re-reviewed 7e1a4022..bde2bfe5, a one-line change: ci.yml's linux job runs on ubuntu-24.04 instead of ubuntu-latest (now 26.04, where Playwright 1.60's install --with-deps refuses chromium-headless-shell). Pinning the runner beats PLAYWRIGHT_HOST_PLATFORM_OVERRIDE in CI: the override would make --with-deps apt-get the 24.04 package list on 26.04 and run a browser build Playwright doesn't support there, while 24.04 is a supported GitHub image and a supported Playwright host. Nothing else needs to match: no other workflow runs Playwright or the frontend tests; the cache key uses runner.os (Linux either way) and the failed 26.04 run saved no cache; CONTRIBUTING.md's local-install override note and 'CI installs it with --with-deps' stay true. Correction to my earlier note: ubuntu-latest was not 24.04. Residual: the pin needs lifting once Playwright supports 26.04 or before GitHub retires 24.04; CI pass unproven until the rerun on PR #215.
