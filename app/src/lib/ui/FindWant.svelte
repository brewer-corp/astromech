<script lang="ts">
  // Type a service, module or ship; Enter or a click opens that lookup. The three Galaxy
  // pages share this box so the commander does not have to pick a kind first.
  import { suggestFind } from "../find";
  import type { PaletteItem } from "../palette";

  let { current }: { current: string } = $props();

  let text = $derived(current);
  let hits = $state<PaletteItem[]>([]);
  let sel = $state(0);
  let open = $state(false);

  function typed(v: string) {
    text = v;
    sel = 0;
    hits = suggestFind(v);
    open = hits.length > 0;
  }

  function go(item: PaletteItem | undefined) {
    open = false;
    hits = [];
    if (item) location.hash = `#/${item.route}`;
    text = item?.label ?? current;
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
      go(open ? hits[sel] : suggestFind(text, 1)[0]);
    } else if (e.key === "Escape") {
      go(undefined);
    }
  }
</script>

<div class="pick">
  <span class="lbl">Find</span>
  <div class="box">
    <input
      value={text}
      placeholder="Service, module or ship"
      spellcheck="false"
      autocomplete="off"
      role="combobox"
      aria-label="Find"
      aria-expanded={open}
      aria-controls="find-hits"
      aria-autocomplete="list"
      oninput={(e) => typed(e.currentTarget.value)}
      onkeydown={key}
      onfocus={(e) => e.currentTarget.select()}
      onblur={() => setTimeout(() => go(open ? undefined : suggestFind(text, 1)[0]), 150)}
    />
    {#if open}
      <ul id="find-hits" class="hits" role="listbox">
        {#each hits as h, i (`${h.detail}:${h.route}`)}
          <li role="option" aria-selected={i === sel}>
            <button class:on={i === sel} onmousedown={(e) => e.preventDefault()} onclick={() => go(h)}>
              {h.label} <span class="cat">{h.detail}</span>
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
  input { all: unset; flex: 1; min-width: 0; width: 100%; height: 32px; padding: 0 10px; color: var(--text-hi); }
  input::placeholder { color: var(--text-lo); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .hits { position: absolute; z-index: 5; top: 100%; left: 0; right: 0; margin: 2px 0 0; padding: 0; list-style: none; background: var(--bg-list); border: 1px solid var(--accent-dim); }
  .hits button { display: block; width: 100%; padding: 5px 8px; color: var(--text); }
  .hits button.on, .hits button:hover { background: var(--cell); color: var(--text-hi); }
  .cat { color: var(--text-lo); margin-left: 6px; }
</style>
