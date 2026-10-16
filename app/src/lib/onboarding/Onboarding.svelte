<script lang="ts">
  // Onboarding (docs/V1-DESIGN.md §1.5 #1, §2.2): a full-window flow from install to "gear down"
  // by voice. Shown on first launch until finished or skipped; Settings re-runs it, or one step
  // of it (#/setup/<step>). One line of explanation per step, nothing that isn't needed.
  import { coreCall, coreStatus, type CoreStatus } from "../core";
  import { STEPS, type StepId } from "./steps";
  import StepElite from "./StepElite.svelte";
  import StepDevices from "./StepDevices.svelte";
  import StepButtons from "./StepButtons.svelte";
  import StepMic from "./StepMic.svelte";
  import StepCalibrate from "./StepCalibrate.svelte";
  import StepHealth from "./StepHealth.svelte";
  import "./onboarding.css";

  let { start = "elite", onfinish }: { start?: string; onfinish: () => void } = $props();

  let i = $state(Math.max(0, STEPS.findIndex((s) => s.id === start)));
  let status = $state<CoreStatus | null>(null);
  const step = $derived(STEPS[i] ?? STEPS[0]);

  async function refresh() {
    status = await coreStatus();
  }
  $effect(() => {
    void refresh();
    const t = setInterval(() => void refresh(), 1000);
    return () => clearInterval(t);
  });

  function go(id: StepId) {
    i = Math.max(0, STEPS.findIndex((s) => s.id === id));
  }

  async function finish() {
    await coreCall("onboarding_done");
    onfinish();
  }
</script>

<div class="frame">
  <header>
    <span class="app">ASTROMECH</span>
    <span class="title">Setup</span>
    {#if i < STEPS.length - 1}<button class="skip" onclick={finish}>Skip setup</button>{/if}
  </header>
  <div class="body">
    <ol class="nav">
      {#each STEPS as s, n (s.id)}
        <li><button class:on={n === i} onclick={() => (i = n)}><span class="n">{n + 1}</span>{s.title}</button></li>
      {/each}
    </ol>
    <main>
      <h1>{step.title}</h1>
      <p class="lead">{step.lead}</p>
      {#key step.id}
        {#if step.id === "elite"}
          <StepElite />
        {:else if step.id === "devices"}
          <StepDevices />
        {:else if step.id === "buttons"}
          <StepButtons {status} {refresh} />
        {:else if step.id === "mic"}
          <StepMic />
        {:else if step.id === "calibrate"}
          <StepCalibrate />
        {:else}
          <StepHealth {status} {go} />
        {/if}
      {/key}
      <footer>
        {#if i > 0}<button onclick={() => i--}>Back</button>{/if}
        {#if i < STEPS.length - 1}
          <button class="primary" onclick={() => i++}>Next</button>
        {:else}
          <button class="primary" onclick={finish}>Finish</button>
        {/if}
      </footer>
    </main>
  </div>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto 1fr; }
  header { display: flex; align-items: baseline; gap: 40px; margin: 0 40px; padding: 26px 0 12px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  .title { flex: 1; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  .body { display: grid; grid-template-columns: 15rem 1fr; gap: 40px; padding: 28px 40px; min-height: 0; overflow: auto; }
  .nav { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--gap); align-content: start; }
  .nav button { width: 100%; box-sizing: border-box; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; gap: 12px; color: var(--text-lo); text-transform: none; letter-spacing: 0; font-weight: 500; font-size: var(--fs); }
  .nav button.on { background: var(--cell); color: var(--text-hi); }
  .nav .n { font-family: var(--mono); color: var(--accent); }
  main { max-width: 52rem; }
  h1 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-xl); }
  .lead { margin: 0 0 22px; color: var(--text-lo); }
  footer { display: flex; gap: 28px; margin-top: 32px; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:hover, button:focus-visible { color: var(--text-hi); }
  .primary { background: var(--accent); color: var(--on-accent); padding: 6px 18px; }
  .primary:hover, .primary:focus-visible { color: var(--on-accent); background: var(--text-hi); }
  .skip { color: var(--text-lo); }
</style>
