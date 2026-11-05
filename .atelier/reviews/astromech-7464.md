---
schema: atelier.review
schema_version: 1
id: astromech-7464
title: finding on astromech-1d92 @5afebb00
target: astromech-1d92
identity: reviewer-fable-1d92
commit: 5afebb00886ec2d015edb250a6fab6d5e189e093
created_at: 2026-11-05T07:56:19.725316610Z
event:
  event: finding
  recommendation: defer
---

low: app/src/lib/fakecore.ts:175 With ?route=exact the preview commander is 'here' in Tascheter Sector LS-T a3-0 for the route desk, but the lookups still answer 'Nearest to Sol' (docs/design/route-strip-services-1100.png shows both on one screen): the preview's lookup origin is not the route desk's here. Preview-only inconsistency that screenshots now expose; fix by taking the lookup origin from state.here in previewLookup.
