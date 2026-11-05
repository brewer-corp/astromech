---
schema: atelier.review
schema_version: 1
id: astromech-x0j4
title: finding on astromech-pey5 @5c36e669
target: astromech-pey5
identity: reviewer-fable-pey5
commit: 5c36e66901b0dc61bd103cf1c8b4951d00cecb0c
created_at: 2026-11-05T06:45:12.674767215Z
event:
  event: finding
  recommendation: fix_now
---

medium: app/src/lib/ui/Calibration.svelte:95 Record is kind=primary, and Calibration also renders inside onboarding (lib/onboarding/StepCalibrate.svelte) under the footer's primary Next, so the Calibration step shows two orange fills (RECORD SILENCE and NEXT; rendered at #/setup/calibrate). DESIGN.md One Fill Per Group: a view has at most one primary button; pey5's Done when names the primary as the one fill per view. Fix: give Calibration a primary prop (true from Settings' dialog, where Record is the only step forward; false in onboarding) or make Record secondary everywhere.
