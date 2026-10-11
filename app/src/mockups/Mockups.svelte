<script lang="ts">
  // Static design mockups (docs/V1-DESIGN.md §3): open #/mockup/<id>.
  import "../design/tokens.css";
  import TrayPopover from "./TrayPopover.svelte";
  import ControllerView from "./ControllerView.svelte";
  import BindsList from "./BindsList.svelte";
  import VoiceSettings from "./VoiceSettings.svelte";
  const pages = { tray: TrayPopover, keybinds: ControllerView, search: BindsList, voice: VoiceSettings };
  let id = $state(location.hash.replace("#/mockup/", ""));
  window.addEventListener("hashchange", () => (id = location.hash.replace("#/mockup/", "")));
  const Page = $derived(pages[id as keyof typeof pages] ?? ControllerView);
</script>

<div class="root"><Page /></div>

<style>
  :global(body) { margin: 0; background: var(--am-bg); color: var(--am-text); font-family: var(--am-font-ui); }
  .root { width: 100vw; height: 100vh; overflow: hidden; }
</style>
