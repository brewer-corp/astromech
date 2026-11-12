/**
 * The one plot setup (docs/PLOTTING.md): the plotter, who works a neutron route out, Spansh's
 * efficiency and the exact plotter's refuelling, shared by the header's plot panel (Plot.svelte)
 * and the Route page's title (Title.svelte), so the two always agree. It also holds what both
 * show beside it: the ship a route would be plotted for, the neutron star file's status, and
 * whether a plot is running (the shell plots one at a time).
 */
import type { NeutronFile } from "./neutron";
import { neutronFile } from "./neutron";
import { plotRoute, plotSetup as readPlotShip, type PlotSetup as PlotShip, type Plotted, type Plotter, type RouteView } from "./route";
import type { Engine } from "./api/types/Engine";
import type { PlotSettings } from "./api/types/PlotSettings";

export type { Engine } from "./api/types/Engine";

const message = (e: unknown) => String(e instanceof Error ? e.message : e);

class Setup {
  plotter = $state<Plotter>("neutron");
  /** Who works a neutron route out, as the commander picked it. "local" needs the file. */
  engine = $state<Engine>("local");
  efficiency = $state(60);
  refuel = $state(true);
  /** The neutron star file's status, as last read (null until then). */
  file = $state<NeutronFile | null>(null);
  /** Why the file's status couldn't be read. */
  fileError = $state<string | null>(null);
  /** Where the commander is and the ship, as last read. */
  ship = $state<PlotShip | null>(null);
  /** A plot is running, here or in the other place. */
  busy = $state(false);
  /** The plot settings last taken from the route followed, so a route is adopted once. */
  private adopted: string | null = null;
  private followers = 0;
  private timer: ReturnType<typeof setInterval> | undefined;

  /** The neutron star file is there, so On this PC can be picked. */
  get hasFile(): boolean {
    return !!this.file?.file;
  }

  /** Who will work the next route out: exact routes and a missing file are Spansh's. */
  get worker(): Engine {
    return this.plotter === "neutron" && this.engine === "local" && this.hasFile ? "local" : "spansh";
  }

  /** Show what the route followed was plotted with, the first time it is seen. Changes the
   * commander makes afterwards stay until another route is plotted. */
  adopt(p: PlotSettings | null | undefined) {
    if (!p) return;
    const key = JSON.stringify(p);
    if (key === this.adopted) return;
    this.adopted = key;
    this.plotter = p.plotter;
    // Not the engine: a route records `spansh` when the file was missing, which says nothing
    // about who the commander wants to work the next one out.
    if (p.plotter === "neutron") this.efficiency = p.efficiency;
    else this.refuel = p.refuel_every_scoopable;
  }

  async readFile(): Promise<void> {
    try {
      this.file = await neutronFile();
      this.fileError = null;
    } catch (e) {
      this.fileError = message(e);
    }
  }

  async readShip(): Promise<void> {
    try {
      this.ship = await readPlotShip();
    } catch (e) {
      this.ship = { here: null, ship: null, ship_error: message(e) };
    }
  }

  /** Keep the ship current while a plotter is on screen (a swap, cargo bought or sold). Returns
   * the stop; the reads run while anything follows. */
  follow(): () => void {
    if (this.followers++ === 0) {
      void this.readShip();
      void this.readFile();
      this.timer = setInterval(() => void this.readShip(), 5000);
    }
    return () => {
      if (--this.followers === 0) clearInterval(this.timer);
    };
  }

  /** Tests: back to the defaults, with nothing read. */
  reset() {
    Object.assign(this, { plotter: "neutron", engine: "local", efficiency: 60, refuel: true, file: null, fileError: null, ship: null, busy: false });
    this.adopted = null;
  }

  /** Plot to `to` from the commander's system with this setup and follow it. */
  async plot(to: string): Promise<{ plotted: Plotted; view: RouteView }> {
    this.busy = true;
    try {
      const eff = Math.min(100, Math.max(1, Math.round(Number(this.efficiency) || 60)));
      const neutron = this.plotter === "neutron";
      const r = await plotRoute({
        plotter: this.plotter,
        to: to.trim(),
        efficiency: eff,
        refuel_every_scoopable: this.refuel,
        ...(neutron ? { engine: this.engine } : {}),
      });
      this.adopted = JSON.stringify(r.plotted.plot);
      return r;
    } finally {
      this.busy = false;
      // A plot may have found the file can't be read, and the ship may have changed.
      void this.readFile();
      void this.readShip();
    }
  }
}

export const setup = new Setup();
