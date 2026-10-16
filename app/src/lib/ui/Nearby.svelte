<script lang="ts">
  // Nearest systems for one faction-state lookup (Famine, an HGE pool), on the search kit.
  import { nearby, type Nearby } from "../galaxy";
  import type { Filters, Origin, Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { query }: { query: string } = $props();

  function states(h: Nearby["hits"][number]): string {
    const f = h.factions[0];
    if (!f) return "";
    const st = f.active_states.length ? f.active_states.join(", ") : (f.state ?? "");
    return `${f.name} (${f.allegiance}${st ? `, ${st}` : ""})`;
  }

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    const r = await nearby(query, origin, filters);
    return {
      origin: r.here.system,
      range: r.range,
      source: "Spansh",
      fetched: r.fetched,
      warning: r.warning,
      rows: r.hits.map((h) => ({ key: h.id64, system: h.name, ly: h.ly, jumps: h.jumps, exact: h.exact, updated_at: h.updated_at, detail: states(h) })),
    };
  }
</script>

<SearchKit lookup={`faction.${query}`} offers={["age"]} {run} busyText="Looking up live faction states…" />
