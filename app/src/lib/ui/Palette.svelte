<script lang="ts">
  // The Ctrl+K command palette (docs/ARCHITECTURE.md §18.2): type a page, material, module or
  // blueprint; arrows and Enter pick, Esc closes. Works from any page, but not while another
  // modal (a bind capture) has the keyboard. The entries load each time it opens.
  import { tick } from "svelte";
  import { rank, type PaletteItem } from "../palette";

  let { load, onpick }: { load: () => Promise<PaletteItem[]>; onpick: (route: string) => void } = $props();

  let open = $state(false);
  let query = $state("");
  let items = $state<PaletteItem[] | null>(null);
  let active = $state(0);
  let input: HTMLInputElement | undefined = $state();
  let before: Element | null = null;
  const shown = $derived(items ? rank(items, query) : []);

  async function show() {
    before = document.activeElement;
    open = true;
    query = "";
    active = 0;
    items = null;
    void load().then((v) => {
      if (open) items = v;
    });
    await tick();
    input?.focus();
  }

  function close() {
    open = false;
    if (before instanceof HTMLElement) before.focus();
  }

  function pick(item: PaletteItem) {
    open = false;
    onpick(item.route);
  }

  function globalKey(e: KeyboardEvent) {
    if (!(e.ctrlKey || e.metaKey) || e.altKey || e.shiftKey || e.key.toLowerCase() !== "k") return;
    if (!open && document.querySelector('[aria-modal="true"]')) return;
    e.preventDefault();
    if (open) close();
    else void show();
  }

  function key(e: KeyboardEvent) {
    const it = shown[active];
    if (e.key === "ArrowDown") active = Math.min(active + 1, shown.length - 1);
    else if (e.key === "ArrowUp") active = Math.max(active - 1, 0);
    else if (e.key === "Enter" && it) pick(it);
    else if (e.key === "Escape") close();
    else return;
    e.preventDefault();
  }
</script>

<svelte:window onkeydown={globalKey} />

{#if open}
  <div class="shade" role="presentation" onmousedown={(e) => e.target === e.currentTarget && close()}>
    <div class="box" role="dialog" aria-modal="true" aria-label="Command palette">
      <input
        bind:this={input}
        bind:value={query}
        oninput={() => (active = 0)}
        onkeydown={key}
        placeholder="Go to a page, material, ship, module or blueprint…"
        role="combobox"
        aria-label="Go to"
        aria-expanded="true"
        aria-controls="palette-list"
        aria-activedescendant={shown.length ? `palette-${active}` : undefined}
        spellcheck="false"
        autocomplete="off"
      />
      <ul id="palette-list" role="listbox" aria-label="Matches">
        {#each shown as it, i (i)}
          <li id="palette-{i}" role="option" aria-selected={i === active}>
            <button tabindex="-1" class:on={i === active} onmousemove={() => (active = i)} onclick={() => pick(it)}>
              <span class="l">{it.label}</span><span class="d">{it.detail}</span>
            </button>
          </li>
        {:else}
          <li class="none">{items ? "Nothing matches." : "Loading…"}</li>
        {/each}
      </ul>
    </div>
  </div>
{/if}

<style>
  .shade { position: fixed; inset: 0; z-index: 50; background: rgb(0 0 0 / 0.7); display: flex; justify-content: center; align-items: flex-start; padding-top: 12vh; }
  .box { width: min(640px, calc(100vw - 80px)); background: var(--bg); border-top: 2px solid var(--accent); box-shadow: 0 12px 40px rgb(0 0 0 / 0.6); }
  input { box-sizing: border-box; width: 100%; padding: 14px 18px; border: 0; border-bottom: 1px solid var(--accent-dim); background: transparent; color: var(--text-hi); font: inherit; font-size: var(--fs-l); outline: none; }
  input::placeholder { color: var(--text-lo); }
  ul { list-style: none; margin: 0; padding: 6px 0; max-height: 60vh; overflow: auto; }
  button { all: unset; box-sizing: border-box; width: 100%; display: flex; justify-content: space-between; gap: 16px; padding: 8px 18px; cursor: pointer; color: var(--text); }
  button.on { background: var(--row-hover); color: var(--text-hi); box-shadow: inset 2px 0 0 var(--accent); }
  .d { color: var(--text-lo); font-size: var(--fs-s); white-space: nowrap; }
  .none { padding: 8px 18px; color: var(--text-lo); }
</style>
