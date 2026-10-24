<script lang="ts">
  // Static design mockups (docs/V1-DESIGN.md §5): open #/mockup/<id>.
  import "../design/tokens.css";
  import KeybindsList from "./KeybindsList.svelte";
  import MyController from "./MyController.svelte";
  import Settings from "./Settings.svelte";
  import TrayMenu from "./TrayMenu.svelte";
  import ActivityView from "./ActivityView.svelte";
  import ShipDiagram from "./ShipDiagram.svelte";
  // DESIGN.md mockups (docs/design/design-system-*.png)
  import DsMaterials from "./ds/Materials.svelte";
  import DsFinder from "./ds/Finder.svelte";
  import DsPlacesHome from "./ds/PlacesHome.svelte";
  const pages = { activities: ActivityView, ship: ShipDiagram, keybinds: KeybindsList, controller: MyController, settings: Settings, tray: TrayMenu };
  const read = () => location.hash.replace("#/mockup/", "");
  let id = $state(read());
  window.addEventListener("hashchange", () => (id = read()));
  const Page = $derived(pages[id as keyof typeof pages] ?? KeybindsList);
</script>

{#if id === "ds-materials"}
  <DsMaterials />
{:else if id === "ds-sell" || id === "ds-mining"}
  <DsFinder kind={id === "ds-sell" ? "sell" : "mining"} />
{:else if id === "ds-places"}
  <DsPlacesHome />
{:else}
  <Page />
{/if}
