<script lang="ts">
  // A screen that arrives in a later milestone (docs/V1-DESIGN.md §8).
  let { title, milestone, lines }: { title: string; milestone: string; lines: string[] } = $props();
</script>

<section>
  <h2>{title}</h2>
  <p class="when">Arrives in {milestone}.</p>
  <ul>
    {#each lines as l (l)}
      <li>{l}</li>
    {/each}
  </ul>
</section>

<style>
  section { padding: 28px 0; max-width: 60rem; }
  h2 { margin: 0 0 6px; font-size: var(--fs-xl); font-weight: 500; color: var(--text-hi); }
  .when { color: var(--accent); margin: 0 0 18px; }
  ul { margin: 0; padding-left: 1.2rem; color: var(--text-lo); line-height: 1.7; }
</style>
