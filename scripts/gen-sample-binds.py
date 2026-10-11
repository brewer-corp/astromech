#!/usr/bin/env python3
"""Write data/samples/Example-DualVKB.4.2.binds: an example preset in the shape of Sumeet's
DualVKB pair (L = 231D3201 Omni Throttle, R = 231D0200), used for tests, the browser preview of
the app and screenshots. Not a Frontier file: every line is generated here.
It has a few deliberate problems (no keyboard key for night vision, a Joy_33 bind, a conflict,
an unbound "Target next system in route") so the health markers show."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
L, R, K = "231D3201", "231D0200", "Keyboard"
B = {  # action: (primary, secondary); each (device, key[, modifier key on keyboard])
    "PitchAxisRaw": [(R, "Joy_YAxis")], "RollAxisRaw": [(R, "Joy_XAxis")], "YawAxisRaw": [(R, "Joy_RZAxis")],
    "ThrottleAxis": [(L, "Joy_YAxis")], "LateralThrustRaw": [(L, "Joy_RXAxis")], "VerticalThrustRaw": [(L, "Joy_RYAxis")],
    "HyperSuperCombination": [(L, "Joy_1"), (K, "Key_J")], "SetSpeed100": [(L, "Joy_16"), (K, "Key_X")],
    "SetSpeedZero": [(L, "Joy_18"), (K, "Key_X", "Key_LeftShift")], "SetSpeed75": [(L, "Joy_17"), None],
    "SetSpeed50": [(L, "Joy_19"), None], "UseBoostJuice": [(R, "Joy_4"), (K, "Key_Tab")],
    "ToggleFlightAssist": [(R, "Joy_5"), (K, "Key_Z")], "LandingGearToggle": [(L, "Joy_4"), (K, "Key_L")],
    "ShipSpotLightToggle": [(L, "Joy_25"), (K, "Key_Insert")], "NightVisionToggle": [(L, "Joy_26"), None],
    "ToggleCargoScoop": [(L, "Joy_5"), (K, "Key_Home")], "DeployHardpointToggle": [(R, "Joy_3"), (K, "Key_U")],
    "PrimaryFire": [(R, "Joy_1"), None], "SecondaryFire": [(R, "Joy_21"), None],
    "CycleFireGroupNext": [(R, "Joy_22"), (K, "Key_N")], "SelectTarget": [(R, "Joy_15"), (K, "Key_T")],
    "CycleNextTarget": [(R, "Joy_12"), (K, "Key_G")], "CyclePreviousTarget": [(R, "Joy_14"), None],
    "SelectHighestThreat": [(R, "Joy_11"), (K, "Key_H")], "CycleNextSubsystem": [(R, "Joy_13"), (K, "Key_Y")],
    "IncreaseEnginesPower": [(R, "Joy_POV1Up"), (K, "Key_UpArrow")], "IncreaseWeaponsPower": [(R, "Joy_POV1Right"), (K, "Key_RightArrow")],
    "IncreaseSystemsPower": [(R, "Joy_POV1Left"), (K, "Key_LeftArrow")], "ResetPowerDistribution": [(R, "Joy_POV1Down"), (K, "Key_DownArrow")],
    "DeployHeatSink": [(R, "Joy_6"), (K, "Key_V")], "FireChaffLauncher": [(R, "Joy_8"), None],
    "UseShieldCell": [(R, "Joy_7"), None], "ToggleButtonUpInput": [(K, "Key_Delete"), None],
    "GalaxyMapOpen": [(L, "Joy_27"), (K, "Key_M")], "SystemMapOpen": [(L, "Joy_28"), (K, "Key_Comma")],
    "FocusLeftPanel": [(L, "Joy_POV1Left"), (K, "Key_1")], "FocusRightPanel": [(L, "Joy_POV1Right"), (K, "Key_4")],
    "FocusCommsPanel": [(L, "Joy_POV1Up"), (K, "Key_2")], "FocusRadarPanel": [(L, "Joy_POV1Down"), (K, "Key_3")],
    "UI_Up": [(L, "Joy_6"), (K, "Key_W")], "UI_Down": [(L, "Joy_8"), (K, "Key_S")], "UI_Left": [(L, "Joy_9"), (K, "Key_A")],
    "UI_Right": [(L, "Joy_7"), (K, "Key_D")], "UI_Select": [(L, "Joy_20"), (K, "Key_Space")], "UI_Back": [(L, "Joy_33"), (K, "Key_Backspace")],
    "CycleNextPanel": [(L, "Joy_11"), (K, "Key_E")], "CyclePreviousPanel": [(L, "Joy_13"), (K, "Key_Q")],
    "PlayerHUDModeToggle": [(L, "Joy_29"), (K, "Key_LeftAlt")], "OrbitLinesToggle": [(K, "Key_O"), None],
    "ExplorationFSSEnter": [(L, "Joy_3"), None], "ExplorationFSSQuit": [(L, "Joy_3"), (K, "Key_Backspace")],
    "ExplorationFSSDiscoveryScan": [(R, "Joy_1"), None], "ExplorationFSSZoomIn": [(L, "Joy_16"), None],
    "ExplorationFSSZoomOut": [(L, "Joy_18"), None], "ExplorationFSSRadioTuningX_Raw": [(L, "Joy_ZAxis")],
    "ExplorationFSSTarget": [(R, "Joy_2"), None], "ExplorationSAAExitThirdPerson": [(L, "Joy_3"), None],
    "UpThrustButton": [(K, "Key_R"), None], "DownThrustButton": [(K, "Key_F"), None],
    "LeftThrustButton": [(K, "Key_Q"), None], "RightThrustButton": [(K, "Key_E"), None],
    "SteeringAxis": [(R, "Joy_XAxis")], "DriveSpeedAxis": [(L, "Joy_YAxis")], "AutoBreakBuggyButton": [(L, "Joy_4"), (K, "Key_X")],
    "VerticalThrustersButton": [(R, "Joy_4"), (K, "Key_Space")], "ToggleBuggyTurretButton": [(R, "Joy_3"), (K, "Key_U")],
    "BuggyPrimaryFireButton": [(R, "Joy_1"), None], "HeadlightsBuggyButton": [(L, "Joy_25"), (K, "Key_Insert")],
    "HumanoidForwardButton": [(K, "Key_W"), None], "HumanoidBackwardButton": [(K, "Key_S"), None],
    "HumanoidStrafeLeftButton": [(K, "Key_A"), None], "HumanoidStrafeRightButton": [(K, "Key_D"), None],
    "HumanoidPrimaryInteractButton": [(K, "Key_E"), None], "HumanoidSprintButton": [(K, "Key_LeftShift"), None],
    "HumanoidJumpButton": [(K, "Key_Space"), None], "HumanoidCrouchButton": [(K, "Key_C"), None],
    "HumanoidPrimaryFireButton": [("Mouse", "Mouse_1"), None], "HumanoidZoomButton": [("Mouse", "Mouse_2"), None],
    "HumanoidItemWheelButton": [(K, "Key_Tab"), None], "HumanoidReloadButton": [(K, "Key_R"), None],
}
cat = json.load(open(os.path.join(HERE, "..", "data", "catalog", "controls.json")))["controls"]
analogue = {c["id"] for c in cat if c["analogue"]}


def slot(name, b):
    if not b:
        return f'\t\t<{name} Device="{{NoDevice}}" Key="" />\n'
    if len(b) == 3:
        return f'\t\t<{name} Device="{b[0]}" Key="{b[1]}">\n\t\t\t<Modifier Device="Keyboard" Key="{b[2]}" />\n\t\t</{name}>\n'
    return f'\t\t<{name} Device="{b[0]}" Key="{b[1]}" />\n'


out = '<?xml version="1.0" encoding="UTF-8" ?>\n<Root PresetName="Example DualVKB" MajorVersion="4" MinorVersion="2">\n\t<KeyboardLayout>en-US</KeyboardLayout>\n'
for c in cat:
    a = c["id"]
    b = B.get(a, [])
    out += f"\t<{a}>\n"
    if a in analogue:
        out += slot("Binding", b[0] if b else None)
        out += '\t\t<Inverted Value="0" />\n\t\t<Deadzone Value="0.00000000" />\n'
    else:
        out += slot("Primary", b[0] if b else None) + slot("Secondary", b[1] if len(b) > 1 else None)
    out += f"\t</{a}>\n"
out += "</Root>\n"
p = os.path.join(HERE, "..", "data", "samples", "Example DualVKB.4.2.binds")
open(p, "w").write(out)
print(p)
