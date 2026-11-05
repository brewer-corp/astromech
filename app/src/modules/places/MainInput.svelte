<script lang="ts">
  // A finder's typed main input (a module, a ship): the current one in the cell; typing suggests
  // from the finder's own list, and Enter or a click picks one. Escape, or leaving the box
  // without picking, puts the current one back.
  import Input from "../../lib/ui/Input.svelte";
  import Suggestions from "../../lib/ui/Suggestions.svelte";
  import { onDestroy } from "svelte";
  import { rank, type PaletteItem } from "../../lib/palette";

  let {
    label,
    current,
    items,
    placeholder,
    onpick,
  }: {
    /** The input's accessible name: "Module". */
    label: string;
    /** The name of what the finder shows now. */
    current: string;
    /** What can be picked: `label` is the value passed to `onpick`, `detail` an aside, `keywords`
     * other names it is found by ("LTDs"). */
    items: { label: string; detail?: string; keywords?: string }[];
    placeholder: string;
    onpick: (label: string) => void;
  } = $props();

  const id = $derived(`main-${label.toLowerCase()}`);
  const listed = $derived<PaletteItem[]>(items.map((i) => ({ label: i.label, detail: i.detail ?? "", route: i.label, keywords: i.keywords })));
  let text = $derived(current);
  let hits = $state<PaletteItem[]>([]);
  let sel = $state(0);
  let open = $state(false);
  let blurTimer: ReturnType<typeof setTimeout> | null = null;

  const suggest = (q: string, limit = 8) => (q.trim() ? rank(listed, q, limit) : []);

  function typed(v: string) {
    text = v;
    sel = 0;
    hits = suggest(v);
    open = hits.length > 0;
  }

  function close() {
    open = false;
    hits = [];
    text = current;
  }

  function pick(item: PaletteItem | undefined) {
    open = false;
    hits = [];
    text = item?.label ?? current;
    if (item && item.label !== current) onpick(item.label);
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
      if (open && hits[sel]) pick(hits[sel]);
      else if (text.trim() && text.trim() !== current) pick(suggest(text, 1)[0]);
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

<Input
  value={text}
  {placeholder}
  spellcheck="false"
  autocomplete="off"
  role="combobox"
  aria-label={label}
  aria-expanded={open}
  aria-controls={open ? id : undefined}
  aria-activedescendant={open ? `${id}-${sel}` : undefined}
  aria-autocomplete="list"
  oninput={(e) => typed(e.currentTarget.value)}
  onkeydown={key}
  onfocus={focus}
  onblur={blur}
>
  {#if open}
    <Suggestions {id} items={hits} {sel} key={(h) => h.label} onpick={pick}>
      {#snippet item(h)}{h.label}{#if h.detail} <span class="sub">{h.detail}</span>{/if}{/snippet}
    </Suggestions>
  {/if}
</Input>
