<script lang="ts">
  // The route as a timeline (astromech-ok29's Design), in the route sidebar's 320 px as the nav
  // spike drew it: each stop a node drawn as its star's type icon, which carries the star type
  // with a Tip, and between stops the leg to the next. Names truncate with a Tip, except the next
  // stop's, which wraps: it is typed in the galaxy map. The next stop is the one raised row: a larger name,
  // its copy icon and "N jumps away". The leg being flown opens out one row a jump, your row marked
  // with the map pin, when the route data lists its jumps (NavRoute once it is plotted in the
  // galaxy map, or an exact route's own list); otherwise it says its jump count and that the stars
  // appear once it is plotted. Other legs show the stars they pass where they are listed, else
  // their jump count, and warn of nowhere to scoop only when every jump is listed and known not
  // scoopable (noScoop). A stop's fuel and neutron notes and an inhabited stop's allegiance,
  // security and faction state sit on a second line. Every stop has its copy icon (the next one's
  // goes through the core, which follows the clipboard); the others take Make next on hover or
  // focus, over the end of their row.
  import { GAME_SOURCE, copyNext, makeNext, noScoop, starKind, type RouteJump, type RouteStop, type RouteView, type StarFacts } from "../../lib/route";
  import { population } from "../../lib/galaxy";
  import { tip } from "../../lib/ui/tip";
  import Button from "../../lib/ui/Button.svelte";
  import Failure from "../../lib/ui/Failure.svelte";
  import Glyph from "../../lib/ui/Glyph.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";

  let {
    view,
    act,
  }: {
    view: RouteView;
    /** Run a route command; resolves false when it failed (the page shows why). */
    act: (f: () => Promise<RouteView>) => Promise<boolean>;
  } = $props();

  type At = "past" | "here" | "ahead";
  type Row =
    | { kind: "stop"; key: string; i: number; stop: RouteStop; at: "past" | "next" | "ahead"; here: boolean }
    /** A jump of the flown leg; `start` is the system the galaxy map's plot starts from, when the
     * commander is still in it and it is no stop of the route (plotted again partway along). */
    | { kind: "jump"; key: string; hop: RouteJump; n: number; of: number; at: At; start?: boolean }
    | { kind: "leg"; key: string; stop: RouteStop; at: "past" | "next" | "ahead"; stars: RouteJump[] | null; count: boolean; unlisted: boolean; dry: boolean };

  const same = (a: string | null | undefined, b: string | null | undefined) => !!a && !!b && a.trim().toLowerCase() === b.trim().toLowerCase();
  const plural = (n: number, one: string, many = `${one}s`) => (n === 1 ? one : many);

  const route = $derived(view.route!);
  const next = $derived(route.next);
  const fromGame = $derived(route.source === GAME_SOURCE);
  const flown = $derived(view.stops[next] ?? null);
  /** How many of the flown leg's listed jumps are behind the commander: 0 is the system its list
   * starts from, which after a plot partway along is not the stop before. */
  const position = $derived(flown?.hops ? view.position : null);
  /** The commander is in the stop before the leg (none before waypoint 0). */
  const inPrev = $derived(position !== null && position > 0 ? false : same(view.here, view.stops[next - 1]?.system));
  /** The commander is where the leg's list starts, and that is no stop of the route. */
  const atStart = $derived(position === 0 && !inPrev && !!view.here);
  /** Jumps to the next stop, where they are known. */
  const away = $derived(flown?.hops && position != null ? flown.hops.length - position : inPrev ? (flown?.jumps ?? null) : null);

  const rows = $derived.by(() => {
    const out: Row[] = [];
    view.stops.forEach((stop, i) => {
      const at = i < next ? "past" : i === next ? "next" : "ahead";
      // Waypoint 0 has a leg only while it is next: the way there from wherever the commander is.
      if (i > 0 || at === "next") {
        const between = stop.hops ? stop.hops.slice(0, -1) : null;
        const dry = at !== "past" && noScoop(stop);
        if (at === "next" && between) {
          if (dry) out.push({ kind: "leg", key: `l${i}`, stop, at, stars: null, count: false, unlisted: false, dry });
          if (atStart) {
            const hop: RouteJump = { system: view.here!, star_class: null, scoopable: "unknown", neutron: null };
            out.push({ kind: "jump", key: `j${i}-start`, hop, n: 0, of: stop.hops!.length, at: "here", start: true });
          }
          between.forEach((hop, j) => {
            const where: At = position == null || j >= position ? "ahead" : j === position - 1 ? "here" : "past";
            out.push({ kind: "jump", key: `j${i}-${j}`, hop, n: j + 1, of: stop.hops!.length, at: where });
          });
        } else if (between?.length || stop.jumps == null || stop.jumps > 1) {
          const unlisted = !between && at === "next";
          out.push({ kind: "leg", key: `l${i}`, stop, at, stars: between?.length ? between : null, count: true, unlisted, dry });
        }
      }
      out.push({ kind: "stop", key: `s${i}`, i, stop, at, here: i === next - 1 && inPrev });
    });
    return out;
  });

  /** The stop whose copy icon shows the check, and the one whose copy failed (with why). */
  let copied = $state<number | null>(null);
  let copyErr = $state<{ i: number; text: string } | null>(null);
  let timer: ReturnType<typeof setTimeout> | undefined;

  function done(i: number) {
    copyErr = null;
    copied = i;
    clearTimeout(timer);
    timer = setTimeout(() => (copied = null), 1500);
  }

  async function copy(i: number, system: string) {
    if (i === next) {
      // The core keeps the next stop on the clipboard; its own failure shows as view.clipboard_error.
      copyErr = null;
      if ((await act(copyNext)) && !view.clipboard_error) done(i);
      return;
    }
    try {
      await navigator.clipboard.writeText(system);
      done(i);
    } catch (e) {
      copied = null;
      copyErr = { i, text: `Not on the clipboard: ${e instanceof Error ? e.message : String(e)}` };
    }
  }

  function starTip(s: StarFacts, title: string, fact?: string) {
    const k = starKind(s);
    return { title, fact: fact ? `${k.name} · ${fact}` : k.name, note: k.note };
  }

  /** An inhabited stop's second line: allegiance, security and the faction state; the rest
   * (government, population, the faction) is in its Tip. */
  function details(stop: RouteStop): string[] {
    const d = stop.details;
    if (!d) return [];
    return [d.allegiance ?? "", d.security ? `${d.security} security` : "", d.faction_state ?? ""].filter(Boolean);
  }

  function detailTip(stop: RouteStop): string {
    const d = stop.details;
    if (!d) return "";
    return [d.government, `${population(d.population)} people`, d.faction].filter(Boolean).join(" · ");
  }
</script>

<ol class="tl" aria-label="Stops and the jumps between them">
  {#each rows as row (row.key)}
    {#if row.kind === "stop"}
      {@const s = row.stop}
      {@const k = starKind(s)}
      {@const cur = row.at === "next"}
      <li class="stop {row.at}" class:first={row === rows[0]} class:last={row === rows.at(-1)} class:here={row.here}>
        <span class="sp">
          {#if row.here}
            <span class="node pin"><Glyph name="here" size={24} label="You are here" /></span>
          {:else}
            <span class="node" data-star={k.glyph} {@attach tip(starTip(s, s.system))}><Glyph name={k.glyph} size={cur ? 24 : 16} label={k.name} /></span>
          {/if}
        </span>
        <div class="main">
          {#if cur}<span class="type-label">Next stop{away != null ? ` · ${away} ${plural(away, "jump")} away` : ""}</span>{/if}
          <div class="nameline">
            {#if cur}
              <!-- The next stop's name wraps between its words, never inside one ("LD-H"). -->
              <span class="type-title name">{#each s.system.split(" ") as w, k (k)}{k ? " " : ""}<span class="word">{w}</span>{/each}</span>
            {:else}
              <span class="type-name name" {@attach tip(s.system)}>{s.system}</span>
            {/if}
            <IconButton
              icon={copied === row.i ? "copied" : "copy"}
              label={copied === row.i ? `Copied ${s.system}` : `Copy ${s.system}`}
              tip={copied === row.i ? "Copied" : "Copy to the clipboard"}
              onclick={() => void copy(row.i, s.system)}
            />
          </div>
          {#if row.here}<span class="type-label you">You are here</span>{/if}
          {#if s.label}<span class="type-label todo">{s.label}</span>{/if}
          {#if s.details}
            <span class="facts type-label" {@attach tip({ title: details(s).join(" · "), fact: detailTip(s) })}>{#each details(s) as f, k (f)}{k ? " · " : ""}<span class:sec-low={f === "Low security"} class:sec-anarchy={f === "Anarchy security"} class:state={f === s.details.faction_state}>{f}</span>{/each}</span>
          {/if}
          {#if cur && view.clipboard_error}
            <Failure text={`Not on the clipboard: ${view.clipboard_error}`} onaction={() => void copy(row.i, s.system)} />
          {:else if copyErr?.i === row.i}
            <Failure text={copyErr.text} onaction={() => void copy(row.i, s.system)} />
          {/if}
        </div>
        <div class="acts">
          {#if !cur && !fromGame}<Button onclick={() => void act(() => makeNext(row.i, s.system))}>Make next</Button>{/if}
        </div>
      </li>
    {:else if row.kind === "jump"}
      {@const h = row.hop}
      {@const k = starKind(h)}
      <li class="jump {row.at}" class:first={row === rows[0]}>
        <span class="sp">
          <span class="node" class:pin={row.at === "here"} data-star={k.glyph} {@attach tip(row.start ? null : starTip(h, h.system, `Jump ${row.n} of ${row.of}`))}>
            {#if row.at === "here"}<Glyph name="here" size={24} label="You are here" />{:else}<Glyph name={k.glyph} label={k.name} />{/if}
          </span>
        </span>
        <div class="main">
          {#if row.at === "here" && !row.start}<span class="inl" data-star={k.glyph}><Glyph name={k.glyph} label={k.name} /></span>{/if}
          <span class="type-name jn" {@attach tip(h.system)}>{h.system}</span>
          {#if row.at === "here"}<span class="type-label you">You are here</span>{/if}
        </div>
      </li>
    {:else}
      {@const s = row.stop}
      {@const n = s.hops?.length ?? s.jumps}
      <li class="leg {row.at}" class:first={row === rows[0]} aria-label={n != null ? `${n} ${plural(n, "jump")}` : "Jumps not listed"}>
        <span class="sp"></span>
        <div class="main">
          {#if row.count && n != null && (!row.stars || n > 5)}<span class="type-label"><span class="type-figures count">{n}</span> {plural(n, "jump")}</span>{/if}
          {#if row.stars}
            <span class="stars">
              {#each row.stars as h, j (j)}
                {@const k = starKind(h)}
                <span class="pass" data-star={k.glyph} {@attach tip(starTip(h, h.system, `Jump ${j + 1} of ${n}`))}><Glyph name={k.glyph} label={k.name} /></span>
              {/each}
            </span>
          {/if}
          {#if row.unlisted}<span class="type-label">The stars appear once the leg is plotted in the galaxy map.</span>{/if}
          {#if row.dry}<span class="type-label warn"><b>!</b> No scoopable star for {n} jumps</span>{/if}
        </div>
      </li>
    {/if}
  {/each}
</ol>

<style>
  /* Two columns shared by every row (subgrid): the spine with its nodes, 24 px so a name gets
     200 px of the sidebar, and the stop or jump. A stop's Make next sits over the end of its row,
     on hover or focus, right after what it acts on. */
  .tl { display: grid; grid-template-columns: 1.5rem minmax(0, 1fr); column-gap: var(--sp-2); margin: 0; padding: 0; list-style: none; }
  .tl > li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; position: relative; isolation: isolate; }
  /* A row's fill, from the stop column on, so the spine stays clear. */
  .tl > li::after { content: ""; position: absolute; inset: 0 0 0 calc(1.5rem + var(--sp-1)); z-index: -1; }
  .stop:not(.past):hover::after { background: var(--row-hover); }

  /* The spine: Label Grey, dashed ahead, Mark Off behind you. */
  .sp { position: relative; display: grid; place-items: center; }
  .sp::before { content: ""; position: absolute; top: 0; bottom: 0; left: calc(50% - 1px); border-left: 2px solid var(--label-grey); }
  .first .sp::before { top: 50%; }
  .last .sp::before { bottom: 50%; }
  .past .sp::before { border-left-color: var(--mark-off); }
  .ahead .sp::before { border-left-style: dashed; }
  /* The node masks the spine with the sidebar's Hull. */
  .node { position: relative; z-index: 1; display: grid; place-items: center; padding: 0.1875rem 0; background: var(--surface); color: var(--label-grey); }
  .node:focus-visible { outline: 1px solid var(--accent); outline-offset: 0; }

  .main { display: flex; flex-direction: column; justify-content: center; min-width: 0; padding: var(--sp-2) 0 var(--sp-2) var(--sp-2); }
  .nameline { display: flex; align-items: center; gap: 0.125rem; min-width: 0; }
  .name, .jn, .facts { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; min-width: 0; }
  .todo { color: var(--accent); font-weight: 600; }
  .facts { display: block; margin-top: 0.125rem; }
  .state { color: var(--text-hi); }
  .you { color: var(--text-hi); font-weight: 600; }
  .acts { position: absolute; top: 0; bottom: 0; right: 0; display: flex; align-items: center; padding-left: var(--sp-2); background: var(--surface-hi); opacity: 0; pointer-events: none; }
  .acts:empty { display: none; }
  .stop:hover > .acts, .stop:focus-within > .acts { opacity: 1; pointer-events: auto; }

  .stop > .main { min-height: var(--sp-12); }
  .stop.past .node { color: var(--mark-off); }
  .stop.past :is(.name, .todo, .facts, .state) { color: var(--grey-dim); }
  /* DESIGN.md's security colours on an inhabited stop: low Orange Mid, anarchy Red Low; the word
     is the second cue. A past stop stays dim. */
  .stop:not(.past) .sec-low { color: var(--orange-mid); }
  .stop:not(.past) .sec-anarchy { color: var(--red-low); }
  /* The next stop: where to go now, raised (Hull Raised: Hull is the frame's), its name Orange Lit
     and its note Orange. */
  .stop.next::after { background: var(--surface-hi); }
  .stop.next .name { white-space: normal; overflow-wrap: break-word; }
  .word { white-space: nowrap; }
  .stop.next .nameline { align-items: flex-start; }
  .stop.next > .main { min-height: calc(var(--sp-12) + var(--sp-8)); }
  .stop.next .node, .stop.next .name { color: var(--orange-lit); }
  .stop.next .nameline :global(.ibtn) { color: var(--orange-mid); }
  .stop.here .node.pin { color: var(--text-hi); }

  /* The leg being flown: one row a jump. */
  .jump > .main { flex-direction: row; justify-content: flex-start; align-items: center; gap: var(--sp-3); min-height: 2.25rem; padding-block: 0; }
  .jump .jn { font-weight: 400; color: var(--options-grey); }
  .jump.past .node { color: var(--mark-off); }
  .jump.past :is(.jn, .type-label) { color: var(--grey-dim); }
  .jump.here::after { background: var(--surface-hi); }
  .jump.here > .main { min-height: var(--sp-12); }
  .jump.here .node { color: var(--text-hi); }
  .jump.here .jn { color: var(--text-hi); font-weight: 600; }

  /* The other legs: the stars passed, or the jump count. */
  .leg > .main { flex-direction: row; flex-wrap: wrap; justify-content: flex-start; align-items: center; gap: var(--sp-2) var(--sp-4); min-height: 2.5rem; padding-block: var(--sp-1); }
  .stars { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2); }
  .pass { display: grid; place-items: center; padding: 0.125rem; color: var(--label-grey); }
  .pass:hover { color: var(--orange-lit); }
  .past .pass { color: var(--mark-off); }
  .past .count { color: var(--grey-dim); }
  .warn { color: var(--accent); font-weight: 500; }
  .warn b { font-weight: 700; }
  /* DESIGN.md's star colours: what you do there. Scoop in Orange Mid, boost (a neutron star) in
     White, nothing in Label Grey. Last, so they win over .node and .pass at the same weight; a
     past star (Mark Off), the next stop, where you are and a hovered star still win. */
  [data-star="star-scoopable"] { color: var(--orange-mid); }
  [data-star="star-neutron"] { color: var(--text-hi); }
</style>
