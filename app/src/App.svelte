<script lang="ts">
  import { modules, findModule } from "./lib/modules";

  let route = $state(location.hash.replace(/^#\//, "") || (modules[0]?.id ?? ""));
  const current = $derived(findModule(route));

  function go(id: string) {
    route = id;
    location.hash = `#/${id}`;
  }
</script>

<div class="shell">
  <nav>
    <h1>astromech</h1>
    {#each modules as m (m.id)}
      <button class:active={m.id === route} title={m.summary} onclick={() => go(m.id)}>{m.title}</button>
    {/each}
    <p class="status">pre-alpha · core not connected</p>
  </nav>
  <main>
    {#if current}
      {#await current.load() then view}
        <view.default />
      {/await}
    {:else}
      <p>No module selected.</p>
    {/if}
  </main>
</div>

<style>
  .shell { display: grid; grid-template-columns: 14rem 1fr; min-height: 100vh; }
  nav { background: var(--panel); padding: 1rem; display: flex; flex-direction: column; gap: 0.25rem; }
  h1 { color: var(--accent); font-size: 1.1rem; letter-spacing: 0.15em; text-transform: uppercase; }
  button { all: unset; cursor: pointer; padding: 0.4rem 0.6rem; border-radius: 4px; }
  button.active, button:hover { background: #1f252c; color: var(--accent); }
  .status { margin-top: auto; color: var(--muted); font-size: 0.8rem; }
  main { padding: 1.5rem; }
</style>
