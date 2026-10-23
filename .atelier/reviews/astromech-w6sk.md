---
schema: atelier.review
schema_version: 1
id: astromech-w6sk
title: 'R5 low: during a workflow run, gh release upload --clobber replaces neutron.bin.gz and then neutron.bin.gz.sha256 one at a time (.github/workflows/neutron.yml:47), and download() writes neutron.bin.sha256 only after the rename (crates/astromech-galaxy/src/neutron.rs:297-299). A download in the upload window gets a 404 or a checksum mismatch, and a failed sidecar write reports an error even though the file was replaced. Every case fails safe: the old file is kept, or the next check says a newer file is available. No change needed now.'
target: astromech-1842
identity: review-agent-1842-1
commit: 059d78381235b475652ae78edab486764504ddb4
created_at: 2026-10-22T21:11:03.461952406Z
event:
  event: finding
  recommendation: no_action
---

R5 low: during a workflow run, gh release upload --clobber replaces neutron.bin.gz and then neutron.bin.gz.sha256 one at a time (.github/workflows/neutron.yml:47), and download() writes neutron.bin.sha256 only after the rename (crates/astromech-galaxy/src/neutron.rs:297-299). A download in the upload window gets a 404 or a checksum mismatch, and a failed sidecar write reports an error even though the file was replaced. Every case fails safe: the old file is kept, or the next check says a newer file is available. No change needed now.
