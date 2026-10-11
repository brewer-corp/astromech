// Sample data for the static mockups (docs/V1-DESIGN.md §3). Device IDs are Sumeet's VKB
// pair from the S7 runs; the binds are illustrative, not his real preset.

export type Callout = { id: string; label: string; x: number; y: number; side: "l" | "r"; action: string; kb?: string; warn?: string; voice?: boolean };

export const leftStick = {
  name: "VKB Gladiator EVO OT L",
  id: "231D3201",
  buttons: 128,
  hand: "Left",
  callouts: [
    { id: "Joy_10", label: "Trigger", x: 0, y: 0, side: "l", action: "Push-to-talk (astromech)", voice: true },
    { id: "Joy_1", label: "Red button", x: 0, y: 0, side: "r", action: "Hyperspace jump", kb: "Key_J" },
    { id: "Joy_16", label: "Hat up", x: 0, y: 0, side: "r", action: "Throttle 100 %", kb: "Key_W" },
    { id: "Joy_3", label: "A2", x: 0, y: 0, side: "l", action: "Landing gear", kb: "Key_L" },
    { id: "Joy_12", label: "A1", x: 0, y: 0, side: "l", action: "Night vision", warn: "no keyboard key" },
    { id: "Joy_5", label: "C1", x: 0, y: 0, side: "r", action: "Cargo scoop", kb: "Key_Home" },
    { id: "Joy_20", label: "Pinky", x: 0, y: 0, side: "l", action: "Kill switch (astromech)" },
  ] as Callout[],
};

export const rightStick = {
  name: "VKB Gladiator EVO R",
  id: "231D0200",
  buttons: 128,
  hand: "Right",
  callouts: [
    { id: "Joy_1", label: "Trigger", x: 0, y: 0, side: "r", action: "Primary fire" },
    { id: "Joy_2", label: "Red button", x: 0, y: 0, side: "l", action: "Secondary fire" },
    { id: "Joy_6", label: "Hat up", x: 0, y: 0, side: "r", action: "Target ahead", kb: "Key_T" },
    { id: "Joy_4", label: "A2", x: 0, y: 0, side: "l", action: "Deploy hardpoints", kb: "Key_U" },
    { id: "Joy_9", label: "C1", x: 0, y: 0, side: "r", action: "Boost", kb: "Key_Tab" },
    { id: "Joy_33", label: "Shifted hat", x: 0, y: 0, side: "l", action: "UI back", warn: "above Joy_32: ignored by Elite" },
  ] as Callout[],
};

export type Row = { action: string; name: string; cat: string; primary: string; secondary: string; status: "ok" | "warn" | "error" | "info"; note: string };

export const rows: Row[] = [
  { action: "LandingGearToggle", name: "Landing Gear", cat: "Ship · Miscellaneous", primary: "Key_L", secondary: "Joy_3 · OT L", status: "ok", note: "Voice: gear up / down" },
  { action: "LandingGearToggle (SRV)", name: "Landing Gear", cat: "Ship · Landing overrides", primary: "Key_L", secondary: "—", status: "info", note: "Same key in landing mode" },
  { action: "NightVisionToggle", name: "Night Vision", cat: "Ship · Miscellaneous", primary: "Joy_12 · OT L", secondary: "—", status: "warn", note: "No keyboard key: voice can't press it" },
  { action: "ShipSpotLightToggle", name: "Ship Lights", cat: "Ship · Miscellaneous", primary: "Key_Insert", secondary: "—", status: "ok", note: "Voice: lights on / off" },
  { action: "ToggleCargoScoop", name: "Cargo Scoop", cat: "Ship · Miscellaneous", primary: "Key_Home", secondary: "Joy_5 · OT L", status: "ok", note: "" },
  { action: "DeployHardpointToggle", name: "Deploy Hardpoints", cat: "Ship · Weapons", primary: "Key_U", secondary: "Joy_4 · EVO R", status: "ok", note: "" },
  { action: "HyperSuperCombination", name: "Hyperspace / Supercruise", cat: "Ship · Flight", primary: "Key_J", secondary: "Joy_1 · OT L", status: "ok", note: "" },
  { action: "UI_Back", name: "UI Back", cat: "Interface", primary: "Key_Backspace", secondary: "Joy_33 · EVO R", status: "error", note: "Joy_33 is above Elite's 32-button limit" },
  { action: "GalaxyMapOpen", name: "Galaxy Map", cat: "Ship · Modes", primary: "—", secondary: "Joy_7 · EVO R", status: "warn", note: "No keyboard key" },
  { action: "SetSpeed100", name: "Throttle 100 %", cat: "Ship · Flight throttle", primary: "Key_W", secondary: "Joy_16 · OT L", status: "info", note: "Joy_16 also bound to Galaxy Map zoom (other context, OK)" },
];
