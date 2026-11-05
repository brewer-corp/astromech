<script lang="ts">
  // Nearest systems for one faction-state lookup (Famine, an HGE pool), on the search kit.
  import { nearby, population, type Nearby } from "../galaxy";
  import { figure, type Filters, type Origin, type Results } from "../search";
  import SearchKit from "./SearchKit.svelte";

  let { query }: { query: string } = $props();

  /** The matching faction, its allegiance and states, and whether the commander's own visit said so. */
  function faction(h: Nearby["hits"][number]): string {
    const f = h.factions[0];
    const st = f ? (f.active_states.length ? f.active_states.join(", ") : (f.state ?? "")) : "";
    return [f ? `${f.name} (${f.allegiance}${st ? `, ${st}` : ""})` : "", h.seen_on_visit ? "seen on your visit" : ""].filter(Boolean).join(" · ");
  }

  /** HGE lookups are ranked by the score, with the population it weighs; faction states by jumps. */
  async function run(origin: Origin, filters: Filters): Promise<Results> {
    const r = await nearby(query, origin, filters);
    const scored = r.hits.some((h) => h.score != null);
    return {
      origin: r.here.system,
      range: r.range,
      source: scored ? "Spansh, ranked by population and distance" : "Spansh",
      fetched: r.fetched,
      warning: r.warning,
      label: r.query.label,
      place: "system",
      ...(scored ? { main: { head: "Score" } } : {}),
      columns: [...(scored ? [{ key: "pop", head: "Population", width: 96 }] : []), { key: "faction", head: "Faction", text: true }],
      rows: r.hits.map((h) => ({
        key: h.id64,
        system: h.name,
        ly: h.ly,
        jumps: h.jumps,
        exact: h.exact,
        updated_at: h.updated_at,
        ...(h.score == null ? {} : { main: figure(h.score) }),
        facts: { pop: population(h.population), faction: faction(h) },
      })),
    };
  }
</script>

<SearchKit lookup={`faction.${query}`} offers={["age"]} {run} noun="systems" busyText="Looking up live faction states…" />
