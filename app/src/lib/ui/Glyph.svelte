<script module lang="ts">
  // DESIGN.md's icon set (Shapes → Icons): one-stroke marks drawn for astromech on a 16 px grid,
  // 1 px stroke, currentColor, no fill, square ends. Copy, route actions, the dropdown chevron,
  // the ⋯ menu and ? help, station kinds, the ship, the factions, and the star, body and station type sets.
  export type GlyphName =
    | "copy"
    | "copied"
    | "add-stop"
    | "destination"
    | "external"
    | "chevron"
    | "clear"
    | "more"
    | "help"
    | "settlement"
    | "planetary"
    | "outpost"
    | "starport"
    | "coriolis"
    | "orbis"
    | "ocellus"
    | "asteroid-base"
    | "carrier"
    | "megaship"
    | "station"
    | "ship"
    | "federation"
    | "empire"
    | "alliance"
    | "independent"
    | StarGlyph
    | BodyGlyph;

  export type StarGlyph =
    | "star-scoopable"
    | "star-neutron"
    | "star-white-dwarf"
    | "star-brown-dwarf"
    | "star-black-hole"
    | "star-other"
    | "star-unknown";

  export type BodyGlyph =
    | "body-rocky"
    | "body-high-metal"
    | "body-metal-rich"
    | "body-icy"
    | "body-rocky-ice"
    | "body-water-world"
    | "body-earth-like"
    | "body-ammonia"
    | "body-gas-giant"
    | "body-water-giant"
    | "body-ringed";

  // The accessible name of each type icon (DESIGN.md: "its accessible name is the type"), for
  // an icon used alone. Beside its name, a caller passes the real type, which may be rarer.
  export const typeNames = {
    "star-scoopable": "Scoopable star",
    "star-neutron": "Neutron star",
    "star-white-dwarf": "White dwarf",
    "star-brown-dwarf": "Brown dwarf",
    "star-black-hole": "Black hole",
    "star-other": "Non-scoopable star",
    "star-unknown": "Unknown star",
    "body-rocky": "Rocky body",
    "body-high-metal": "High metal content world",
    "body-metal-rich": "Metal-rich body",
    "body-icy": "Icy body",
    "body-rocky-ice": "Rocky ice world",
    "body-water-world": "Water world",
    "body-earth-like": "Earth-like world",
    "body-ammonia": "Ammonia world",
    "body-gas-giant": "Gas giant",
    "body-water-giant": "Water giant",
    "body-ringed": "Ringed body",
    coriolis: "Coriolis starport",
    orbis: "Orbis starport",
    ocellus: "Ocellus starport",
    "asteroid-base": "Asteroid base",
    outpost: "Outpost",
    planetary: "Planetary port",
    settlement: "Settlement",
    carrier: "Fleet carrier",
    megaship: "Megaship",
  } as const satisfies Partial<Record<GlyphName, string>>;
</script>

<script lang="ts">
  // size is the drawn size in px: 16 in a row, 24 beside a title (DESIGN.md). The grid stays 16.
  let { name, label, size = 16 }: { name: GlyphName; label?: string; size?: number } = $props();
</script>

<svg class="glyph" viewBox="0 0 16 16" width={size} height={size} role={label ? "img" : undefined} aria-hidden={label ? undefined : true} aria-label={label}>
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
  {:else if name === "more"}
    <path d="M2.5 8h1M7.5 8h1M12.5 8h1" />
  {:else if name === "help"}
    <path d="M5.5 6a2.5 2.5 0 1 1 3.5 2.3c-.6.3-1 .8-1 1.5v1" />
    <path d="M8 12.5v1.5" />
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
  {:else if name === "coriolis"}
    <path d="M5.5 1.5h5l4 4v5l-4 4h-5l-4-4v-5z" />
    <rect x="5.5" y="7" width="5" height="2" />
  {:else if name === "orbis"}
    <ellipse cx="8" cy="8" rx="6.5" ry="2.5" />
    <path d="M8 1v14" />
  {:else if name === "ocellus"}
    <circle cx="8" cy="8" r="6" />
    <path d="M4.5 8q3.5-3 7 0q-3.5 3-7 0z" />
  {:else if name === "asteroid-base"}
    <path d="M4.5 2.5l5.5-.5 4 3.5-.5 5.5-4 3.5-5.5-.5-2.5-5z" />
    <rect x="6.5" y="7" width="3" height="2" />
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
  {:else if name === "star-scoopable"}
    <circle cx="8" cy="8" r="5.5" />
  {:else if name === "star-other"}
    <circle cx="8" cy="8" r="3.5" />
    <path d="M8 .5v2.5M8 13v2.5M.5 8H3M13 8h2.5M2.7 2.7l1.8 1.8M11.5 11.5l1.8 1.8M13.3 2.7l-1.8 1.8M2.7 13.3l1.8-1.8" />
  {:else if name === "star-white-dwarf"}
    <circle cx="8" cy="8" r="2" />
    <path d="M8 2.5V5M8 11v2.5M2.5 8H5M11 8h2.5" />
  {:else if name === "star-neutron"}
    <circle cx="8" cy="8" r="1.5" />
    <path d="M9.5 6.5L14.5 1.5M6.5 9.5L1.5 14.5" />
    <ellipse cx="8" cy="8" rx="5.5" ry="2.5" stroke-opacity="0.5" />
  {:else if name === "star-brown-dwarf"}
    <circle cx="8" cy="8" r="4.5" />
    <path d="M3.8 6.5h8.4M3.8 9.5h8.4" />
  {:else if name === "star-black-hole"}
    <circle cx="8" cy="8" r="3.5" />
    <ellipse cx="8" cy="8" rx="7.5" ry="1.5" />
  {:else if name === "star-unknown"}
    <circle cx="8" cy="8" r="5.5" stroke-dasharray="2 2.32" />
  {:else if name === "body-rocky"}
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="6" cy="6.5" r="1.5" />
    <circle cx="10.5" cy="10" r="1" />
  {:else if name === "body-high-metal"}
    <circle cx="8" cy="8" r="5.5" />
    <circle cx="8" cy="8" r="2.5" />
  {:else if name === "body-metal-rich"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M8 5l3 3-3 3-3-3z" />
  {:else if name === "body-icy"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M8 4.5v7M5 6.25l6 3.5M5 9.75l6-3.5" />
  {:else if name === "body-rocky-ice"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M4 4.5h8" />
    <circle cx="8" cy="9.5" r="1.5" />
  {:else if name === "body-water-world"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M2.75 7q1.75-1.2 3.5 0t3.5 0t3.5 0M3.5 10q1.5-1.2 3 0t3 0t3 0" />
  {:else if name === "body-earth-like"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M5 5l3-.5 1.5 1.5-1.5 1.5-2.5-.5z" />
    <path d="M3.5 10q1.5-1.2 3 0t3 0t3 0" />
  {:else if name === "body-ammonia"}
    <circle cx="8" cy="8" r="5.5" />
    <path d="M8 8a1.25 1.25 0 1 1 1.25 1.25A2.75 2.75 0 1 1 11 5.5" />
  {:else if name === "body-gas-giant"}
    <circle cx="8" cy="8" r="6.5" />
    <path d="M1.8 6h12.4M2 10.5h12" />
    <ellipse cx="10" cy="8.25" rx="1.5" ry="1" />
  {:else if name === "body-water-giant"}
    <circle cx="8" cy="8" r="6.5" />
    <path d="M2 6q1.5-1.2 3 0t3 0t3 0t3 0M2 10.5q1.5-1.2 3 0t3 0t3 0t3 0" />
  {:else if name === "body-ringed"}
    <circle cx="8" cy="8" r="4.5" />
    <ellipse cx="8" cy="8" rx="7.5" ry="2" transform="rotate(-25 8 8)" />
  {:else}
    <rect x="3.5" y="3.5" width="9" height="9" />
    <path d="M3.5 7h9" />
  {/if}
</svg>

<style>
  .glyph { display: block; fill: none; stroke: currentColor; stroke-width: 1; stroke-linecap: square; stroke-linejoin: miter; flex: none; }
</style>
