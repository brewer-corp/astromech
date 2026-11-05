<script lang="ts">
  // DESIGN.md's hard error (States): a red "!", plain words saying what failed, and the action
  // that recovers ("! Could not read the route. TRY AGAIN"). Never a filled block, and red only
  // here (The Narrow Red Rule). `inline` keeps it on one line inside a band such as the route
  // strip, where the words give way before the action does.
  import Button from "./Button.svelte";

  let {
    text,
    action = "Try again",
    onaction,
    inline = false,
  }: {
    text: string;
    /** The recovering action's words. */
    action?: string;
    onaction: () => void;
    inline?: boolean;
  } = $props();
</script>

<svelte:element this={inline ? "span" : "p"} class="failure" class:inline role="alert">
  <span class="words" title={inline ? text : undefined}><b>!</b> {text}</span>
  <Button onclick={onaction}>{action}</Button>
</svelte:element>

<style>
  .failure { display: flex; align-items: center; flex-wrap: wrap; gap: 0 var(--sp-2); min-height: var(--row); margin: 0; color: var(--alarm); font-size: var(--fs-s); }
  b { font-weight: 600; }
  .inline { display: inline-flex; flex-wrap: nowrap; min-width: 3em; min-height: 0; white-space: nowrap; }
  .inline .words { overflow: hidden; text-overflow: ellipsis; min-width: 0; }
</style>
