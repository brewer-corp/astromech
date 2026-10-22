---
schema: atelier.review
schema_version: 1
id: astromech-ofpw
title: 'R1 low crates/astromech-history/src/lib.rs:298: open_read_only now sets busy_timeout 2 s, but rusqlite already sets 5 s on every open (inner_connection.rs: sqlite3_busy_timeout(db, 5000)), so the change shortens the wait the comment says it adds; read_only_open_waits_when_busy only asserts the constant (it would fail on main only because main has 5000, not because main lacked a timeout). Scenario: a WAL checkpoint/recovery lock of 2-5 s now fails the window''s read where it used to wait. Fix: drop the line and the test, or set >= 5 s.'
target: astromech-fdh3
identity: review-agent-fdh3-1
commit: 63470471264d143f5c0e854f4641ee757634d5a8
created_at: 2026-10-22T21:45:44.096516624Z
event:
  event: finding
  recommendation: defer
---

R1 low crates/astromech-history/src/lib.rs:298: open_read_only now sets busy_timeout 2 s, but rusqlite already sets 5 s on every open (inner_connection.rs: sqlite3_busy_timeout(db, 5000)), so the change shortens the wait the comment says it adds; read_only_open_waits_when_busy only asserts the constant (it would fail on main only because main has 5000, not because main lacked a timeout). Scenario: a WAL checkpoint/recovery lock of 2-5 s now fails the window's read where it used to wait. Fix: drop the line and the test, or set >= 5 s.
