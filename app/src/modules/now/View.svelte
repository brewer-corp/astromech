<script lang="ts">
  // Now: what the commander is doing at the moment, where the window opens (DESIGN.md Top tabs).
  // Docked, the station leads with its services board; in space, the system; with Elite not
  // running, the last place. Under it the few answers that change while playing, each shown
  // only when it has something to say: what the hold would fetch, this session, the Powerplay
  // cycle and a mining run. The next route stop is the sidebar's, not this page's.
  //
  // It reads the frame's status and ship polls, and polls the rest as the page each block
  // summarises does. The page's one main value is the first block that shows a figure of: the
  // mining run's tons per hour, the hold's best total, the session's credits per hour. Only that
  // block draws its value in Value XL, Orange Lit; every other block's value steps down to
  // Value L, Options Grey (DESIGN.md Board), so size and colour agree on which one leads.
  import { useFrame } from "../../lib/frame.svelte";
  import { creditParts, loadUnsold, type Unsold } from "../../lib/exploration";
  import { failure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { duration, EARNED, loadSession, type Session } from "../../lib/logbook";
  import { cycleMerits, loadOverview, num, timeLeft, type Overview } from "../../lib/powerplay";
  import { loadHere, type DockedHere } from "../../lib/station";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Blocks from "../../lib/ui/layout/Blocks.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import { tip } from "../../lib/ui/tip";
  import Head from "../station/Head.svelte";
  import Services from "../station/Board.svelte";
  import Block from "./Block.svelte";
  import { Poll } from "./poll.svelte";
  import Sale from "./Sale.svelte";

  const { frame, own } = useFrame();
  if (own) $effect(() => frame.start());

  // As often as the pages they summarise read them.
  const here = new Poll<DockedHere>(loadHere, "read the docked station", 5000);
  const session = new Poll<Session | null>(loadSession, "read the session", 5000);
  const power = new Poll<Overview>(loadOverview, "read your Powerplay standing", 5000);
  const unsold = new Poll<Unsold | null>(loadUnsold, "read your unsold data", 15000);
  for (const p of [here, session, power, unsold]) $effect(() => p.start());

  // A failed frame read keeps the last answer, as the page's own polls do: one missed read
  // neither hides the blocks nor makes Sell your cargo ask again with nothing changed.
  let status = $state<typeof frame.status>(null);
  let shipKept = $state<typeof frame.ship>(null);
  $effect(() => {
    if (frame.status) status = frame.status;
  });
  $effect(() => {
    if (frame.ship) shipKept = frame.ship;
  });
  const running = $derived(status?.game === "running");
  /** The last status or ship read failed, in words: beside what it may have left out of date. */
  const statusReason = $derived(frame.statusError == null ? null : failure("read whether Elite is running", frame.statusError));
  const shipReason = $derived(frame.shipError == null ? null : failure("read the ship", frame.shipError));
  /** Whether Elite runs has never been read: the core hasn't answered yet. */
  const unknownGame = $derived(frame.read && !status && !!statusReason);
  /** The frame has read the status once: until then, whether Elite runs isn't known. */
  const known = $derived(frame.read);
  const at = $derived(here.value);
  /** docked_here knows no place at all: no game in the journal yet. */
  const noGame = $derived(here.loaded && !!at && !at.docked && !at.system);
  const docked = $derived(running && !!at?.docked && !!at.station);
  const inSpace = $derived(running && !!at && !at.docked && !!at.system);

  const ship = $derived(shipKept);
  const hold = $derived(running && !!ship?.sellable?.length);
  const run = $derived(running ? (status?.mining ?? null) : null);
  const mining = $derived(!!run?.started);
  const pledged = $derived(power.value?.standing.power != null);
  const showPower = $derived(!power.loaded || pledged);
  const showSession = $derived(!session.loaded || session.value != null);

  /** The Sell your cargo block has a total on screen. */
  let saleFigure = $state(false);
  const main = $derived(
    mining && run?.per_hour != null ? "mining" : hold && saleFigure ? "sale" : showSession && session.value?.per_hour != null ? "session" : null,
  );

  const risky = $derived(unsold.value?.risky ? unsold.value : null);
  const lastPlace = $derived(at?.docked && at.station ? `Last docked at ${at.station}, ${at.system}` : at?.system ? `Last in ${at.system}` : null);
  const parts = (n: number) => creditParts(n);
</script>

<Board>
  <div class="now">
    {#if noGame}
      <p class="type-body">No game in the journal yet. Play Elite once and this page follows what you are doing.</p>
    {:else}
      <div class="lead">
        {#if !known || (!here.loaded && !here.failed)}
          <p class="type-label">Reading the journal…</p>
        {:else if unknownGame}
          <p class="type-label err" role="alert"><b>!</b> {statusReason}</p>
        {:else if docked && at}
          <Head here={at} />
        {:else if inSpace && at?.system}
          <h2 class="type-title system"><span>{at.system}</span><SystemChip name={at.system} icon /></h2>
        {:else if running}
          <h2 class="type-title">Elite is running</h2>
        {:else}
          <h2 class="type-title">Elite isn't running</h2>
          {#if lastPlace}<p class="type-label">{lastPlace}</p>{/if}
        {/if}
        {#if known && !here.loaded && here.failed}<p class="type-label err" role="alert"><b>!</b> {here.failed}</p>{/if}
        {#if !unsold.loaded && unsold.failed}<p class="type-label err" role="alert"><b>!</b> {unsold.failed}</p>{/if}
        <!-- A later read failed: what is shown stays, and each line says what may be out of date. -->
        {#if status && !unknownGame}<Stale error={statusReason} />{/if}
        {#if here.loaded}<Stale error={here.reason} />{/if}
        {#if unsold.loaded}<Stale error={unsold.reason} />{/if}
        {#if risky}
          {@const [amount, unit] = parts(risky.total)}
          <p class="risk" role="alert">
            <b>!</b> {amount} {unit} of exploration data aboard, unsold. Dying loses all of it.
            <a class="type-label link" href="#/exploration">System</a>
          </p>
        {/if}
      </div>

      <Blocks>
        <!-- Asked once where the commander is is known, so opening the page asks once. -->
        {#if hold && ship && here.loaded}
          <Sale {ship} system={at?.system ?? null} main={main === "sale"} stale={shipReason} onfigure={(f) => (saleFigure = f)} />
        {:else if running && shipReason}
          <!-- The hold isn't known (the ship read failed with none, or an empty hold, shown): say so. -->
          <Block title="Sell your cargo" href="#/places/sell-cargo" link="Sell my cargo" failed={shipReason} />
        {/if}
        {#if showSession}
          <Block
            title={running ? "This session" : "Last session"}
            href="#/session"
            link="Session"
            loading={session.loaded || session.failed ? null : "Reading the journal history…"}
            failed={session.loaded ? null : session.failed}
            stale={session.loaded ? session.failed : null}
          >
            {#if session.value}
              {@const s = session.value}
              {@const rate = s.per_hour == null ? null : parts(s.per_hour)}
              {@const [earned, earnedUnit] = parts(s.total)}
              <p class="value">
                <span class={main === "session" ? "type-value-xl main" : "type-value-l"}>{rate ? rate[0] : "–"}</span><span class="type-unit">{rate ? `${rate[1]} per hour` : "per hour"}</span>
              </p>
              <p class="type-label"><span {@attach tip(EARNED)}>{earned} {earnedUnit} earned</span> in {duration(s.duration_secs)} · {s.jumps.toLocaleString("en")} {s.jumps === 1 ? "jump" : "jumps"}</p>
            {/if}
          </Block>
        {/if}
        {#if showPower}
          <Block
            title="Powerplay"
            href="#/powerplay"
            link="Powerplay"
            loading={power.loaded || power.failed ? null : "Reading the journal…"}
            failed={power.loaded ? null : power.failed}
            stale={power.loaded ? power.failed : null}
          >
            {#if power.value && pledged}
              {@const st = power.value.standing}
              {@const now = power.at}
              <p class="value">
                <span class="type-value-l" {@attach tip("What the journal logs since Thursday 07:00 UTC; weekly assignment rewards aren’t logged")}>{num(cycleMerits(st, now))}</span><span class="type-unit">merits this cycle</span>
              </p>
              <p class="type-label">{timeLeft(st.week_ends, now)} left in the cycle</p>
              <p class="type-label">
                {st.power}{#if st.rank != null}{` · rank ${st.rank}`}{/if}{#if st.next}{` · ${num(st.next.to_go)} merits to rank ${st.next.rank}`}{/if}
              </p>
            {/if}
          </Block>
        {/if}
        {#if mining && run}
          <Block title="Mining" href="#/mining" link="Mining" stale={statusReason}>
            <p class="value">
              <span class={main === "mining" ? "type-value-xl main" : "type-value-l"}>{run.per_hour == null ? "–" : run.per_hour.toFixed(0)}</span><span class="type-unit">t per hour</span>
            </p>
            <p class="type-label">{run.total.toLocaleString("en")} t refined</p>
          </Block>
        {/if}
      </Blocks>

      {#if docked && at}
        <Services services={at.services} />
      {/if}
    {/if}
  </div>
</Board>

<style>
  p, h2 { margin: 0; }
  .now { display: flex; flex-direction: column; gap: var(--sp-8); }
  .lead { display: flex; flex-direction: column; gap: var(--sp-3); }
  .system { display: flex; align-items: center; gap: var(--sp-1); }
  .value { display: flex; flex-wrap: wrap; align-items: baseline; }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
  /* A warning (DESIGN.md States): the orange "!" and plain words, where it applies. */
  .risk { color: var(--accent); font-weight: 500; }
  .risk b { font-weight: 600; }
  .link { margin-left: var(--sp-2); color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .link:hover { color: var(--orange-lit); }
</style>
