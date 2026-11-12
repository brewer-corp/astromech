<script lang="ts">
  // The plot setup as two dropdowns, on the Route page's title and in the header's plot panel,
  // over the one shared setup (lib/plot.svelte.ts), so both always agree. Plotter: Neutron or
  // Exact, with Exact's refuelling under it. Worked out: On this PC or By Spansh for a neutron
  // route, with the neutron star file's status, download and check under the options, and
  // Spansh's efficiency when Spansh will plot; an exact route is always Spansh's, so it is a fact.
  import { setup, type Engine } from "../../lib/plot.svelte";
  import type { Plotter } from "../../lib/route";
  import Input from "../../lib/ui/Input.svelte";
  import Picker from "../../lib/ui/Picker.svelte";
  import NeutronFile from "./NeutronFile.svelte";

  const plotters = [
    { value: "neutron" as Plotter, label: "Neutron", hint: "Stops at neutron stars to boost from" },
    { value: "exact" as Plotter, label: "Exact", hint: "Every jump, with refuel stops" },
  ];
  const engines = $derived([
    { value: "local" as Engine, label: "On this PC", hint: setup.hasFile ? "From the neutron star file, in seconds" : "Needs the neutron star file", disabled: !setup.hasFile },
    { value: "spansh" as Engine, label: "By Spansh", hint: "Online; can take a minute" },
  ]);
</script>

<span class="setup">
  <Picker label="Plotter" value={setup.plotter} options={plotters} onchange={(v) => (setup.plotter = v)}>
    {#snippet extra()}
      {#if setup.plotter === "exact"}
        <label class="check"><input type="checkbox" bind:checked={setup.refuel} /> Refuel at every scoopable star</label>
      {:else}
        <p class="type-label">Plot each leg in the galaxy map.</p>
      {/if}
    {/snippet}
  </Picker>
  {#if setup.plotter === "neutron"}
    <Picker label="Worked out" value={setup.worker} options={engines} onchange={(v) => (setup.engine = v)}>
      {#snippet extra()}
        <NeutronFile />
        {#if setup.worker === "spansh"}
          <label class="eff"><span class="type-label">Efficiency</span><Input aria-label="Efficiency" type="number" min="1" max="100" bind:value={setup.efficiency}>{#snippet end()}%{/snippet}</Input></label>
        {/if}
      {/snippet}
    </Picker>
  {:else}
    <span class="fact"><span class="type-label">Worked out</span> By Spansh</span>
  {/if}
</span>

<style>
  .setup { display: inline-flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2) var(--sp-1); }
  .fact { display: inline-flex; align-items: center; gap: var(--sp-3); height: var(--row); padding: 0 var(--sp-3); white-space: nowrap; }
  p { margin: 0; }
  .check { display: flex; align-items: center; gap: var(--sp-2); min-height: var(--row); color: var(--accent); }
  .eff { display: flex; align-items: center; gap: var(--sp-3); margin-top: var(--sp-2); }
</style>
