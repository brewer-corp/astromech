---
schema: atelier.review
schema_version: 1
id: astromech-y988
title: 'docs/releases/v1.0.0-alpha.3.md:108 (astromech-rk4s test step): ''the waypoint answer ends "For fuel." at a refuel stop'' is wrong per route.rs answer_line: it ends '' N waypoints left.'' whenever more than one is left (and ''Not copied.'' can follow), and a neutron refuel stop says ''For neutron boost and fuel.''. Only exact routes carry fuel labels (plot.rs:224 passes refuel=false for neutron routes). Fix: ''the waypoint answer says "For fuel." (or "For neutron boost and fuel.") after the system name at a refuel stop of the exact route''.'
target: astromech-rcui
identity: review-agent-rcui-1
commit: ecf43e6eede049aa060234ca4b46625c66f1651c
created_at: 2026-10-23T00:54:04.235023496Z
event:
  event: finding
  recommendation: fix_now
---

docs/releases/v1.0.0-alpha.3.md:108 (astromech-rk4s test step): 'the waypoint answer ends "For fuel." at a refuel stop' is wrong per route.rs answer_line: it ends ' N waypoints left.' whenever more than one is left (and 'Not copied.' can follow), and a neutron refuel stop says 'For neutron boost and fuel.'. Only exact routes carry fuel labels (plot.rs:224 passes refuel=false for neutron routes). Fix: 'the waypoint answer says "For fuel." (or "For neutron boost and fuel.") after the system name at a refuel stop of the exact route'.
