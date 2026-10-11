// Sample data for the keybinds mockups. Device names follow Sumeet's pair: L = Gladiator EVO
// OT L (231D3201), R = Gladiator EVO R (231D0200). Bindings are illustrative.
export type Bind = string | null; // "[L]" keyboard, "L · JOY 3" joystick, null = empty slot
export type Row = { name: string; action: string; p: Bind; s: Bind; mark?: string; sel?: boolean; about?: string };
export type Group = { name: string; open?: boolean; issues?: number; rows?: Row[] };

export const groups: Group[] = [
  { name: "Mouse controls" },
  { name: "Flight rotation" },
  { name: "Flight thrust" },
  { name: "Flight throttle", issues: 1 },
  { name: "Flight landing overrides" },
  {
    name: "Flight miscellaneous",
    open: true,
    issues: 1,
    rows: [
      { name: "Toggle flight assist", action: "ToggleFlightAssist", p: "[Z]", s: "R · JOY 7" },
      { name: "Engine boost", action: "UseBoostJuice", p: "[TAB]", s: "R · JOY 9" },
      { name: "Frame shift drive", action: "HyperSuperCombination", p: "[J]", s: "L · JOY 1" },
      { name: "Landing gear", action: "LandingGearToggle", p: "[L]", s: "L · JOY 3" },
      { name: "Cargo scoop", action: "ToggleCargoScoop", p: "[HOME]", s: "L · JOY 4" },
      { name: "Ship lights", action: "ShipSpotLightToggle", p: "[INSERT]", s: null },
      { name: "Night vision", action: "NightVisionToggle", p: null, s: "L · JOY 12", mark: "No keyboard key", sel: true },
      { name: "Rotational correction", action: "ToggleRotationCorrection", p: null, s: null },
    ],
  },
  { name: "Targeting" },
  { name: "Weapons" },
  { name: "Cooling" },
  {
    name: "Interface mode",
    open: true,
    issues: 1,
    rows: [
      { name: "UI back", action: "UI_Back", p: "[BACKSPACE]", s: "R · JOY 33", mark: "Joy 33: ignored" },
      { name: "UI select", action: "UI_Select", p: "[SPACE]", s: "R · POV1 PUSH" },
    ],
  },
  { name: "Headlook mode" },
  { name: "Galaxy map" },
];
