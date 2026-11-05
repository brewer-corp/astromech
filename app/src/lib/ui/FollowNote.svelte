<script lang="ts">
  // The line under an action that changed the route followed (a result's Add stop or Set
  // destination, the engineer trip's Follow, the plotter): what it did and, when it replaced a
  // route, "Replaced <name>." with RESUME, so a route is never dropped silently (decision
  // astromech-bi94). Resume follows the parked route again and says so here.
  import { followedNote, failedNote, resumeRoute, type FollowNote, type RouteView } from "../route";
  import Button from "./Button.svelte";

  let { note = $bindable(), onresume }: { note: FollowNote | null; onresume?: (v: RouteView) => void } = $props();

  async function resume() {
    try {
      const v = await resumeRoute();
      note = followedNote(v);
      onresume?.(v);
    } catch (e) {
      note = failedNote("Couldn't resume it", e);
    }
  }
</script>

{#if note && (note.text || note.replaced)}
  <p class="note">
    {note.text}
    {#if note.replaced}
      <span>Replaced {note.replaced}.</span>
      <Button class="resume" onclick={() => void resume()}>Resume</Button>
    {/if}
  </p>
{/if}

<style>
  .note { display: flex; flex-wrap: wrap; align-items: center; gap: 0 var(--sp-2); margin: 14px 0 8px; color: var(--text-lo); font-size: var(--fs-s); }
  .note :global(.resume) { height: auto; padding-block: 2px; }
</style>
