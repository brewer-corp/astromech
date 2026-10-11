/**
 * Shared keybinds state for all screens: the preset list, the preset being shown (the active
 * ship preset unless the user picked another), its rows, and the last control pressed.
 */
import { api, onPress, simulatedPress, type Press, type Row, type State, type View } from "./binds";

class Binds {
  state = $state<State | null>(null);
  view = $state<View | null>(null);
  error = $state<string | null>(null);
  last = $state<Press | null>(null);
  /** Bumped on every press, so screens can react even to the same control twice. */
  presses = $state(0);
  /** A dialog (press-to-bind) owns presses while open. */
  capturing = $state(false);
  log = $state<Press[]>([]);
  private started = false;

  get rows(): Row[] {
    return this.view?.groups.flatMap((g) => g.rows) ?? [];
  }

  get shownPath(): string | null {
    if (!this.state) return null;
    const picked = localStorage.getItem("astromech.preset");
    if (picked && this.state.presets.some((p) => p.path === picked)) return picked;
    const ship = this.state.active[1];
    const p = this.state.presets.find((x) => x.name === ship && x.source === "yours") ?? this.state.presets.find((x) => x.name === ship);
    return p?.path ?? this.state.presets[0]?.path ?? null;
  }

  async load() {
    try {
      this.state = await api.state();
      const p = this.shownPath;
      this.view = p ? await api.view(p) : null;
      this.error = null;
    } catch (e) {
      this.error = String(e);
    }
  }

  async show(path: string) {
    localStorage.setItem("astromech.preset", path);
    await this.load();
  }

  start() {
    if (this.started) return;
    this.started = true;
    void this.load();
    onPress((p) => this.press(p));
    const sim = simulatedPress();
    if (sim) setTimeout(() => this.press(sim), 300);
  }

  press(p: Press) {
    if (p.down === false) return;
    this.last = p;
    this.presses++;
    this.log = [p, ...this.log].slice(0, 40);
  }

  /** Rows bound to this control (any slot, as the main key or a modifier). */
  rowsFor(device: string, key: string): Row[] {
    return this.rows.filter((r) => r.slots.some((s) => s.device === device && s.key === key));
  }
}

export const binds = new Binds();
