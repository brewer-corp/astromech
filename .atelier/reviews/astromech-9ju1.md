---
schema: atelier.review
schema_version: 1
id: astromech-9ju1
title: finding on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:19.436224629Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/fixtures/plot.json:1 Only the 'exact' sample gained per-stop 'jumps'; 'neutron' (3 stops) and 'local' (151 stops) have none, so Plot and follow on a neutron route in the browser preview shows 'N stops left' where astromech_galaxy::plot (Stop.jumps is always set, tests/plot.rs) would give jumps. route.ts says the fixtures are what plot makes; they no longer are. Preview only. Fix: add each leg's jumps to the two samples (the neutron and local legs' jumps from the recorded jobs).
