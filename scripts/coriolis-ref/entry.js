// Reference stats for astromech-ship's fixture builds (crates/astromech-ship/tests/coriolis.rs),
// from Coriolis's own calculation code.
//
// Setup, outside the repo (the Coriolis checkouts are third-party code, read as data):
//   git clone https://github.com/EDCD/coriolis REF/coriolis
//   git clone https://github.com/EDCD/coriolis-data REF/coriolis-data
//   cp scripts/coriolis-ref/* WORK/ && cd WORK && npm i esbuild@0.25 lodash lz-string react
//   node build.mjs REF                       # bundles this file and Coriolis into out.js
// Run:
//   CORIOLIS_DATA=REF/coriolis-data node out.js crates/astromech-ship/tests/fixtures/*.json
//   RUST=1 ... prints the test blocks; FULL=1 ... prints unrounded values.
import fs from 'fs';
import { shipFromLoadoutJSON } from 'coriolis/src/app/utils/JournalUtils';
import * as Calc from 'coriolis/src/app/shipyard/Calculations';
import Ship from 'coriolis/src/app/shipyard/Ship';
import { shipModelFromJson } from 'coriolis/src/app/utils/CompanionApiUtils';
import { Ships } from 'coriolis-data/dist';

// Coriolis's display formats (src/app/i18n/Language.jsx; d3 ',.Nf' is toFixed).
const FULL = !!process.env.FULL;
const fixed = (n) => (x) => FULL ? x : Number(x).toFixed(n);
const int = fixed(0), f1 = fixed(1), f2 = fixed(2);
const round = (x) => FULL ? x : String(Math.round(x * 100) / 100);
const round1 = (x) => FULL ? x : String(Math.round(Math.round(x * 100) / 100 * 10) / 10); // getPretty then round1
const pct1 = (x) => FULL ? x * 100 : (x * 100).toFixed(1);
const time = (d) => FULL ? d : (d < 0 ? '-' : '') + Math.floor(Math.abs(d) / 60) + ':' + ('00' + Math.floor(Math.abs(d) % 60)).substr(-2, 2);

const quiet = console.log;
for (const file of process.argv.slice(2)) {
  const json = JSON.parse(fs.readFileSync(file, 'utf8'));
  console.log = () => {};
  const imported = shipFromLoadoutJSON(json);
  // The import page stores the build as a code and the outfitting page rebuilds the ship from
  // it (ModalImport.jsx, OutfittingPage.jsx); the displayed stats come from that rebuild.
  const model = shipModelFromJson(json);
  const ship = new Ship(model, Ships[model].properties, Ships[model].slots).buildFrom(imported.toString());
  console.log = quiet;
  // Every fitted module must have landed in a slot as itself, with its blueprint applied.
  const fitted = [imported.bulkheads, ...imported.standard, ...imported.hardpoints, ...imported.internal].map(s => s.m).filter(Boolean);
  const rebuilt = [ship.bulkheads, ...ship.standard, ...ship.hardpoints, ...ship.internal].map(s => s.m).filter(Boolean);
  const syms = (l) => l.map(m => (m.symbol || m.name) + (m.blueprint && m.blueprint.grade ? '/' + m.blueprint.grade + (m.blueprint.special ? m.blueprint.special.edname : '') : '')).join(',');
  if (syms(fitted) !== syms(rebuilt)) throw new Error(file + ': rebuild differs\n' + syms(fitted) + '\n' + syms(rebuilt));
  for (const jm of json.Modules) {
    if (jm.Slot === 'CargoHatch') continue;
    const m = fitted.find(x => x.symbol && x.symbol.toLowerCase() === jm.Item.toLowerCase() && !x._seen);
    if (!m && !/armour/i.test(jm.Slot)) throw new Error(file + ': not imported: ' + jm.Slot + ' ' + jm.Item);
    if (m) {
      m._seen = true;
      if (m.preEngineered) throw new Error(file + ': pre-engineered variant picked for ' + jm.Item);
      if (jm.Engineering && (!m.blueprint || m.blueprint.grade !== jm.Engineering.Level)) throw new Error(file + ': blueprint not applied ' + jm.Slot);
      if (jm.Engineering && jm.Engineering.ExperimentalEffect && !(m.blueprint.special)) throw new Error(file + ': experimental not applied ' + jm.Slot);
    }
  }
  const fsd = ship.standard[2].m;
  const pd = ship.standard[4].m;
  const canJump = ship.getSlotStatus(ship.standard[2]) == 3;
  const canThrust = ship.canThrust(0, ship.fuelCapacity);
  const canBoost = ship.canBoost(0, ship.fuelCapacity);
  const armour = Calc.armourMetrics(ship);
  // Offence panel total at an engagement range inside every falloff (0 m).
  let sdps = 0;
  for (const w of Calc.offenceMetrics(ship, ship, 4, 0, 0)) {
    for (const k of ['absolute', 'explosive', 'kinetic', 'thermal']) sdps += w.sdps.base[k] ? w.sdps.base[k] : 0; // Offence.jsx addSDps
  }
  const ttd = Calc.timeToDrainWep(ship, 4);
  const hasShield = !!ship.findInternalByGroup('sg');
  const out = {
    hull_mass: String(ship.hullMass),
    unladen_mass: int(ship.unladenMass),
    laden_mass: int(ship.ladenMass),
    fuel_capacity: round(ship.fuelCapacity),
    cargo_capacity: round(ship.cargoCapacity),
    jump_max: canJump ? f2(Calc.jumpRange(ship.unladenMass - ship.fuelCapacity + fsd.getMaxFuelPerJump(), fsd, fsd.getMaxFuelPerJump(), ship)) : '0',
    jump_unladen: canJump ? f2(Calc.jumpRange(ship.unladenMass, fsd, ship.fuelCapacity, ship)) : '0',
    jump_laden: canJump ? f2(Calc.jumpRange(ship.unladenMass + ship.cargoCapacity, fsd, ship.fuelCapacity, ship)) : '0',
    jump_total_unladen: canJump ? f2(Calc.totalJumpRange(ship.unladenMass, fsd, ship.fuelCapacity, ship)) : '0',
    jump_total_laden: canJump ? f2(Calc.totalJumpRange(ship.unladenMass + ship.cargoCapacity, fsd, ship.fuelCapacity, ship)) : '0',
    power_generation: f2(ship.standard[0].m.getPowerGeneration()),
    power_retracted: f2(ship.powerRetracted),
    power_deployed: f2(ship.powerDeployed),
    shield: hasShield ? int(ship.shield) : null,
    shield_explosive: hasShield ? pct1(ship.shieldExplRes) : null,
    shield_kinetic: hasShield ? pct1(ship.shieldKinRes) : null,
    shield_thermal: hasShield ? pct1(ship.shieldThermRes) : null,
    armour: int(ship.armour),
    hull_explosive: pct1(ship.hullExplRes),
    hull_kinetic: pct1(ship.hullKinRes),
    hull_thermal: pct1(ship.hullThermRes),
    hull_caustic: pct1(ship.hullCausRes),
    module_armour: int(armour.modulearmour),
    dps: f1(ship.totalDps),
    sustained_dps: f1(sdps),
    eps: f1(ship.totalEps),
    time_to_drain: ttd === Infinity ? null : time(ttd),
    wep_capacity: round1(pd.getWeaponsCapacity()),
    wep_recharge: round1(pd.getWeaponsRechargeRate()),
    sys_capacity: round1(pd.getSystemsCapacity()),
    sys_recharge: round1(pd.getSystemsRechargeRate()),
    eng_capacity: round1(pd.getEnginesCapacity()),
    eng_recharge: round1(pd.getEnginesRechargeRate()),
    speed: canThrust ? int(ship.calcSpeed(4, ship.fuelCapacity, 0, false)) : '0',
    boost: canBoost ? int(ship.calcSpeed(4, ship.fuelCapacity, 0, true)) : '0',
  };
  const name = file.split('/').pop().replace(/\.json$/, '');
  if (process.env.RUST) {
    // The expectations block for crates/astromech-ship/tests/coriolis.rs.
    const rows = Object.entries(out).map(([k, v]) => `        ("${k}", ${v === null ? 'None' : 'Some("' + v + '")'}),`);
    process.stdout.write(`#[test]\nfn ${name}() {\n    check(\n        "${name}",\n        &[\n${rows.join('\n')}\n        ],\n    );\n}\n\n`);
  } else {
    process.stdout.write(name + ' ' + JSON.stringify(out, null, 1) + '\n');
  }
}
