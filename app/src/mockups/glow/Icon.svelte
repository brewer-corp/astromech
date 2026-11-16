<script lang="ts" module>
  // The icon sets the Glow-up mission proposes for DESIGN.md (Shapes → Icons): star types, body
  // types and station types, drawn for astromech on a 16 px grid, 1 px stroke, currentColor,
  // no fill, square ends. No Frontier art. Plus the few UI marks the mockups need.
  export type IconName =
    // stars
    | "star" | "star-dwarf" | "white-dwarf" | "neutron" | "black-hole"
    // bodies
    | "rocky" | "hmc" | "metal-rich" | "icy" | "rocky-ice" | "water" | "earth-like" | "ammonia" | "gas-giant" | "water-giant"
    // stations
    | "coriolis" | "orbis" | "ocellus" | "outpost" | "planetary" | "settlement" | "asteroid" | "carrier" | "megaship"
    // UI
    | "copy" | "add-stop" | "destination" | "ship" | "fuel" | "cargo" | "jump" | "clock" | "aisling";
</script>

<script lang="ts">
  let { name, size = 16, label }: { name: IconName; size?: number; label?: string } = $props();
</script>

<svg class="icon" viewBox="0 0 16 16" width={size} height={size} role={label ? "img" : undefined} aria-hidden={label ? undefined : true} aria-label={label}>
  {#if name === "star"}
    <!-- main sequence: a disc with eight rays -->
    <circle cx="8" cy="8" r="3.5" />
    <path d="M8 .5v2M8 13.5v2M.5 8h2M13.5 8h2M2.7 2.7l1.4 1.4M11.9 11.9l1.4 1.4M2.7 13.3l1.4-1.4M11.9 4.1l1.4-1.4" />
  {:else if name === "star-dwarf"}
    <!-- brown dwarf: a banded disc, no rays -->
    <circle cx="8" cy="8" r="5.5" />
    <path d="M3 6.5h10M2.8 9.5h10.4" />
  {:else if name === "white-dwarf"}
    <circle cx="8" cy="8" r="2" />
    <path d="M8 2.5v2M8 11.5v2M2.5 8h2M11.5 8h2" />
  {:else if name === "neutron"}
    <!-- a small core with its two jets -->
    <circle cx="8" cy="8" r="2" />
    <path d="M8 .5v5M8 10.5v5M4.5 6.5l-2-1M11.5 9.5l2 1" />
  {:else if name === "black-hole"}
    <circle cx="8" cy="8" r="2.5" />
    <ellipse cx="8" cy="8" rx="7" ry="2.5" />
  {:else if name === "rocky"}
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="6" cy="6.5" r="1.3" />
    <path d="M9 10.5h2" />
  {:else if name === "hmc"}
    <!-- high metal content: a metal core -->
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="8" cy="8" r="2.5" />
  {:else if name === "metal-rich"}
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="8" cy="8" r="3" />
    <circle cx="8" cy="8" r="1" />
  {:else if name === "icy"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M8 4.5v7M5 6.2l6 3.6M5 9.8l6-3.6" />
  {:else if name === "rocky-ice"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M5.5 5.5v3M4 7h3" />
    <circle cx="10" cy="10" r="1.2" />
  {:else if name === "water"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M3.5 7c1.5-1 3 1 4.5 0s3-1 4.5 0M3.5 10c1.5-1 3 1 4.5 0s3-1 4.5 0" />
  {:else if name === "earth-like"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M5 4.5c1 1.5 3 1 3 3s-2 2-1.5 4M10 9.5c1 .5 2.5 0 3-1" />
  {:else if name === "ammonia"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M5.5 6.5h1M9.5 6.5h1M7.5 10h1" />
  {:else if name === "gas-giant"}
    <circle cx="8" cy="8" r="6.5" />
    <path d="M2.2 5.5h11.6M1.6 8.5h12.8M2.6 11.5h10.8" />
  {:else if name === "water-giant"}
    <circle cx="8" cy="8" r="6.5" />
    <path d="M2 6.5c2-1 4 1 6 0s4-1 6 0M2 10c2-1 4 1 6 0s4-1 6 0" />
  {:else if name === "coriolis"}
    <!-- a many-faced hull with its letterbox -->
    <path d="M8 1.5l5.5 3.2v6.6L8 14.5 2.5 11.3V4.7z" />
    <path d="M6.5 7.5h3v1h-3z" />
  {:else if name === "orbis"}
    <!-- a wheel on a spindle -->
    <ellipse cx="8" cy="8" rx="6.5" ry="3" />
    <path d="M8 1.5v13M3 8h10" />
  {:else if name === "ocellus"}
    <!-- a ball with its docking eye -->
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="8" cy="8" r="1.5" />
    <path d="M8 1v1.5M8 13.5V15" />
  {:else if name === "outpost"}
    <rect x="4.5" y="5.5" width="7" height="7" />
    <path d="M8 5.5V2.5M6.5 2.5h3M1.5 9h3M11.5 9h3" />
  {:else if name === "planetary"}
    <!-- a port on the horizon -->
    <path d="M1 13.5h14" />
    <path d="M3.5 13.5a4.5 4.5 0 0 1 9 0" />
    <path d="M8 9V6.5M6.5 6.5h3" />
  {:else if name === "settlement"}
    <rect x="1.5" y="8.5" width="4" height="5" />
    <rect x="6.5" y="4.5" width="3" height="9" />
    <rect x="10.5" y="6.5" width="4" height="7" />
  {:else if name === "asteroid"}
    <path d="M3 4.5l4-2.5 5 1.5 2.5 4-2 5.5-5.5 1-4.5-3.5z" />
    <path d="M6.5 7.5h3v1h-3z" />
  {:else if name === "carrier"}
    <path d="M2 10.5h12l-2 3H4z" />
    <path d="M3.5 10.5V7.5h6.5l2 3" />
  {:else if name === "megaship"}
    <path d="M1.5 9.5h11l2-2.5H4.5z" />
    <path d="M3 9.5v3h8" />
  {:else if name === "copy"}
    <rect x="5.5" y="2.5" width="7" height="9" />
    <path d="M3.5 5.5h6v8h-6z" />
  {:else if name === "add-stop"}
    <circle cx="4.5" cy="3.5" r="1.5" />
    <circle cx="4.5" cy="12.5" r="1.5" />
    <path d="M4.5 5v6M10.5 8h5M13 5.5v5" />
  {:else if name === "destination"}
    <path d="M3.5 15V1.5" />
    <path d="M3.5 2h9l-2.5 3 2.5 3h-9" />
  {:else if name === "ship"}
    <path d="M8 2.5l5 11-5-2.2-5 2.2z" />
  {:else if name === "fuel"}
    <path d="M3.5 14.5v-12h6v12M2 14.5h9M3.5 7.5h6" />
    <path d="M9.5 4.5l2.5 2v6a1 1 0 0 0 2 0V5.5l-1.5-1.5" />
  {:else if name === "cargo"}
    <rect x="2.5" y="4.5" width="11" height="9" />
    <path d="M2.5 8h11M8 4.5v3.5" />
  {:else if name === "jump"}
    <path d="M1.5 12.5l4-9 3 6 2-3 4 6" />
  {:else if name === "clock"}
    <circle cx="8" cy="8" r="6.5" />
    <path d="M8 4v4.5l3 2" />
  {:else if name === "aisling"}
    <!-- the Empire mark used for Imperial powers -->
    <path d="M8 2.5l5 4.5-5-1.5-5 1.5z" />
    <path d="M8 5.5v8M4.5 13.5h7" />
  {/if}
</svg>

<style>
  .icon { display: block; fill: none; stroke: currentColor; stroke-width: 1; stroke-linecap: square; stroke-linejoin: miter; flex: none; }
</style>
