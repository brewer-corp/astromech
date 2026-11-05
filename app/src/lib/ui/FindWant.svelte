<script lang="ts">
  // Type a service, module or ship; Enter or a click opens that lookup. The three Galaxy
  // pages share this box so the commander does not have to pick a kind first.
  import Input from "./Input.svelte";
  import Suggestions from "./Suggestions.svelte";
  import { onDestroy } from "svelte";
  import { suggestFind } from "../find";
  import type { PaletteItem } from "../palette";

  let { current }: { current: string } = $props();

  let text = $derived(current);
  let hits = $state<PaletteItem[]>([]);
  let sel = $state(0);
  let open = $state(false);
  let blurTimer: ReturnType<typeof setTimeout> | null = null;

  function typed(v: string) {
    text = v;
    sel = 0;
    hits = suggestFind(v);
    open = hits.length > 0;
  }

  function close() {
    open = false;
    hits = [];
    text = current;
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
      if (open && hits[sel]) go(hits[sel]);
      else if (text.trim() && text.trim() !== current) go(suggestFind(text, 1)[0]);
      else close();
    } else if (e.key === "Escape") {
      close();
    }
  }

  function blur() {
    blurTimer = setTimeout(close, 150);
  }

  function focus(e: FocusEvent & { currentTarget: HTMLInputElement }) {
    if (blurTimer) {
      clearTimeout(blurTimer);
      blurTimer = null;
    }
    e.currentTarget.select();
  }

  onDestroy(() => {
    if (blurTimer) clearTimeout(blurTimer);
  });
</script>

<div class="pick">
  <span class="lbl">Find</span>
  <Input
    class="box"
    value={text}
    placeholder="Service, module or ship"
    spellcheck="false"
    autocomplete="off"
    role="combobox"
    aria-label="Find"
    aria-expanded={open}
    aria-controls={open ? "find-hits" : undefined}
    aria-activedescendant={open ? `find-hits-${sel}` : undefined}
    aria-autocomplete="list"
    oninput={(e) => typed(e.currentTarget.value)}
    onkeydown={key}
    onfocus={focus}
    onblur={blur}
  >
    {#if open}
      <Suggestions id="find-hits" items={hits} {sel} key={(h) => `${h.detail}:${h.route}`} onpick={go}>
        {#snippet item(h)}{h.label} <span class="sub">{h.detail}</span>{/snippet}
      </Suggestions>
    {/if}
  </Input>
</div>

<style>
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; font-size: var(--fs-s); }
  .pick :global(.box) { flex: 1; }
</style>
