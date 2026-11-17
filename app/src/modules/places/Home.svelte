<script lang="ts">
  // DESIGN.md Places home, on the Board: every finder as a tile under its section heading, in the
  // Board's tile grid. A tile is a link in a Seam outline: the finder's name in Orange, since it is
  // what you press, and one Label line; where the finder already answered this session, that
  // answer's figure in Value L, Options Grey, with its context in Label. Opening this page asks
  // nothing (decision astromech-g8d5, item 6).
  import { homeSections, lastAnswer, placesRoute } from "../../lib/places";
  import Board from "../../lib/ui/layout/Board.svelte";
  import Tiles from "../../lib/ui/layout/Tiles.svelte";

  const sections = homeSections();
</script>

<Board>
  {#each sections as [heading, finders] (heading)}
    <section aria-labelledby="places-{heading}">
      <h2 class="section heading" id="places-{heading}">{heading}</h2>
      <Tiles>
        {#each finders as f (f.id)}
          {@const a = lastAnswer(f.id)}
          <a class="tile" href="#/{a?.route ?? placesRoute(f.id)}">
            <span class="type-name">{f.title}</span>
            <span class="type-label">{f.what}</span>
            {#if a}
              <span class="answer"
                ><span class="type-label context" title={a.context}>{a.context}</span><span class="fig"
                  ><span class="type-value-l">{a.figure}</span>{#if a.unit}<span class="type-unit">{a.unit}</span>{/if}</span
                ></span
              >
            {/if}
          </a>
        {/each}
      </Tiles>
    </section>
  {/each}
</Board>

<style>
  section + section { margin-top: var(--sp-8); }
  .heading { margin: 0 0 var(--sp-3); }
  .tile { display: grid; grid-template-rows: auto auto 1fr; gap: var(--sp-1); min-height: 6.5rem; padding: var(--sp-3); outline: 1px solid var(--line); outline-offset: -1px; text-decoration: none; }
  /* DESIGN.md Places home: a link in a Seam outline; the finder's name in Orange, since it is what
     you press, and a live answer's figure in Value L, Options Grey. */
  .tile .type-name { color: var(--accent); }
  .tile:hover { background: var(--row-hover); }
  .tile:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .tile:hover .type-name { color: var(--orange-lit); }
  .answer { align-self: end; display: flex; justify-content: space-between; align-items: baseline; gap: var(--sp-2); min-width: 0; }
  .context { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .fig { flex: none; white-space: nowrap; }
</style>
