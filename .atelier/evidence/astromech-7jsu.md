---
created_at: "2026-10-17T04:48:21.025806866+00:00"
id: "astromech-7jsu"
evidence_type: "test"
commit: "5352762d1ca59f5576c0aca95ad4acc2065535d5"
result: "pass"
captured_at: "2026-10-17T04:48:21.025751303+00:00"
agent_identity: "implementer (Claude Opus)"
target:
  kind: "issue"
  id: "astromech-0jqu"
  role: "validates"
relationships:
  blocks: []
  children: []
  attachments:
  - kind: "issue"
    id: "astromech-0jqu"
    role: "validates"
  relates: []
schema: "atelier.evidence"
schema_version: 2
status: "recorded"
title: "cargo test -p astromech-exobiology --test surface: 4 passed at 5352762 (table of known bearings/distances on Earth, Moon, 2964 km, 1000 km and 300 km radii: London-Paris 343.6 km/148.1°, JFK-LHR 5540 km/51.4°, meridian, equator E/W, south, 517 m sampling step, date line; flipping the bearing's sign fails known_bearings_and_distances). cargo test -p astromech-status: Status.json Latitude/Longitude/Altitude/Heading/PlanetRadius parsed."
updated_at: "2026-10-17T04:48:21.026164874+00:00"
---

cargo test -p astromech-exobiology --test surface: 4 passed at 5352762 (table of known bearings/distances on Earth, Moon, 2964 km, 1000 km and 300 km radii: London-Paris 343.6 km/148.1°, JFK-LHR 5540 km/51.4°, meridian, equator E/W, south, 517 m sampling step, date line; flipping the bearing's sign fails known_bearings_and_distances). cargo test -p astromech-status: Status.json Latitude/Longitude/Altitude/Heading/PlanetRadius parsed.
