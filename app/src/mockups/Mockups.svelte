<script lang="ts">
  // Static design mockups (docs/V1-DESIGN.md §5): open #/mockup/<id>.
  import "../design/tokens.css";
  import KeybindsList from "./KeybindsList.svelte";
  import MyController from "./MyController.svelte";
  import Settings from "./Settings.svelte";
  import TrayMenu from "./TrayMenu.svelte";
  import ActivityView from "./ActivityView.svelte";
  const pages = { activities: ActivityView, keybinds: KeybindsList, controller: MyController, settings: Settings, tray: TrayMenu };
  const read = () => location.hash.replace("#/mockup/", "");
  let id = $state(read());
  window.addEventListener("hashchange", () => (id = read()));
  const Page = $derived(pages[id as keyof typeof pages] ?? KeybindsList);
</script>

<Page />
