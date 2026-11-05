<script lang="ts" module>
  let focusBox: (() => void) | null = null;

  /** Put the cursor in the header's Find or plot box (the route strip's shortcut). */
  export function focusFindOrPlot(): void {
    focusBox?.();
  }
</script>

<script lang="ts">
  // The header's Find or plot box, on every page (DESIGN.md Find or plot, decision astromech-bi94):
  // type a system, a finder phrase ("raw trader", "outfitting fsd"), a page, a material, a module
  // or a ship. The popover ranks them together (lib/find.ts): pages and finders open; a system
  // has Plot, Add stop and Set destination. Plot (or Enter) opens a panel under the box that plots
  // a route from the current system (Plot.svelte, docs/PLOTTING.md) or opens the system on Spansh
  // or Inara; Enter again plots. Ctrl+K puts the cursor here from any page, but not while a dialog
  // (a bind capture) has the keyboard. Escape closes the popover, then the panel, then leaves the
  // box. The entries load each time the box is focused; the systems come from Spansh as you type.
  import { find, type Match } from "../../lib/find";
  import type { PaletteItem } from "../../lib/palette";
  import { addedNote, addStop, failedNote, followedNote, followRoute, type FollowNote } from "../../lib/route";
  import { systemNames } from "../../lib/search";
  import Button from "../../lib/ui/Button.svelte";
  import ExtLinks from "../../lib/ui/ExtLinks.svelte";
  import FollowNoteLine from "../../lib/ui/FollowNote.svelte";
  import IconButton from "../../lib/ui/IconButton.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import Plot from "./Plot.svelte";

  let {
    load = () => Promise.resolve([]),
    onpick = (route: string) => (location.hash = `#/${route}`),
  }: {
    /** The entries besides systems (lib/palette.ts `collect`). */
    load?: () => Promise<PaletteItem[]>;
    /** Open an entry's route (without "#/"). */
    onpick?: (route: string) => void;
  } = $props();

  let text = $state("");
  let items = $state<PaletteItem[] | null>(null);
  let systems = $state<string[]>([]);
  /** Asking Spansh for systems, or it failed. */
  let asking = $state(false);
  /** The highlight was moved with the arrows since the last keystroke. */
  let moved = false;
  let failed = $state(false);
  let sel = $state(0);
  let open = $state(false);
  /** Under the box: the plot panel for a system, or what Add stop or Set destination did. */
  let panel = $state<{ plot: string } | { note: FollowNote | null } | null>(null);
  let asked = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;
  let box = $state<HTMLElement>();
  let plot = $state<{ go: () => Promise<void>; plotting: () => boolean }>();
  let input = $state<HTMLInputElement | HTMLTextAreaElement>();
  /** What had the keyboard before Ctrl+K, for the last Escape. */
  let before: Element | null = null;

  const shown = $derived(open ? find(items ?? [], systems, text) : []);
  const picked = $derived(panel && "plot" in panel ? panel.plot : null);

  $effect(() => {
    const el = input;
    focusBox = () => el?.focus();
    return () => (focusBox = null);
  });

  function loadItems() {
    void load()
      .then((v) => (items = v))
      .catch(() => (items ??= []));
  }

  function typed(v: string) {
    text = v;
    sel = 0;
    moved = false;
    open = v.trim().length > 0;
    clearTimeout(timer);
    const q = v.trim();
    const n = ++asked;
    failed = false;
    if (q.length < 2) {
      systems = [];
      asking = false;
      return;
    }
    asking = true;
    timer = setTimeout(() => {
      systemNames(q)
        .then((h) => {
          if (n === asked) systems = h;
        })
        .catch(() => {
          if (n !== asked) return;
          systems = [];
          failed = true;
        })
        .finally(() => {
          if (n === asked) asking = false;
        });
    }, 250);
  }

  /** Close the popover and the panel, unless a plot is running there (its result would be lost). */
  function close() {
    open = false;
    if (!plot?.plotting()) panel = null;
  }

  function clear() {
    clearTimeout(timer);
    asked++;
    text = "";
    systems = [];
    asking = false;
    failed = false;
    open = false;
  }

  /** Enter or a click on a row: an entry opens its page; a system opens the plot panel. */
  function choose(m: Match) {
    if (m.kind === "system") return plotTo(m.name);
    clear();
    panel = null;
    input?.blur();
    onpick(m.item.route);
  }

  function plotTo(name: string) {
    const p = name.trim();
    if (!p) return;
    clearTimeout(timer);
    asked++;
    asking = false;
    open = false;
    text = p;
    panel = { plot: p };
  }

  /** Add stop: the system becomes the next stop of the route followed. */
  async function add(name: string) {
    open = false;
    try {
      panel = { note: addedNote(await addStop({ system: name }, "search")) };
    } catch (e) {
      panel = { note: failedNote("Couldn't add it", e, () => void add(name)) };
    }
  }

  /** Set destination: a one-stop route to the system; the route it replaces is parked. */
  async function destination(name: string) {
    open = false;
    try {
      panel = { note: followedNote(await followRoute(name, "search", [{ system: name }])) };
    } catch (e) {
      panel = { note: failedNote("Couldn't follow it", e, () => void destination(name)) };
    }
  }

  function key(e: KeyboardEvent) {
    const n = shown.length;
    const row = shown[sel];
    if (e.key === "ArrowDown" && !open && text.trim()) {
      open = true;
      sel = 0;
    } else if (e.key === "ArrowDown" && n) {
      sel = (sel + 1) % n;
      moved = true;
    } else if (e.key === "ArrowUp" && n) {
      sel = (sel - 1 + n) % n;
      moved = true;
    } else if (e.key === "Enter" && !e.isComposing) {
      // The plot panel is open for what is typed: Enter plots. Otherwise it takes the highlighted
      // row, or, with nothing listed, plots to the text as a system.
      // While Spansh hasn't answered, a system typed or pasted and entered at once plots: a looser
      // entry match ("Frontline Solutions" for "Sol") doesn't take it unless the highlight was
      // moved to it or the match is sure (whole label, its start, a finder phrase; astromech-gasg).
      if (!open && picked && text.trim() === picked) void plot?.go();
      else if (open && row && asking && !moved && !(row.kind === "entry" && row.sure)) plotTo(text);
      else if (open && row) choose(row);
      else plotTo(text);
    } else if (e.key === "Escape") {
      if (open) open = false;
      else if (panel) close();
      else {
        input?.blur();
        if (before instanceof HTMLElement && before.isConnected) before.focus();
      }
    } else return;
    e.preventDefault();
  }

  function globalKey(e: KeyboardEvent) {
    if (!(e.ctrlKey || e.metaKey) || e.altKey || e.shiftKey || e.key.toLowerCase() !== "k") return;
    if (document.querySelector('[aria-modal="true"]')) return;
    e.preventDefault();
    if (document.activeElement !== input) before = document.activeElement;
    input?.focus();
    input?.select();
  }

  // A click or focus outside, or moving to another page, closes the popover and panel (not mid-plot).
  $effect(() => {
    const outside = (e: Event) => {
      if (box && !box.contains(e.target as Node)) close();
    };
    window.addEventListener("pointerdown", outside);
    window.addEventListener("focusin", outside);
    window.addEventListener("hashchange", close);
    return () => {
      window.removeEventListener("pointerdown", outside);
      window.removeEventListener("focusin", outside);
      window.removeEventListener("hashchange", close);
    };
  });

  const keep = (e: MouseEvent) => e.preventDefault(); // a click in the popover leaves the cursor in the box
</script>

<svelte:window onkeydown={globalKey} />

<div class="find" bind:this={box}>
  <Input
    bind:input
    value={text}
    placeholder="Find or plot"
    spellcheck="false"
    autocomplete="off"
    role="combobox"
    aria-label="Find or plot"
    aria-autocomplete="list"
    aria-expanded={open && shown.length > 0}
    aria-controls="find-list"
    aria-activedescendant={open && shown.length ? `find-${sel}` : undefined}
    oninput={(e) => typed(e.currentTarget.value)}
    onkeydown={key}
    onfocus={() => {
      loadItems();
      open = text.trim().length > 0 && !picked;
    }}
  >
    {#snippet end()}
      {#if text}
        <IconButton icon="clear" label="Clear the search" tip="Clear" onclick={() => { clear(); close(); input?.focus(); }} />
      {:else}
        <span class="hint">Ctrl+K</span>
      {/if}
    {/snippet}
    {#if open}
      <div class="pop">
        {#if shown.length}
          <ul id="find-list" role="listbox" aria-label="Matches">
            {#each shown as m, i (i)}
              <li id="find-{i}" role="option" aria-selected={i === sel} class:on={i === sel}>
                {#if m.kind === "entry"}
                  <button type="button" class="pick" tabindex="-1" onmousedown={keep} onmousemove={() => (sel = i)} onclick={() => choose(m)}>
                    <span class="l">{m.item.label}</span><span class="sub">{m.item.detail}</span>
                  </button>
                {:else}
                  <button type="button" class="pick" tabindex="-1" onmousedown={keep} onmousemove={() => (sel = i)} onclick={() => choose(m)}>
                    <span class="l">{m.name}</span><span class="sub">System</span>
                  </button>
                  <span class="acts">
                    <Button onmousedown={keep} onclick={() => plotTo(m.name)} aria-label={`Plot a route to ${m.name}`}>Plot</Button>
                    <IconButton icon="add-stop" onmousedown={keep} label={`Add ${m.name} as a stop`} tip={`Add ${m.name} as the next stop`} onclick={() => void add(m.name)} />
                    <IconButton
                      icon="destination"
                      onmousedown={keep}
                      label={`Set ${m.name} as the destination`}
                      tip={`Set ${m.name} as the destination (replaces the route, which Resume brings back)`}
                      onclick={() => void destination(m.name)}
                    />
                  </span>
                {/if}
              </li>
            {/each}
          </ul>
        {:else}
          <p class="line">{asking ? "Asking Spansh…" : items === null ? "Loading…" : "Nothing matches. Type a system, page, finder, material, module or ship."}</p>
        {/if}
        {#if failed}<p class="line">Spansh didn't answer, so no systems are listed.</p>{/if}
      </div>
    {/if}
  </Input>
  {#if !open && panel}
    <div class="panel" role="dialog" aria-label={picked ? `Plot to ${picked}` : "Route"}>
      {#if "plot" in panel}
        <Plot bind:this={plot} to={panel.plot} />
        <div class="links"><ExtLinks system={panel.plot} /></div>
      {:else}
        <FollowNoteLine bind:note={panel.note} />
      {/if}
    </div>
  {/if}
</div>

<style>
  /* DESIGN.md's header box is 320 px; it gives way first when the header is short of room. */
  .find { position: relative; flex: 0 1 320px; min-width: 12rem; font-size: var(--fs); }
  .hint { color: var(--accent); white-space: nowrap; }
  /* DESIGN.md's popover (the open dropdown list): Hull Raised, a Seam border, 34 px rows. */
  .pop { position: absolute; z-index: 20; top: 100%; right: 0; margin: var(--gap) 0 0; width: max(100%, 28rem); background: var(--surface-hi); border: 1px solid var(--line); }
  ul { margin: 0; padding: 0; list-style: none; }
  li { display: flex; align-items: center; min-height: var(--row); color: var(--accent); }
  li:hover { background: var(--row-hover); }
  li.on { background: var(--accent); color: var(--on-accent); }
  .pick { all: unset; box-sizing: border-box; cursor: pointer; flex: 1; min-width: 0; display: flex; align-items: baseline; gap: var(--sp-2); padding: 0 var(--sp-3); line-height: var(--row); }
  .l { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .sub { flex: none; font-size: var(--fs-s); color: var(--orange-mid); }
  li.on .sub, li.on .acts :global(.btn), li.on .acts :global(.ibtn) { color: var(--on-accent); }
  li.on .acts :global(.btn:hover) { background: none; }
  .acts { display: flex; align-items: center; flex: none; padding-right: var(--sp-1); }
  .line { margin: 0; padding: 0 var(--sp-3); line-height: var(--row); font-size: var(--fs-s); color: var(--orange-mid); }
  .panel { position: absolute; z-index: 20; top: 100%; right: 0; margin: var(--gap) 0 0; border: 1px solid var(--line); width: min(28rem, calc(100vw - 32px)); padding: 16px; background: var(--surface-hi); max-height: calc(100vh - 120px); overflow: auto; }
  .panel > :global(.note) { margin: 0; }
  .links { margin-top: 10px; padding-top: 10px; border-top: 1px solid var(--line); }
</style>
