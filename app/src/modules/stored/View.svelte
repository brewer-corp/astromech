<script lang="ts">
  // Modules in storage, moved off the Ships page so the fleet can keep a bottom rail.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { loadFleet, type Fleet } from "../../lib/engineering";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let fleet = $state<Fleet | null>(null);
  let fleetErr = $state<string | null>(null);

  $effect(() => {
    let key = "";
    const tick = () => {
      void loadFleet()
        .then((f) => {
          fleetErr = null;
          const k = JSON.stringify(f);
          if (k !== key) {
            key = k;
            fleet = f;
          }
        })
        .catch((e: unknown) => (fleetErr = pollFailure("read the stored modules", e, fleet !== null)));
    };
    tick();
    const t = setInterval(tick, 1000);
    return () => clearInterval(t);
  });

  function cr(n: number | null): string {
    if (n === null) return "–";
    if (n >= 1e6) return `${+(n / 1e6).toFixed(n >= 1e8 ? 0 : 1)}M`;
    if (n >= 1e3) return `${Math.round(n / 1e3)}k`;
    return String(n);
  }
</script>

<section>
  <div class="query">
    <p class="sub">
      {#if !fleet}
        {fleetErr ?? "Reading the journal…"}
      {:else}
        {fleet.stored_modules.length} stored {fleet.stored_modules.length === 1 ? "module" : "modules"} · {fleet.source}
      {/if}
    </p>
    <Stale error={fleet ? fleetErr : null} />
  </div>
  <div class="out">
    <ul class="stored">
      {#each fleet?.stored_modules ?? [] as m, i (i)}
        <li>
          <span class="mn">{m.name}</span>
          <span class:lo={!m.blueprint_name}>{m.blueprint_name ? `G${m.grade ?? "?"} ${m.blueprint_name}` : "–"}</span>
          <span class:lo={!m.system}>{#if m.system}<SystemChip name={m.system} />{:else}In transit{/if}</span>
          <span class="r q">{cr(m.value)}</span>
        </li>
      {:else}
        {#if fleet}<li class="none">No modules in storage.</li>{/if}
      {/each}
    </ul>
  </div>
</section>

<style>
  section { padding: 0 0 28px; }
  .query { display: flex; flex-direction: column; gap: 8px; padding: 18px 20px 20px; background: var(--bg-list); border-bottom: 1px solid var(--accent-dim); }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 0; }
  .out { padding: 24px 4px 8px; min-width: 0; }
  .stored { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; }
  .stored li { display: grid; grid-template-columns: minmax(12rem, 1.4fr) minmax(10rem, 1fr) minmax(8rem, 1fr) 6rem; gap: 10px 18px; align-items: baseline; padding: 16px 8px; border-bottom: 1px solid #1a1c1e; font-size: var(--fs); }
  .mn { color: var(--text-hi); font-weight: 500; }
  .lo { color: var(--text-lo); }
  .r { text-align: right; }
  .q { font-family: var(--mono); }
  .none { display: block; color: var(--text-lo); border: 0; }
  @media (max-width: 900px) {
    .stored li { grid-template-columns: minmax(0, 1fr) auto; }
    .stored li > :nth-child(2), .stored li > :nth-child(3) { grid-column: 1 / -1; }
  }
</style>
