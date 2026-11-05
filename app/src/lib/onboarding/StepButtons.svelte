<script lang="ts">
  // Step 3: press-to-bind push-to-talk and the kill switch. astromech only listens to the
  // button; nothing is sent to the game. Saved to astromech.toml by the core.
  import Button from "../ui/Button.svelte";
  import { coreCall, parseButton, type CoreStatus } from "../core";
  import { isWindowClick, type Press } from "../binds";
  import { binds } from "../store.svelte";
  import { control } from "../format";

  // `unread`: why there is no status yet (none read, or the reads failed), so a button is not
  // shown as "Not set" before astromech knows.
  let {
    status,
    unread = "checking",
    refresh,
  }: { status: CoreStatus | null; unread?: "checking" | "failed"; refresh: () => Promise<void> } = $props();

  type Which = "ptt" | "kill_switch";
  let capturing = $state<Which | null>(null);
  let seen = 0;
  let got = $state<Press | null>(null);
  let error = $state<string | null>(null);
  let saved = $state<string | null>(null);

  binds.start();

  function capture(w: Which) {
    capturing = w;
    got = null;
    error = null;
    saved = null;
    seen = binds.presses;
    binds.capturing = true;
  }
  function stop() {
    capturing = null;
    got = null;
    binds.capturing = false;
  }
  $effect(() => () => (binds.capturing = false));
  $effect(() => {
    const p = binds.last;
    if (!capturing || got || !p || binds.presses === seen || p.kind === "axis" || isWindowClick(p)) return;
    got = p;
  });

  const other = $derived(capturing === "ptt" ? status?.kill_switch : status?.ptt);
  const clash = $derived(!!got && !!other && other === `${got.device} ${got.key}`);
  const inGame = $derived(got ? binds.rowsFor(got.device, got.key).map((r) => r.name) : []);

  async function save() {
    if (!capturing || !got) return;
    const r = await coreCall(capturing === "ptt" ? "set_ptt" : "set_kill_switch", { device: got.device, key: got.key });
    if (!r.ok) {
      error = String(r.error);
      return;
    }
    saved = r.applies_next_session ? "Saved. Voice uses it the next time Elite starts." : "Saved.";
    stop();
    await refresh();
  }

  const shown = (s: string | null | undefined) => {
    if (!status) return unread === "failed" ? "Could not read" : "Checking…";
    const b = parseButton(s);
    return b ? control(b.device, b.key) : "Not set";
  };
</script>

<div class="setup-step">
  <dl>
    <dt>Push-to-talk</dt>
    <dd class:warn={status ? !status.ptt : unread === "failed"}><span>{shown(status?.ptt)}</span><Button onclick={() => capture("ptt")}>{status?.ptt ? "Change" : "Set"}</Button></dd>
    <dt>Kill switch</dt>
    <dd><span>{shown(status?.kill_switch)}</span><Button onclick={() => capture("kill_switch")}>{status?.kill_switch ? "Change" : "Set"}</Button></dd>
  </dl>
  {#if capturing}
    <p>{capturing === "ptt" ? "Press the button you'll hold to speak." : "Press a button Elite doesn't use, for the kill switch."}</p>
    <div class="press" class:got={!!got}>{got ? control(got.device, got.key) : "Press a button, hat or key…"}</div>
    {#if clash}
      <p class="warn-text">That's already the {capturing === "ptt" ? "kill switch" : "push-to-talk button"}. Pick another.</p>
    {:else if inGame.length}
      <p class="warn-text">Elite also uses it for {inGame.join(", ")}.{capturing === "ptt" ? " That's fine if you want both." : " The game would act on it too."}</p>
    {/if}
    <div class="acts">
      <Button disabled={!got || clash} onclick={save}>Save</Button>
      {#if got}<Button onclick={() => capture(capturing ?? "ptt")}>Press again</Button>{/if}
      <Button onclick={stop}>Cancel</Button>
    </div>
  {/if}
  {#if saved}<p>{saved}</p>{/if}
  {#if error}<p class="warn-text">{error}</p>{/if}
</div>
