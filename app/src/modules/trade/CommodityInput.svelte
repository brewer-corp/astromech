<script lang="ts">
  // Pick a market commodity by typing part of its name (or an alias such as "LTDs"). Enter or a
  // click picks; an unknown name keeps the previous pick.
  import Input from "../../lib/ui/Input.svelte";
  import Suggestions from "../../lib/ui/Suggestions.svelte";
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
  <Input
    class="box"
    value={text}
    spellcheck="false"
    autocomplete="off"
    aria-label="Commodity"
    oninput={(e) => typed(e.currentTarget.value)}
    onkeydown={key}
    onfocus={(e) => e.currentTarget.select()}
    onblur={() => setTimeout(() => pick(open ? undefined : findCommodity(text)), 150)}
  >
    {#if open}
      <Suggestions items={hits} {sel} key={(h) => h.symbol} onpick={pick}>
        {#snippet item(h)}{h.name} <span class="sub">{h.category}</span>{/snippet}
      </Suggestions>
    {/if}
  </Input>
</div>

<style>
  .pick { display: flex; align-items: center; gap: 8px; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); min-width: 5.5em; font-size: var(--fs-s); }
  .pick :global(.box) { flex: 1; }
</style>
