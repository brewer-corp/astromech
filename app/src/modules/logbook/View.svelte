<script lang="ts">
  // The logbook (DESIGN.md Board): travel by day. The totals line and the search box lead; under
  // them the places matching the search beside the timeline of arrivals and dockings grouped by
  // day (newest first), or the places above the days where both don't fit. From the journal
  // history the core keeps; search narrows both. Rows are read-only: there is nothing more to
  // show about a place.
  import Input from "../../lib/ui/Input.svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Board from "../../lib/ui/layout/Board.svelte";
  import { coreStatus, type HistoryProgress } from "../../lib/core";
  import { byDay, clock, loadLogbook, when, type Logbook, type Place } from "../../lib/logbook";
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
      failed = pollFailure("read the journal history", e, book !== null);
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
  const n = (x: number) => x.toLocaleString("en");
  const days = $derived(book ? byDay(book.timeline) : []);
  const key = (p: Place) => `${p.kind}:${p.market_id ?? p.system_address ?? p.star_system}:${p.station_name ?? ""}`;
  const plural = (x: number, one: string, many: string) => `${n(x)} ${x === 1 ? one : many}`;
</script>

<Board>
  <div class="head">
    {#if book?.commander}
      <p class="type-label">
        CMDR {book.commander.name}{book.commander.legacy ? " (Legacy)" : ""}{#if book.source}{` · ${book.source}`}{/if}
      </p>
      <p class="totals">
        <span><span class="type-value-l">{n(book.totals.systems)}</span><span class="type-unit">systems</span></span>
        <span><span class="type-value-l">{n(book.totals.stations)}</span><span class="type-unit">stations</span></span>
        <span><span class="type-value-l">{n(book.totals.system_visits)}</span><span class="type-unit">arrivals</span></span>
        <span><span class="type-value-l">{n(book.totals.station_visits)}</span><span class="type-unit">dockings</span></span>
      </p>
    {:else if book}
      <p class="type-label">No commander in the journal history yet.{#if book.source}{` · ${book.source}`}{/if}</p>
    {:else if !failed}
      <p class="type-label">Reading the journal history…</p>
    {/if}
    {#if progress?.running && !progress.ready}
      <p class="type-label">Importing your journals{pct !== null ? `: ${pct}%` : ""} ({n(progress.files)} files, {n(progress.lines)} lines so far). The logbook fills in as it goes.</p>
    {:else if progress?.error}
      <p class="type-label err" role="alert"><b>!</b> Journal history: {progress.error}</p>
    {/if}

    <label class="search">
      <span class="type-label">Search</span>
      <Input class="q" type="search" bind:value={query} placeholder="System or station" spellcheck="false" autocomplete="off" />
    </label>
  </div>

  <Stale error={book ? failed : null} />
  {#if failed && !book}
    <p class="type-label err" role="alert"><b>!</b> {failed}</p>
  {:else if book}
    <div class="room"><div class="lists">
      <section aria-labelledby="logbook-places">
        <h3 class="section" id="logbook-places">Places{book.query ? ` matching “${book.query}”` : ""}</h3>
        {#if book.places.length}
          <ol class="places">
            {#each book.places as p (key(p))}
              <li>
                {#if p.kind === "station"}
                  <span class="line"><span class="type-name">{p.station_name}</span> <span class="type-label">{p.star_system}</span><SystemChip name={p.star_system} icon /></span>
                {:else}
                  <span class="line"><span class="type-name">{p.star_system}</span><SystemChip name={p.star_system} icon /></span>
                {/if}
                <span class="type-label">{plural(p.visits, "visit", "visits")} · first {when(p.first)} · last {when(p.last)}</span>
              </li>
            {/each}
          </ol>
          {#if book.matches > book.places.length}
            <p class="type-label">Showing the {n(book.places.length)} most recent of {n(book.matches)}. Search to narrow it down.</p>
          {/if}
        {:else}
          <p class="type-label">{book.query ? `No visited system or station matches “${book.query}”.` : "No visits yet."}</p>
        {/if}
      </section>

      <div class="days">
        {#each days as d (d.date)}
          <section aria-labelledby={`logbook-${d.date}`}>
            <h3 class="section" id={`logbook-${d.date}`}>
              <span>{d.date}</span>
              <span class="type-label count" {@attach tip("Of the arrivals and dockings listed: FSD jumps (not carrier jumps or logins) and dockings")}>
                {plural(d.jumps, "jump", "jumps")} · {plural(d.dockings, "docking", "dockings")}
              </span>
            </h3>
            <ol class="visits">
              {#each d.visits as v, i (`${v.ts}:${i}`)}
                <li>
                  <span class="type-figures">{clock(v.ts)}</span>
                  {#if v.kind === "station"}
                    <span class="line"><span class="type-name">{v.station_name}</span> <span class="type-label">{v.star_system}</span><SystemChip name={v.star_system} icon /></span>
                    <span></span>
                  {:else}
                    <span class="line"><span class="type-name">{v.star_system}</span><SystemChip name={v.star_system} icon /></span>
                    <span class="type-label">
                      {#if v.jump_dist}{v.jump_dist.toFixed(2)} ly{:else if v.via === "CarrierJump"}carrier jump{/if}
                    </span>
                  {/if}
                </li>
              {/each}
            </ol>
          </section>
        {:else}
          <p class="type-label">Nothing yet.</p>
        {/each}
      </div>
    </div></div>
  {/if}
</Board>

<style>
  p { margin: 0; }
  .head { display: flex; flex-direction: column; align-items: flex-start; gap: var(--sp-3); margin: 0 0 var(--sp-8); }
  .totals { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-6); }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
  .search { display: flex; align-items: center; gap: var(--sp-3); align-self: stretch; }
  .search > :global(.q) { flex: 0 1 22.5rem; }

  /* The places beside the days where both fit, else the places above the days. */
  .room { container: logbook / inline-size; }
  .lists { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sp-8) var(--gutter); align-items: start; }
  @container logbook (min-width: 58rem) {
    .lists { grid-template-columns: minmax(0, max-content) minmax(0, max-content); }
  }
  section { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }
  .days { display: flex; flex-direction: column; gap: var(--sp-8); min-width: 0; }
  .count { text-transform: none; letter-spacing: normal; font-weight: 400; }
  ol { margin: 0; padding: 0; list-style: none; }
  .line { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); }
  .places { display: flex; flex-direction: column; gap: var(--sp-3); }
  .places li { display: flex; flex-direction: column; gap: var(--gap); }
  /* The Aligned Columns Rule: the time, the place, the jump's distance, down each day. */
  .visits { display: grid; grid-template-columns: max-content max-content max-content; gap: var(--sp-2) var(--sp-4); }
  .visits li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center; }
  .visits li > :last-child { justify-self: end; }
</style>
