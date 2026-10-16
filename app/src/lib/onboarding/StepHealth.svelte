<script lang="ts">
  // Step 6: everything voice needs, each with a fix: Elite's folders, push-to-talk, speech
  // models, calibration, and the keybinds health (actions voice presses need a keyboard key;
  // "Bind a key" is the same press-to-bind dialog as All keybinds).
  import { coreCall, type CoreStatus, type Found, type MicList } from "../core";
  import type { Row } from "../binds";
  import { binds } from "../store.svelte";
  import BindDialog from "../ui/BindDialog.svelte";
  import type { StepId } from "./steps";

  let { status, go }: { status: CoreStatus | null; go: (id: StepId) => void } = $props();

  let found = $state<Found | null>(null);
  let mics = $state<MicList | null>(null);
  let bind = $state<{ row: Row; slot: string } | null>(null);

  binds.start();
  $effect(() => {
    void coreCall("find_elite").then((r) => (found = r.ok ? (r.found as Found) : null));
    void coreCall("mic_list").then((r) => (mics = r.ok ? (r as unknown as MicList) : null));
    void binds.load();
  });

  interface Check { id: string; label: string; ok: boolean; required: boolean; text: string; fix?: { label: string; run: () => void } }

  const checks = $derived.by((): Check[] => {
    const s = status;
    const out: Check[] = [
      {
        id: "elite",
        label: "Elite's folders",
        ok: !!found?.ok,
        required: true,
        text: found?.ok ? "Found" : "Not found",
        fix: { label: "Find", run: () => go("elite") },
      },
      {
        id: "ptt",
        label: "Push-to-talk",
        ok: !!s?.ptt,
        required: true,
        text: s?.ptt ? "Set" : "Not set",
        fix: { label: "Set", run: () => go("buttons") },
      },
      {
        id: "kill",
        label: "Kill switch",
        ok: !!s?.kill_switch,
        required: false,
        text: s?.kill_switch ? "Set" : "Not set (recommended)",
        fix: { label: "Set", run: () => go("buttons") },
      },
      {
        id: "models",
        label: "Speech models",
        ok: !!s && !s.models_missing,
        required: true,
        text: s?.models_progress ?? (s && !s.models_missing ? "Installed" : "Not downloaded (about 800 MB)"),
        fix: s?.models_progress ? undefined : { label: "Download", run: () => void coreCall("download_models") },
      },
      {
        id: "calibration",
        label: "Microphone",
        ok: mics?.calibration.min_level_dbfs != null,
        required: false,
        text: mics?.calibration.min_level_dbfs != null ? "Calibrated" : mics?.names.length === 0 ? "No microphone found" : "Not calibrated",
        fix: { label: mics?.names.length === 0 ? "Pick" : "Calibrate", run: () => go(mics?.names.length === 0 ? "mic" : "calibrate") },
      },
    ];
    if (!binds.view) {
      out.push({ id: "binds", label: "Keybindings", ok: false, required: true, text: "No preset found to check", fix: { label: "Find", run: () => go("elite") } });
    } else {
      const bad = binds.rows.filter((r) => r.issues.some((i) => i.level === "error"));
      if (!bad.length) out.push({ id: "binds", label: "Keybindings", ok: true, required: true, text: `${binds.view.name}: no problems` });
      for (const r of bad) {
        const issue = r.issues.find((i) => i.level === "error");
        const empty = r.slots.find((x) => !x.device)?.slot ?? "Secondary";
        out.push({
          id: `binds-${r.id}`,
          label: r.name,
          ok: false,
          required: issue?.code === "no_keyboard",
          text: issue?.text ?? "",
          fix: issue?.code === "no_keyboard" ? { label: "Bind a key", run: () => (bind = { row: r, slot: empty }) } : undefined,
        });
      }
    }
    return out;
  });
  const ready = $derived(checks.every((c) => c.ok || !c.required));
</script>

<div class="setup-step">
  <dl>
    {#each checks as c (c.id)}
      <dt>{c.label}</dt>
      <dd class:warn={!c.ok && c.required}>
        <span>{c.text}</span>
        {#if !c.ok && c.fix}<button onclick={c.fix.run}>{c.fix.label}</button>{/if}
      </dd>
    {/each}
  </dl>
  {#if ready}
    <p>Ready. Start Elite, hold push-to-talk and say “gear down”.</p>
  {:else}
    <p class="lo">Fix the highlighted rows; voice needs them. The rest can wait.</p>
  {/if}
</div>

{#if bind}
  <BindDialog row={bind.row} slot={bind.slot} onclose={() => { bind = null; void binds.load(); }} />
{/if}
