<script lang="ts">
  // The logbook: every system and station visited, how often and when, and the timeline of
  // arrivals and dockings, from the journal history the core keeps. Search narrows both.
  import { coreStatus, type HistoryProgress } from "../../lib/core";
  import { loadLogbook, when, type Logbook } from "../../lib/logbook";
  import SystemChip from "../../lib/ui/SystemChip.svelte";

  let query = $state("");
  let book = $state<Logbook | null>(null);
  let failed = $state<string | null>(null);
  let progress = $state<HistoryProgress | null>(null);
  let seq = 0;

  async function refresh(q: string) {
    const mine = ++seq;
    try {
      const b = await loadLogbook(q);
      if (mine !== seq) return; // a newer search is in flight
      book = b;
      failed = null;
    } catch (e) {
      if (mine !== seq) return;
      failed = String(e);
    }
  }

  // Search as you type (debounced); re-read every 5 s, since the history grows while you play.
  $effect(() => {
    const q = query;
    const t = setTimeout(() => void refresh(q), 200);
    const poll = setInterval(() => {
      void refresh(q);
      void coreStatus().then((s) => (progress = s?.history ?? null));
    }, 5000);
    return () => {
      clearTimeout(t);
      clearInterval(poll);
    };
  });

  $effect(() => {
    void coreStatus().then((s) => (progress = s?.history ?? null));
  });

  const pct = $derived(progress && progress.bytes_total > 0 ? Math.floor((100 * progress.bytes) / progress.bytes_total) : null);
  const n = (x: number) => x.toLocaleString();
</script>

<section>
  <p class="sub">
    {#if book?.commander}
      CMDR {book.commander.name}{book.commander.legacy ? " (Legacy)" : ""} ·
      {n(book.totals.systems)} systems · {n(book.totals.stations)} stations ·
      {n(book.totals.system_visits)} arrivals · {n(book.totals.station_visits)} dockings
    {:else if book}
      No commander in the journal history yet.
    {:else if !failed}
      Reading the journal history…
    {/if}
    {#if book?.source}· {book.source}{/if}
  </p>
  {#if progress?.running && !progress.ready}
    <p class="sub">Importing your journals{pct !== null ? `: ${pct}%` : ""} ({n(progress.files)} files, {n(progress.lines)} lines so far). The logbook fills in as it goes.</p>
  {:else if progress?.error}
    <p class="sub err">Journal history: {progress.error}</p>
  {/if}

  <label class="search">
    <span>Search</span>
    <input type="search" bind:value={query} placeholder="System or station" spellcheck="false" autocomplete="off" />
  </label>

  {#if failed}
    <p class="err">{failed}</p>
  {:else if book}
    <h3>Places{book.query ? ` matching “${book.query}”` : ""}</h3>
    {#if book.places.length}
      <table>
        <thead><tr><th>Place</th><th class="num">Visits</th><th>First</th><th>Last</th></tr></thead>
        <tbody>
          {#each book.places as p (`${p.kind}:${p.market_id ?? p.system_address ?? p.star_system}:${p.station_name ?? ""}`)}
            <tr>
              <td>
                {#if p.kind === "station"}
                  <span class="station">{p.station_name}</span> <SystemChip name={p.star_system} />
                {:else}
                  <SystemChip name={p.star_system} />
                {/if}
              </td>
              <td class="num">{n(p.visits)}</td>
              <td class="mono">{when(p.first)}</td>
              <td class="mono">{when(p.last)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
      {#if book.matches > book.places.length}
        <p class="lo">Showing the {n(book.places.length)} most recent of {n(book.matches)}. Search to narrow it down.</p>
      {/if}
    {:else}
      <p class="lo">{book.query ? `No visited system or station matches “${book.query}”.` : "No visits yet."}</p>
    {/if}

    <h3>Timeline</h3>
    {#if book.timeline.length}
      <ol class="timeline">
        {#each book.timeline as v, i (`${v.ts}:${i}`)}
          <li>
            <span class="mono lo">{when(v.ts)}</span>
            {#if v.kind === "station"}
              <span>Docked at <span class="station">{v.station_name}</span></span> <SystemChip name={v.star_system} />
            {:else}
              <span>Arrived in</span> <SystemChip name={v.star_system} />
              {#if v.jump_dist}<span class="lo">{v.jump_dist.toFixed(2)} ly</span>{/if}
            {/if}
          </li>
        {/each}
      </ol>
    {:else}
      <p class="lo">Nothing yet.</p>
    {/if}
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0 6px; }
  .err { color: var(--accent); }
  .search { display: flex; align-items: center; gap: 12px; margin: 14px 0 6px; text-transform: uppercase; letter-spacing: var(--caps); color: var(--text-lo); font-size: var(--fs-s); }
  .search input { flex: 0 1 24rem; background: var(--cell); color: var(--text); border: 1px solid var(--accent-dim); padding: 6px 8px; font: inherit; text-transform: none; letter-spacing: normal; font-size: var(--fs); }
  .search input:focus { outline: 1px solid var(--accent); outline-offset: -1px; }
  h3 { margin: 22px 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  table { border-collapse: collapse; width: 100%; max-width: 60rem; }
  th { text-align: left; color: var(--text-lo); font-size: var(--fs-s); font-weight: 400; text-transform: uppercase; letter-spacing: var(--caps); padding: 0 12px 6px 0; }
  td { padding: 5px 12px 5px 0; border-top: 1px solid var(--cell); vertical-align: baseline; }
  .num { text-align: right; font-family: var(--mono); }
  .mono { font-family: var(--mono); font-size: var(--fs-s); }
  .station { color: var(--text-hi); }
  .lo { color: var(--text-lo); }
  .timeline { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 6px; }
  .timeline li { display: flex; gap: 10px; align-items: baseline; flex-wrap: wrap; }
</style>
