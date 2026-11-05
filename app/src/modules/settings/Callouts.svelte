<script lang="ts">
  // Each callout's switch and cooldown, written to [callouts.<id>] in astromech.toml by the core.
  // A change applies from the next session. The cooldown is how long a callout stays quiet after
  // it speaks, so a burst of the same event gives one line.
  import Button from "../../lib/ui/Button.svelte";
  import Choice from "../../lib/ui/Choice.svelte";
  import Input from "../../lib/ui/Input.svelte";
  import { changeCallout, loadCallouts, parseCooldown, type CalloutSetting } from "../../lib/callouts";
  import { announcer } from "../../lib/ui/announce";

  let list = $state<CalloutSetting[]>([]);
  /** The last save's "Saved" or error, for the live region below. */
  let status = $state("");
  let warn = $state(false);

  const announce = announcer((t) => (status = t));
  function say(text: string, isError: boolean) {
    warn = isError;
    return announce(text);
  }

  async function reload() {
    list = await loadCallouts();
  }
  $effect(() => {
    reload().catch((e: unknown) => say(String(e), true));
  });

  /** `box`: the cooldown box the change came from; it goes back to the saved value if refused. */
  async function change(c: CalloutSetting, what: { enabled: boolean } | { cooldown_s: number | null }, box?: HTMLInputElement) {
    try {
      await changeCallout(c.id, what);
      await reload();
      await say("Saved. It applies the next time Elite starts.", false);
    } catch (e) {
      if (box) box.value = String(c.cooldown_s);
      await say(`${c.title}: ${String(e)}`, true);
    }
  }

  function setCooldown(c: CalloutSetting, box: HTMLInputElement) {
    const s = parseCooldown(box.value);
    if (s === "invalid") {
      box.value = String(c.cooldown_s);
      void say(`${c.title}: enter a number of seconds from 0 to 86400.`, true);
      return;
    }
    void change(c, { cooldown_s: s }, box);
  }
</script>

<dl>
  {#each list as c (c.id)}
    <dt>{c.title}</dt>
    <dd>
      <Choice
        label={`${c.title} on or off`}
        value={c.enabled ? "on" : "off"}
        options={[{ value: "on", label: "On" }, { value: "off", label: "Off" }]}
        onchange={(v) => {
          if ((v === "on") !== c.enabled) void change(c, { enabled: v === "on" });
        }}
      />
      <label>
        <Input
          class="num"
          type="text"
          inputmode="decimal"
          aria-label={`${c.title} cooldown, seconds`}
          value={c.cooldown_s}
          onchange={(e) => setCooldown(c, e.currentTarget)}
        />
        <span class="lo">s quiet after it speaks</span>
      </label>
      {#if c.cooldown_s !== c.default_cooldown_s}
        <Button onclick={() => change(c, { cooldown_s: null })}>Reset to {c.default_cooldown_s} s</Button>
      {/if}
    </dd>
  {/each}
</dl>
<!-- Always there, so a screen reader announces each "Saved" or error, repeats included. -->
<p class="hint" class:warn-text={warn} aria-live="polite">{status}</p>

<style>
  dl { display: grid; grid-template-columns: 14rem 1fr; gap: var(--gap); margin: 0; }
  dt, dd { margin: 0; padding: 0 12px; min-height: var(--row); display: flex; align-items: center; }
  dt { color: var(--options-grey); }
  /* The switch and the cooldown are controls of their own, so the row has no cell around them. */
  dd { padding: 0; gap: var(--sp-3); }
  label { display: flex; align-items: center; gap: 8px; }
  label > :global(.num) { width: 4.5rem; }
  .lo { color: var(--orange-mid); font-size: var(--fs-s); }
  .hint { color: var(--prose); margin: 8px 0 0 calc(14rem + var(--gap)); font-size: var(--fs-s); line-height: 1.6; }
  .hint:empty { margin: 0; }
  /* A failed save or a refused value: a red "!" and plain words under the field. */
  .warn-text { color: var(--alarm); }
  .warn-text::before { content: "! "; font-weight: 600; }
</style>
