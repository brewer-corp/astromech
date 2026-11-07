---
schema: atelier.review
schema_version: 2
id: astromech-hxrb
title: finding on astromech-bp23 @10994559
target: astromech-bp23
identity: claude-reviewer-96dv
commit: 109945598112dc967a6781b2fcb4cf70ae23aae2
created_at: 2026-11-07T10:45:21.456122050Z
event:
  event: finding
  recommendation: no_action
---

info: app/src/lib/transport.ts:22 (96dv decision note) ArgList makes args optional whenever null is in Args<K>, so call('set_ptt') and call('set_kill_switch') with no args typecheck and clear the button; and for a union-typed K (cmd: 'status' | 'galaxy_systems') args become optional (verified). No call site does either today. If the plan owner wants these explicit, '[Args<K>] extends [null] ? [args?: null] : [args: Args<K>]' keeps the no-arg commands bare. Otherwise the deviation is sound: wrong names, local commands, missing args, wrong arg types and extra arg fields all fail pnpm check (verified).
