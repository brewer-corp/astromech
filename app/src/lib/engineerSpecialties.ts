/** Module groups each engineer works on, from data/catalog/engineering.json and odyssey.json. */
export const SHIP_SPECIALTIES: Record<string, string[]> = {
  "Bill Turner": ["Plasma Accelerator", "Sensors"],
  "Broo Tarquin": ["Pulse Laser", "Beam Laser"],
  "Chloe Sedesi": ["Frame Shift Drive", "Thrusters"],
  "Colonel Bris Dekker": ["Frame Shift Drive"],
  "Didi Vatermann": ["Shield Generator", "Shield Booster"],
  "Elvira Martuuk": ["Frame Shift Drive", "Thrusters"],
  "Etienne Dorn": ["Rail Gun", "Power Distributor"],
  "Felicity Farseer": ["Frame Shift Drive", "Thrusters"],
  "Hera Tani": ["Power Distributor", "Power Plant"],
  "Juri Ishmaak": ["Missile Rack", "Mine Launcher"],
  "Lei Cheung": ["Shield Generator", "Shield Booster"],
  "Liz Ryder": ["Missile Rack", "Mine Launcher"],
  "Lori Jameson": ["Sensors", "Refinery"],
  "Marco Qwent": ["Power Distributor", "Power Plant"],
  "Marsha Hicks": ["Multi-Cannon", "Fragment Cannon"],
  "Mel Brandon": ["Frame Shift Drive", "Thrusters"],
  "Petra Olmanova": ["Missile Rack", "Mine Launcher"],
  "Professor Palin": ["Frame Shift Drive", "Thrusters"],
  "Ram Tah": ["Point Defence", "Chaff Launcher"],
  "Selene Jean": ["Armour", "Hull Reinforcement Package"],
  "The Dweller": ["Power Distributor", "Pulse Laser"],
  "The Sarge": ["Rail Gun", "Cannon"],
  "Tiana Fortune": ["Frame Shift Drive Interdictor", "Sensors"],
  "Tod \"The Blaster\" McQuinn": ["Multi-Cannon", "Fragment Cannon"],
  "Zacariah Nemo": ["Multi-Cannon", "Fragment Cannon"],
};
export const SUIT_SPECIALTIES: Record<string, string[]> = {
  "Baltanos": ["Combat Movement Speed", "Faster Handling"],
  "Domino Green": ["Enhanced Tracking", "Extra Backpack Capacity"],
  "Eleanor Bresa": ["Added Melee Damage", "Damage Resistance"],
  "Hero Ferrari": ["Faster Handling", "Improved Jump Assist"],
  "Jude Navarro": ["Added Melee Damage", "Damage Resistance"],
  "Kit Fowler": ["Added Melee Damage", "Extra Ammo Capacity"],
  "Oden Geiger": ["Enhanced Tracking", "Improved Battery Capacity"],
  "Rosa Dayette": ["Enhanced Tracking", "Extra Backpack Capacity"],
  "Terra Velasquez": ["Combat Movement Speed", "Improved Hip Fire Accuracy"],
  "Uma Laszlo": ["Damage Resistance", "Faster Shield Regen"],
  "Wellington Beck": ["Extra Backpack Capacity", "Greater Range"],
  "Yarden Bond": ["Audio Masking", "Combat Movement Speed"],
  "Yi Shen": ["Audio Masking", "Headshot Damage"],
};

export function specialties(name: string, kind: "ship" | "suit"): string[] {
  return (kind === "suit" ? SUIT_SPECIALTIES : SHIP_SPECIALTIES)[name] ?? [];
}
