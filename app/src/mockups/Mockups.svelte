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
  import DsSettings from "./ds/Settings.svelte";
  // Glow-up mockups (docs/design/glow-up-*.png)
  import GlowIcons from "./GlowIcons.svelte";
  // Nav spike mockups (spike/nav-rework; docs/design/nav-spike/*.png)
  import NavMaterials from "./nav/Materials.svelte";
  import NavSell from "./nav/Sell.svelte";
  import NavShips from "./nav/Ships.svelte";
  import NavSuits from "./nav/Suits.svelte";
  import NavEngineers from "./nav/Engineers.svelte";
  import Sell5 from "./nav/Sell5.svelte";
  import Engineers5 from "./nav/Engineers5.svelte";
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
{:else if id === "ds-settings"}
  <DsSettings />
{:else if id === "glow-icons"}
  <GlowIcons />
{:else if id === "nav-route"}
  <NavMaterials route="open" />
{:else if id === "nav-route-over"}
  <NavMaterials route="over" />
{:else if id === "nav-panel"}
  <NavSell route="open" />
{:else if id === "nav-collapsed"}
  <NavMaterials route="collapsed" />
{:else if id === "nav-noroute"}
  <NavSell route="none" />
{:else if id === "nav-ships"}
  <NavShips />
{:else if id === "nav-suits"}
  <NavSuits />
{:else if id === "nav-materials"}
  <NavMaterials kind="foot" />
{:else if id === "nav-engineers"}
  <NavEngineers />
{:else if id === "hier-A-sell" || id === "hier-B-sell"}
  <NavSell skin={id === "hier-A-sell" ? "A" : "B"} />
{:else if id === "hier-A-engineers" || id === "hier-B-engineers"}
  <NavEngineers query="laser" skin={id === "hier-A-engineers" ? "A" : "B"} />
{:else if id === "hier-A-over"}
  <NavSell route="over" skin="A" />
{:else if id === "hier-B-collapsed"}
  <NavSell route="collapsed" skin="B" />
{:else if /^tone-\d-sell$/.test(id)}
  <NavSell skin="B" tone={Number(id[5])} />
{:else if /^tone-\d-engineers$/.test(id)}
  <NavEngineers query="laser" skin="B" tone={Number(id[5])} />
{:else if id === "tone5-sell"}
  <Sell5 />
{:else if id === "tone5-engineers-laser"}
  <Engineers5 query="laser" />
{:else if id === "tone5-engineers-rest"}
  <Engineers5 />
{:else if id === "tone5-suits"}
  <NavSuits tone={5} />
{:else if id === "nav-engineers-laser"}
  <NavEngineers query="laser" />
{:else}
  <div class="first-draft"><Page /></div>
{/if}

<style>
  /* The V1 mockups record the first, grey-based direction (docs/design/mockup-*.png), so they keep
     its palette here; the app's tokens.css no longer has it (astromech-yxtz). */
  .first-draft { display: contents; --text: #e8e8e8; --text-lo: #a0a0a0; --accent-dim: #8a3d00; --cell-opt: #2b2d2e; color: var(--text); }
</style>
