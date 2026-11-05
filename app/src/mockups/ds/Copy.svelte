<script lang="ts">
  // The copy icon button beside a system name: always visible, tooltip and label.
  import Glyph from "../../lib/ui/Glyph.svelte";
  let { what = "system name" }: { what?: string } = $props();
</script>

<button class="ibtn" title="Copy {what}" aria-label="Copy {what}"><Glyph name="copy" /></button>
