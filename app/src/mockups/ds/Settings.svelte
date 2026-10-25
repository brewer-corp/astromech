<script lang="ts">
  // DESIGN.md mockup: Settings, the game's Options screen. Grey labels, ember cells holding
  // orange values, one orange action word per cell, the chosen option filled, orange section
  // heads with a rule. Read-only facts (Speech models) sit outside a cell.
  import Chrome from "./Chrome.svelte";
  import Icon from "./Icon.svelte";

  type Row =
    | { label: string; kind: "value"; value: string; action?: string; binding?: boolean; dropdown?: boolean }
    | { label: string; kind: "fact"; value: string }
    | { label: string; kind: "choice"; options: string[]; on: number; seconds?: number };
  const groups: [string, Row[], string?][] = [
    ["Startup", [
      { label: "Start with Windows", kind: "choice", options: ["On", "Off"], on: 0 },
      { label: "Setup", kind: "value", value: "Done", action: "Run again" },
    ], "Starts quietly in the tray and waits for Elite, so voice is ready when you launch the game."],
    ["Voice", [
      { label: "Push-to-talk", kind: "value", value: "L · Joy 4", action: "Change", binding: true },
      { label: "Kill switch", kind: "value", value: "Scroll Lock", action: "Change", binding: true },
      { label: "Speech models", kind: "fact", value: "Installed" },
      { label: "Voice", kind: "value", value: "Listening while Elite runs", action: "Pause" },
    ]],
    ["Voice responses", [
      { label: "Speaker", kind: "value", value: "Emma · female", action: "Preview", dropdown: true },
      { label: "Colour", kind: "choice", options: ["Natural", "Cockpit", "Harsh"], on: 1 },
      { label: "Speed", kind: "value", value: "Normal (the colour's own pace)", dropdown: true },
    ], "Cockpit sounds like the ship's PA, close to Elite's own voice. Changes apply to the next reply."],
    ["Callouts", [
      { label: "Arrival briefing", kind: "choice", options: ["On", "Off"], on: 0, seconds: 0 },
      { label: "Powerplay arrival", kind: "choice", options: ["On", "Off"], on: 1, seconds: 0 },
      { label: "Scans", kind: "choice", options: ["On", "Off"], on: 0, seconds: 30 },
    ]],
  ];
</script>

<Chrome top="Settings" tabs={["General", "Data sharing", "Diagnostics", "Voice tester"]} tab="General"
  strip={{ next: "Wregoe OI-C b54-6", of: "3 of 41", flags: ["Scoopable"], dest: "Colonia", left: "38 jumps left" }}>
  <div class="settings">
    {#each groups as [head, rows, note] (head)}
      <section>
        <h2 class="section">{head}</h2>
        {#each rows as r (r.label)}
          <div class="row">
            <span class="lab">{r.label}</span>
            {#if r.kind === "fact"}
              <span class="fact">{r.value}</span>
            {:else if r.kind === "choice"}
              <span class="choiceline">
                <span class="choices">{#each r.options as o, i (o)}<button class="choice" class:on={i === r.on}>{o}</button>{/each}</span>
                {#if r.seconds !== undefined}<span class="cell num">{r.seconds}</span><span class="label">s quiet after it speaks</span>{/if}
              </span>
            {:else}
              <span class="cell value" class:binding={r.binding}>
                <span>{r.value}</span>
                <span class="end">{#if r.dropdown}<Icon name="chevron" />{/if}{#if r.action}<button class="act">{r.action}</button>{/if}</span>
              </span>
            {/if}
          </div>
        {/each}
        {#if note}<p class="note prose">{note}</p>{/if}
      </section>
    {/each}
  </div>
</Chrome>

<style>
  .settings { max-width: var(--form-w); display: grid; gap: var(--sp-8); }
  .section { margin-bottom: var(--sp-3); }
  .row { display: grid; grid-template-columns: 200px 1fr; align-items: center; min-height: var(--row); margin-bottom: var(--gap); }
  .lab { color: var(--options-grey); padding-left: var(--sp-3); }
  .fact { padding: 0 var(--sp-3); color: var(--accent); }
  .value { justify-content: space-between; }
  .value.binding > span:first-child { color: var(--options-grey); }
  .end { display: flex; align-items: center; gap: var(--sp-2); margin-right: calc(-1 * var(--sp-2)); }
  .choiceline { display: flex; align-items: center; gap: var(--sp-3); }
  .num { width: 56px; justify-content: flex-end; }
  .note { margin: var(--sp-2) 0 0 200px; font-size: var(--fs-s); padding-left: var(--sp-3); max-width: var(--measure); }
</style>
