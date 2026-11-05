<script lang="ts">
  // Where a search starts: empty means the current system; typing suggests system names
  // (Spansh autocomplete). Enter or a click picks; × goes back to the current system.
  // `around` (a Places finder, DESIGN.md mockup): labelled "Around", and focusing the box also
  // offers the route's next stop ("Next stop: Deciat") and, once another system is picked, Here.
  import { getRoute } from "../route";
  import { enterChoice, systemNames, type Origin } from "../search";
  import IconButton from "./IconButton.svelte";
  import Input from "./Input.svelte";
  import Suggestions from "./Suggestions.svelte";

  let { origin, onchange, around = false }: { origin: Origin; onchange: (o: Origin) => void; around?: boolean } = $props();

  /** A suggestion: a system to search from (null: here), as shown. */
  type Hit = { system: string | null; label: string };

  /** The origin was picked as the route's next stop: the box says so. */
  let viaNext = $state<string | null>(null);
  const shown = $derived(origin == null ? "" : viaNext === origin ? `Next stop: ${origin}` : origin);
  let text = $derived(shown);
  let hits = $state<Hit[]>([]);
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
          hits = h.map((s) => ({ system: s, label: s }));
          open = h.length > 0;
        })
        .catch(() => {
          if (n === asked) hits = [];
        });
    }, 250);
  }

  /** Around: what the box offers before anything is typed: Here (when searching elsewhere) and
   * the route's next stop (read now, so it is the current one). */
  async function offer() {
    if (!around) return;
    const n = ++asked;
    let next: string | null = null;
    try {
      next = (await getRoute()).next?.system ?? null;
    } catch {
      // no route to offer; a typed system still works
    }
    if (n !== asked || text !== shown) return;
    const fixed: Hit[] = [];
    if (origin != null) fixed.push({ system: null, label: "Here" });
    if (next && !(viaNext === origin && origin === next)) fixed.push({ system: next, label: `Next stop: ${next}` });
    hits = fixed;
    sel = 0;
    moved = false;
    open = fixed.length > 0;
  }

  function pick(h: Hit | null) {
    clearTimeout(timer);
    asked++;
    open = false;
    hits = [];
    const o = h?.system?.trim() || null;
    viaNext = o && h?.label.startsWith("Next stop: ") ? o : null;
    text = o == null ? "" : viaNext ? `Next stop: ${o}` : o;
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
      if (!open && text === shown) return; // nothing changed
      const said = open ? enterChoice(text, hits.map((h) => h.label), sel, moved) : text;
      pick(hits.find((h) => h.label === said) ?? { system: said, label: said });
    } else if (e.key === "Escape") {
      open = false;
    }
  }
</script>

<div class="origin">
  <span class="lbl">{around ? "Around" : "From"}</span>
  <Input
    class="box"
    value={text}
    placeholder={around ? "Here" : "Current system"}
    spellcheck="false"
    autocomplete="off"
    aria-label="Search from system"
    oninput={(e) => typed(e.currentTarget.value)}
    onkeydown={key}
    onfocus={() => void offer()}
    onblur={() => setTimeout(() => (open = false), 150)}
  >
    {#snippet end()}
      {#if origin}<IconButton icon="clear" label="Search from the current system" onclick={() => pick(null)} />{/if}
    {/snippet}
    {#if open}
      <Suggestions items={hits} {sel} key={(h) => h.label} onpick={pick}>
        {#snippet item(h)}{h.label}{/snippet}
      </Suggestions>
    {/if}
  </Input>
</div>

<style>
  .origin { display: flex; align-items: center; gap: 8px; margin: 0; font-size: var(--fs); }
  .lbl { color: var(--text-lo); text-transform: uppercase; letter-spacing: var(--caps); font-size: var(--fs-s); min-width: 5.5em; }
  .origin :global(.box) { flex: 1; }
</style>
