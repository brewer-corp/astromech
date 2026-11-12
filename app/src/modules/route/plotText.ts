// What a finished plot says, in the header's plot panel and under the Route page's title.
import type { Plotted } from "../../lib/route";

const ly = (n: number) => Math.round(n).toLocaleString("en-GB");
const count = (n: number, one: string, many: string) => `${n.toLocaleString("en-GB")} ${n === 1 ? one : many}`;

/** "12 jumps · 452 ly · 2 refuel stops, at 31.20 ly a jump". */
export function plotSummary(p: Plotted): string {
  const parts = [count(p.jumps, "jump", "jumps"), `${ly(p.ly)} ly`];
  if (p.neutrons) parts.push(count(p.neutrons, "neutron boost", "neutron boosts"));
  if (p.refuels) parts.push(count(p.refuels, "refuel stop", "refuel stops"));
  return `${parts.join(" · ")}, at ${p.range.toFixed(2)} ly a jump`;
}

/** "Plotted on this PC." or "Plotted by Spansh." */
export function plotWhere(p: Plotted): string {
  if (!p.local) return "Plotted by Spansh.";
  return p.fewest === false ? "Plotted on this PC, near the fewest jumps." : "Plotted on this PC.";
}
