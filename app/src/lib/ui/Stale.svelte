<script lang="ts">
  // The line a polling page shows beside its last good data when a later read failed
  // (astromech-spxx): "Could not refresh: <reason>." Shown only while the data is on screen;
  // with no data the page shows the full error instead.
  let { error }: { error: string | null } = $props();
</script>

{#if error}
  <p class="stale" role="status"><b class="mark">!</b> {error}</p>
{/if}

<style>
  .stale {
    margin: 0 0 8px;
    font-size: var(--fs-s);
    color: var(--text-lo);
  }
  .mark {
    color: var(--accent);
  }
</style>
