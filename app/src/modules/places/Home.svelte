<script lang="ts">
  // DESIGN.md Places home: every finder as a tile under its section heading. A tile is a
  // read-only Hull link with one main value: the finder's name in Orange and one Label line;
  // where the finder already answered this session, that answer's figure is the main value, in
  // Value L, Orange Lit, with its context in Label. Opening this page asks nothing (decision
  // astromech-g8d5, item 6), and a tile with no answer has no Lit text.
  import { homeSections, lastAnswer, placesRoute } from "../../lib/places";

  const sections = homeSections();
</script>

<section class="home">
  {#each sections as [heading, finders] (heading)}
    <section aria-labelledby="places-{heading}">
      <h2 class="section heading" id="places-{heading}">{heading}</h2>
      <div class="tiles">
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
      </div>
    </section>
  {/each}
</section>

<style>
  /* Three 220 px tiles wide, as the mockup (docs/design/design-system-places-home-after.png). */
  .home { --w: calc(3 * 220px + 2 * var(--sp-2)); display: flex; flex-direction: column; gap: var(--sp-6); padding: var(--sp-8) 0 72px; max-width: var(--w); }
  .heading { margin: 0 0 var(--sp-3); }
  .tiles { display: grid; grid-template-columns: repeat(auto-fill, 220px); gap: var(--sp-2); }
  .tile { display: grid; grid-template-rows: auto auto 1fr; gap: var(--sp-1); min-height: 104px; padding: var(--sp-3); background: var(--surface); color: var(--accent); text-decoration: none; }
  .tile:hover { background: var(--row-hover); }
  .tile:focus-visible { outline: 1px solid var(--accent); outline-offset: 2px; }
  .answer { align-self: end; display: flex; justify-content: space-between; align-items: baseline; gap: var(--sp-2); min-width: 0; }
  .context { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .fig { flex: none; white-space: nowrap; }
</style>
