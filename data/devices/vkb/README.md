# VKB device maps and reference sheets

Built-in maps for Sumeet's pair. astromech uses them to put each binding on the right control
of VKB's own reference sheet.

| File | What it is |
|---|---|
| `gladiator-evo-r.231D0200.json` | Gladiator EVO, right hand: each control, its stock `Joy_N`, the sheet box it goes in, and how sure we are |
| `gladiator-evo-ot-l.231D3201.json` | Gladiator EVO Omni Throttle, left hand: the same |
| `VKB-Gladiator-SCG-RH-v1.0.pdf` | VKB's reference sheet for the right stick |
| `VKB-Gladiator-SCG-OTA-LH-v1.0.pdf` | VKB's reference sheet for the left Omni Throttle |

Regenerate the JSON with `scripts/vkb-device-maps.py` (needs `pypdf`).

## Credit
The reference sheets are by **VKB Gaming** (sheet design by UntoldForce), from
<https://www.vkbgaming.com/fillable-templates/>. They are included unchanged so the app can show
them. All rights to the sheets stay with VKB.

## Where the button numbers come from
- **Stock numbering:** EDRefCard's device data (MIT), entries `231D0200` and `231D3201`. Both
  sticks use the same numbers. Examples: trigger stage 1 = `Joy_1`, red button A2 = `Joy_3`,
  side button B1 = `Joy_4`, pinky D1 = `Joy_5`, centre hat A3 = `Joy_6`–`Joy_10` (push =
  `Joy_10`), top hat A4 = `Joy_11`–`Joy_15`, thumb hat C1 = `Joy_16`–`Joy_20`, ministick in hat
  mode = `POV1`, F1–F3 = `Joy_27`–`Joy_29`.
- **Checked against Sumeet's devices:** the controls his sticks actually sent in the s7 runs
  and his DualVKB preset agree with this map (for example push-to-talk `Joy_10` is the A3 push
  on the OT L, and the right stick's ministick sends `POV1`).
- **Confidence per control:** `high` = stock map and seen on his device; `medium` = stock map
  only; `low` = inferred or unknown (ministick push, Sw1, which rapid-fire direction is "up",
  the base wheel axis).
