<script lang="ts">
  // Powerplay for daily use, from the journal: my standing.
  import { loadOverview, num, pledged, until, type Overview } from "../../lib/powerplay";

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
  .lo { color: var(--text-lo); }
  .small { font-size: var(--fs-s); margin-top: 12px; }
  p { margin: 0 0 8px; line-height: 1.45; }
</style>
