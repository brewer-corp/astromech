<script lang="ts">
  import type { Callout } from "./data";
  // A generic flight-stick drawn by us (no vendor artwork). Anchor points are generic grip
  // positions; the callouts list which button sits there on this device.
  let { callouts, hand, active = "" }: { callouts: Callout[]; hand: string; active?: string } = $props();
  const anchors: Record<string, [number, number]> = {
    Trigger: [176, 252], "Red button": [222, 132], "Hat up": [168, 96], A2: [128, 140], A1: [150, 178],
    C1: [212, 182], Pinky: [150, 352], "Shifted hat": [140, 108],
  };
  const mirror = $derived(hand === "Left");
  const W = 620, H = 470, CX = 310;
  const placed = $derived.by(() => {
    const axOf = (c: Callout) => {
      const a = anchors[c.label] ?? [180, 200];
      return CX - 180 + (mirror ? 360 - a[0] : a[0]);
    };
    // Each label goes on the side its button is on, so leader lines never cross the grip.
    const L = callouts.filter((c) => axOf(c) <= CX), R = callouts.filter((c) => axOf(c) > CX);
    const ay = (c: Callout) => (anchors[c.label] ?? [180, 200])[1];
    const pos = (list: Callout[], side: "l" | "r") =>
      [...list].sort((a, b) => ay(a) - ay(b)).map((c, i) => {
        const a = anchors[c.label] ?? [180, 200];
        const ax = CX - 180 + (mirror ? 360 - a[0] : a[0]);
        return { c, ax, ay: a[1] + 20, lx: side === "l" ? 20 : W - 20, ly: 60 + i * (360 / Math.max(list.length, 1)) + 20, side };
      });
    return [...pos(L, "l"), ...pos(R, "r")];
  });
</script>

<svg viewBox="0 0 {W} {H}" class="stick" aria-label="{hand} stick">
  <defs>
    <linearGradient id="grip" x1="0" x2="1">
      <stop offset="0" stop-color="#1c130b" /><stop offset="0.5" stop-color="#2a1b0e" /><stop offset="1" stop-color="#140d07" />
    </linearGradient>
  </defs>
  <g transform="translate({CX - 180} 20) {mirror ? 'translate(360 0) scale(-1 1)' : ''}">
    <!-- base -->
    <path d="M60 410 L300 410 L320 440 L40 440 Z" fill="#120c07" stroke="var(--am-orange-line)" />
    <rect x="160" y="370" width="40" height="42" fill="#160e08" stroke="var(--am-orange-line)" />
    <!-- grip -->
    <path d="M120 70 Q180 40 240 70 L262 150 Q270 200 246 240 L236 330 Q230 372 180 376 Q130 372 124 330 L116 240 Q96 200 104 150 Z"
      fill="url(#grip)" stroke="var(--am-orange)" stroke-width="1.5" />
    <path d="M130 250 Q180 262 230 250" fill="none" stroke="var(--am-orange-line)" />
    <path d="M128 290 Q180 302 232 290" fill="none" stroke="var(--am-orange-line)" />
    <!-- head controls -->
    <circle cx="168" cy="96" r="13" fill="none" stroke="var(--am-orange)" /><circle cx="168" cy="96" r="4" fill="var(--am-orange)" />
    <circle cx="222" cy="132" r="9" fill="var(--am-red)" opacity="0.85" />
    <rect x="118" y="130" width="18" height="18" fill="none" stroke="var(--am-orange)" />
    <rect x="142" y="168" width="16" height="16" fill="none" stroke="var(--am-orange)" />
    <circle cx="212" cy="182" r="10" fill="none" stroke="var(--am-orange)" />
    <path d="M196 236 Q176 232 168 262 L176 274 Q186 252 200 252 Z" fill="var(--am-orange-dim)" stroke="var(--am-orange)" />
    <rect x="140" y="342" width="22" height="12" fill="none" stroke="var(--am-orange)" />
  </g>
  {#each placed as p (p.c.id)}
    {@const on = p.c.id === active}
    {@const col = p.c.warn ? "var(--am-amber)" : p.c.voice ? "var(--am-blue)" : "var(--am-orange)"}
    <polyline points="{p.ax},{p.ay} {p.side === 'l' ? p.lx + 160 : p.lx - 160},{p.ly} {p.lx + (p.side === 'l' ? 150 : -150)},{p.ly}"
      fill="none" stroke={col} stroke-width={on ? 2 : 1} opacity={on ? 1 : 0.7} />
    <circle cx={p.ax} cy={p.ay} r={on ? 6 : 3.5} fill={col} />
    {#if on}<circle cx={p.ax} cy={p.ay} r="13" fill="none" stroke={col} opacity="0.6" />{/if}
  {/each}
</svg>
<div class="labels">
  {#each placed as p (p.c.id)}
    <div class="lab {p.side}" class:on={p.c.id === active} class:warn={!!p.c.warn} class:voice={p.c.voice}
      style="top:{(p.ly / H) * 100}%">
      <span class="id">{p.c.id}</span>
      <span class="act">{p.c.action}</span>
      {#if p.c.warn}<span class="sub">! {p.c.warn}</span>{:else if p.c.kb}<span class="sub">⌨ {p.c.kb.replace("Key_", "")}</span>{/if}
    </div>
  {/each}
</div>

<style>
  .stick { width: 100%; height: auto; display: block; }
  .labels { position: absolute; inset: 0; pointer-events: none; }
  .lab {
    position: absolute;
    width: 25%;
    transform: translateY(-100%);
    padding: 0 0 3px;
    font: 500 var(--am-size-s) var(--am-font-ui);
    color: var(--am-text);
    display: flex; flex-direction: column;
  }
  .lab.l { left: 3%; }
  .lab.r { right: 3%; text-align: right; }
  .id { font: 400 var(--am-size-xs) var(--am-font-mono); color: var(--am-orange); }
  .act { color: var(--am-text-strong); }
  .sub { font-size: var(--am-size-xs); color: var(--am-muted); }
  .warn .id, .warn .sub { color: var(--am-amber); }
  .voice .id { color: var(--am-blue); }
  .on .act { color: var(--am-orange-bright); text-shadow: var(--am-orange-glow); }
</style>
