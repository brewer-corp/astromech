<script lang="ts">
  // The copy icon button beside a system name: always visible, Tip and label.
  import { tip } from "../../lib/ui/tip";
  import Glyph from "../../lib/ui/Glyph.svelte";
  let { what = "system name" }: { what?: string } = $props();
</script>

<button class="ibtn" aria-label="Copy {what}" {@attach tip(`Copy ${what}`)}><Glyph name="copy" /></button>
