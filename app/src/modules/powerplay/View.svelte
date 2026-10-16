<script lang="ts">
  // Powerplay for daily use, from the journal: my standing and the current system's state.
  import { loadOverview, num, pct, pledged, until, WORK_LABEL, type Overview } from "../../lib/powerplay";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let ov = $state<Overview | null>(null);
  let err = $state<string | null>(null);

  $effect(() => {
    const load = () =>
      loadOverview()
        .then((v) => {
          ov = v;
          err = null;
        })
        .catch((e: unknown) => (err = String(e)));
    void load();
    const t = setInterval(() => void load(), 2000);
    return () => clearInterval(t);
  });

  const st = $derived(ov?.standing);
  const sys = $derived(ov?.system);
  /** The control bar's fill, clamped to the band. */
  const fill = $derived(Math.max(0, Math.min(1, sys?.control_progress ?? 0)) * 100);
</script>

<section>
  <div class="panel">
    <h3>My standing</h3>
    {#if err && !ov}
      <p class="lo">{err}</p>
    {:else if !st}
      <p class="lo">Reading the journal…</p>
    {:else if !st.power}
      <p class="lo">Not pledged to a power. Pledge at a Power Contact or from the galaxy map’s Powerplay tab.</p>
    {:else}
      <p class="power">{st.power}</p>
      <dl>
        <dt>Rank</dt><dd class="q">{st.rank ?? "unknown"}</dd>
        <dt>Merits</dt><dd class="q">{num(st.merits)}</dd>
        <dt>This week</dt><dd><span class="q">{num(st.week_merits)}</span> <span class="lo">· cycle ends in {until(st.week_ends)}</span></dd>
        <dt>This session</dt><dd class="q">{num(st.session_merits)}</dd>
        <dt>Pledged</dt><dd>{pledged(st.time_pledged_secs)}</dd>
        {#if st.next}
          <dt>Next rank</dt>
          <dd>
            <span class="q">{st.next.rank}</span>
            {#if st.next.note}
              · {st.next.note}
            {:else}
              · <span class="q">{num(st.next.to_go)}</span> merits to go
            {/if}
            <span class="lo">· {st.next.reward}</span>
          </dd>
        {/if}
      </dl>
      <p class="lo small">Week merits count what the journal logs since Thursday 07:00 UTC; weekly assignment rewards aren’t logged.</p>
    {/if}
  </div>

  <div class="panel">
    <h3>This system</h3>
    {#if !ov}
      <p class="lo">Reading the journal…</p>
    {:else if !ov.here}
      <p class="lo">No jump or location in the journal yet.</p>
    {:else if !sys}
      <p><SystemChip name={ov.here} /></p>
      <p class="lo">No Powerplay activity here.</p>
    {:else}
      <p><SystemChip name={sys.system} /></p>
      {#if sys.work}<p class="work">{WORK_LABEL[sys.work]}</p>{/if}
      <dl>
        {#if sys.controlling_power}
          <dt>Controlled by</dt><dd>{sys.controlling_power}</dd>
          <dt>State</dt><dd>{sys.state ?? "unknown"}</dd>
          <dt>Control</dt>
          <dd>
            <span class="bar" title="Progress through the {sys.state?.toLowerCase() ?? ''} band"><span style:width="{fill}%"></span></span>
            <span class="q">{pct(sys.control_progress)}</span>
          </dd>
          <dt>This cycle</dt>
          <dd>reinforcement <span class="q">{num(sys.reinforcement)}</span> · undermining <span class="q">{num(sys.undermining)}</span></dd>
          {#if sys.contesting.length}<dt>Also active</dt><dd>{sys.contesting.join(", ")}</dd>{/if}
        {:else}
          <dt>State</dt><dd>{sys.state ?? "Unoccupied"}</dd>
          {#each sys.acquisition as a (a.power)}
            <dt>Acquiring</dt><dd>{a.power} <span class="q">{pct(a.progress)}</span></dd>
          {:else}
            {#if sys.contesting.length}<dt>Powers</dt><dd>{sys.contesting.join(", ")}</dd>{/if}
          {/each}
        {/if}
      </dl>
      <p class="lo small">As of your last jump. The game shows this in the system map, not on the HUD.</p>
    {/if}
  </div>
</section>

<style>
  section { padding: 22px 0 28px; display: grid; grid-template-columns: repeat(auto-fit, minmax(22rem, 1fr)); gap: 40px; align-items: start; }
  .panel { min-width: 0; }
  h3 { margin: 0 0 10px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; text-transform: uppercase; letter-spacing: var(--caps); }
  .power { margin: 0 0 10px; color: var(--accent); font-size: var(--fs-l); }
  dl { display: grid; grid-template-columns: 8rem 1fr; gap: 6px 14px; margin: 0; }
  dt { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  dd { margin: 0; color: var(--text); }
  .q { font-family: var(--mono); color: var(--text-hi); }
  .work { color: var(--accent); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); }
  .bar { display: inline-block; width: 8rem; height: 6px; background: var(--cell-empty); vertical-align: middle; margin-right: 8px; }
  .bar span { display: block; height: 100%; background: var(--accent); }
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); margin-top: 12px; }
  p { margin: 0 0 8px; line-height: 1.45; }
</style>
