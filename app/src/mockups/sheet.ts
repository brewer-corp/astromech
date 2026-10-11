// Field geometry of VKB's fillable template "VKB Gladiator SCG OTA (LH)" v1.0
// (https://www.vkbgaming.com/wp-content/uploads/2026/03/VKB-Gladiator-SCG-OTA-LH-v1.0.pdf).
// Rects are the PDF's own AcroForm widget rectangles in points (x1, y1, x2, y2; origin bottom
// left; page 792 × 612). Each control has a narrow "_1" cell (button number) and a wide "_2"
// cell (what it does). In the app these are read from the user's own copy of the PDF at
// runtime; they are copied here only to draw the placeholder for the design screenshot.

export type Rect = [number, number, number, number];
export type Field = { id: string; sym: string; num: Rect; txt: Rect };
export type Section = { title: string; x: number; top: number; rows: Field[] };
type Col = { num: [number, number]; txt: [number, number] };

const L: Col = { num: [60, 82], txt: [83, 259] }; // left column
const R: Col = { num: [556, 577], txt: [578, 755] }; // right column
const r = (col: Col, id: string, sym: string, y1: number, y2: number): Field => ({ id, sym, num: [col.num[0], y1, col.num[1], y2], txt: [col.txt[0], y1, col.txt[1], y2] });
const HAT = ["Push", "Up", "Down", "Left", "Right"];
const hat = (col: Col, id: string, ys: [number, number][]) =>
  ys.map(([y1, y2], i) => r(col, `${id}_${"ABCDE".charAt(i)}`, HAT[i] ?? "", y1, y2));

export const OTA_LH: Section[] = [
  { title: "B1 (Side Button)", x: 47, top: 532, rows: [r(L, "B1_A", "", 503, 520)] },
  { title: "A4 (Top Left Hat)", x: 47, top: 487, rows: hat(L, "A4", [[459, 475], [442, 458], [426, 441], [409, 425], [392, 408]]) },
  { title: "A3 (Center Hat)", x: 47, top: 377, rows: hat(L, "A3", [[349, 365], [333, 348], [316, 332], [299, 315], [282, 298]]) },
  { title: "A1 (Ministick)", x: 47, top: 267, rows: hat(L, "A1", [[240, 255], [223, 239], [206, 222], [190, 205], [173, 189]]) },
  { title: "Base Buttons", x: 47, top: 156, rows: [r(L, "BASE_A", "F2", 128, 144), r(L, "BASE_B", "F1", 112, 127), r(L, "BASE_C", "F3", 95, 111)] },
  { title: "Sw1 (Base Left Side)", x: 47, top: 80, rows: [r(L, "SW1_A", "Up", 53, 68), r(L, "SW1_B", "Down", 36, 52)] },
  { title: "A2 (Red Button)", x: 543, top: 496, rows: [r(R, "A2_A", "", 467, 484)] },
  { title: "Rapid Fire Trigger", x: 543, top: 444, rows: [r(R, "RFT_A", "Up", 416, 432), r(R, "RFT_B", "Down", 400, 415)] },
  { title: "Main Trigger", x: 543, top: 377, rows: [r(R, "MT_A", "1", 349, 365), r(R, "MT_B", "2", 333, 348)] },
  { title: "C1 (Thumb Hat)", x: 543, top: 312, rows: hat(R, "C1", [[284, 300], [268, 283], [251, 267], [234, 250], [217, 233]]) },
  { title: "D1 (Pinky Button)", x: 543, top: 197, rows: [r(R, "D1_A", "", 167, 185)] },
  { title: "En1 (Base Right Side)", x: 543, top: 147, rows: [r(R, "EN1_A", "Up", 119, 135), r(R, "EN1_B", "Down", 102, 118)] },
];

// Example capture (what the guided "press each control" step would record) and Elite actions.
// Illustrative values, not Sumeet's real numbering.
export const capture: Record<string, [string, string]> = {
  B1_A: ["4", "Cargo scoop"],
  A4_A: ["", ""], A4_B: ["13", "Throttle 100 %"], A4_C: ["15", "Throttle 0 %"], A4_D: ["", ""], A4_E: ["", ""],
  A3_A: ["7", "Galaxy map"], A3_B: ["8", "Thrust up"], A3_C: ["9", "Thrust down"], A3_D: ["10", "Push-to-talk (astromech)"], A3_E: ["11", "Ship lights"],
  A1_A: ["12", "Night vision"],
  BASE_A: ["29", "Silent running"], BASE_B: ["30", ""], BASE_C: ["31", ""],
  A2_A: ["3", "Landing gear"],
  RFT_A: ["", ""], RFT_B: ["", ""],
  MT_A: ["1", "Frame shift drive"], MT_B: ["2", ""],
  C1_A: ["", ""], C1_B: ["", ""],
  D1_A: ["5", "Kill switch (astromech)"],
};
