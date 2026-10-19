<script lang="ts">
  // Where a search starts: empty means the current system; typing suggests system names
  // (Spansh autocomplete). Enter or a click picks; × goes back to the current system.
  import { enterChoice, systemNames, type Origin } from "../search";

  let { origin, onchange }: { origin: Origin; onchange: (o: Origin) => void } = $props();

  let text = $derived(origin ?? "");
  let hits = $state<string[]>([]);
  let sel = $state(0);
  let moved = false; // the commander arrowed to a suggestion
  let open = $state(false);
  let asked = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;

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

  function pick(name: string | null) {
    clearTimeout(timer);
    asked++;
    open = false;
    hits = [];
    const o = name?.trim() || null;
    text = o ?? "";
    onchange(o);
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
      pick(open ? enterChoice(text, hits, sel, moved) : text);
    } else if (e.key === "Escape") {
      open = false;
    }
  }
</script>

<div class="origin">
  <span class="lbl">From</span>
  <div class="box">
    <input
      value={text}
      placeholder="current system"
      spellcheck="false"
      autocomplete="off"
      aria-label="Search from system"
      oninput={(e) => typed(e.currentTarget.value)}
      onkeydown={key}
      onblur={() => setTimeout(() => (open = false), 150)}
    />
    {#if origin}
      <button class="clear" title="Search from the current system" onclick={() => pick(null)}>×</button>
    {/if}
    {#if open}
      <ul class="hits" role="listbox">
        {#each hits as h, i (h)}
          <li role="option" aria-selected={i === sel}>
            <button class:on={i === sel} onmousedown={(e) => e.preventDefault()} onclick={() => pick(h)}>{h}</button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .origin { display: flex; align-items: center; gap: 8px; margin: 0; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); min-width: 5.5em; }
  .box { position: relative; flex: 1; min-width: 0; display: flex; align-items: center; background: var(--cell-opt); border-bottom: 2px solid var(--accent); }
  input { all: unset; flex: 1; min-width: 0; width: 100%; height: 32px; padding: 0 10px; color: var(--text-hi); }
  input::placeholder { color: var(--text-lo); }
  button { all: unset; cursor: pointer; box-sizing: border-box; }
  .clear { padding: 0 8px; color: var(--text-lo); font-size: var(--fs); }
  .clear:hover { color: var(--accent); }
  .hits { position: absolute; z-index: 5; top: 100%; left: 0; right: 0; margin: 2px 0 0; padding: 0; list-style: none; background: var(--bg-list); border: 1px solid var(--accent-dim); }
  .hits button { display: block; width: 100%; padding: 5px 8px; color: var(--text); }
  .hits button.on, .hits button:hover { background: var(--cell); color: var(--text-hi); }
</style>
