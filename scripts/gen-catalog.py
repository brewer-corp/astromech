#!/usr/bin/env python3
"""Generate data/catalog/controls.json from EDRefCard's control list (MIT, Copyright (c) 2017
Richard Buckle; github.com/brammmers/edrefcard2, www/scripts/controlsData.py).

Usage: scripts/gen-catalog.py <path to edrefcard2 checkout>
Labels are turned into sentence case (the app shows mixed case, not the game's capitals).
"""
import json, os, re, sys

GROUPS = {  # (EDRefCard Group, Category) -> (list group, context, sort)
    ("Ship", "Navigation"): ("Flight", "Ship", 10),
    ("Ship", "General"): ("Ship systems", "Ship", 20),
    ("Ship", None): ("Ship systems", "Ship", 20),
    ("Ship", "Combat"): ("Weapons and combat", "Ship", 30),
    ("Scanners", "General"): ("Scanners (FSS, DSS)", "Ship", 40),
    ("Head look", "Navigation"): ("Head look", "Ship", 50),
    ("Fighter", "Combat"): ("Fighter orders", "Ship", 60),
    ("Multicrew", "General"): ("Multicrew", "Ship", 70),
    ("SRV", "Navigation"): ("SRV driving", "SRV", 110),
    ("SRV", "Combat"): ("SRV turret", "SRV", 120),
    ("SRV", None): ("SRV systems", "SRV", 130),
    ("OnFoot", "Navigation"): ("On foot movement", "On foot", 210),
    ("OnFoot", "Combat"): ("On foot combat", "On foot", 220),
    ("OnFoot", None): ("On foot tools and interface", "On foot", 230),
    ("UI", "UI"): ("Interface", "General", 310),
    ("Galaxy map", "UI"): ("Galaxy map", "General", 320),
    ("Camera", "General"): ("Cameras", "General", 330),
    ("Camera", "Camera"): ("Cameras", "General", 330),
    ("Holo-Me", "General"): ("Holo-Me", "General", 340),
    ("Misc", "General"): ("General", "General", 300),
    ("Misc", "Navigation"): ("General", "General", 300),
}
KEEP = {"FSD", "FSS", "DSS", "SRV", "UI", "HUD", "ECM", "FA", "GalMap", "SysMap", "ACS", "SCO", "AI", "ADS", "PIP", "PIPs", "X", "Y", "Z", "FOV", "DSD", "SLF", "NPC", "VR", "II", "III"}


# Clearer names for the common actions (EDRefCard's are written for a printed card).
NAMES = {
    "PrimaryFire": "Primary fire", "SecondaryFire": "Secondary fire",
    "HumanoidForwardButton": "Walk forward", "HumanoidBackwardButton": "Walk backward",
    "HumanoidStrafeLeftButton": "Strafe left", "HumanoidStrafeRightButton": "Strafe right",
    "HumanoidZoomButton": "Zoom (aim down sights)", "HumanoidPrimaryFireButton": "Fire",
    "HyperSuperCombination": "Frame shift drive (supercruise / jump)", "Supercruise": "Supercruise",
    "Hyperspace": "Hyperspace jump", "TargetNextRouteSystem": "Target next system in route",
    "ToggleFlightAssist": "Flight assist", "UseBoostJuice": "Engine boost",
    "LandingGearToggle": "Landing gear", "ShipSpotLightToggle": "Ship lights",
    "NightVisionToggle": "Night vision", "PlayerHUDModeToggle": "HUD mode (analysis / combat)",
    "DeployHardpointToggle": "Hardpoints", "ToggleCargoScoop": "Cargo scoop",
    "DeployHeatSink": "Heat sink", "ToggleButtonUpInput": "Silent running",
    "SetSpeedZero": "Throttle 0%", "SetSpeed25": "Throttle 25%", "SetSpeed50": "Throttle 50%",
    "SetSpeed75": "Throttle 75%", "SetSpeed100": "Throttle 100%",
    "GalaxyMapOpen": "Galaxy map", "SystemMapOpen": "System map",
    "FocusLeftPanel": "Left panel (navigation, contacts)", "FocusRightPanel": "Right panel (ship)",
    "ExplorationFSSEnter": "Enter FSS", "ExplorationFSSQuit": "Leave FSS",
    "ExplorationFSSDiscoveryScan": "Discovery scan (honk)", "SelectTarget": "Select target ahead",
    "SelectHighestThreat": "Highest threat", "IncreaseEnginesPower": "Power to engines",
    "IncreaseSystemsPower": "Power to systems", "IncreaseWeaponsPower": "Power to weapons",
    "ResetPowerDistribution": "Balance power", "ThrottleAxis": "Throttle",
    "PitchAxisRaw": "Pitch", "RollAxisRaw": "Roll", "YawAxisRaw": "Yaw",
}


def sentence(name: str) -> str:
    words = name.split(" ")
    out = []
    for i, w in enumerate(words):
        core = re.sub(r"[^A-Za-z]", "", w)
        if core in KEEP or (len(core) > 1 and core.isupper()):
            out.append(w)
        elif i == 0:
            out.append(w[:1].upper() + w[1:].lower() if w[:1].isalpha() else w)
        else:
            out.append(w.lower())
    return " ".join(out)


def main():
    root = sys.argv[1]
    sys.path.insert(0, os.path.join(root, "www", "scripts"))
    import controlsData  # noqa: E402
    rows = []
    for cid, c in controlsData.controls.items():
        g = GROUPS.get((c["Group"], c.get("Category")))
        if not g:
            raise SystemExit(f"unmapped group {c['Group']}/{c.get('Category')} for {cid}")
        rows.append({
            "id": cid,
            "name": NAMES.get(cid) or sentence(c["Name"].strip()),
            "group": g[0],
            "context": g[1],
            "group_order": g[2],
            "order": c.get("Order", 0),
            "analogue": c.get("Type") == "Analogue",
        })
    rows.sort(key=lambda r: (r["group_order"], r["order"], r["id"]))
    out = {
        "source": "EDRefCard controlsData.py (MIT, Copyright (c) 2017 Richard Buckle), github.com/brammmers/edrefcard2",
        "controls": rows,
    }
    path = os.path.join(os.path.dirname(__file__), "..", "data", "catalog", "controls.json")
    with open(path, "w") as f:
        json.dump(out, f, indent=0)
        f.write("\n")
    print(f"{len(rows)} controls -> {path}")


if __name__ == "__main__":
    main()
