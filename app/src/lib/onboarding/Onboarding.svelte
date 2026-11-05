<script lang="ts">
  // Onboarding (docs/V1-DESIGN.md §1.5 #1, §2.2): a full-window flow from install to "gear down"
  // by voice. Shown on first launch until finished or skipped; Settings re-runs it, or one step
  // of it (#/setup/<step>). One line of explanation per step, nothing that isn't needed.
  import Button from "../ui/Button.svelte";
  import { coreCall, coreStatus, type CoreStatus } from "../core";
  import { STEPS, type StepId } from "./steps";
  import StepElite from "./StepElite.svelte";
  import StepDevices from "./StepDevices.svelte";
  import StepButtons from "./StepButtons.svelte";
  import StepMic from "./StepMic.svelte";
  import StepCalibrate from "./StepCalibrate.svelte";
  import StepHealth from "./StepHealth.svelte";
  import { openHelp, SETUP_HELP } from "../help";
  import "./onboarding.css";
  import Stale from "../ui/Stale.svelte";
  import { pollFailure } from "../ui/errors";

  let { start = "elite", onfinish }: { start?: string; onfinish: () => void } = $props();

  let i = $state(Math.max(0, STEPS.findIndex((s) => s.id === start)));
  let status = $state<CoreStatus | null>(null);
  const step = $derived(STEPS[i] ?? STEPS[0]);

  /** A status read failed after a good one: the last status stays, with a line saying so. A failed
   * read is null (coreStatus), so the window can't tell a stopped core from one that didn't answer. */
  let statusErr = $state<string | null>(null);
  /** No good status read yet: still waiting for the first, or the reads so far failed. */
  let unread = $state<"checking" | "failed">("checking");
  async function refresh() {
    const s = await coreStatus();
    if (s) {
      status = s;
      statusErr = null;
    } else if (status) statusErr = pollFailure("read the status", "the core did not answer", true);
    else unread = "failed";
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
    <Button title="Open the setup guide in your browser" onclick={() => void openHelp(SETUP_HELP)}>Help</Button>
    {#if i < STEPS.length - 1}<Button onclick={finish}>Skip setup</Button>{/if}
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
      <Stale error={statusErr} />
      {#key step.id}
        {#if step.id === "elite"}
          <StepElite />
        {:else if step.id === "devices"}
          <StepDevices />
        {:else if step.id === "buttons"}
          <StepButtons {status} {unread} {refresh} />
        {:else if step.id === "mic"}
          <StepMic />
        {:else if step.id === "calibrate"}
          <StepCalibrate />
        {:else}
          <StepHealth {status} {go} />
        {/if}
      {/key}
      <footer>
        {#if i > 0}<Button onclick={() => i--}>Back</Button>{/if}
        {#if i < STEPS.length - 1}
          <Button kind="primary" onclick={() => i++}>Next</Button>
        {:else}
          <Button kind="primary" onclick={finish}>Finish</Button>
        {/if}
      </footer>
    </main>
  </div>
</div>

<style>
  .frame { height: 100vh; display: grid; grid-template-rows: auto 1fr; }
  header { display: flex; align-items: center; gap: 40px; height: 64px; margin: 0 40px; border-bottom: 1px solid var(--accent); }
  .app { font-weight: 600; font-size: var(--fs-l); letter-spacing: 0.12em; color: var(--text-hi); }
  /* The area's one top tab, active: Nav caps on a solid orange box. */
  .title { display: flex; align-items: center; height: var(--row); padding: 0 var(--sp-3); margin-right: auto; background: var(--accent); color: var(--on-accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 500; }
  .body { display: grid; grid-template-columns: 15rem 1fr; gap: 40px; padding: 28px 40px; min-height: 0; overflow: auto; }
  .nav { list-style: none; margin: 0; padding: 0; display: grid; gap: var(--gap); align-content: start; }
  .nav button { all: unset; cursor: pointer; width: 100%; box-sizing: border-box; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; gap: 12px; color: var(--accent); font-weight: 500; font-size: var(--fs); }
  /* The step you are on: hull and Orange Lit, a place marker rather than a value (no cell). */
  .nav button.on { background: var(--surface); color: var(--orange-lit); }
  .nav .n { font-family: var(--mono); color: var(--orange-mid); }
  .nav button.on .n { color: inherit; }
  main { max-width: 52rem; }
  h1 { margin: 0 0 6px; color: var(--text-hi); font-weight: 500; font-size: var(--fs-xl); }
  .lead { margin: 0 0 22px; color: var(--prose); }
  footer { display: flex; gap: 28px; margin-top: 32px; }
</style>
