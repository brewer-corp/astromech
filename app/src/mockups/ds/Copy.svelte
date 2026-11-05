<script lang="ts">
  // The copy icon button beside a system name: always visible, tooltip and label.
  import Icon from "./Icon.svelte";
  let { what = "system name" }: { what?: string } = $props();
</script>

<button class="ibtn" title="Copy {what}" aria-label="Copy {what}"><Icon name="copy" /></button>
