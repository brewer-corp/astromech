<script lang="ts">
  // Nearest systems for one faction-state lookup (Famine, an HGE pool), on the search kit.
  import { nearby, population, type Nearby } from "../galaxy";
  import type { Filters, Origin, Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { query }: { query: string } = $props();

  function states(h: Nearby["hits"][number]): string {
    const f = h.factions[0];
    if (!f) return "";
    const st = f.active_states.length ? f.active_states.join(", ") : (f.state ?? "");
    return `${f.name} (${f.allegiance}${st ? `, ${st}` : ""})`;
  }

  /** HGE lookups lead with the score and population they are ranked by. */
  function detail(h: Nearby["hits"][number]): string {
    const ranked = h.score == null ? [] : [`score ${h.score}`, `pop ${population(h.population)}`];
    return [...ranked, states(h)].filter(Boolean).join(" · ");
  }

  async function run(origin: Origin, filters: Filters): Promise<Results> {
    const r = await nearby(query, origin, filters);
    return {
      origin: r.here.system,
      range: r.range,
      source: r.hits.some((h) => h.score != null) ? "Spansh, ranked by population and distance" : "Spansh",
      fetched: r.fetched,
      warning: r.warning,
      rows: r.hits.map((h) => ({ key: h.id64, system: h.name, ly: h.ly, jumps: h.jumps, exact: h.exact, updated_at: h.updated_at, detail: detail(h) })),
    };
  }
</script>

<SearchKit lookup={`faction.${query}`} offers={["age"]} {run} busyText="Looking up live faction states…" />
