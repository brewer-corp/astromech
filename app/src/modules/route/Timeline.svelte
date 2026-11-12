<script lang="ts">
  // The route as a timeline (astromech-ok29's Design): each stop a node drawn as its star's type
  // icon, and between stops the leg to the next. The next stop is the one raised row: a larger name,
  // its copy icon and "N jumps away". The leg being flown opens out one row a jump, your row marked
  // with the map pin, when the route data lists its jumps (NavRoute once it is plotted in the
  // galaxy map, or an exact route's own list); otherwise it says its jump count and that the stars
  // appear once it is plotted. Other legs show the stars they pass where they are listed, else
  // their jump count, and warn of nowhere to scoop only when every jump is listed and known not
  // scoopable (noScoop). Inhabited stops carry a second line of details. Every stop has its copy
  // icon (the next one's goes through the core, which follows the clipboard); the others take Make
  // next on hover or focus.
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
    | { kind: "jump"; key: string; hop: RouteJump; n: number; of: number; at: At }
    | { kind: "leg"; key: string; stop: RouteStop; at: "past" | "next" | "ahead"; stars: RouteJump[] | null; unlisted: boolean; dry: boolean };

  const same = (a: string | null | undefined, b: string | null | undefined) => !!a && !!b && a.trim().toLowerCase() === b.trim().toLowerCase();
  const plural = (n: number, one: string, many = `${one}s`) => (n === 1 ? one : many);

  const route = $derived(view.route!);
  const next = $derived(route.next);
  const fromGame = $derived(route.source === GAME_SOURCE);
  const flown = $derived(view.stops[next] ?? null);
  /** How many of the flown leg's listed jumps are behind the commander (0: still in the stop before). */
  const position = $derived(flown?.hops ? view.position : null);
  /** The stop the commander is in: the one the flown leg starts from, while still there. */
  const hereStop = $derived(position === 0 || (position == null && same(view.here, view.stops[next - 1]?.system)) ? next - 1 : -1);
  /** Jumps to the next stop, where they are known. */
  const away = $derived(flown?.hops && position != null ? flown.hops.length - position : hereStop === next - 1 ? (flown?.jumps ?? null) : null);

  const rows = $derived.by(() => {
    const out: Row[] = [];
    view.stops.forEach((stop, i) => {
      const at = i < next ? "past" : i === next ? "next" : "ahead";
      if (i > 0) {
        const between = stop.hops ? stop.hops.slice(0, -1) : null;
        if (at === "next" && between) {
          between.forEach((hop, j) => {
            const where: At = position == null || j >= position ? "ahead" : j === position - 1 ? "here" : "past";
            out.push({ kind: "jump", key: `j${i}-${j}`, hop, n: j + 1, of: stop.hops!.length, at: where });
          });
        } else if (between?.length || stop.jumps == null || stop.jumps > 1) {
          const unlisted = !between && at === "next";
          out.push({ kind: "leg", key: `l${i}`, stop, at, stars: between?.length ? between : null, unlisted, dry: at !== "past" && noScoop(stop) });
        }
      }
      out.push({ kind: "stop", key: `s${i}`, i, stop, at, here: i === hereStop });
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

  function facts(stop: RouteStop): string[] {
    const d = stop.details;
    if (!d) return [];
    const rule = [d.allegiance, d.government].filter(Boolean).join(" · ");
    return [rule, d.security ? `${d.security} security` : "", `${population(d.population)} people`].filter(Boolean);
  }
</script>

<ol class="tl" aria-label="Stops and the jumps between them">
  {#each rows as row (row.key)}
    {#if row.kind === "stop"}
      {@const s = row.stop}
      {@const k = starKind(s)}
      {@const cur = row.at === "next"}
      <li class="stop {row.at}" class:first={row.i === 0} class:last={row.i === view.stops.length - 1} class:here={row.here}>
        <span class="sp">
          {#if row.here}
            <span class="node pin"><Glyph name="here" size={24} label="You are here" /></span>
          {:else}
            <span class="node" {@attach tip(starTip(s, s.system))}><Glyph name={k.glyph} size={cur ? 24 : 16} label={k.name} /></span>
          {/if}
        </span>
        <div class="main">
          {#if cur}<span class="type-label">Next stop{away != null ? ` · ${away} ${plural(away, "jump")} away` : ""}</span>{/if}
          <div class="nameline">
            <span class={cur ? "type-title name" : "type-name name"}>{s.system}</span>
            <IconButton
              icon={copied === row.i ? "copied" : "copy"}
              label={copied === row.i ? `Copied ${s.system}` : `Copy ${s.system}`}
              tip={copied === row.i ? "Copied" : "Copy to the clipboard"}
              onclick={() => void copy(row.i, s.system)}
            />
            <span class="type-label">{k.name}</span>
            {#if s.label}<span class="type-label todo">{s.label}</span>{/if}
            {#if row.here}<span class="type-label you">You are here</span>{/if}
          </div>
          {#if s.details}
            <div class="facts type-label">
              {#each facts(s) as f (f)}<span>{f}</span>{/each}
              {#if s.details.faction}<span>{s.details.faction}{#if s.details.faction_state}&nbsp;· <span class="state">{s.details.faction_state}</span>{/if}</span>{/if}
            </div>
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
      <li class="jump {row.at}" class:sc={h.scoopable === "yes"}>
        <span class="sp">
          <span class="node" class:pin={row.at === "here"} {@attach tip(starTip(h, h.system, `Jump ${row.n} of ${row.of}`))}>
            {#if row.at === "here"}<Glyph name="here" size={24} label="You are here" />{:else}<Glyph name={k.glyph} label={k.name} />{/if}
          </span>
        </span>
        <div class="main">
          {#if row.at === "here"}<span class="inl"><Glyph name={k.glyph} label={k.name} /></span>{/if}
          <span class="type-name jn">{h.system}</span>
          <span class="type-label">{k.name}</span>
          {#if row.at === "here"}<span class="type-label you">You are here</span>{/if}
        </div>
        <div></div>
      </li>
    {:else}
      {@const s = row.stop}
      {@const n = s.hops?.length ?? s.jumps}
      <li class="leg {row.at}" aria-label={n != null ? `${n} ${plural(n, "jump")}` : "Jumps not listed"}>
        <span class="sp"></span>
        <div class="main">
          {#if n != null && (!row.stars || n > 5)}<span class="type-label"><span class="type-figures count">{n}</span> {plural(n, "jump")}</span>{/if}
          {#if row.stars}
            <span class="stars">
              {#each row.stars as h, j (j)}
                {@const k = starKind(h)}
                <span class="pass" class:sc={h.scoopable === "yes"} {@attach tip(starTip(h, h.system, `Jump ${j + 1} of ${n}`))}><Glyph name={k.glyph} label={k.name} /></span>
              {/each}
            </span>
          {/if}
          {#if row.unlisted}<span class="type-label">The stars appear once the leg is plotted in the galaxy map.</span>{/if}
          {#if row.dry}<span class="type-label warn"><b>!</b> No scoopable star for {n} jumps</span>{/if}
        </div>
        <div></div>
      </li>
    {/if}
  {/each}
</ol>

<style>
  /* Three columns shared by every row (subgrid): the spine with its nodes, the stop or jump, and its
     action right after it (The Next To It Rule). */
  .tl { display: grid; grid-template-columns: 40px minmax(0, max-content) max-content; justify-content: start; column-gap: var(--sp-4); margin: 0; padding: 0; list-style: none; }
  .tl > li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; position: relative; isolation: isolate; }
  /* A row's fill, from the stop column on, so the spine stays clear. */
  .tl > li::after { content: ""; position: absolute; inset: 0 0 0 calc(40px + var(--sp-2)); z-index: -1; }
  .stop:not(.past):hover::after { background: var(--row-hover); }

  /* The spine: solid where flown, dim behind you, dashed ahead. */
  .sp { position: relative; display: grid; place-items: center; }
  .sp::before { content: ""; position: absolute; top: 0; bottom: 0; left: calc(50% - 1px); border-left: 2px solid var(--accent); }
  .first .sp::before { top: 50%; }
  .last .sp::before { bottom: 50%; }
  .past .sp::before { border-left-color: var(--orange-dim); }
  .ahead .sp::before { border-left-style: dashed; }
  .node { position: relative; z-index: 1; display: grid; place-items: center; padding: 3px; background: var(--bg); color: var(--accent); }
  .node:focus-visible { outline: 1px solid var(--accent); outline-offset: 0; }

  .main { display: flex; flex-direction: column; justify-content: center; min-width: 0; padding-block: var(--sp-2); }
  .nameline { display: flex; flex-wrap: wrap; align-items: center; gap: 2px var(--sp-2); }
  .todo { color: var(--orange-lit); font-weight: 600; }
  .facts { display: flex; flex-wrap: wrap; gap: 0 var(--sp-4); margin-top: 2px; }
  .state { color: var(--text-hi); }
  .you { color: var(--text-hi); font-weight: 600; }
  .acts { display: flex; align-items: center; padding-left: var(--sp-4); opacity: 0; }
  .stop:hover > .acts, .stop:focus-within > .acts { opacity: 1; }

  .stop > .main { min-height: var(--sp-12); }
  .stop.past .node, .stop.past .name, .stop.past .todo { color: var(--orange-dim); }
  .stop.past .facts, .stop.past .state { color: var(--orange-dim); }
  /* The next stop: the one thing to act on, raised on Hull. */
  .stop.next::after { background: var(--surface); }
  .stop.next > .main { min-height: calc(var(--sp-12) + var(--sp-8)); }
  .stop.next .node, .stop.next .name { color: var(--orange-lit); }
  .stop.next .nameline :global(.ibtn) { color: var(--accent); }
  .stop.here .node.pin { color: var(--text-hi); }

  /* The leg being flown: one row a jump. */
  .jump > .main { flex-direction: row; justify-content: flex-start; align-items: center; gap: var(--sp-3); min-height: 36px; padding-block: 0; }
  .jump .node, .jump .jn { color: var(--orange-mid); }
  .jump.sc .node { color: var(--accent); }
  .jump .jn { font-weight: 400; color: var(--accent); }
  .jump.past .node, .jump.past .jn, .jump.past .type-label { color: var(--orange-dim); }
  .jump.here::after { background: var(--surface); }
  .jump.here > .main { min-height: var(--sp-12); }
  .jump.here .node { color: var(--text-hi); }
  .jump.here .jn { color: var(--text-hi); font-weight: 600; }
  .inl { color: var(--accent); }

  /* The other legs: the stars passed, or the jump count. */
  .leg > .main { flex-direction: row; flex-wrap: wrap; justify-content: flex-start; align-items: center; gap: var(--sp-2) var(--sp-4); min-height: 40px; padding-block: var(--sp-1); }
  .stars { display: flex; flex-wrap: wrap; align-items: center; gap: var(--sp-2); }
  .pass { display: grid; place-items: center; padding: 2px; color: var(--orange-mid); }
  .pass.sc { color: var(--accent); }
  .pass:hover { color: var(--orange-lit); }
  .past .pass, .past .count { color: var(--orange-dim); }
  .warn { color: var(--accent); font-weight: 500; }
  .warn b { font-weight: 700; }
</style>
