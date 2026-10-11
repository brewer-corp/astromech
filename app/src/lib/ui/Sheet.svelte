<script lang="ts">
  // VKB's reference sheet with our text in VKB's own boxes. Box positions come from the device
  // map (PDF points, origin bottom left, page 792 x 612).
  import type { MapControl } from "../devices";
  export type Cell = { text: string; other?: string; hi?: boolean; sel?: boolean; empty?: boolean };
  let {
    image,
    controls,
    cells,
    width,
    dimOthers = false,
    onpick,
  }: {
    image: string;
    controls: MapControl[];
    cells: Record<string, Cell>;
    width: number;
    dimOthers?: boolean;
    onpick?: (c: MapControl) => void;
  } = $props();
  const k = $derived(width / 792);
  const label = (joy: string) => (joy.startsWith("Joy_POV") ? "P" + joy.charAt(7) : joy.replace("Joy_", "").replace("Axis", ""));
  const pos = (r: number[]) => {
    const [x1 = 0, y1 = 0, x2 = 0, y2 = 0] = r;
    return `left:${x1 * k}px;top:${(612 - y2) * k}px;width:${(x2 - x1) * k}px;height:${(y2 - y1) * k}px`;
  };
</script>

<div class="sheet" style="width:{width}px;height:{612 * k}px;--k:{k}">
  <img src={image} alt="VKB reference sheet" />
  {#each controls as c (c.sheet_field)}
    {@const cell = cells[c.sheet_field]}
    {#if c.box?.num && c.joy}
      <span class="num" style={pos(c.box.num)}>{label(c.joy)}</span>
    {/if}
    {#if c.box?.txt}
      <button
        type="button"
        class="txt"
        class:hi={cell?.hi}
        class:sel={cell?.sel}
        class:dim={dimOthers && !cell?.hi}
        class:empty={!cell || cell.empty}
        style={pos(c.box.txt)}
        title={c.control}
        onclick={() => onpick?.(c)}
      >
        {#if cell}<b>{cell.text}</b>{#if cell.other}<i>{cell.other}</i>{/if}{/if}
      </button>
    {/if}
  {/each}
</div>

<style>
  .sheet { position: relative; }
  img { width: 100%; height: 100%; display: block; }
  .num, .txt { position: absolute; display: flex; align-items: center; overflow: hidden; white-space: nowrap; font-family: var(--font); }
  .num { justify-content: center; font-size: calc(9.5px * var(--k)); font-weight: 600; color: var(--text-lo); pointer-events: none; }
  .txt { all: unset; position: absolute; display: flex; align-items: center; overflow: hidden; white-space: nowrap; box-sizing: border-box; padding: 0 calc(4px * var(--k)); gap: calc(6px * var(--k)); font-size: calc(10px * var(--k)); color: var(--text); cursor: pointer; }
  .txt.dim { color: var(--text-lo); }
  .txt b { font-weight: 500; overflow: hidden; text-overflow: ellipsis; }
  .txt i { font-style: normal; margin-left: auto; color: var(--text-lo); }
  .txt.hi { background: var(--accent); color: var(--on-accent); }
  .txt.hi i { color: var(--on-accent); }
  .txt.hi b { font-weight: 600; }
  .txt.sel { outline: 2px solid var(--text-hi); outline-offset: -2px; }
  .txt:focus-visible { outline: 2px solid var(--accent); outline-offset: -2px; }
</style>
