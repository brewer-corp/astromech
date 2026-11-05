<script lang="ts">
  // DESIGN.md's icon set (Shapes → Icons): one-stroke marks drawn for astromech on a 16 px grid,
  // 1 px stroke, currentColor, no fill, square ends. Copy, route actions, the dropdown chevron,
  // station kinds, the ship and the factions.
  export type GlyphName =
    | "copy"
    | "copied"
    | "add-stop"
    | "destination"
    | "external"
    | "chevron"
    | "clear"
    | "settlement"
    | "planetary"
    | "outpost"
    | "starport"
    | "carrier"
    | "megaship"
    | "station"
    | "ship"
    | "federation"
    | "empire"
    | "alliance"
    | "independent";

  let { name, label }: { name: GlyphName; label?: string } = $props();
</script>

<svg class="glyph" viewBox="0 0 16 16" width="16" height="16" role={label ? "img" : undefined} aria-hidden={label ? undefined : true} aria-label={label}>
  {#if name === "copy"}
    <rect x="5.5" y="2.5" width="7" height="9" />
    <path d="M3.5 5.5h6v8h-6z" />
  {:else if name === "copied"}
    <rect x="5.5" y="2.5" width="7" height="9" />
    <path d="M3.5 5.5h6v8h-6z" />
    <path d="M5 10.2l1.4 1.4 2.6-3" />
  {:else if name === "add-stop"}
    <circle cx="4.5" cy="3.5" r="1.5" />
    <circle cx="4.5" cy="12.5" r="1.5" />
    <path d="M4.5 5v6M10.5 8h5M13 5.5v5" />
  {:else if name === "destination"}
    <path d="M3.5 15V1.5" />
    <path d="M3.5 2h9l-2.5 3 2.5 3h-9" />
  {:else if name === "external"}
    <path d="M7 3.5H2.5v10h10V9" />
    <path d="M9.5 2.5h4v4M13.5 2.5L7.5 8.5" />
  {:else if name === "chevron"}
    <path d="M4 6l4 4 4-4" />
  {:else if name === "clear"}
    <path d="M4.5 4.5l7 7M11.5 4.5l-7 7" />
  {:else if name === "settlement"}
    <rect x="1.5" y="8.5" width="4" height="5" />
    <rect x="6.5" y="4.5" width="3" height="9" />
    <rect x="10.5" y="6.5" width="4" height="7" />
  {:else if name === "planetary"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M3 8.5h10" />
  {:else if name === "outpost"}
    <rect x="4.5" y="5.5" width="7" height="7" />
    <path d="M8 5.5V2.5M6.5 2.5h3" />
  {:else if name === "starport"}
    <path d="M8 1.5l5.5 3.2v6.6L8 14.5 2.5 11.3V4.7z" />
  {:else if name === "carrier"}
    <path d="M2 10.5h12l-2 3H4z" />
    <path d="M3.5 10.5V7.5h6.5l2 3" />
  {:else if name === "megaship"}
    <path d="M1.5 9.5h11l2-2.5H4.5z" />
    <path d="M3 9.5v3h8" />
  {:else if name === "ship"}
    <path d="M8 2.5l5 11-5-2.2-5 2.2z" />
  {:else if name === "federation"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M8 4.8l.9 2.4 2.5.1-2 1.6.7 2.4L8 9.9l-2.1 1.4.7-2.4-2-1.6 2.5-.1z" />
  {:else if name === "empire"}
    <path d="M8 2.5l5 4.5-5-1.5-5 1.5z" />
    <path d="M8 5.5v8M4.5 13.5h7" />
  {:else if name === "alliance"}
    <path d="M8 2.5l5 9H3z" />
    <path d="M5.5 7.5h5" />
  {:else if name === "independent"}
    <path d="M8 2.5l1.3 4.2 4.2 1.3-4.2 1.3L8 13.5l-1.3-4.2-4.2-1.3 4.2-1.3z" />
  {:else}
    <rect x="3.5" y="3.5" width="9" height="9" />
    <path d="M3.5 7h9" />
  {/if}
</svg>

<style>
  .glyph { display: block; fill: none; stroke: currentColor; stroke-width: 1; stroke-linecap: square; stroke-linejoin: miter; flex: none; }
</style>
