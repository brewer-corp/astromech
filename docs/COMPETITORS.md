# Competitive landscape: copilots, companions, and binds tools

*Prepared 11 Oct 2026. Companion to [REPORT.md](REPORT.md) (8 Oct, data sources and the then-current tool survey) and [V1-DESIGN.md](V1-DESIGN.md) (what v1 is). Catalog: [ed.tools](https://ed.tools/); recent entries also on [EDCodex](https://edcodex.info/?m=tools).*

**How to read this.** REPORT.md is still the reference for journal/Status/cAPI, how `SendInput` works, and EDCoPilot / COVAS:NEXT internals as of 8 Oct. This document is the three-week flood after that: who showed up, how they differ from astromech, and what to copy.

- **(verified)** means confirmed from a live public page, release notes, or a current source file on 11 Oct.
- **(inferred)** is reasoning from that public evidence.
- **(unverified)** is reported but not confirmed.

No EDCoPilot binary was inspected (its EULA; see REPORT §8.1). Closed tools (AURA, CanopyNav, WickedWake, HCS) are described from their public sites and EDCodex entries only.

---

## 1. Executive summary

A cluster of projects shipped in Aug–Oct 2026 that aspire to the same two jobs as astromech: **talk to the ship**, and **make binds usable**. Almost all of them are **LLM copilots that also parse `.binds`**. astromech’s v1 bet is the inverse: a **grammar-first local PTT path that works with no LLM**, plus a **keybinds product**.

**Direct copilots (voice → keys):** EliteIntel 1.1, COVAS:NEXT 4.3, Directive 47 (d47), AURA App Suite, CanopyNav, ED Mission Operator (EDMO), WickedWake-Copilot, plus the incumbents EDCoPilot + VoiceAttack + HCS.

**The real split** is who is allowed to press a key. EliteIntel, COVAS, WickedWake and AURA (LLM mode) need a language model to act. d47 and astromech put a **deterministic matcher in front**; d47 still offers an optional model for the rest. CanopyNav uses a wake word. astromech v1 has no model and no wake word.

**Unoccupied ground** (as of 11 Oct): manufacturer sheets on *your* devices (Activities / My controller), PID-drift / Joy_32 / firmware health, a kill switch and policy tiers rather than a prompt, and “gear down” on a quiet CPU with no API key.

**Copy from them, in order:** EliteIntel BindForge’s edit/conflict pipeline (CC0); PTT as exclusive gate plus barge-in; d47’s router-before-model, fuzzy-offer-not-run, and capability registry that *is* the docs. Do not copy LLM-required startup, wake words, auto-honk / auto-neutron, or their panel zoos.

---

## 2. Method and limits

**Catalogs used (verified):**

- [ed.tools](https://ed.tools/) — community directory, tags include Voice, Companion, Controls, HUD, AI.
- [EDCodex tools](https://edcodex.info/?m=tools) — dated entries; CanopyNav (entry 661, 5 Oct), AURA (entry 659, 4 Oct), EDMO (entry 642, 15 Aug), EDU (entry 636).

**Primary pages read 11 Oct:**

- EliteIntel: https://www.elite-intel.org/, GitHub `SudoKrondor/EliteIntel` README / TECHNICAL.md / bindforge `PACKAGE.md` / recent commits, v1.1 video description.
- COVAS:NEXT: GitHub `RatherRude/Elite-Dangerous-AI-Integration` latest release **v4.3.0** (17 Sep 2026).
- Directive 47: https://github.com/dseelinger/d47, https://dseelinger.github.io/d47/ and `conversation.html` / `capabilities/help.html`.
- AURA: https://edcodex.info/?m=tools&entry=659, https://aura-command.net, Frontier thread “Aura app suite pre-release”.
- CanopyNav: https://edcodex.info/?m=tools&entry=661.
- EDMO: https://edcodex.info/?m=tools&entry=642, https://github.com/Mallock/edmo.
- WickedWake: https://wwt.wtf/copilot, Frontier thread 637182.
- WOLPERTINGER: https://github.com/KeilerHirsch/WOLPERTINGER.
- ED Unified: https://edcodex.info/index.php?m=tools&entry=636.

**Not done:** no new clones of third-party trees (REPORT’s 8 Oct clones are not in this repo). BindForge details are from the live `PACKAGE.md` on GitHub, not a local copy.

---

## 3. Direct copilots

| Project | Licence / cost | Stack | Brain | What it is | vs astromech |
|---|---|---|---|---|---|
| **[EliteIntel](https://www.elite-intel.org/)** VEGA | CC0-1.0 | Java 21, Swing, C OpenVR overlay | **LLM required.** Parakeet STT, Kokoro TTS; LM Studio / Gemma local or Mistral free cloud; also Anthropic, OpenAI, xAI, Gemini | Natural-language ship control, Spansh/EDSM analysis, binds editor, overlay, custom commands, 9 languages | Same voice stack. They cannot start without a model. Binds editor is the thing to copy. v1.2 = BindForge / controller management (commits 3–5 Oct). |
| **[COVAS:NEXT](https://github.com/RatherRude/Elite-Dangerous-AI-Integration)** | MIT | Python + Electron | LLM + tools. Parakeet / faster-whisper; Kokoro / Edge / Mistral STT+TTS+LLM | Conversational ship computer: event reactions, plotting, OCR, VR overlay, plugin API, EDCoPilot as TTS plugin | Closest “talk to the ship” product. Heavier, cloud-happy. Plot macros. Your policy + verification is stricter. |
| **[Directive 47](https://github.com/dseelinger/d47)** | MIT | .NET 10, Avalonia, SteamVR overlay | **Keyword router first, LLM second.** Provider `none` is supported | Guardian-flavoured companion: journal Q&A, callouts, keypresses, ~45 documented capabilities, one panel on desktop and VR | Philosophically the closest. Router-before-model, receipts, help generated from a capability registry. Tiny user base. |
| **[AURA App Suite](https://aura-command.net)** | Proprietary, Microsoft Store | WebView2 | Optional OpenAI + ElevenLabs. 54 one-word voice shortcuts | Cockpit-style companion: Jump Dynamics, auto-focus panels, engineering/trade/exo, VEIL HUD, voice | Pretty dashboard + optional chat. Not local-first. Binds catalog is a feature, not a product. |
| **[CanopyNav](https://edcodex.info/?m=tools&entry=661)** | Commercial: 14-day trial, then $19.99 lifetime | Closed Windows app | Wake word “Computer” / “Epiphany”. Core offline; optional Google chat | Transparent HUD + voice: gear/scoop/docking/FSS, Spansh neutron/FC routes, colonisation shopping, exo, mining, Rhino cargo | Activity HUD you parked for later. Wake word. Overlay-in-cockpit (v1 dropped VR UI). |
| **[ED Mission Operator / EDMO](https://github.com/Mallock/edmo)** | MIT | **Tauri** + llama.cpp + Piper + whisper.cpp | Local LLM optional | Always-on-top mission HUD: synthesized checklists, expiry timers, PTT with barge-in, colonisation shopping list | Same shell as you. Mission cards + barge-in are stealable. Scope is missions, not binds. |
| **[WickedWake-Copilot](https://wwt.wtf/copilot)** | Closed, signed binary | Unknown | Google AI Studio key required | “Reliable tool calling” LLM agent, Edge STT, WebRTC VAD, binds validation | Cloud-only. Alpha (May 2025); signups full as of 11 Oct. |
| **EDCoPilot** + VoiceAttack + **HCS** | EDCoPilot free (Patreon); VA and HCS paid | Python/Kivy + locked VA profile | No STT of its own; ~3,000 VA commands; Edge/cloud TTS | Feature ceiling of the genre: 30+ panels, plotting, Spansh, exo, colonisation, chatter | The zoo v1 is not building. Still the comparison target. Internals: REPORT §8. |

### 3.1 EliteIntel (verified, 11 Oct)

- **v1.1** (Aug 2026): redesigned UI and LLM pipeline, **push-to-talk**, bindings editor (conflict/missing detect, auto-fix, save/load), configurable overlay, any input/output device, conversational memory, localisation (EN/ES/FR/DE/IT/PT/BR/UK/RU), custom commands, ~50% lower token use. Source: [v1.1 video description](https://www.youtube.com/watch?v=Fh8Za7_NLF0) and [elite-intel.org](https://www.elite-intel.org/).
- **LLM is required.** Site copy: “Elite Intel is the voice and the hands — the LLM is the brain. Without one, the app cannot run.” Recommended: Mistral free cloud or `google/gemma-4-e4b` locally. Older Tulu is “no longer sufficient.”
- **PTT (TECHNICAL.md):** the mapped controller button alone opens and closes recording. Nothing said before the press is in the buffer; the noise gate never opens a window of its own.
- **Echo (commit 4 Oct):** “Stopped hands-free STT from hearing VEGA’s own voice over speakers.”
- **Chat channel (commit 2 Oct):** typed VEGA commands through in-game chat (`@Vega …`).
- **BindForge** (`app/src/main/java/elite/intel/bindforge/PACKAGE.md`, CC0):
  - Working copy per preset with SHA-256 baseline. Three states: clean / pending draft / conflict (both EI and the game changed → apply blocked).
  - Surgical regex XML (preserve unknown bytes). Stale-file guards on mtime+size. Atomic write.
  - Conflict rules: same chord on ship vs SRV vs on-foot is a layout, not a clash; UI-mode overlays (FSS, SAA, FreeCam, map camera) ignored; dangerous pairs have curated descriptions.
  - Missing-binding auto-assigner with a safe unused-key pool.
  - Spoken chords: “Left Control plus W”.
  - v1.2 work: `bindforge.devices` / `bindforge.install` — “comprehensive controller management” (YouTube comment 1 Sep; commits 3 Oct “Refactoring hands package in preparation for V1.2 - BindForge”).
- Latest installer seen: **v-1.1.0024**, 4 Oct 2026. Linux installer creates Proton `ed-bindings` / `ed-journal` symlinks.

### 3.2 COVAS:NEXT v4.3.0 (verified, 17 Sep 2026 release)

New since REPORT §7:

- Mistral as STT/TTS/embedding/LLM provider.
- Event reactions for SCA/SCO activated/deactivated.
- **Actions tab:** trigger mainship, SLF, SRV and suit actions manually (same tools the voice uses).
- VR: auto-install a custom OpenXR API layer when the overlay extension is missing; Linux SteamVR overlays; Vulkan/DXVK.
- Flatscreen: follow game window position; Wayland layer-shell.
- **In-system plot** (stations and landfall planets) exists and is **disabled by default** as not yet reliable.
- Plotter still verifies via `NavRoute.json` (REPORT §6). That remains the benchmark for any later plot feature.

### 3.3 Directive 47 (verified, 11 Oct)

- Windows 11, .NET 10, MIT. Core has no UI/audio/vendor SDK; spokes: Audio, Knowledge, Llm, Stt, Tts, Vr. Per-user installer, no elevation.
- **Two answer paths, fixed order:** keyword router first, language model only on miss. A setting that gates keyboard input is reachable by voice **only through the router**. Source: [conversation.html](https://dseelinger.github.io/d47/conversation.html).
- Provider `none` is a supported setup. Lookups and keypresses the router owns still work.
- Fuzzy: treat `{on,to}`, `{put,set}`, `{elite,the game}` as the same word. One unguarded unique match **runs**; anything that presses a key, or a protected setting, is **offered** (“Did you mean …?”), never auto-run.
- Every turn prints a **receipt**: route (router vs model), effort, tokens, cost, voice characters billed.
- Effort is chosen per turn from the question shape (Low/Medium/High/Max); floor and ceiling are settings; the model cannot turn effort off.
- Guardrails sit **above** the persona. Switching personality off cannot reach them. The model cannot reset safety settings; per-row ↺ reset never clears API keys.
- **Capability registry is the docs.** CI fails if a registered capability has no page, the page’s schema doesn’t match, the page is in the wrong group, or a published page is unreachable from the nav. “What can you do?” is projected from the registry, never generated. Spoken walkthroughs: “walk me through engineers” → step / next / back / stop.
- Frontier media-usage permission for shipped game data; NOTICE splits code licence from Frontier tables.

### 3.4 AURA App Suite (verified from EDCodex + Frontier thread)

- Proprietary, Microsoft Store, WebView2. Journal + Status plus EDSM/Spansh. Distinguishes public catalog data from confirmed in-game discovery.
- Jump Dynamics: destination lookup when you lock a target.
- **Auto Focus:** bring relevant panels forward from flight context; pinned panels and unfinished edits win.
- Voice: PTT or text, optional OpenAI, optional ElevenLabs, personality + reply voice.
- **VC Catalog:** 54 one-word shortcuts, active binding details, guidance for menu navigation (Frontier thread, page 4).
- Suite: AURA + VEIL (seven context HUDs over an authenticated local link) + XENO_X + SDR. 43 EDHM themes.
- Internet required for “online system intelligence.” Some voice features need API credentials.

### 3.5 CanopyNav (verified, EDCodex 5 Oct 2026)

- Author: Cmdr Chikote. Wake word “Computer” or “Epiphany”.
- Voice: landing gear, cargo scoop, docking, supercruise, FSS/DSS, more.
- Routes: in-game, Spansh neutron, multi-jump fleet carrier, typed into the galaxy map.
- Colonisation: depot vs ship vs carrier, supplier stations, plot to them.
- Exo: predicted species, payouts, sample tracking, clone-distance warnings.
- Mining: ring/surface, Rhino cargo, surface site survey, best-buyer.
- HUD pins via Oculus Dash or Virtual Desktop. Core offline; optional Google AI chat.
- Commercial. SmartScreen on first run (unsigned).

### 3.6 ED Mission Operator (verified, EDCodex 15 Aug 2026)

- Mallock, MIT, Tauri. Windows per-user installer + Linux .deb/AppImage. Same Tauri-shell family as astromech’s app window; REPORT appendix already cites `Mallock/edmo`.
- Journal read-only. Never writes the game folders.
- Mission cards with **synthesized objective checklists** the game never shows; expiry countdowns; restore from `Missions.json`.
- Local llama.cpp or LM Studio; Piper TTS; whisper.cpp PTT with **barge-in** (press to talk, operator stops mid-sentence).
- Heartbeat nudges: idle at a station with hand-ins, drifting without jumping, in a kill-mission system without engaging.
- Extra tabs: neutron/FC plotter, World of Death landing clock, colonisation shopping list, “local wire” (fictional news from true journal facts; must not invent factions/stations/prices).

### 3.7 WickedWake-Copilot (verified from site + Frontier thread May 2025)

- Author: John Weeks. Closed, compiled and signed. Google AI Studio key required.
- Customizable system prompt, Edge STT, Google generative TTS, PTT or open-mic WebRTC VAD, keybinding validation, event-based AI responses.
- Positioned as “reliable task calling” after the author found other bots unreliable in-game.
- Downloads page still “Coming Soon / alpha full” on 11 Oct.

### 3.8 Incumbents (unchanged in role; see REPORT §7–8)

EDCoPilot remains the feature-complete “Alexa in space”: panels, chatter, Spansh, plotting, VoiceAttack grammar. HCS Singularity 4.13 (Sep 2026) added Rhino throttle-by-voice and Nomad ship commands. EDDI is still the mature event→speech engine. None of these are the new deluge; they are the ceiling commanders already know.

---

## 4. Adjacent tools (same itch, different shape)

| Project | What it is | Why it matters |
|---|---|---|
| **[ED Unified](https://github.com/RedUnix/ed-unified)** (MIT, Jun 2026) | Launcher: browse EDCodex, chain game + companions, pin any tab as overlay, `!inara Sol` from chat | Admits the fragmentation problem v1 is trying not to join. |
| **[WOLPERTINGER](https://github.com/KeilerHirsch/WOLPERTINGER)** | Local-first “copilot should know what to do”; tray / overlay / hub; pre-alpha, no end-user release | Same pitch as astromech, earlier. Optional providers “add capability, not ownership.” |
| **ED Outrider** (EDCodex I/O, Sep 2026) | Live browser: nearby undiscovered, body values, likely exo, overlay | Exploration HUD, not voice. |
| **Icarus Terminal** | Context-sensitive second screen | Panel zoo. |
| **LunaPanel** (Sep 2026) | Phone/tablet as MFD; tap fires a command on the PC | Tablet-as-controls, no voice. |
| **EDGPT** (Sep 2026) | Journal/state as MCP for an external LLM | Brain outside the app. Opposite of local-first. |
| **edBindingsManager** | Flask app: view/analyse/fix device IDs, visualise maps | Binds-only. EliteIntel 1.2 is about to eat this niche. |
| **EDRefCard**, **Elite Custom Button Names**, **Spacejock KeybindsBackup** | Printable cards, button labels, backup | Chore tools. Health check + My controller replace the first two for v1. |
| **Joystick Overlay** (Élite Dangereuse) | Live stick/pad visualisation for streams | Related to “press a button to jump to it,” not a copilot. |

ed.tools also lists the usual data plane (INARA, EDSM, EDMC, Spansh, EDDiscovery, Observatory, SrvSurvey, Coriolis, EDDI, VoiceAttack, HCS). Those are inputs and peers, not competitors for the v1 headline.

---

## 5. How they differ

### 5.1 Who may press a key

| | LLM can press keys | Grammar / router can press keys | Works with no model |
|---|---|---|---|
| EliteIntel, COVAS, WickedWake, AURA (LLM mode) | yes | no / thin | no |
| CanopyNav | wake-word macros | yes | core yes |
| d47 | only after router miss; guarded phrases always offered | yes | yes |
| **astromech v1** | later, opt-in, never default ([V1-DESIGN](V1-DESIGN.md) §1.4, [ARCHITECTURE](ARCHITECTURE.md) decision 3) | **yes, only path** | **yes** |

This is the product split. EliteIntel’s marketing is “no command lists.” That is also why “gear down” waits on a cloud round-trip (~750 ms end-to-end, REPORT §11) and why a model can invent a tool. d47 and astromech both put a deterministic matcher in front. Keep that.

### 5.2 Binds

Everyone still injects **keyboard scancodes** via `SendInput`. Nobody presses HOTAS buttons. Keyboard secondary binds remain the requirement (ARCHITECTURE C6, REPORT §5).

EliteIntel BindForge is the best public design for the **file**. astromech’s planned **product** on top of the file is still unoccupied: Activities on manufacturer sheets, My controller, PID-drift / firmware / Joy_32 health ([KEYBINDS.md](KEYBINDS.md), V1-DESIGN §3).

### 5.3 Verification

| | Toggle (gear, lights, …) | Course plot |
|---|---|---|
| EliteIntel | `WAIT_UNTIL` predicate poll (REPORT §5) | No NavRoute check; beeps (REPORT §6) |
| COVAS:NEXT | key timings | **NavRoute.json last system == target** (REPORT §6) |
| EDCoPilot | undocumented | Hope + extra pauses (REPORT §6) |
| **astromech** | Silent Status.json watch, 2 s, **no retry, no spoken failure** (ARCHITECTURE v0.12) | Out of v1 |

astromech’s silent verify is stricter than the others. Keep it.

### 5.4 Voice UX they learned the hard way

- **PTT is the gate** (EliteIntel TECHNICAL.md). The noise gate must not open its own window. astromech already has hold/level/bounce (ARCHITECTURE v0.13); their wording is the rule to write down.
- **TTS must not enter STT** (EliteIntel, 4 Oct). Headset speakers + Kokoro will false-trigger Parakeet.
- **Barge-in** (EDMO): PTT cuts TTS mid-sentence. d47: “shut up” / “be quiet” / “stop” as first-class phrases, plus a shut-up hotkey.
- **Wake word** (CanopyNav, older VA). Rejected for cockpit noise (V1-DESIGN §1.4). Correct.

### 5.5 Product shape

EDCoPilot, AURA, CanopyNav, EDMO and Icarus all grow **panels**. v1 decided the window is for **setup** and the session is **voice + tray** (V1-DESIGN §2). The deluge is mostly second-screen HUDs. Activities on **your sticks** is the one visual idea nobody else has.

---

## 6. What to steal

Ranked against v1 scope. “Steal” means copy the *property* (safety, UX rule, state machine), not their UI, brand, or code dump. EliteIntel is CC0; COVAS and d47 and EDMO are MIT; AURA/CanopyNav/WickedWake are closed — take only public behaviour.

### 6.1 Steal now (fits v1)

| # | Take | From | Where it lands |
|---|---|---|---|
| 1 | **Edit pipeline:** working copy, SHA-256 three-state (clean / draft / conflict), surgical XML, timestamped backups, apply only when the game file still matches baseline | EliteIntel BindForge | `astromech-keybinds` write path (KEYBINDS §7 already says backup + copy) |
| 2 | **Conflict rules by vehicle, not by key.** Same chord on ship scoop and SRV scoop is a layout. Same chord on `UI_Left` and `CamTranslateLeft` is a plotter-breaker | BindForge `BindingConflictRules`; COVAS/EDCoPilot WASD-vs-camera failures | Health check, **before** “Bind this” |
| 3 | **Auto-assign keyboard secondaries from a safe unused-key pool** | EliteIntel auto-assigner; COVAS prefers secondary so HOTAS primary stays | Onboarding + health “one-click fix” (V1-DESIGN §1.5 #1) |
| 4 | **PTT as exclusive gate + barge-in.** PTT down = stop talking, start recording. PTT up = transcribe. Never let Kokoro leak into Parakeet | EliteIntel PTT + echo fix; EDMO barge-in | `astromech-voice` + core PTT path |
| 5 | **Keyword router before any future model; fuzzy match offers, does not run, if the phrase would press a key** | d47 conversation.html; astromech already has did-you-mean | `astromech-intent`. Write the rule: unguarded unique near-miss may run; **any keypress is offered** |
| 6 | **Capability registry that is the docs.** Phrase, tier, Status bit, help sentence, one list. “What can you do?” is projected | d47 registry + CI | `astromech-tools` + intent grammar + a help page generated from it |
| 7 | **Per-row reset; model (if any) cannot touch safety.** Kill switch, PTT bind, policy tiers, “press keys” are panel-only | d47 settings | Settings + policy |

### 6.2 Steal soon (cheap, still v1-shaped)

| # | Take | From | Note |
|---|---|---|---|
| 8 | Spoken conflict names (“Left Control plus W”) | BindForge `BindingChordSpeech` | Health about-panel + voice |
| 9 | Custom commands as **data**: user sequences of existing tools, capped by policy | EliteIntel custom-commands | Not a plugin ABI |
| 10 | In-game chat as a typed command channel (`@Vega …`) | EliteIntel 2 Oct | Same grammar as voice; useful when the mic is wrong |
| 11 | Actions panel / typed console that fires the **same** tools as voice | COVAS Actions tab; gear-down preview already has `gear down` / `!kill` on stdin | One registry |
| 12 | Diagnostics zip: logs, redacted config, health HTML, device IDs, PTT clip stats | V1-DESIGN §1.5 #2; EliteIntel already points missing-bind names at the log | Keep as planned |
| 13 | Phrase book × live bind join: one page that is the grammar plus the current key | AURA VC Catalog, without the LLM | All keybinds × voice; Activities already wants the short version |

### 6.3 Steal the idea, not the feature (after v1)

- COVAS **NavRoute-verified plotter** + clipboard fallback if the macro fails. Already in REPORT §6. Out of v1 (V1-DESIGN §1.3).
- CanopyNav / SrvSurvey **colonisation shopping list** and exo clone-distance. Module roadmap (ARCHITECTURE §18.7).
- COVAS **follow-game-window overlay** for flatscreen. v1 dropped in-headset UI; a tiny “listening / last phrase” chip is optional. Tray icon may be enough.
- EliteIntel **per-ship voice**. Fun, not trust.
- ED Unified **launch graph**. If astromech stays one app, a “start with Elite” checkbox is enough.
- d47 **turn receipts** (tokens, $). Only if an optional LLM ships.
- EDMO **mission cards** and heartbeat nudges. Later module. Heartbeats need a mute; they are one step from nagging.

### 6.4 Do not steal

- LLM-required startup (EliteIntel, WickedWake).
- Wake word (CanopyNav).
- Auto-honk / auto-neutron / auto-target (EDCoPilot grey zone; V1-DESIGN §1.4, ARCHITECTURE §2 non-goals).
- Unofficial edge-tts as default (EDCoPilot; ToS and availability, REPORT §11).
- Personality-as-product (Guardian cores, VEGA, Epiphany). The voice should sound like a ship computer (V1-DESIGN §6.4).
- Their panel zoos. Activities on manufacturer sheets + health markers is the visual to keep.

---

## 7. Positioning

They are **ship AIs**. astromech is a **binds doctor that can hear**.

EliteIntel 1.2 will collide with the binds *file* half. Ship BindForge’s safety properties before they ship a controller UI, and keep the things they still do not have:

- manufacturer sheets on the user’s devices (Activities / My controller)
- PID-drift / Joy_32 / firmware health
- a kill switch and a policy tier, not a prompt
- “gear down” on a quiet CPU with no API key

**d47 is the one to keep reading.** Same MIT, same “router owns the keys,” same overlay-or-audio split, same “docs are generated from the registry.” Steal the discipline; do not steal the Guardian bit.

**COVAS** remains the plotter/search benchmark. Ignore it until v1 voice + binds are boringly solid.

**ED Unified** is the alternative failure mode: a launcher for everyone else’s tools. v1 already chose one comprehensive app (ARCHITECTURE v0.7).

---

## 8. Sources

- ed.tools: https://ed.tools/
- EDCodex: CanopyNav https://edcodex.info/?m=tools&entry=661 ; AURA https://edcodex.info/?m=tools&entry=659 ; EDMO https://edcodex.info/?m=tools&entry=642 ; EDU https://edcodex.info/index.php?m=tools&entry=636
- EliteIntel site: https://www.elite-intel.org/
- EliteIntel repo: https://github.com/SudoKrondor/EliteIntel
- BindForge: https://github.com/SudoKrondor/EliteIntel/blob/master/app/src/main/java/elite/intel/bindforge/PACKAGE.md
- EliteIntel TECHNICAL.md: https://github.com/SudoKrondor/EliteIntel/blob/master/TECHNICAL.md
- EliteIntel v1.1 video: https://www.youtube.com/watch?v=Fh8Za7_NLF0
- COVAS:NEXT: https://github.com/RatherRude/Elite-Dangerous-AI-Integration
- COVAS v4.3.0: https://github.com/RatherRude/Elite-Dangerous-AI-Integration/releases/latest
- Directive 47: https://github.com/dseelinger/d47 ; docs https://dseelinger.github.io/d47/ ; conversation https://dseelinger.github.io/d47/conversation.html ; help https://dseelinger.github.io/d47/capabilities/help.html
- AURA: https://aura-command.net ; Frontier thread https://forums.frontier.co.uk/threads/aura-app-suite-pre-release.649103/
- WickedWake: https://wwt.wtf/copilot ; Frontier thread https://forums.frontier.co.uk/threads/new-ai-copilot-entering-alpha-friday-5-2.637182/
- EDMO: https://github.com/Mallock/edmo ; https://edmo.blinkki.com
- WOLPERTINGER: https://github.com/KeilerHirsch/WOLPERTINGER
- EDCoPilot: https://www.razzafrag.com (see REPORT §8)
- HCS Singularity 4.13 (Rhino/Nomad voice): https://www.youtube.com/watch?v=LkqoaudoOv8
- Voxta ED COVAS (experimental chat augmentation, not a standalone app): https://doc.voxta.ai/docs/server/services/chat-augmentations/elite-dangerous
- This repo: [REPORT.md](REPORT.md) §5–8, [V1-DESIGN.md](V1-DESIGN.md) §1, [ARCHITECTURE.md](ARCHITECTURE.md) decisions 3–7, [KEYBINDS.md](KEYBINDS.md)
