<script lang="ts">
  // The session (DESIGN.md Board): what the latest game session earned over its length, by
  // activity (EDMC's hourly income), from the journal history the core keeps. The head names the
  // session; the main value is credits per hour, beside what was earned, jumps and distance. Over
  // time draws each payment on a band from the session's start to its last event, a row per
  // activity; By activity ranks the activities by what they paid. Nothing here can be changed.
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Board from "../../lib/ui/layout/Board.svelte";
  import { clock, duration, EARNED, EARNING_NAMES, loadSession, when, type Session } from "../../lib/logbook";

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

  const n = (x: number) => x.toLocaleString("en");
  const name = (kind: string) => EARNING_NAMES[kind] ?? kind;
  const payments = (c: number) => (c === 1 ? "1 payment" : `${n(c)} payments`);

  /** Where a moment sits on the band, 0 to 100, from the session's start to its last event. */
  const at = $derived.by(() => {
    if (!s) return () => 0;
    const from = Date.parse(s.start);
    const span = Date.parse(s.end) - from;
    return (ts: string) => (span > 0 ? Math.min(100, Math.max(0, ((Date.parse(ts) - from) / span) * 100)) : 0);
  });
  /** The band's rows: one per activity, in the totals' order, each with its payments. */
  const rows = $derived.by(() => {
    const v = s;
    return v ? v.earnings.map((e) => ({ kind: e.kind, marks: v.timeline.filter((p) => p.kind === e.kind) })) : [];
  });
</script>

<Board>
  {#if failed && !s}
    <p class="type-label err" role="alert"><b>!</b> {failed}</p>
  {:else if s}
    <div class="head">
      <h2 class="type-title">{s.latest ? "Latest session" : "Session"}</h2>
      <p class="type-label">
        CMDR {s.commander.name} · {when(s.start)} to {when(s.end)} UTC · {duration(s.duration_secs)}{#if s.source}{` · ${s.source}`}{/if}
      </p>
    </div>
    <Stale error={failed} />

    <div class="values">
      <p {@attach tip("What was earned over the session's length, from loading the game to its last event; shown once it has lasted a minute")}>
        <span class="type-value-xl main">{s.per_hour === null ? "–" : n(s.per_hour)}</span><span class="type-unit">cr per hour</span>
      </p>
      <p {@attach tip(EARNED)}><span class="type-value-xl">{n(s.total)}</span><span class="type-unit">cr earned</span></p>
      <p {@attach tip("Jumps by your own frame shift drive; carrier jumps aren't counted")}>
        <span class="type-value-xl">{n(s.jumps)}</span><span class="type-unit">{s.jumps === 1 ? "jump" : "jumps"}</span>
      </p>
      <p {@attach tip("Light years jumped by your own frame shift drive")}>
        <span class="type-value-xl">{s.distance_ly.toFixed(1)}</span><span class="type-unit">ly</span>
      </p>
    </div>
    <p class="type-label also">
      <span {@attach tip("Exploration and organic data, included in what was earned")}>{n(s.data_sold)} cr of data sold</span>
      · <span {@attach tip("Paid when you hand in the vouchers")}>{n(s.bounties_awarded)} {s.bounties_awarded === 1 ? "bounty" : "bounties"} awarded worth {n(s.bounty_value)} cr</span>
      {#if s.carrier_jumps}· <span {@attach tip("Not in jumps or distance")}>{n(s.carrier_jumps)} carrier {s.carrier_jumps === 1 ? "jump" : "jumps"}</span>{/if}
    </p>

    {#if s.earnings.length}
      <div class="room"><div class="parts">
        <section class="by" aria-labelledby="session-by">
          <h3 class="section" id="session-by">By activity</h3>
          <ol class="acts">
            {#each s.earnings as e (e.kind)}
              <li>
                <span class="type-name">{name(e.kind)}</span>
                <span class="amount"><span class="type-value-l ranked">{n(e.amount)}</span><span class="type-unit">cr</span></span>
                <span class="type-label">
                  {payments(e.count)}
                  {#if e.kind === "trade"}
                    · <span {@attach tip("Trade income minus what the goods cost, where the game reported it")}>{n(s.trade_profit)} cr profit</span>
                  {/if}
                </span>
              </li>
            {/each}
          </ol>
        </section>

        <section class="time" aria-labelledby="session-time">
          <h3 class="section" id="session-time">Over time</h3>
          <div class="band">
            {#each rows as r (r.kind)}
              <span class="type-label kind">{name(r.kind)}</span>
              <span class="track">
                {#each r.marks as p, i (`${p.ts}:${i}`)}
                  <span
                    class="mark"
                    style:left={`${at(p.ts)}%`}
                    role="img"
                    aria-label={`${name(p.kind)}, ${n(p.amount)} cr at ${clock(p.ts)} UTC`}
                    {@attach tip({ title: `${n(p.amount)} cr`, fact: name(p.kind), note: `${clock(p.ts)} UTC`, stop: false })}
                  ></span>
                {/each}
              </span>
            {/each}
            <span class="type-label ends"><span>{clock(s.start)}</span><span>{clock(s.end)}</span></span>
          </div>
        </section>
      </div></div>
    {:else}
      <p class="type-label">Nothing earned yet this session.</p>
    {/if}
  {:else if loaded}
    <p class="type-label">No game session in the journal history yet.</p>
  {:else}
    <p class="type-label">Reading the journal history…</p>
  {/if}
</Board>

<style>
  p, h2 { margin: 0; }
  .head { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-6); margin: 0 0 var(--sp-6); }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
  .values { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-3) var(--sp-12); }
  .also { margin-top: var(--sp-3); }

  /* By activity beside Over time where both fit, the list first; else the list above the band. */
  .room { container: session / inline-size; margin-top: var(--sp-8); }
  .parts { display: grid; grid-template-columns: minmax(0, 1fr); gap: var(--sp-8) var(--gutter); align-items: start; }
  @container session (min-width: 62rem) {
    .parts { grid-template-columns: max-content minmax(20rem, 40rem); }
  }
  section { display: flex; flex-direction: column; gap: var(--sp-3); min-width: 0; }

  /* The Aligned Columns Rule: name, then the ranked amount right-aligned, then the count. */
  .acts { display: grid; grid-template-columns: max-content max-content max-content; column-gap: var(--sp-6); row-gap: var(--sp-2); margin: 0; padding: 0; list-style: none; }
  .acts li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: baseline; }
  .amount { justify-self: end; }
  .ranked { color: var(--accent); }

  /* One row of marks per activity, labelled at its left; the session's start and end under it. */
  .band { display: grid; grid-template-columns: max-content minmax(0, 1fr); column-gap: var(--sp-4); row-gap: var(--sp-1); align-items: center; }
  .track { position: relative; height: var(--sp-4); border-bottom: 1px solid var(--line); }
  /* The hover target is wider than the 2 px tick it draws. */
  .mark { position: absolute; top: 0; bottom: 0; padding: 0 var(--sp-1); transform: translateX(-50%); }
  .mark::after { content: ""; display: block; height: 100%; border-left: 2px solid var(--prose); }
  .ends { grid-column: 2; display: flex; justify-content: space-between; }
</style>
