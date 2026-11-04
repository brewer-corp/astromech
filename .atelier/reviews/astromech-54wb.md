---
schema: atelier.review
schema_version: 1
id: astromech-54wb
title: finding on astromech-8e8m @f770596a
target: astromech-8e8m
identity: review-agent-8e8m-1
commit: f770596ae368145412c57e18182e842f45253d84
created_at: 2026-11-04T22:56:27.746527486Z
event:
  event: finding
  recommendation: fix_now
---

medium: .github/workflows/atelier.yml:17-19 concurrency group atelier-${{ github.ref }} with cancel-in-progress: true is shared by every push to main, so a second push while the first run is in progress (or queued) cancels the first run. The second run checks only its own event.before..sha, so the first push's commits are never land-checked: an unreviewed code commit or out-of-review done claim pushed just before another push (e.g. a records-only planning push right after, common in agent flows) passes CI with only a 'cancelled' run. Doctor on later pushes did not depend on this because it checks the whole history; the land step relies on every push's run completing. !cancelled() does not help. Note GitHub also cancels any already-pending run in a group when a new one queues, so cancel-in-progress: false alone is not enough. Fix: give push runs their own group, e.g. group: atelier-${{ github.event_name == 'push' && github.sha || github.ref }} (PRs keep cancelling superseded runs), and keep CONTRIBUTING.md:67 'the job fails' true.
