/**
 * Placeholder pitch / roll / yaw (°/s at four pips to engines) until astromech-ship
 * computes handling from Coriolis hull rates and thruster mass. Keys are journal ship types.
 */
const RATES: Record<string, { pitch: number; roll: number; yaw: number }> = {
  adder: { pitch: 42, roll: 110, yaw: 16 },
  anaconda: { pitch: 25, roll: 80, yaw: 10 },
  asp: { pitch: 38, roll: 100, yaw: 16 },
  cobramkiii: { pitch: 44, roll: 140, yaw: 18 },
  diamondbackxl: { pitch: 35, roll: 100, yaw: 15 },
  eagle: { pitch: 50, roll: 120, yaw: 20 },
  federation_corvette: { pitch: 28, roll: 90, yaw: 12 },
  ferdelance: { pitch: 38, roll: 100, yaw: 16 },
  cutter: { pitch: 18, roll: 45, yaw: 8 },
  krait_mkii: { pitch: 36, roll: 90, yaw: 16 },
  krait_light: { pitch: 38, roll: 90, yaw: 16 },
  python: { pitch: 29, roll: 90, yaw: 12 },
  type9: { pitch: 20, roll: 40, yaw: 8 },
  viper: { pitch: 42, roll: 110, yaw: 18 },
};

export function handling(shipType: string): { pitch: number; roll: number; yaw: number } | null {
  return RATES[shipType.toLowerCase()] ?? null;
}
