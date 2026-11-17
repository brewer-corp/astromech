<script lang="ts">
  // Exploration > Biology (DESIGN.md Board): the current system's biology as a card per body,
  // most valuable first. Each card holds the body's genera (from the DSS or a sample, or
  // predicted from its scan) with the species they could be and what they are worth, and what
  // other players reported for it (Spansh) merged in; a body only others reported gets its own
  // card after the commander's. While the commander samples a species, the sampling leads the
  // page: the samples taken and how far the nearest earlier sample is against the colony range.
  // Live while Elite runs.
  import Badge from "../../lib/ui/Badge.svelte";
  import { pollFailure } from "../../lib/ui/errors";
  import Stale from "../../lib/ui/Stale.svelte";
  import { tip } from "../../lib/ui/tip";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Tiles from "../../lib/ui/layout/Tiles.svelte";
  import Samples from "../../lib/ui/Samples.svelte";
  import { bodyCards, loadBiology, millions, ofGenus, shortName, valueRange, type BiologyView, type CardGenus } from "../../lib/exobiology";

  let v = $state<BiologyView | null>(null);
  let failed = $state<string | null>(null);

  // Follows the game: scans and samples arrive while the window is open; on foot the distance
  // to the last sample changes by the second.
  $effect(() => {
    const tick = () =>
      loadBiology()
        .then((r) => {
          v = r;
          failed = null;
        })
        .catch((e: unknown) => (failed = pollFailure("read the biology", e, v !== null)));
    void tick();
    const t = setInterval(() => void tick(), 2000);
    return () => clearInterval(t);
  });

  const cards = $derived(v ? bodyCards(v.bodies, v.known) : []);
  const total = $derived(cards.reduce<[number, number]>((a, c) => [a[0] + c.value[0], a[1] + c.value[1]], [0, 0]));
  const short = (name: string) => shortName(name, v?.system ?? null);
  const plural = (n: number, one: string, many: string) => `${n} ${n === 1 ? one : many}`;
  const sampling = $derived(v?.sampling ?? null);
  const sampledBody = $derived(sampling ? v?.bodies.find((b) => b.body_id !== null && b.body_id === sampling.body_id) : undefined);
  /** The distance to the nearest earlier sample is the page's main value while there is one. */
  const distanceLeads = $derived(!!sampling && !sampling.complete && !!sampling.clearance);

  /** The samples taken of a genus on a card: the one being sampled to its count, analysed ones in full. */
  function taken(bodyId: number | null, g: CardGenus): number | null {
    if (sampling && bodyId !== null && sampling.body_id === bodyId && sampling.genus === g.genus) return sampling.complete ? 3 : sampling.count;
    return g.analysed ? 3 : null;
  }

  /** The species a genus could be, without the genus's name: "Tectonicas", "Roseum" (Brain Tree). */
  function speciesName(genus: string, species: string): string {
    if (species === genus || !ofGenus(species, genus)) return species;
    return species.startsWith(`${genus} `) ? species.slice(genus.length + 1) : species.slice(0, -genus.length - 1);
  }

  /** "1.4–19 M" as its figure and its unit, so the unit can be drawn as one. */
  const parts = (r: [number, number]) => {
    const t = valueRange(r);
    return [t.replace(/ M$/, ""), "M"] as const;
  };

  /** What a genus's name means, in its Tip; none for one the commander knows and nobody else reported. */
  function genusTip(g: CardGenus) {
    if (g.reported) return { title: "Reported", fact: REPORTED };
    if (!g.known) return { title: "Predicted", fact: g.others ? `${PREDICTED} Other players reported it here.` : PREDICTED };
    return null;
  }

  const VALUE = "Vista Genomics base value; a first logged species pays five times";
  const RANGE = "Sample again at least the colony range from every earlier sample";
  const PREDICTED = "From the body's scan: map it with the DSS to know.";
  const REPORTED = "Other players reported it to EDDN (from Spansh).";
</script>

<Board>
  <div class="page">
    <Stale error={v ? failed : null} />
    {#if failed && !v}
      <!-- The page reads again every 2 s on its own: the next read is the recovery, so no action. -->
      <p class="type-label err" role="alert"><b>!</b> {failed} Biology follows the game while it runs.</p>
    {:else if !v}
      <p class="type-label">Reading the journal…</p>
    {:else}
      <header>
        {#if v.system}<h2 class="type-title">{v.system}</h2>{/if}
        <p class="type-label">
          {[!v.system && "No system yet", cards.length && `biology on ${plural(cards.length, "body", "bodies")}`, cards.length && valueRange(total), v.source]
            .filter(Boolean)
            .join(" · ")}
        </p>
      </header>

      {#if sampling}
        {@const c = sampling.clearance}
        <section class="sampling" aria-label="Sampling">
          <div class="what">
            <span class="type-name">{sampling.species}</span>
            {#if sampledBody}<span class="type-label">on {short(sampledBody.name)}</span>{/if}
          </div>
          <div class="count">
            <Samples count={sampling.complete ? 3 : sampling.count} />
            <span class="type-label">{sampling.complete ? 3 : sampling.count} of 3</span>
          </div>
          {#if sampling.complete}
            <p class="type-value-xl">Analysed</p>
          {:else if c}
            <p class="far" {@attach tip(RANGE)}>
              <span><span class="type-value-xl main">{Math.round(c.nearest_m)}</span> <span class="type-unit">m of {c.colony_range_m} m</span></span>
              {#if c.far_enough}
                <span class="type-name go">Far enough</span>
              {:else}
                <span><span class="type-name">{Math.max(0, Math.ceil(c.colony_range_m - c.nearest_m))}</span> <span class="type-unit">m to go</span></span>
              {/if}
            </p>
          {:else if sampling.colony_range_m !== null}
            <p class="type-label" {@attach tip(RANGE)}>Colony range {sampling.colony_range_m} m</p>
          {/if}
        </section>
      {/if}

      {#if !v.bodies.length}
        <p class="type-label">No biological signals found in this system yet. The FSS shows them.</p>
      {/if}
      {#if cards.length}
        <Tiles wide list label="Bodies with biology">
          {#each cards as card, i (card.name)}
            <article class="card" role="listitem" aria-label={short(card.name)}>
              <div class="top">
                <span class="name">
                  <span class="type-name">{short(card.name)}</span>
                  {#if card.reported}<Badge {@attach tip(REPORTED)}>Reported</Badge>{/if}
                </span>
                <span {@attach tip(VALUE)}><span class="type-value-l" class:main={i === 0 && !distanceLeads}>{parts(card.value)[0]}</span> <span class="type-unit">{parts(card.value)[1]}</span></span>
              </div>
              <p class="type-label">
                {card.signals === null ? "Signals not known" : plural(card.signals, "signal", "signals")}
                {#if card.reported_value}· <span {@attach tip(REPORTED)}>reported {valueRange(card.reported_value)}</span>{/if}
              </p>
              {#if card.genera.length}
                <ul class="genera">
                  {#each card.genera as g (g.genus)}
                    {@const n = taken(card.body_id, g)}
                    <li>
                      <span class="genus {g.reported ? 'type-label' : 'type-figures'}" class:known={g.known && !g.reported} {@attach tip(genusTip(g))}>{g.genus}</span>
                      <span class="species type-label">
                        {#each g.species as s, j (s.name)}
                          {@const sep = j < g.species.length - 1 ? ", " : ""}
                          {#if s.reported}
                            <span {@attach tip(REPORTED)}>{speciesName(g.genus, s.name)}{s.value !== null ? ` ${millions(s.value)}` : ""}</span>{sep}
                          {:else}
                            {speciesName(g.genus, s.name)}{sep}
                          {/if}
                        {/each}
                      </span>
                      <span class="worth">{#if g.value}{@const [fig, unit] = parts(g.value)}<span class="type-figures">{fig}</span> <span class="type-unit">{unit}</span>{/if}</span>
                      <span class="marks">{#if n !== null}<Samples count={n} small />{/if}</span>
                    </li>
                  {/each}
                </ul>
              {:else}
                <p class="type-label">Scan it to predict its genera</p>
              {/if}
            </article>
          {/each}
        </Tiles>
      {/if}
    {/if}
  </div>
</Board>

<style>
  p, h2 { margin: 0; }
  .page { display: flex; flex-direction: column; gap: var(--sp-8); min-width: 0; }
  header { display: flex; flex-direction: column; gap: var(--sp-1); }
  .err { color: var(--alarm); }
  .err b { font-weight: 600; }

  /* The page's one leading block (DESIGN.md Hull Raised): the species, its samples, and the
     distance to the nearest earlier sample as the page's main value. */
  .sampling { display: flex; flex-direction: column; gap: var(--sp-3); width: fit-content; max-width: 100%; padding: var(--sp-4) var(--sp-6); background: var(--surface-hi); border: 1px solid var(--line); }
  .what, .count { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2); }
  .count { align-items: center; gap: var(--sp-3); }
  .far { display: flex; flex-wrap: wrap; align-items: baseline; gap: var(--sp-2) var(--sp-6); }
  .go { color: var(--accent); }

  /* A body card: a Seam outline on Void, its name and value range, then a line per genus. */
  .card { display: flex; flex-direction: column; gap: var(--sp-2); min-width: 0; padding: var(--sp-3) var(--sp-4); outline: 1px solid var(--line); outline-offset: -1px; }
  .top { display: flex; align-items: baseline; justify-content: space-between; gap: var(--sp-4); }
  .name { display: flex; align-items: center; gap: var(--sp-2); min-width: 0; }
  .genera { list-style: none; margin: var(--sp-1) 0 0; padding: 0; display: grid; grid-template-columns: max-content minmax(0, 1fr) max-content max-content; column-gap: var(--sp-3); row-gap: var(--sp-1); }
  .genera li { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: baseline; }
  .genus { color: var(--label-grey); }
  .genus.known { color: var(--prose); }
  .worth { text-align: right; white-space: nowrap; }
  .marks { align-self: center; display: flex; }
</style>
