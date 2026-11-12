<script lang="ts">
  // DESIGN.md Status bar: the foot of every page. On the left the current ship (its name, type and
  // unladen maximum jump range, from engineering_fleet) and, while Elite runs, its fuel and cargo
  // (ship_status); on the right the core's one-line state. App.svelte asks for `status` and `ship`
  // on its 2 s tick; the fleet is read again when the game's view of the ship changes (another
  // ship, or the same one refitted or renamed), or after a failed read.
  import { untrack } from "svelte";
  import type { Fleet } from "../api/types/Fleet";
  import { headerState, type CoreStatus, type ShipStatus } from "../core";
  import { loadFleet } from "../engineering";

  let { status, ship }: { status: CoreStatus | null; ship: ShipStatus | null } = $props();

  let fleet = $state<Fleet | null>(null);
  let reading = false;
  /** The ship as the game last reported it when the fleet was read (`undefined`: never read). */
  let readFor: string | undefined;
  const current = $derived(fleet?.ships.find((s) => s.current) ?? null);

  async function read(key: string) {
    reading = true;
    readFor = key;
    try {
      fleet = await loadFleet();
    } catch {
      fleet = null;
    } finally {
      reading = false;
    }
  }

  // Runs once per tick (a new `status` and `ship` each time the core answers), never on the
  // fleet's own change: one read per change the game reports, and a failed read is retried on the
  // next tick, not in a loop.
  $effect(() => {
    void status;
    const key = JSON.stringify([ship?.ship_id ?? null, ship?.ship_name ?? null, ship?.max_jump_range ?? null]);
    untrack(() => {
      if (reading) return;
      // Outside a session the game reports no ship: keep the fleet read while it was in one.
      if (!fleet || (ship?.ship_id != null && key !== readFor)) void read(key);
    });
  });

  const tonnes = (t: number) => String(Math.round(t));
  const fuel = $derived(ship?.fuel != null ? ship.fuel.toFixed(1) : null);
  const cargo = $derived(ship?.cargo != null ? tonnes(ship.cargo) : null);
</script>

<footer class="bar">
  {#if current}
    <span class="ship">{current.name ?? current.type_name}</span>
    {#if current.name}<span class="type">{current.type_name}</span>{/if}
    {#if current.max_jump_range}
      <span class="stat"><span class="label">Jump</span><span>{current.max_jump_range.toFixed(1)}<span class="unit">ly</span></span></span>
    {/if}
  {/if}
  {#if fuel !== null}
    <span class="stat"><span class="label">Fuel</span><span>{fuel}{#if ship?.fuel_capacity != null}<span class="sep">|</span>{tonnes(ship.fuel_capacity)}{/if}<span class="unit">t</span></span></span>
  {/if}
  {#if cargo !== null}
    <span class="stat"><span class="label">Cargo</span><span>{cargo}{#if ship?.cargo_capacity != null}<span class="sep">|</span>{ship.cargo_capacity}{/if}<span class="unit">t</span></span></span>
  {/if}
  <span class="state" class:warn={!status || status.game === "not_running"}>{headerState(status)}</span>
</footer>

<style>
  /* DESIGN.md Status bar: a 32 px band with a 1 px Seam rule over it, one line. */
  .bar { box-sizing: border-box; display: flex; align-items: center; gap: var(--sp-6); height: var(--status-bar); padding: 0 clamp(16px, 3vw, 40px); border-top: 1px solid var(--line); white-space: nowrap; overflow: hidden; font-size: var(--fs-s); }
  .ship { color: var(--accent); font-weight: 500; overflow: hidden; text-overflow: ellipsis; min-width: 3em; margin-right: calc(var(--sp-3) - var(--sp-6)); }
  .type, .label, .unit, .sep { color: var(--orange-mid); }
  .stat { display: flex; align-items: baseline; gap: var(--sp-2); flex: none; color: var(--accent); }
  .sep { margin: 0 var(--sp-1); }
  .unit { margin-left: var(--sp-1); }
  .state { margin-left: auto; padding-left: var(--sp-6); color: var(--orange-mid); flex: none; }
  .state.warn { color: var(--accent); }
</style>
