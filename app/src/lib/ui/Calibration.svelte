<script lang="ts">
  // Mic calibration (docs/V1-DESIGN.md §6.2 step 3): a moment of silence, then three phrases.
  // The core records each clip from the chosen mic and stores the per-device gate, the same
  // one `astromech-core --calibrate-files` writes. Used by onboarding and Settings → Microphone.
  import { coreCall, type Level } from "../core";

  interface Next { step: number; steps: number; phrase: string | null; ms: number }
  interface Result { device: string; noise_dbfs: number; speech_peak_dbfs: number; min_level_dbfs: number; gain_db: number }

  let { ondone }: { ondone?: (r: Result) => void } = $props();

  let next = $state<Next | null>(null);
  let levels = $state<Level[]>([]);
  let busy = $state(false);
  let error = $state<string | null>(null);
  let result = $state<Result | null>(null);

  async function start() {
    error = null;
    result = null;
    levels = [];
    const r = await coreCall("calibrate_start");
    if (r.ok) next = r.next as Next;
    else error = String(r.error);
  }

  async function record() {
    busy = true;
    error = null;
    const r = await coreCall("calibrate_record");
    busy = false;
    if (!r.ok) {
      error = String(r.error);
      return;
    }
    levels = [...levels, r.level as Level];
    next = r.next as Next;
    if (next.phrase === null) await finish();
  }

  async function finish() {
    busy = true;
    const r = await coreCall("calibrate_finish");
    busy = false;
    if (!r.ok) {
      error = String(r.error);
      return;
    }
    result = r as unknown as Result;
    next = null;
    ondone?.(result);
  }

  $effect(() => {
    void start();
    return () => {
      if (next) void coreCall("calibrate_cancel");
    };
  });

  const db = (x: number) => `${x > 0 ? "+" : ""}${x.toFixed(0)} dB`;
  const dbfs = (x: number) => `${x.toFixed(0)} dBFS`;
</script>

<div class="cal">
  {#if result}
    <p class="done">Calibrated <span class="mono">{result.device}</span>.</p>
    <dl>
      <dt>Background noise</dt><dd>{dbfs(result.noise_dbfs)}</dd>
      <dt>Your voice (loudest)</dt><dd>{dbfs(result.speech_peak_dbfs)}</dd>
      <dt>Ignore clips quieter than</dt><dd>{dbfs(result.min_level_dbfs)}</dd>
      <dt>Level correction</dt><dd>{result.gain_db > 0 ? `up to ${db(result.gain_db)} per clip` : "not needed"}</dd>
    </dl>
    <div class="acts"><button onclick={start}>Run again</button></div>
  {:else if next}
    <ol class="steps">
      {#each ["Silence", "gear down", "deploy hardpoints", "request docking"] as label, i (label)}
        <li class:now={i === next.step} class:past={i < next.step}>
          <span>{i === 0 ? label : `“${label}”`}</span>
          {#if levels[i]}<span class="mono lo">{dbfs(levels[i].peak_dbfs)}</span>{/if}
        </li>
      {/each}
    </ol>
    <div class="prompt" class:busy>
      {#if busy}
        Recording…
      {:else if next.step === 0}
        Stay quiet for {next.ms / 1000} seconds: this measures the room.
      {:else}
        Say “{next.phrase}” as you would in the cockpit.
      {/if}
    </div>
    <div class="acts">
      <button disabled={busy} onclick={record}>{next.step === 0 ? "Record silence" : "Record"}</button>
    </div>
  {/if}
  {#if error}
    <p class="warn">{error}</p>
    <div class="acts"><button onclick={start}>Start over</button></div>
  {/if}
</div>

<style>
  .cal { display: grid; gap: 12px; }
  p { margin: 0; line-height: 1.45; }
  .steps { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--gap); }
  .steps li { display: flex; justify-content: space-between; min-height: 30px; align-items: center; padding: 0 12px; background: var(--cell-empty); color: var(--text-lo); }
  .steps li.past { background: var(--cell-opt); color: var(--text); }
  .steps li.now { background: var(--cell); color: var(--text-hi); }
  .prompt { min-height: 48px; display: flex; align-items: center; padding: 0 14px; background: var(--cell-opt); color: var(--text-hi); font-size: var(--fs-l); }
  .prompt.busy { background: var(--cell); }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: 30px; display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .lo { color: var(--text-lo); }
  .warn { color: var(--accent); }
  .acts { display: flex; gap: 28px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover, button:focus-visible { color: var(--text-hi); }
</style>
