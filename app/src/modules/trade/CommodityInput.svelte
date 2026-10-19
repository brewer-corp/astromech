<script lang="ts">
  // Pick a market commodity by typing part of its name (or an alias such as "LTDs"). Enter or a
  // click picks; an unknown name keeps the previous pick.
  import { findCommodity, suggestCommodities, type Commodity } from "../../lib/trade";

  let { value, onchange }: { value: Commodity; onchange: (c: Commodity) => void } = $props();

  let text = $derived(value.name);
  let hits = $state<Commodity[]>([]);
  let sel = $state(0);
  let open = $state(false);

  function typed(v: string) {
    text = v;
    sel = 0;
    hits = suggestCommodities(v);
    open = hits.length > 0;
  }

  function pick(c: Commodity | undefined) {
    open = false;
    hits = [];
    if (c) onchange(c);
    text = (c ?? value).name;
  }

  function key(e: KeyboardEvent) {
    if (e.key === "ArrowDown" && hits.length) {
      e.preventDefault();
      open = true;
      sel = (sel + 1) % hits.length;
    } else if (e.key === "ArrowUp" && hits.length) {
      e.preventDefault();
      sel = (sel - 1 + hits.length) % hits.length;
    } else if (e.key === "Enter") {
      e.preventDefault();
      pick((open && hits[sel]) || findCommodity(text));
    } else if (e.key === "Escape") {
      pick(undefined);
    }
  }
</script>

<div class="pick">
  <span class="lbl">Commodity</span>
  <div class="box">
    <input
      value={text}
      spellcheck="false"
      autocomplete="off"
      aria-label="Commodity"
      oninput={(e) => typed(e.currentTarget.value)}
      onkeydown={key}
      onfocus={(e) => e.currentTarget.select()}
      onblur={() => setTimeout(() => pick(open ? undefined : findCommodity(text)), 150)}
    />
    {#if open}
      <ul class="hits" role="listbox">
        {#each hits as h, i (h.symbol)}
          <li role="option" aria-selected={i === sel}>
            <button class:on={i === sel} onmousedown={(e) => e.preventDefault()} onclick={() => pick(h)}>
              {h.name} <span class="cat">{h.category}</span>
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; font-size: var(--fs-s); }
  .box { position: relative; flex: 1; min-width: 0; display: flex; align-items: center; background: var(--cell-opt); border-bottom: 2px solid var(--accent); }
  input { all: unset; flex: 1; min-width: 0; width: 100%; height: 28px; padding: 0 8px; color: var(--text-hi); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .hits { position: absolute; z-index: 5; top: 100%; left: 0; right: 0; margin: 2px 0 0; padding: 0; list-style: none; background: var(--bg-list); border: 1px solid var(--accent-dim); }
  .hits button { display: block; width: 100%; padding: 5px 8px; color: var(--text); }
  .hits button.on, .hits button:hover { background: var(--cell); color: var(--text-hi); }
  .cat { color: var(--text-lo); margin-left: 6px; }
</style>
