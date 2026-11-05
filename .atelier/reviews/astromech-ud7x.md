---
schema: atelier.review
schema_version: 1
id: astromech-ud7x
title: finding on astromech-1o77 @66eb8d47
target: astromech-1o77
identity: review-agent-1o77-1
commit: 66eb8d47ec68d2519ae308605f23070b68573c3f
created_at: 2026-11-04T23:50:36.685194105Z
event:
  event: finding
  recommendation: defer
---

low: crates/astromech-intent/src/lib.rs:495 NOT_A_NAME matches only the place's first word, so other quantity phrasings still become an origin: 'where can I sell gold in small quantities' / 'in big quantities' / 'in huge amounts' give origin 'small quantities' etc. and answer 'Small quantities. Spansh does not know that system.' The Done-when phrases ('in bulk', 'in large quantities', plus 'in quantity') are covered and pass. Suggest, when such a transcript is seen, treating a place that contains 'quantity'/'quantities'/'bulk' anywhere as no system name, with a test case.
