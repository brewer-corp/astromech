<script lang="ts">
  // VKB's reference sheet with our labels in VKB's own boxes. Box positions come from the
  // device map (PDF points, origin bottom left, page 792 x 612).
  type Box = { num?: number[]; txt?: number[] };
  type Control = { sheet_field: string; joy: string | null; box?: Box | null };
  export type Cell = { text: string; other?: string; hi?: boolean; sel?: boolean };
  let { image, controls, cells, width, dimOthers = false }: { image: string; controls: Control[]; cells: Record<string, Cell>; width: number; dimOthers?: boolean } = $props();
  const k = $derived(width / 792);
  const label = (joy: string) => (joy.startsWith("Joy_POV") ? "P" + joy.charAt(7) : joy.replace("Joy_", ""));
  const pos = (r: number[]) => {
    const [x1 = 0, y1 = 0, x2 = 0, y2 = 0] = r;
    return `left:${x1 * k}px;top:${(612 - y2) * k}px;width:${(x2 - x1) * k}px;height:${(y2 - y1) * k}px`;
  };
</script>

<div class="sheet" style="width:{width}px;height:{612 * k}px;--k:{k}">
  <img src={image} alt="VKB reference sheet" />
  {#each controls as c (c.sheet_field)}
    {@const cell = c.joy ? cells[c.joy] : undefined}
    {#if c.box?.num && c.joy && !c.joy.endsWith("Axis")}
      <span class="num" style={pos(c.box.num)}>{label(c.joy)}</span>
    {/if}
    {#if c.box?.txt && cell}
      <span class="txt" class:hi={cell.hi} class:sel={cell.sel} class:dim={dimOthers && !cell.hi} style={pos(c.box.txt)}>
        <b>{cell.text}</b>{#if cell.other}<i>{cell.other}</i>{/if}
      </span>
    {/if}
  {/each}
</div>

<style>
  .sheet { position: relative; }
  img { width: 100%; height: 100%; display: block; }
  span { position: absolute; display: flex; align-items: center; overflow: hidden; white-space: nowrap; font-family: var(--font); }
  .num { justify-content: center; font-size: calc(9.5px * var(--k)); font-weight: 600; color: var(--text-lo); }
  .txt { padding: 0 calc(4px * var(--k)); gap: calc(6px * var(--k)); font-size: calc(10px * var(--k)); color: var(--text); }
  .txt.dim { color: var(--text-lo); }
  .txt b { font-weight: 500; }
  .txt i { font-style: normal; margin-left: auto; }
  .txt.hi { background: var(--accent); color: #000; }
  .txt.hi b { font-weight: 600; }
  .txt.sel { outline: 2px solid #000; outline-offset: -2px; }
</style>
