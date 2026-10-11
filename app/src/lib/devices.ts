/**
 * Device pictures and stock button maps (docs/V1-DESIGN.md §3.2): VKB's reference sheets
 * (recoloured PNGs in public/devices/vkb) and data/devices/vkb/*.json. A user override (from
 * "That's not where I pressed") replaces one control's Joy name.
 */
import left from "../../../data/devices/vkb/gladiator-evo-ot-l.231D3201.json";
import right from "../../../data/devices/vkb/gladiator-evo-r.231D0200.json";

export interface Box { num?: number[]; txt?: number[] }
export interface MapControl { control: string; sheet_field: string; joy: string | null; confidence: string; box?: Box | null }
export interface DeviceMap {
  device_id: string;
  product: string;
  side: "L" | "R";
  title: string;
  image: string;
  controls: MapControl[];
}

export const maps: DeviceMap[] = [
  { ...left, side: "L", title: "Left · Gladiator EVO OT L", image: "/devices/vkb/VKB-Gladiator-SCG-OTA-LH-v1.0.png", controls: left.controls as MapControl[] },
  { ...right, side: "R", title: "Right · Gladiator EVO R", image: "/devices/vkb/VKB-Gladiator-SCG-RH-v1.0.png", controls: right.controls as MapControl[] },
];

export function mapFor(device: string): DeviceMap | undefined {
  return maps.find((m) => m.device_id.toUpperCase() === device.toUpperCase());
}

/** Controls with the user's overrides applied. */
export function controlsWithOverrides(m: DeviceMap, overrides: Record<string, Record<string, string>>): MapControl[] {
  const o = overrides[m.device_id] ?? {};
  return m.controls.map((c) => (o[c.sheet_field] ? { ...c, joy: o[c.sheet_field] ?? c.joy, confidence: "yours" } : c));
}

/** The sheet control that sends `key` on `device`, if the map knows it. */
export function controlFor(device: string, key: string, overrides: Record<string, Record<string, string>>): MapControl | undefined {
  const m = mapFor(device);
  if (!m) return undefined;
  return controlsWithOverrides(m, overrides).find((c) => c.joy === key);
}
