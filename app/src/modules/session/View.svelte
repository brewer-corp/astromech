<script lang="ts">
  // The session summary: what the latest game session earned and covered (EDMC's hourly
  // income), from the journal history the core keeps.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Stat from "../../lib/ui/Stat.svelte";
  import { duration, EARNED, EARNING_NAMES, loadSession, when, type Session } from "../../lib/logbook";

  let s = $state<Session | null>(null);
  let loaded = $state(false);
  let failed = $state<string | null>(null);

  $effect(() => {
    const tick = () =>
      loadSession()
        .then((v) => {
          s = v;
          loaded = true;
          failed = null;
        })
        .catch((e: unknown) => (failed = pollFailure("read the session", e, s !== null)));
    void tick();
    const t = setInterval(() => void tick(), 5000);
    return () => clearInterval(t);
  });

  const cr = (n: number) => `${n.toLocaleString()} cr`;
</script>

<section>
  <Stale error={s ? failed : null} />
  {#if failed && !s}
    <p class="err">{failed}</p>
  {:else if s}
    <p class="type-label sub">
      CMDR {s.commander.name} · {s.latest ? "Latest session" : "Session"} from {when(s.start)} to {when(s.end)} UTC ({duration(s.duration_secs)})
      {#if s.source}· {s.source}{/if}
    </p>
    <div class="tiles">
      <Stat main label="Earned" value={s.total.toLocaleString()} unit="cr" {@attach tip(EARNED)} />
      <Stat label="Per hour" value={s.per_hour === null ? "–" : s.per_hour.toLocaleString()} unit={s.per_hour === null ? "" : "cr"} />
      <Stat label="Jumps" value={s.jumps.toLocaleString()} />
      <Stat label="Distance" value={s.distance_ly.toFixed(1)} unit="ly" />
    </div>

    <h3 class="section">Credits by activity</h3>
    {#if s.earnings.length}
      <table>
        <tbody>
          {#each s.earnings as e (e.kind)}
            <tr>
              <td>{EARNING_NAMES[e.kind] ?? e.kind}</td>
              <td class="num">{cr(e.amount)}</td>
              <td class="lo">{e.count === 1 ? "1 payment" : `${e.count} payments`}{e.kind === "trade" ? ` · profit ${cr(s.trade_profit)}` : ""}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    {:else}
      <p class="lo">Nothing earned yet this session.</p>
    {/if}

    <h3 class="section">Also</h3>
    <ul class="also">
      <li {@attach tip("Exploration and organic data, included in Earned")}>Data sold: {cr(s.data_sold)}</li>
      <li {@attach tip("Paid when you hand in the vouchers")}>Bounties awarded: {s.bounties_awarded} worth {cr(s.bounty_value)}</li>
      {#if s.carrier_jumps}<li {@attach tip("Not in Jumps or Distance")}>Carrier jumps: {s.carrier_jumps}</li>{/if}
    </ul>
  {:else if loaded}
    <p class="type-label sub">No game session in the journal history yet.</p>
  {:else}
    <p class="type-label sub">Reading the journal history…</p>
  {/if}
</section>

<style>
  section { padding: var(--sp-2) 0 1.75rem; min-height: 0; }
  .sub { margin: 0.875rem 0 0.875rem; }
  .err { color: var(--accent); font-weight: 500; }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  h3 { margin: 1.375rem 0 var(--sp-2); }
  table { border-collapse: collapse; max-width: 45rem; width: 100%; }
  td { padding: 0.3125rem var(--sp-4) 0.3125rem 0; border-top: 1px solid var(--line); }
  .num { text-align: right; color: var(--accent); font-weight: 500; } /* the column the list is ranked by */
  .lo { color: var(--label-grey); }
  .also { margin: 0; padding: 0 0 0 1.125rem; display: flex; flex-direction: column; gap: 0.375rem; }
</style>
