<script lang="ts">
  // Step 4: pick the recording device by name (the core records that one, not whatever Windows
  // makes the default, e.g. SteamVR's headset switch) and check its level.
  import { coreCall, type Level, type MicList } from "../core";

  let mics = $state<MicList | null>(null);
  let level = $state<Level | null>(null);
  let checking = $state(false);
  let error = $state<string | null>(null);

  async function load() {
    const r = await coreCall("mic_list");
    if (r.ok) mics = r as unknown as MicList;
    else error = String(r.error);
  }
  $effect(() => {
    void load();
  });

  async function pick(name: string) {
    error = null;
    level = null;
    const r = await coreCall("set_mic", { mic: name });
    if (!r.ok) error = String(r.error);
    await load();
  }

  async function check() {
    checking = true;
    error = null;
    level = null;
    const r = await coreCall("mic_check");
    checking = false;
    if (r.ok) level = r.level as Level;
    else error = String(r.error);
  }

  const VERDICT = {
    ok: "Good level.",
    quiet: "Quiet. Level correction will boost it; moving the mic closer helps more.",
    silent: "Nothing heard. Wrong mic, muted, or nobody spoke?",
  } as const;
</script>

<div class="setup-step">
  {#if mics}
    {#if mics.names.length}
      <dl>
        <dt>Use</dt>
        <dd class:on={!mics.setting}>
          <span>Windows default{mics.default ? ` · ${mics.default}` : ""}</span>
          {#if mics.setting}<button onclick={() => pick("")}>Use</button>{:else}<span class="lo">Chosen</span>{/if}
        </dd>
        {#each mics.names as n (n)}
          <dt></dt>
          <dd class:on={mics.chosen === n}>
            <span>{n}</span>
            {#if mics.chosen === n}<span class="lo">Chosen</span>{:else}<button onclick={() => pick(n)}>Use</button>{/if}
          </dd>
        {/each}
      </dl>
      {#if mics.setting && !mics.chosen}
        <p class="warn-text">“{mics.setting}” isn't connected. Plug it in, or pick another.</p>
      {/if}
      <div class="press" class:got={!!level}>
        {#if checking}
          Speak now…
        {:else if level}
          {level.peak_dbfs.toFixed(0)} dBFS · {VERDICT[level.verdict]}
        {:else}
          Check level, then say something for two seconds.
        {/if}
      </div>
      <div class="acts"><button disabled={checking} onclick={check}>Check level</button></div>
    {:else}
      <p class="warn-text">No microphones found.</p>
      <p class="lo">Plug in your headset or mic and check again.</p>
      <div class="acts"><button onclick={load}>Check again</button></div>
    {/if}
  {/if}
  {#if error}<p class="warn-text">{error}</p>{/if}
</div>
