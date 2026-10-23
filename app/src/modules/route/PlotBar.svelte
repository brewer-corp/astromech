<script lang="ts">
  // The top bar's system search, on every page (docs/PLOTTING.md): type a system (Spansh
  // autocomplete), pick it with Enter or a click, and a panel under the bar plots a route to it
  // from the current system (Plot) or opens it on Spansh or Inara. Enter again plots. Escape or
  // a click outside closes the panel; the route itself is on the Route page and the Galaxy rail.
  import { systemNames } from "../../lib/search";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import Plot from "./Plot.svelte";

  let text = $state("");
  let hits = $state<string[]>([]);
  let sel = $state(0);
  let moved = false; // the commander arrowed to a suggestion
  let open = $state(false);
  /** The system the panel plots to. */
  let picked = $state<string | null>(null);
  let asked = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let box = $state<HTMLElement>();
  let plot = $state<{ go: () => Promise<void>; plotting: () => boolean }>();

  function typed(v: string) {
    text = v;
    sel = 0;
    moved = false;
    clearTimeout(timer);
    const q = v.trim();
    if (q.length < 2) {
      hits = [];
      open = false;
      return;
    }
    const n = ++asked;
    timer = setTimeout(() => {
      systemNames(q)
        .then((h) => {
          if (n !== asked) return;
          hits = h;
          open = h.length > 0;
        })
        .catch(() => {
          if (n === asked) hits = [];
        });
    }, 250);
  }

  function pick(name: string) {
    clearTimeout(timer);
    asked++;
    open = false;
    hits = [];
    const p = name.trim();
    if (!p) return;
    text = p;
    picked = p;
  }

  /** What Enter takes from the suggestions: one arrowed to, else one equal to the text, else the
   * first (highlighted). */
  function suggestion(): string {
    if (moved) return hits[sel] ?? text;
    const t = text.trim().toLowerCase();
    return hits.find((h) => h.toLowerCase() === t) ?? hits[sel] ?? text;
  }

  /** Close the panel, unless a plot is running there (its result or error would be lost). */
  function close() {
    open = false;
    if (!plot?.plotting()) picked = null;
  }

  function key(e: KeyboardEvent) {
    if (e.key === "ArrowDown" && hits.length) {
      e.preventDefault();
      open = true;
      moved = true;
      sel = (sel + 1) % hits.length;
    } else if (e.key === "ArrowUp" && hits.length) {
      e.preventDefault();
      moved = true;
      sel = (sel - 1 + hits.length) % hits.length;
    } else if (e.key === "Enter") {
      e.preventDefault();
      // The system is picked and the text unchanged: Enter plots. With suggestions showing, it
      // takes the highlighted one unless one is exactly what was typed (a partial name is no
      // system to plot to).
      if (!open && picked && text.trim() === picked) void plot?.go();
      else pick(open ? suggestion() : text);
    } else if (e.key === "Escape") {
      if (open) open = false;
      else close();
    }
  }

  // A click outside, or moving to another page, closes the panel (not mid-plot).
  $effect(() => {
    const down = (e: PointerEvent) => {
      if (box && !box.contains(e.target as Node)) close();
    };
    window.addEventListener("pointerdown", down);
    window.addEventListener("hashchange", close);
    return () => {
      window.removeEventListener("pointerdown", down);
      window.removeEventListener("hashchange", close);
    };
  });
</script>

<div class="plotbar" bind:this={box}>
  <div class="field">
    <input
      value={text}
      placeholder="Plot to a system"
      spellcheck="false"
      autocomplete="off"
      aria-label="Plot to a system"
      oninput={(e) => typed(e.currentTarget.value)}
      onkeydown={key}
      onfocus={() => (open = hits.length > 0)}
      onblur={() => setTimeout(() => (open = false), 150)}
    />
    {#if text}
      <button class="clear" title="Clear" aria-label="Clear the search" onclick={() => { typed(""); close(); }}>×</button>
    {/if}
  </div>
  {#if open}
    <ul class="hits" role="listbox">
      {#each hits as h, i (h)}
        <li role="option" aria-selected={i === sel}>
          <button class:on={i === sel} onmousedown={(e) => e.preventDefault()} onclick={() => pick(h)}>{h}</button>
        </li>
      {/each}
    </ul>
  {:else if picked}
    <div class="panel" role="dialog" aria-label={`Plot to ${picked}`}>
      <Plot bind:this={plot} to={picked} />
      <div class="links"><ExtLinks system={picked} /></div>
    </div>
  {/if}
</div>

<style>
  .plotbar { position: relative; flex: 0 1 22rem; min-width: 12rem; font-size: var(--fs); }
  .field { display: flex; align-items: center; background: var(--cell-opt); border-bottom: 2px solid var(--accent); }
  input { all: unset; flex: 1; min-width: 0; height: 32px; padding: 0 10px; color: var(--text-hi); text-transform: none; letter-spacing: normal; }
  input::placeholder { color: var(--text-lo); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .clear { padding: 0 10px; color: var(--text-lo); }
  .clear:hover { color: var(--accent); }
  .hits, .panel { position: absolute; z-index: 20; top: 100%; right: 0; margin: 2px 0 0; background: var(--bg-list); border: 1px solid var(--accent-dim); }
  .hits { left: 0; padding: 0; list-style: none; }
  .hits button { display: block; width: 100%; padding: 6px 10px; color: var(--text); }
  .hits button.on, .hits button:hover { background: var(--cell); color: var(--text-hi); }
  .panel { width: min(28rem, calc(100vw - 32px)); padding: 16px; background: var(--bg); border-top: 2px solid var(--accent); box-shadow: 0 12px 40px rgb(0 0 0 / 0.6); max-height: calc(100vh - 120px); overflow: auto; }
  .links { margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--accent-dim); }
</style>
