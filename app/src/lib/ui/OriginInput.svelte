<script lang="ts">
  // Where a search starts: empty means the current system; typing suggests system names
  // (Spansh autocomplete). Enter or a click picks; × goes back to the current system.
  import { enterChoice, systemNames, type Origin } from "../search";
  import IconButton from "./IconButton.svelte";
  import Input from "./Input.svelte";
  import Suggestions from "./Suggestions.svelte";

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
  <Input
    class="box"
    value={text}
    placeholder="Current system"
    spellcheck="false"
    autocomplete="off"
    aria-label="Search from system"
    oninput={(e) => typed(e.currentTarget.value)}
    onkeydown={key}
    onblur={() => setTimeout(() => (open = false), 150)}
  >
    {#snippet end()}
      {#if origin}<IconButton icon="clear" label="Search from the current system" onclick={() => pick(null)} />{/if}
    {/snippet}
    {#if open}
      <Suggestions items={hits} {sel} key={(h) => h} onpick={pick}>
        {#snippet item(h)}{h}{/snippet}
      </Suggestions>
    {/if}
  </Input>
</div>

<style>
  .origin { display: flex; align-items: center; gap: 8px; margin: 0; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); min-width: 5.5em; }
  .origin :global(.box) { flex: 1; }
</style>
