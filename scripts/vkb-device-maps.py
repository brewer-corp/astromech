#!/usr/bin/env python3
"""Build data/devices/vkb/*.json: stock button maps for Sumeet's VKB pair, joined to the
form-field boxes of VKB's reference sheets. Needs `pypdf`. Run from the repo root."""
import json, hashlib
from pypdf import PdfReader

# Stock (factory profile) numbering. Source: EDRefCard bindingsData.py (MIT), entries
# '231D0200' (Gladiator NXT Premium Right) and '231D3201' (Gladiator NXT Premium Left OTA);
# both sticks use the same numbering. Row order on the VKB sheet is push, up, down, left, right.
STOCK = {
    "MT_A": ("Main trigger, stage 1", "Joy_1"),
    "MT_B": ("Main trigger, stage 2", "Joy_2"),
    "A2_A": ("A2 red button", "Joy_3"),
    "B1_A": ("B1 side button", "Joy_4"),
    "D1_A": ("D1 pinky button", "Joy_5"),
    "A3_A": ("A3 centre hat, push", "Joy_10"), "A3_B": ("A3 centre hat, up", "Joy_6"),
    "A3_C": ("A3 centre hat, down", "Joy_8"), "A3_D": ("A3 centre hat, left", "Joy_9"),
    "A3_E": ("A3 centre hat, right", "Joy_7"),
    "A4_A": ("A4 top hat, push", "Joy_15"), "A4_B": ("A4 top hat, up", "Joy_11"),
    "A4_C": ("A4 top hat, down", "Joy_13"), "A4_D": ("A4 top hat, left", "Joy_14"),
    "A4_E": ("A4 top hat, right", "Joy_12"),
    "C1_A": ("C1 thumb hat, push", "Joy_20"), "C1_B": ("C1 thumb hat, up", "Joy_16"),
    "C1_C": ("C1 thumb hat, down", "Joy_18"), "C1_D": ("C1 thumb hat, left", "Joy_19"),
    "C1_E": ("C1 thumb hat, right", "Joy_17"),
    "A1_A": ("A1 ministick, push", None),
    "A1_B": ("A1 ministick, up (hat mode)", "Joy_POV1Up"), "A1_C": ("A1 ministick, down (hat mode)", "Joy_POV1Down"),
    "A1_D": ("A1 ministick, left (hat mode)", "Joy_POV1Left"), "A1_E": ("A1 ministick, right (hat mode)", "Joy_POV1Right"),
    "RFT_A": ("Rapid-fire trigger, up (push)", "Joy_21"),
    "RFT_B": ("Rapid-fire trigger, down (pull)", "Joy_22"),
    "EN1_A": ("En1 base wheel, up", "Joy_23"), "EN1_B": ("En1 base wheel, down", "Joy_24"),
    "SW1_A": ("Sw1 base switch, up", "Joy_25"), "SW1_B": ("Sw1 base switch, down", "Joy_26"),
    "BASE_B": ("F1 base button", "Joy_27"), "BASE_A": ("F2 base button", "Joy_28"),
    "BASE_C": ("F3 base button", "Joy_29"),
    "AX_X": ("Stick X", "Joy_XAxis"), "AX_Y": ("Stick Y", "Joy_YAxis"),
    "AX_TWIST": ("Twist", "Joy_RZAxis"), "AX_THROTTLE": ("Base throttle wheel", "Joy_ZAxis"),
    "A1_X": ("A1 ministick X (analogue mode)", "Joy_RXAxis"), "A1_Y": ("A1 ministick Y (analogue mode)", "Joy_RYAxis"),
}
# Controls Sumeet's devices actually produced in the s7 runs (9-10 Oct) and his DualVKB preset.
SEEN = {
    "231D0200": {"Joy_1", "Joy_12", "Joy_14", "Joy_POV1Up", "Joy_POV1Down", "Joy_POV1Left", "Joy_POV1Right",
                 "Joy_XAxis", "Joy_YAxis", "Joy_RZAxis"},
    "231D3201": {"Joy_1", "Joy_3", "Joy_10", "Joy_16", "Joy_POV1Up", "Joy_POV1Down",
                 "Joy_POV1Right", "Joy_XAxis", "Joy_YAxis", "Joy_RZAxis"},
}
LOW = {"A1_A": "EDRefCard has no entry for the ministick push; number unknown.",
       "SW1_A": "EDRefCard calls Joy_25/26 'En2'; the EVO base has Sw1 there instead. Likely, not confirmed.",
       "SW1_B": "As SW1_A.",
       "RFT_A": "Which of push/pull is the sheet's up arrow is our reading of the sheet.",
       "RFT_B": "As RFT_A.",
       "AX_THROTTLE": "EDRefCard calls Z 'Throttle'; on the EVO it's the base wheel."}

DEVICES = [
    ("231D0200", "VKBsim Gladiator EVO R", "gladiator-evo-r", "VKB-Gladiator-SCG-RH-v1.0.pdf", "VKB Gladiator SCG (RH)"),
    ("231D3201", "VKBsim Gladiator EVO OT L", "gladiator-evo-ot-l", "VKB-Gladiator-SCG-OTA-LH-v1.0.pdf", "VKB Gladiator SCG OTA (LH)"),
]

def fields(pdf):
    out = {}
    for a in PdfReader(pdf).pages[0].get("/Annots", []):
        a = a.get_object()
        name = str(a.get("/T") or "")
        rect = [round(float(v), 1) for v in a["/Rect"]]
        if name.endswith("_1"):
            out.setdefault(name[:-2], {})["num"] = rect
        elif name.endswith("_2"):
            out.setdefault(name[:-2], {})["txt"] = rect
        elif name.startswith(("AX_", "A1_X", "A1_Y")):
            out.setdefault(name, {})["txt"] = rect
    return out

for pid, product, slug, pdf, title in DEVICES:
    path = f"data/devices/vkb/{pdf}"
    f = fields(path)
    controls = []
    for key, (label, joy) in STOCK.items():
        seen = joy in SEEN[pid]
        if key in LOW or joy is None:
            conf = "low"
        elif seen:
            conf = "high"
        else:
            conf = "medium"
        c = {"control": label, "sheet_field": key, "joy": joy, "confidence": conf,
             "seen_on_device": seen, "box": f.get(key)}
        if key in LOW:
            c["note"] = LOW[key]
        controls.append(c)
    doc = {
        "device_id": pid, "product": product, "profile": "VKB factory default",
        "sheet": {"file": pdf, "title": title, "version": "1.0", "page_pt": [792, 612],
                  "sha256": hashlib.sha256(open(path, "rb").read()).hexdigest(),
                  "url": f"https://www.vkbgaming.com/wp-content/uploads/2026/03/{pdf}",
                  "credit": "VKB Gaming; sheet design by UntoldForce"},
        "sources": [
            "EDRefCard bindingsData.py (MIT), github.com/brammmers/edrefcard2 @ f2e91f8, entry " + pid,
            "VKB fillable templates page, vkbgaming.com/fillable-templates (control names and sheet boxes)",
            "Sumeet's s7 listener runs, 9-10 Oct 2026, and his DualVKB.4.2 preset (controls seen)",
        ],
        "confidence_key": {"high": "stock map and seen on his device", "medium": "stock map only",
                           "low": "inferred or unknown; confirm by pressing it"},
        "controls": controls,
    }
    json.dump(doc, open(f"data/devices/vkb/{slug}.{pid}.json", "w"), indent=1)
    print(slug, {k: sum(c["confidence"] == k for c in controls) for k in ("high", "medium", "low")})
