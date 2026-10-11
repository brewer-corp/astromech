<script lang="ts">
  import Planned from "../Planned.svelte";
  const lines = [
    "Every voice and control setting in one list: push-to-talk, kill switch, microphone, voice responses (colour Natural / Cockpit / Harsh, default Cockpit; low-fuel warning at 25%, adjustable), debug.",
    "Until then: tray menu → Open config file.",
  ];
</script>

<Planned title="Settings" milestone="M4" {lines} />
