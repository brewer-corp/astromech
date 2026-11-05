<script lang="ts">
  // The line under an action that changed the route followed (a result's Add stop or Set
  // destination, the engineer trip's Follow, the plotter): what it did and, when it replaced a
  // route, "Replaced <name>." with RESUME, so a route is never dropped silently (decision
  // astromech-bi94). Resume follows the parked route again and says so here. A failed action is
  // DESIGN.md's hard error: a red "!", why, and TRY AGAIN.
  import { followedNote, failedNote, resumeRoute, type FollowNote, type RouteView } from "../route";
  import Button from "./Button.svelte";
  import Failure from "./Failure.svelte";

  let { note = $bindable(), onresume }: { note: FollowNote | null; onresume?: (v: RouteView) => void } = $props();

  async function resume() {
    try {
      const v = await resumeRoute();
      note = followedNote(v);
      onresume?.(v);
    } catch (e) {
      note = failedNote("Couldn't resume it", e, () => void resume());
    }
  }
</script>

{#if note?.retry}
  <div class="note"><Failure text={note.text} onaction={note.retry} /></div>
{:else if note && (note.text || note.replaced)}
  <p class="note">
    {note.text}
    {#if note.replaced}
      <span>Replaced {note.replaced}.</span>
      <Button onclick={() => void resume()}>Resume</Button>
    {/if}
  </p>
{/if}

<style>
  /* One control row high, so Resume keeps the 34 px hit area of every button. */
  .note { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); min-height: var(--row); margin: var(--sp-3) 0 var(--sp-2); color: var(--text-lo); font-size: var(--fs-s); }
</style>
