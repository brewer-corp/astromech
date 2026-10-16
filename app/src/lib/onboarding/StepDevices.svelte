<script lang="ts">
  // Step 2: the controllers Elite can see (the window's device scan, as in the key tester), and
  // a live "last press" so the user knows presses arrive.
  import { api, type Controller } from "../binds";
  import { binds } from "../store.svelte";
  import { mapFor } from "../devices";
  import { control } from "../format";

  let controllers = $state<Controller[] | null>(null);
  let error = $state<string | null>(null);

  async function scan() {
    try {
      controllers = (await api.refreshDevices()).inventory.controllers;
      error = null;
    } catch (e) {
      error = String(e);
    }
  }
  binds.start();
  $effect(() => {
    void scan();
  });
</script>

<div class="setup-step">
  {#if controllers}
    {#if controllers.length}
      <dl>
        {#each controllers as c (c.elite_id)}
          <dt class="mono">{c.elite_id}</dt>
          <dd>
            <span>{c.product.trim()}</span>
            <span class="lo">{c.buttons} buttons{mapFor(c.elite_id) ? " · stock map" : ""}</span>
          </dd>
        {/each}
      </dl>
      {#if controllers.some((c) => c.buttons > 32)}
        <p class="lo">Elite reads only buttons 1–32 on each device; higher buttons can't be bound in the game.</p>
      {/if}
    {:else}
      <p class="warn-text">No sticks, throttles or pads found.</p>
      <p class="lo">Keyboard keys work for push-to-talk too. If you fly with a stick, plug it in and check again.</p>
    {/if}
  {/if}
  <div class="press" class:got={!!binds.last}>
    {binds.last ? `Last press: ${control(binds.last.device, binds.last.key)}` : "Press any button or key…"}
  </div>
  <div class="acts"><button onclick={scan}>Check again</button></div>
  {#if error}<p class="warn-text">{error}</p>{/if}
</div>
