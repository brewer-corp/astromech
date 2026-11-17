<script lang="ts">
  // One of the Now page's answers (DESIGN.md Board): a section heading, the block's one main
  // value with what qualifies it, and a link to the page that has the rest. While its read has
  // not answered, one Label Grey line names what it reads; a failed first read is the failure in
  // words with no action (the page reads again on its own); a failed later read keeps the answer
  // with the shared Stale line.
  import type { Snippet } from "svelte";
  import Stale from "../../lib/ui/Stale.svelte";

  let {
    title,
    href,
    link,
    loading = null,
    failed = null,
    stale = null,
    children,
  }: {
    title: string;
    /** The page with the rest, and the link's words. */
    href: string;
    link: string;
    /** "Reading the journal history…" while there is no answer yet. */
    loading?: string | null;
    /** The first read failed: its words. */
    failed?: string | null;
    /** A later read failed: the answer stays, with this line. */
    stale?: string | null;
    children?: Snippet;
  } = $props();

  const id = $props.id();
</script>

<section class="block" aria-labelledby={id}>
  <h3 class="section" {id}>{title}</h3>
  {#if failed}
    <p class="type-label err" role="alert"><b>!</b> {failed}</p>
  {:else if loading}
    <p class="type-label">{loading}</p>
  {:else}
    <Stale error={stale} />
    {@render children?.()}
  {/if}
  <a class="type-label link" {href}>{link}</a>
</section>

<style>
  .block { display: flex; flex-direction: column; align-items: flex-start; gap: var(--sp-2); min-width: 0; }
  .block > :global(.section) { align-self: stretch; margin-bottom: var(--sp-1); }
  p { margin: 0; }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }
  /* DESIGN.md Buttons → Links: Orange, a 1 px Orange Mid underline. */
  .link { color: var(--accent); text-decoration: underline 1px var(--orange-mid); text-underline-offset: 3px; }
  .link:hover { color: var(--orange-lit); }
  .link:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
</style>
