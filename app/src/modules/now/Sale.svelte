<script lang="ts">
  // Sell your cargo: the best station for the hold the commander has now, from where they are,
  // in the ship they fly (trade_cargo, with the Sell my cargo finder's filters, so it is that
  // finder's first row). It asks when it opens and whenever the change key (lib/now.ts) differs
  // from the one its answer was asked with, never on a timer alone. While a new answer is on its
  // way the old one stays, Grey Dim with its actions off (DESIGN.md States, Loading).
  import { untrack } from "svelte";
  import { jumps } from "../../lib/galaxy";
  import { saleKey } from "../../lib/now";
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote, type Waypoint } from "../../lib/route";
  import { loadFilters } from "../../lib/search";
  import type { ShipStatus } from "../../lib/core";
  import { credits, DEFAULT_DISTANCE_LY, takes, tradeCargo, type CargoSearch, type StationSale } from "../../lib/trade";
  import { failure } from "../../lib/ui/errors";
  import Failure from "../../lib/ui/Failure.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import SystemChip from "../../lib/ui/SystemChip.svelte";
  import PadLetter from "../../lib/ui/PadLetter.svelte";
  import Block from "./Block.svelte";

  let {
    ship,
    system,
    main = false,
    stale = null,
    onfigure,
  }: {
    ship: ShipStatus;
    /** Where the commander is (docked_here's system). */
    system: string | null;
    /** Its total is the page's one main value. */
    main?: boolean;
    /** The ship's last read failed: the hold shown may be out of date. */
    stale?: string | null;
    /** Told whether a station's total is on screen. */
    onfigure?: (shown: boolean) => void;
  } = $props();

  const key = $derived(saleKey(ship, system));
  /** The key the last ask was made with, answered or not: a failed ask isn't made again until
   * the key changes or TRY AGAIN. */
  let asked: string | null = null;
  let answer = $state<CargoSearch | null>(null);
  /** The search's radius the answer was asked with. */
  let radius = $state(DEFAULT_DISTANCE_LY);
  let busy = $state(false);
  let err = $state<string | null>(null);
  let note = $state<FollowNote | null>(null);
  let seq = 0;

  async function ask(k: string) {
    asked = k;
    const n = ++seq;
    busy = true;
    err = null;
    note = null;
    try {
      const filters = await loadFilters("trade.cargo");
      const r = await tradeCargo(null, filters);
      if (n !== seq) return;
      answer = r;
      radius = filters.max_distance_ly ?? DEFAULT_DISTANCE_LY;
    } catch (e) {
      if (n === seq) err = failure("ask Ardent Insight", e);
    } finally {
      if (n === seq) busy = false;
    }
  }

  $effect(() => {
    const k = key;
    untrack(() => k !== asked && void ask(k));
  });

  const best = $derived(answer?.plan.stations[0] ?? null);
  $effect(() => {
    onfigure?.(!err && !!best);
  });

  const stop = (s: StationSale): Waypoint => ({ system: s.system, label: s.station });

  async function add(s: StationSale) {
    try {
      note = addedNote(await addStop(stop(s), "sell cargo"));
    } catch (e) {
      note = failedNote("Couldn't add it", e, () => void add(s));
    }
  }

  async function destination(s: StationSale) {
    try {
      note = followedNote(await followRoute(s.system, "sell cargo", [stop(s)]));
    } catch (e) {
      note = failedNote("Couldn't follow it", e, () => void destination(s));
    }
  }

</script>

<Block title="Sell your cargo" href="#/places/sell-cargo" link="Sell my cargo" {stale}>
  {#if err}
    <Failure text={err} onaction={() => void ask(key)} />
  {:else}
    {#if busy}<p class="type-label">Asking Ardent…</p>{/if}
    {#if answer}
      <div class="answer" class:dim={busy}>
        {#if best}
          <p class="value">
            <span class={main ? "type-value-xl" : "type-value-l"} class:main={main && !busy}>{credits(best.total)}</span><span class="type-unit">cr</span>
            <span class="type-label takes">takes {takes(best.sold, answer.plan.load)} t</span>
          </p>
          <p class="place">
            <span class="type-name station">{best.station}</span>
            <span class="type-label">{best.system}</span><SystemChip name={best.system} icon />
            <span class="type-label">{jumps(best.jumps, best.exact)}</span>
            {#if best.pad}<PadLetter pad={best.pad} />{/if}
            <span class="acts">
              <IconButton icon="add-stop" label={`Add ${best.system} as a stop`} tip={`Add ${best.system} as the next stop`} disabled={busy} onclick={() => void add(best)} />
              <IconButton icon="destination" label={`Set ${best.system} as the destination`} tip="Set destination: replaces the route" disabled={busy} onclick={() => void destination(best)} />
            </span>
          </p>
          <FollowNoteLine bind:note />
        {:else}
          <p class="type-body none">No station within {radius.toLocaleString("en")} ly buys your hold.</p>
        {/if}
      </div>
    {/if}
  {/if}
</Block>

<style>
  p { margin: 0; }
  .answer { display: flex; flex-direction: column; gap: var(--sp-2); min-width: 0; }
  .value { display: flex; flex-wrap: wrap; align-items: baseline; column-gap: var(--sp-2); }
  .value .type-unit { margin-left: calc(var(--sp-1) - var(--sp-2)); }
  /* The Lifted Name Rule: the station a Now answer names is White. */
  .station { color: var(--text-hi); }
  .place { display: flex; flex-wrap: wrap; align-items: center; column-gap: var(--sp-2); }
  .place :global(.chip) { margin-left: calc(-1 * var(--sp-2)); }
  .acts { display: inline-flex; }
  /* A refresh keeps the old answer Grey Dim until it is replaced (DESIGN.md States, Loading). */
  .dim, .dim :global(*) { color: var(--grey-dim); }
</style>
