<script lang="ts">
  // The copy icon button beside a system name. Its name is its accessible label; the hover
  // explanation is a Tip, never a native title tooltip.
  import Icon from "./Icon.svelte";
  let { what = "system name" }: { what?: string } = $props();
</script>

<button class="ibtn copy" aria-label="Copy {what}"><Icon name="copy" /></button>

<style>
  .copy { margin-left: -4px; }
</style>
