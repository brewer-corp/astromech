<script lang="ts">
  // The session summary: what the latest game session earned and covered (EDMC's hourly
  // income), from the journal history the core keeps.
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
        .catch((e: unknown) => (failed = String(e)));
    void tick();
    const t = setInterval(() => void tick(), 5000);
    return () => clearInterval(t);
  });

  const cr = (n: number) => `${n.toLocaleString()} cr`;
</script>

<section>
  {#if failed}
    <p class="err">{failed}</p>
  {:else if s}
    <p class="sub">
      CMDR {s.commander.name} · {s.latest ? "Latest session" : "Session"} from {when(s.start)} to {when(s.end)} UTC ({duration(s.duration_secs)})
      {#if s.source}· {s.source}{/if}
    </p>
    <div class="tiles">
      <div><span class="k">Earned</span><span class="v">{cr(s.total)}</span></div>
      <div><span class="k">Per hour</span><span class="v">{s.per_hour === null ? "–" : cr(s.per_hour)}</span></div>
      <div><span class="k">Jumps</span><span class="v">{s.jumps.toLocaleString()}</span></div>
      <div><span class="k">Distance</span><span class="v">{s.distance_ly.toFixed(1)} ly</span></div>
    </div>

    <h3>Credits by activity</h3>
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

    <h3>Also</h3>
    <ul class="also">
      <li>Data sold: {cr(s.data_sold)} <span class="lo">(exploration and organic, included above)</span></li>
      <li>Bounties awarded: {s.bounties_awarded} worth {cr(s.bounty_value)} <span class="lo">(paid when you hand in the vouchers)</span></li>
      {#if s.carrier_jumps}<li>Carrier jumps: {s.carrier_jumps} <span class="lo">(not in jumps or distance)</span></li>{/if}
    </ul>
    <p class="lo note">Earnings are credits paid for what you did: trade sales, data, vouchers handed in, mission rewards, rescue, community goals, Odyssey goods and Powerplay salary. Selling ships, modules or drones isn't counted.</p>
  {:else if loaded}
    <p class="sub">No game session in the journal history yet.</p>
  {:else}
    <p class="sub">Reading the journal history…</p>
  {/if}
</section>

<style>
  section { padding: 8px 0 28px; min-height: 0; }
  .sub { color: var(--text-lo); font-size: var(--fs-s); margin: 14px 0 14px; }
  .err { color: var(--accent); }
  .tiles { display: flex; flex-wrap: wrap; gap: var(--gap); }
  .tiles div { display: flex; flex-direction: column; gap: 4px; min-width: 11rem; padding: 10px 14px; background: var(--cell); }
  .k { color: var(--text-lo); font-size: var(--fs-s); text-transform: uppercase; letter-spacing: var(--caps); }
  .v { color: var(--text-hi); font-family: var(--mono); font-size: var(--fs-l); }
  h3 { margin: 22px 0 8px; color: var(--text-hi); font-size: var(--fs-l); font-weight: 500; }
  table { border-collapse: collapse; max-width: 48rem; width: 100%; }
  td { padding: 5px 16px 5px 0; border-top: 1px solid var(--cell); }
  .num { text-align: right; font-family: var(--mono); }
  .lo { color: var(--text-lo); }
  .also { margin: 0; padding: 0 0 0 18px; display: flex; flex-direction: column; gap: 6px; }
  .note { margin-top: 18px; font-size: var(--fs-s); max-width: 48rem; }
</style>
