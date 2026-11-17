<script lang="ts">
  // Voice tester: try a voice command safely. Type a phrase, or use push-to-talk while this page
  // is open, and see what was heard, the intent it matched, how sure the match is and the keys it
  // would send. Tests are dry runs; nothing reaches the game unless you click Run live.
  import Button from "../../lib/ui/Button.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import Board from "../../lib/ui/layout/Board.svelte";
  import { tip } from "../../lib/ui/tip";
  import { keyName } from "../../lib/format";
  import {
    SWITCH_SECONDS,
    forgetPhrase,
    learnedPhrases,
    voiceLive,
    voicePoll,
    voiceStop,
    voiceTest,
    type DryRun,
    type LearnedPhrase,
    type Would,
  } from "../../lib/voice";

  let text = $state("");
  let result = $state<DryRun | null>(null);
  let error = $state("");
  let listening = $state(false);
  let liveNote = $state("");
  let liveErr = $state(false);
  let running = $state(false);

  function show(r: DryRun) {
    result = r;
    error = "";
    liveNote = "";
  }

  // Polling keeps push-to-talk in test mode, but only while the page can be seen: a minimised
  // window, or the page hidden behind the game, hands push-to-talk back to the assistant.
  $effect(() => {
    let seen: number | null = null;
    let t: ReturnType<typeof setInterval> | undefined;
    const tick = async () => {
      const p = await voicePoll();
      listening = p?.listening ?? false;
      const seq = p?.last?.seq ?? 0;
      // The first poll only notes what an earlier visit already showed.
      if (seen !== null && seq !== seen && p?.last) show(p.last.result);
      seen = seq;
    };
    const start = () => {
      if (t) return;
      void tick();
      t = setInterval(() => void tick(), 1000);
    };
    const stop = () => {
      if (t) clearInterval(t);
      t = undefined;
      listening = false;
      void voiceStop();
    };
    const onVisibility = () => (document.visibilityState === "visible" ? start() : stop());
    onVisibility();
    document.addEventListener("visibilitychange", onVisibility);
    return () => {
      document.removeEventListener("visibilitychange", onVisibility);
      stop();
    };
  });

  // Phrases learned from "Did you mean ...?" answered yes. A wrong yes teaches a wrong phrase for
  // good, so each one can be removed here.
  let learned = $state<LearnedPhrase[]>([]);
  let learnedError = $state("");
  $effect(() => {
    learnedPhrases()
      .then((l) => (learned = l))
      .catch((e: unknown) => (learnedError = String(e instanceof Error ? e.message : e)));
  });
  async function forget(p: LearnedPhrase) {
    try {
      learned = await forgetPhrase(p.heard);
      learnedError = "";
    } catch (e) {
      learnedError = String(e instanceof Error ? e.message : e);
    }
  }

  async function test(e: SubmitEvent) {
    e.preventDefault();
    if (!text.trim()) return;
    try {
      show(await voiceTest(text.trim()));
    } catch (err) {
      error = String(err instanceof Error ? err.message : err);
    }
  }

  // The core presses only when Elite is the active window, so a live run gives you a few seconds
  // to switch to the game; the countdown shows how long is left.
  let countdown = $state(0);
  async function runLive() {
    if (!result?.live) return;
    running = true;
    liveNote = "";
    liveErr = false;
    countdown = SWITCH_SECONDS;
    const t = setInterval(() => (countdown = Math.max(0, countdown - 1)), 1000);
    try {
      liveNote = await voiceLive(result.live, result.live_from);
    } catch (err) {
      liveErr = true;
      liveNote = String(err instanceof Error ? err.message : err);
    }
    clearInterval(t);
    countdown = 0;
    running = false;
  }
  /** A spoken command would ask first; a live run doesn't. */
  const skipsAsk = (r: DryRun) => r.would === "confirm" || r.would === "did_you_mean";

  const WOULD: Record<Would, string> = {
    press: "Press the keys, then say",
    confirm: "Ask first",
    already: "Press nothing, and say",
    refused: "Press nothing, and say",
    speak: "Answer",
    no_binding: "Press nothing (no keyboard key), and say",
    unavailable: "Press nothing, and say",
    did_you_mean: "Ask",
    not_understood: "Say",
    ignored: "Ignore it as noise",
  };
  const DECISION: Record<string, string> = { execute: "acts on it", ask: "asks first", no_match: "too low to act" };
  const LEARNED = {
    title: "Learned phrases",
    fact: "Phrases astromech runs without asking, because you once answered yes to \"Did you mean ...?\"",
    note: "Remove one that was taught wrongly",
  };
  const keys = (k: NonNullable<DryRun["keys"]>) => [...k.modifiers, k.key].map(keyName).join(" + ");
</script>

<Board>
  <!-- Whether push-to-talk is being tested: a state, so a line, not a Tip. -->
  {#if listening}
    <p class="type-body state">Push-to-talk is in test mode while this page is open. Nothing is sent to the game.</p>
  {:else}
    <p class="type-label state">Push-to-talk testing needs Elite running.</p>
  {/if}
  <form class="ask" onsubmit={test}>
    <Input class="page-search" bind:value={text} placeholder="Type a phrase, e.g. gear down" aria-label="Phrase">
      {#snippet end()}<Button type="submit" disabled={!text.trim()}>Test</Button>{/snippet}
    </Input>
  </form>
  {#if error}<p class="type-label note err" role="alert">{error}</p>{/if}

  {#if result}
    <dl>
      <dt class="type-label">Heard</dt>
      <dd>{result.transcript || "(nothing)"}<span class="type-label">&nbsp;· {result.source === "voice" ? "push-to-talk" : "typed"}</span></dd>
      <dt class="type-label">Intent</dt>
      <dd>
        {#if result.intent && result.intent.decision !== "no_match"}
          {result.intent.phrase}<span class="type-label">&nbsp;· {result.intent.command}</span>
        {:else}
          No match
        {/if}
      </dd>
      <dt class="type-label">Confidence</dt>
      <dd>{#if result.intent}{Math.round(result.intent.score * 100)}%<span class="type-label">&nbsp;· {DECISION[result.intent.decision] ?? result.intent.decision}</span>{:else}—{/if}</dd>
      <dt class="type-label">Would</dt>
      <dd>{WOULD[result.would]}{#if result.message}&nbsp;“{result.message}”{/if}</dd>
      <dt class="type-label">Keys</dt>
      <dd>
        {#if result.keys}
          <span class="key">{keys(result.keys)}</span>
          {#if result.would !== "press"}<span class="type-label">&nbsp;· not pressed now</span>{/if}
        {:else if result.action}
          <span class="type-label">No keyboard key for {result.action}</span>
        {:else}
          <span class="type-label">None</span>
        {/if}
      </dd>
    </dl>
    <p class="type-label safe">Nothing was sent to the game.</p>
    {#if result.live}
      <div class="actions">
        <Button
          onclick={runLive}
          disabled={!listening || running}
          {@attach tip(
            skipsAsk(result)
              ? `Runs ${result.intent?.phrase ?? "it"} without the question a spoken command would ask. Switch to Elite within ${SWITCH_SECONDS} s after clicking`
              : `Sends it to Elite like a spoken command. Switch to Elite within ${SWITCH_SECONDS} s after clicking`,
          )}>{skipsAsk(result) ? "Run live (no confirmation)" : "Run live"}</Button
        >
        {#if running}
          <span class="type-label">Switch to Elite now: pressing when it's in front ({countdown} s left).</span>
        {:else if !listening}
          <span class="type-label">Live runs need Elite running.</span>
        {/if}
      </div>
    {/if}
    {#if liveNote}<p class="type-label note" class:err={liveErr} role={liveErr ? "alert" : undefined}>{liveNote}</p>{/if}
  {/if}

  <h2 class="section" {@attach tip(LEARNED)}>Learned phrases</h2>
  {#if learned.length === 0}
    <p class="type-body">None yet.</p>
  {:else}
    <ul class="learned">
      {#each learned as p (p.heard)}
        <li>
          <span class="type-name">{p.heard}</span>
          <span class="type-label">runs {p.does}</span>
          <Button onclick={() => forget(p)} aria-label={`Remove ${p.heard}`}>Remove</Button>
        </li>
      {/each}
    </ul>
  {/if}
  {#if learnedError}<p class="type-label note err" role="alert">{learnedError}</p>{/if}
</Board>

<style>
  p { margin: 0; }
  .state { margin-bottom: var(--sp-4); }
  .ask { display: flex; margin: 0 0 var(--sp-6); }
  dl { display: grid; grid-template-columns: max-content minmax(0, 1fr); column-gap: var(--sp-12); row-gap: var(--gap); margin: 0 0 var(--sp-4); }
  dt, dd { margin: 0; min-height: var(--row); display: flex; align-items: center; }
  dt { padding-left: var(--sp-3); }
  /* What the tester worked out is the page's answer: read-only facts outside a cell (DESIGN.md Fact
     list), brighter than their labels; the keys outlined like a badge. */
  dd { color: var(--options-grey); }
  .key { outline: 1px solid var(--line); outline-offset: -1px; color: var(--options-grey); padding: var(--gap) var(--sp-2); }
  .safe { margin-bottom: var(--sp-4); }
  .actions { display: flex; gap: var(--sp-6); align-items: center; }
  h2 { margin: var(--sp-8) 0 var(--sp-3); }
  /* A learned phrase, what it runs, then Remove right after it (DESIGN.md The Next To It Rule). */
  .learned { list-style: none; margin: var(--sp-3) 0 0; padding: 0; display: grid; grid-template-columns: repeat(3, max-content); column-gap: var(--sp-6); row-gap: var(--gap); }
  .learned li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; outline: 1px solid var(--line); outline-offset: -1px; padding: 0 var(--sp-3); min-height: var(--row); }
  .note { margin-top: var(--sp-3); }
  .note.err { color: var(--alarm); }
  .note.err::before { content: "! "; font-weight: 600; }
</style>
