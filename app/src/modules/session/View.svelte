<script lang="ts">
  // The session summary: what the latest game session earned and covered (EDMC's hourly
  // income), from the journal history the core keeps.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Stat from "../../lib/ui/Stat.svelte";
  import { duration, EARNING_NAMES, loadSession, when, type Session } from "../../lib/logbook";

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

  /** What Earned counts, in its tile's Tip. */
  const EARNED = "Credits paid for what you did: trade sales, data, vouchers handed in, mission rewards, rescue, community goals, Odyssey goods and Powerplay salary. Selling ships, modules or drones isn't counted.";
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
      <Stat label="Earned" value={s.total.toLocaleString()} unit="cr" {@attach tip(EARNED)} />
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
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { margin: 14px 0 14px; }
  .err { color: var(--accent); }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  h3 { margin: 22px 0 8px; }
  table { border-collapse: collapse; max-width: 48rem; width: 100%; }
  td { padding: 5px 16px 5px 0; border-top: 1px solid var(--line); }
  .num { text-align: right; }
  .lo { color: var(--orange-mid); }
  .also { margin: 0; padding: 0 0 0 18px; display: flex; flex-direction: column; gap: 6px; }
</style>
