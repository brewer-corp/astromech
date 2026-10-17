<script lang="ts">
  // Voice tester: try a voice command safely. Type a phrase, or use push-to-talk while this page
  // is open, and see what was heard, the intent it matched, how sure the match is and the keys it
  // would send. Tests are dry runs; nothing reaches the game unless you click Run live.
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
    countdown = SWITCH_SECONDS;
    const t = setInterval(() => (countdown = Math.max(0, countdown - 1)), 1000);
    try {
      liveNote = await voiceLive(result.live);
    } catch (err) {
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
  const DECISION = { execute: "acts on it", ask: "asks first", no_match: "too low to act" } as const;
  const keys = (k: NonNullable<DryRun["keys"]>) => [...k.modifiers, k.key].map(keyName).join(" + ");
</script>

<section>
  <p class="lo">
    {#if listening}
      Push-to-talk is in test mode while this page is open: speak a command and it shows here. Nothing is sent to the game.
    {:else}
      Type a phrase to see what it would do. Push-to-talk can be tested here while Elite is running.
    {/if}
  </p>
  <form class="ask" onsubmit={test}>
    <input bind:value={text} placeholder="Type a phrase, e.g. gear down" aria-label="Phrase" />
    <button type="submit" disabled={!text.trim()}>Test</button>
  </form>
  {#if error}<p class="note">{error}</p>{/if}

  {#if result}
    <dl>
      <dt>Heard</dt>
      <dd>{result.transcript || "(nothing)"}<span class="lo">&nbsp;· {result.source === "voice" ? "push-to-talk" : "typed"}</span></dd>
      <dt>Intent</dt>
      <dd>
        {#if result.intent && result.intent.decision !== "no_match"}
          {result.intent.phrase}<span class="mono lo">&nbsp;· {result.intent.command}</span>
        {:else}
          No match
        {/if}
      </dd>
      <dt>Confidence</dt>
      <dd>{#if result.intent}{Math.round(result.intent.score * 100)}%<span class="lo">&nbsp;· {DECISION[result.intent.decision]}</span>{:else}—{/if}</dd>
      <dt>Would</dt>
      <dd>{WOULD[result.would]}{#if result.message}&nbsp;“{result.message}”{/if}</dd>
      <dt>Keys</dt>
      <dd>
        {#if result.keys}
          <span class="key">{keys(result.keys)}</span>
          {#if result.would !== "press"}<span class="lo">&nbsp;· not pressed now</span>{/if}
        {:else if result.action}
          <span class="lo">No keyboard key for {result.action}</span>
        {:else}
          <span class="lo">None</span>
        {/if}
      </dd>
    </dl>
    <p class="safe">Nothing was sent to the game.</p>
    {#if result.live}
      <div class="actions">
        <button onclick={runLive} disabled={!listening || running}>{skipsAsk(result) ? "Run live (no confirmation)" : "Run live"}</button>
        <span class="lo">
          {#if running}
            Switch to Elite now: pressing when it's in front ({countdown} s left).
          {:else if !listening}
            Live runs need Elite running.
          {:else if skipsAsk(result)}
            Runs {result.intent?.phrase ?? "it"} without the question a spoken command would ask. Switch to Elite within {SWITCH_SECONDS} s after clicking.
          {:else}
            Sends it to Elite like a spoken command. Switch to Elite within {SWITCH_SECONDS} s after clicking.
          {/if}
        </span>
      </div>
    {/if}
    {#if liveNote}<p class="note">{liveNote}</p>{/if}
  {/if}

  <h2>Learned phrases</h2>
  {#if learned.length === 0}
    <p class="lo">None yet. When you answer yes to "Did you mean ...?", astromech remembers what it heard and runs that command next time.</p>
  {:else}
    <p class="lo">Phrases astromech runs without asking, because you once answered yes. Remove one that was taught wrongly.</p>
    <ul class="learned">
      {#each learned as p (p.heard)}
        <li>
          <span class="said">{p.heard}</span>
          <span class="lo">runs {p.does}</span>
          <button onclick={() => forget(p)} aria-label={`Remove ${p.heard}`}>Remove</button>
        </li>
      {/each}
    </ul>
  {/if}
  {#if learnedError}<p class="note">{learnedError}</p>{/if}
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; }
  p { margin: 0; }
  .lo { color: var(--text-lo); }
  .ask { display: flex; gap: 28px; align-items: center; margin: 20px 0 24px; background: var(--cell-opt); padding: 0 12px; min-height: var(--row); }
  .ask input { all: unset; flex: 1; color: var(--text-hi); font-size: var(--fs-l); }
  .ask input::placeholder { color: var(--text-lo); }
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0 0 16px; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--text-lo); }
  dd { background: var(--cell-opt); color: var(--text); }
  .key { background: var(--cell); color: var(--accent); padding: 2px 8px; font-family: var(--mono); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .safe { color: var(--text-lo); margin-bottom: 16px; }
  .actions { display: flex; gap: 28px; align-items: center; }
  button { all: unset; cursor: pointer; color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-weight: 600; font-size: var(--fs-s); }
  button:disabled { color: var(--text-lo); cursor: default; }
  button:not(:disabled):hover { color: var(--text-hi); }
  h2 { margin: 32px 0 12px; font-size: var(--fs-l); color: var(--text-hi); font-weight: 600; }
  .learned { list-style: none; margin: 12px 0 0; padding: 0; display: grid; gap: var(--gap); }
  .learned li { display: flex; gap: 24px; align-items: center; background: var(--cell-opt); padding: 0 12px; min-height: var(--row); }
  .said { color: var(--text); flex: 1; }
  .note { color: var(--text-lo); font-family: var(--mono); font-size: var(--fs-s); margin-top: 12px; }
</style>
