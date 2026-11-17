<script lang="ts">
  // Powerplay on one screen (DESIGN.md Board): the power and how long pledged, then three
  // answers across the top (how far to the next rank, the main value; how long the cycle has
  // left; this cycle's merits) and three blocks under them (this system, what earns merits here,
  // where to work nearby). From the journal, and Spansh for Where to work.
  import { tip } from "../../lib/ui/tip";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import Meter from "../../lib/ui/Meter.svelte";
  import Board from "../../lib/ui/layout/Board.svelte";
  import { endsAt, loadOverview, num, pct, pledged, timeLeft, WORK_LABEL, type Overview } from "../../lib/powerplay";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import Suggestions from "./Suggestions.svelte";
  import Targets from "./Targets.svelte";

  let ov = $state<Overview | null>(null);
  let err = $state<string | null>(null);
  /** The clock the time left is told from: the moment of the last read. */
  let now = $state(Date.now());

  $effect(() => {
    const load = () =>
      loadOverview()
        .then((v) => {
          ov = v;
          err = null;
          now = Date.now();
        })
        .catch((e: unknown) => (err = pollFailure("read your Powerplay standing", e, ov !== null)));
    void load();
    const t = setInterval(() => void load(), 2000);
    return () => clearInterval(t);
  });

  const st = $derived(ov?.standing);
  const sys = $derived(ov?.system);
  const next = $derived(st?.next);
  /** Where the lifetime merits are between the current rank's start and the next rank's need. */
  const climb = $derived(next && next.merits > next.from ? ((st?.merits ?? 0) - next.from) / (next.merits - next.from) : 0);
  /** This cycle's reinforcement against undermining, as reinforcement's share of both. */
  const tug = $derived.by(() => {
    const r = sys?.reinforcement ?? 0;
    const u = sys?.undermining ?? 0;
    return { share: r + u ? r / (r + u) : 0.5, lead: Math.abs(r - u), ahead: r > u ? "Reinforcement" : r < u ? "Undermining" : null };
  });
  /** A block's line before the journal answers, or after it failed with nothing to show. */
  const waiting = $derived(err ? "Not shown until the journal can be read." : "Reading the journal…");
</script>

<Board>
  <header>
    {#if err && !ov}
      <p class="err" role="alert">{err}</p>
    {:else if !st}
      <p class="type-label">Reading the journal…</p>
    {:else if !st.power}
      <h2 class="type-title">Not pledged</h2>
      <p class="type-label">Pledge at a Power Contact or from the galaxy map’s Powerplay tab.</p>
    {:else}
      <h2 class="type-title">{st.power}</h2>
      <p class="type-label">Pledged {pledged(st.time_pledged_secs)} · rank {st.rank ?? "unknown"}</p>
    {/if}
  </header>
  <Stale error={ov ? err : null} />

  <div class="blocks" class:pledged={!!st?.power}>
    {#if !st || st.power}
      <section class="rank" aria-labelledby="pp-rank">
        <h3 class="section" id="pp-rank">Next rank</h3>
        {#if !st}
          <p class="type-label">{waiting}</p>
        {:else if !next}
          <p class="type-label">Rank unknown</p>
        {:else if next.note}
          <p class="type-name">{next.note}</p>
          <p class="type-label">Rank {next.rank} · {next.reward}</p>
        {:else}
          <p><span class="type-value-xl main">{num(next.to_go)}</span><span class="type-unit">merits to rank {next.rank}</span></p>
          <Meter value={climb} />
          <p class="type-label">Rank {next.rank} needs <span>{num(next.merits)}</span> · {next.reward}</p>
        {/if}
      </section>

      <section class="cycle" aria-labelledby="pp-cycle">
        <h3 class="section" id="pp-cycle">This cycle</h3>
        {#if !st}
          <p class="type-label">{waiting}</p>
        {:else}
          <p><span class="type-value-xl">{timeLeft(st.week_ends, now)}</span><span class="type-unit">left</span></p>
          <p class="type-label">Ends {endsAt(st.week_ends)}</p>
        {/if}
      </section>

      <section class="merits" aria-labelledby="pp-merits">
        <h3 class="section" id="pp-merits">Your merits</h3>
        {#if !st}
          <p class="type-label">{waiting}</p>
        {:else}
          <p>
            <span class="type-value-xl" {@attach tip("What the journal logs since the cycle began; weekly assignment rewards aren’t logged")}>{num(st.week_merits)}</span><span class="type-unit">this cycle</span>
          </p>
          <p class="type-label"><span>{num(st.session_merits)}</span> this session · <span>{num(st.merits)}</span> since you pledged</p>
        {/if}
      </section>
    {/if}

    <section class="sys" aria-labelledby="pp-sys">
      <h3 class="section" id="pp-sys">This system</h3>
      {#if !ov}
        <p class="type-label">{waiting}</p>
      {:else if !ov.here}
        <p class="type-label">No jump or location in the journal yet.</p>
      {:else}
        <p class="name">
          <span class="type-name" {@attach tip("As of your last jump")}>{sys?.system ?? ov.here}</span><SystemChip name={sys?.system ?? ov.here} icon />
          <ExtLinks system={sys?.system ?? ov.here} />
        </p>
        {#if !sys}
          <p class="type-label">No Powerplay activity here.</p>
        {:else}
          <p class="type-label">
            {#if sys.controlling_power}<span>{sys.state ?? "Controlled"}</span> by <span>{sys.controlling_power}</span>{:else}<span>{sys.state ?? "Unoccupied"}</span>{/if}
            {#if sys.work}· <span>{WORK_LABEL[sys.work]}</span>{/if}
          </p>
          {#if sys.controlling_power}
            <div class="tug">
              <Meter value={tug.share} rest />
              <p class="ends">
                <span><span class="type-figures">{num(sys.reinforcement)}</span><span class="type-unit">reinforcement</span></span>
                <span><span class="type-figures">{num(sys.undermining)}</span><span class="type-unit">undermining</span></span>
              </p>
              <p class="type-label">
                {#if tug.ahead}{tug.ahead} is ahead by {num(tug.lead)} this cycle{:else}Level this cycle{/if}
              </p>
            </div>
            <div class="band">
              <Meter value={sys.control_progress ?? 0} thin />
              <span class="type-figures" {@attach tip(`Progress through the ${sys.state?.toLowerCase() ?? ""} band`.replace("  ", " "))}>{pct(sys.control_progress)}</span>
            </div>
          {:else}
            {#each sys.acquisition as a (a.power)}
              <div class="acq">
                <span class="type-label">{a.power}</span>
                <Meter value={a.progress} thin />
                <span class="type-figures" {@attach tip(`${a.power}'s progress to acquiring it`)}>{pct(a.progress)}</span>
              </div>
            {/each}
          {/if}
          {#if sys.contesting.length && (sys.controlling_power || !sys.acquisition.length)}
            <p class="type-label">{sys.controlling_power ? "Also active here" : "Active here"}: {sys.contesting.join(", ")}</p>
          {/if}
        {/if}
      {/if}
    </section>

    <section class="earn" aria-labelledby="pp-earn">
      <h3 class="section" id="pp-earn">What earns merits here</h3>
      {#if !ov}
        <p class="type-label">{waiting}</p>
      {:else if !st?.power}
        <p class="type-label">Pledge to a power to see what earns it merits here.</p>
      {:else if !sys}
        <p class="type-label">Nothing: no Powerplay activity here.</p>
      {:else if !sys.suggestions.length}
        <p class="type-label">Nothing here earns {st.power} merits: it isn’t acquiring this system.</p>
      {:else}
        <Suggestions items={sys.suggestions} rules={sys.activity_rules} />
      {/if}
    </section>

    <section class="work" aria-labelledby="pp-work">
      <h3 class="section" id="pp-work">Where to work</h3>
      {#if st && !st.power}
        <p class="type-label">Pledge to a power to see where it wants work.</p>
      {:else}
        <Targets />
      {/if}
    </section>
  </div>
</Board>

<style>
  header { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-6); margin: 0 0 var(--sp-8); }
  h2 { margin: 0; }
  p { margin: 0; }
  .err { color: var(--alarm); }
  .err::before { content: "! "; font-weight: 600; }

  /* Three blocks across the top and three under them. Where to work, the widest, has the track its
     full rows need (SearchKit's `fits` for Targets' groups, reinforcement and undermining and the
     report's age included: 720 px at 100 percent, with 8 px to spare), so no system name or state
     is cut beside the docked sidebar and no row goes compact. Two columns fit the 1100 x 700
     window's 64.6rem page column. Narrower, it keeps that track and the rest stack in one column beside it, so the
     choice and its rows stay in the first screen; narrower still, everything is one column. */
  .blocks {
    display: grid; gap: var(--sp-8) var(--gutter); align-items: start;
    grid-template-columns: minmax(0, 1fr);
    grid-template-areas: "rank" "cycle" "merits" "sys" "work" "earn";
  }
  @container win (min-width: 64.5rem) {
    .blocks {
      grid-template-columns: minmax(16.5rem, 25rem) 45.5rem;
      grid-template-rows: auto auto auto auto 1fr;
      grid-template-areas: "rank work" "cycle work" "merits work" "sys work" "earn work";
    }
    .blocks:not(.pledged) { grid-template-rows: auto 1fr; grid-template-areas: "sys work" "earn work"; }
  }
  @container win (min-width: 85.5rem) {
    .blocks, .blocks:not(.pledged) {
      grid-template-columns: minmax(17.5rem, 25rem) minmax(17.5rem, 25rem) 45.5rem;
      grid-template-rows: auto 1fr;
      grid-template-areas: "rank cycle merits" "sys earn work";
    }
    .blocks:not(.pledged) { grid-template-areas: "sys earn work"; }
  }
  .rank { grid-area: rank; }
  .cycle { grid-area: cycle; }
  .merits { grid-area: merits; }
  .sys { grid-area: sys; }
  .earn { grid-area: earn; }
  .work { grid-area: work; }
  section { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }
  .name { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-1) var(--sp-3); }
  .name > span:first-child { margin-right: calc(-1 * var(--sp-2)); }
  .tug { display: flex; flex-direction: column; gap: var(--sp-2); }
  .ends { display: flex; justify-content: space-between; gap: var(--sp-4); }
  .band, .acq { display: grid; grid-template-columns: minmax(0, 1fr) auto; align-items: center; gap: var(--sp-3); }
  .acq { grid-template-columns: auto minmax(0, 1fr) auto; }
</style>
