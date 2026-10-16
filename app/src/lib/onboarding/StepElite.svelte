<script lang="ts">
  // Step 1: where Elite keeps its keybindings and journals (core setup.rs `find_elite`). Both
  // appear the first time the game runs on this Windows account; either can be pointed elsewhere.
  import { coreCall, type Found } from "../core";
  import { api } from "../binds";
  import { binds } from "../store.svelte";

  let found = $state<Found | null>(null);
  let dir = $state("");
  let error = $state<string | null>(null);

  async function check() {
    const r = await coreCall("find_elite");
    if (r.ok) found = r.found as Found;
    else error = String(r.error);
  }
  $effect(() => {
    void check();
  });

  async function save() {
    error = null;
    const r = await coreCall("set_folders", { bindings_dir: dir.trim() });
    if (!r.ok) {
      error = String(r.error);
      return;
    }
    // The window reads presets itself: make it look in the new folder.
    await api.reset().catch(() => undefined);
    await binds.load();
    await check();
  }
</script>

<div class="setup-step">
  {#if found}
    <dl>
      <dt>Keybindings</dt>
      <dd class:warn={!found.bindings_found}><span class="mono">{found.bindings_dir ?? "—"}</span><span>{found.bindings_found ? "Found" : "Not found"}</span></dd>
      <dt>Journals</dt>
      <dd class:warn={!found.journal_found}><span class="mono">{found.journal_dir ?? "—"}</span><span>{found.journal_found ? "Found" : "Not found"}</span></dd>
      <dt>Game install</dt>
      <dd><span class="mono">{found.install ?? "Not found"}</span><span>{found.install_source ?? "Optional"}</span></dd>
    </dl>
    {#if found.ok}
      <p>Found Elite. {found.install ? "" : "Without the install folder, Elite's built-in presets aren't listed; your own presets work."}</p>
    {:else}
      <p class="warn-text">Elite hasn't run on this Windows account yet, or its folders are somewhere else.</p>
      <p class="lo">Start Elite once and come back, or paste your Bindings folder (Options\Bindings, under Frontier Developments).</p>
      <div class="acts">
        <input bind:value={dir} placeholder="Bindings folder" aria-label="Bindings folder" />
        <button disabled={!dir.trim()} onclick={save}>Use this folder</button>
      </div>
    {/if}
    <div class="acts"><button onclick={check}>Check again</button></div>
  {/if}
  {#if error}<p class="warn-text">{error}</p>{/if}
</div>
