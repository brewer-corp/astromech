// A ship's plan: made in the Loadout's side panel, read on the Plan view. The stats come from the
// fixture the real core dumps in the world worlds/preview/krait (preview/fixtures/ships.json): the
// synthetic Krait MkII combat build and its stats for each build plan. The expected numbers are
// what Coriolis shows for those builds, the same values crates/astromech-ship/tests/coriolis.rs
// checks (krait_mkii_combat and plan_*); a change is the planned value minus the current one, both
// as shown. The sample journal's Krait (ship 7) has the recorded plan (engineering.json).
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import ships from "../../preview/fixtures/ships.json";
import sample from "../../preview/fixtures/engineering.json";
import { samePlan, type Compared, type Fleet, type Plan, type StatsComparison } from "../../lib/engineering";
import View from "./View.svelte";
import { answer, previewAnswer, sent, startPreview } from "../../test/transport";

const asked: Plan[] = [];
/** Answer every plan with this instead of the fixture. */
let override: Compared | null = null;
/** The fleet the page reads; null, the preview's (the sample journal's and the synthetic Krait). */
let fleet: Fleet | null = null;

// engineering_stats answered from the fixture: a plan it doesn't have fails. Another ship's plan
// gets the preview's own answer, which has no stats for it and says so.
answer("engineering_stats", ({ plan }: { plan: Plan }): Compared | Promise<Compared> => {
  asked.push(plan);
  if (override) return override;
  if (plan.ship_id !== ships.fleet.ships[0]!.id) return previewAnswer<Compared>("engineering_stats", { plan });
  const c = ships.cases.find((c) => samePlan(c.plan as unknown as Plan, plan));
  if (!c) throw new Error(`no fixture stats for ${JSON.stringify(plan.targets)}`);
  return c.stats as unknown as StatsComparison;
});
answer("engineering_fleet", () => fleet ?? previewAnswer("engineering_fleet"));

const krait = ships.fleet.ships[0]!.id;

/** Now, Plan and Change for one stat. */
function stat(group: string, label: string): string[] {
  const g = screen.getByRole("rowgroup", { name: group });
  const th = within(g).getByRole("rowheader", { name: label });
  return [...th.parentElement!.querySelectorAll("td")].map((td) => td.textContent ?? "");
}

async function pick(label: string, value: string) {
  await fireEvent.change(await screen.findByRole("combobox", { name: label }), { target: { value } });
}

/** Select a module on the Loadout by its slot. */
async function module(slot: string) {
  await fireEvent.click(await screen.findByRole("button", { name: (n) => n.startsWith(`${slot}: `) || n.startsWith(`${slot} (`) }));
}

async function toPlan() {
  await fireEvent.click(screen.getByRole("button", { name: "Plan", pressed: false }));
}

beforeEach(() => {
  localStorage.clear();
  asked.length = 0;
  override = null;
  fleet = null;
  location.hash = `#/engineering/${krait}`;
});
afterEach(cleanup);

describe("ship stats on the Plan", () => {
  it("shows current stats as the plan until the plan changes", async () => {
    location.hash = `#/engineering/${krait}/plan`;
    render(View);
    await waitFor(() => expect(stat("Distributor", "WEP capacity (MJ)")).toEqual(["92.2", "92.2", ""]));
    expect(stat("Jump range (ly)", "Max")).toEqual(["24.87", "24.87", ""]);
    expect(stat("Shields", "Strength (MJ)")).toEqual(["1428", "1428", ""]);
    expect(stat("Shields", "Thermal")).toEqual(["36.2", "36.2", ""]);
    expect(stat("Distributor", "Time to drain")).toEqual(["1:10", "1:10", ""]);
    expect(stat("Weapons and hull", "DPS")).toEqual(["107.2", "107.2", ""]);
    expect(stat("Ship", "Power retracted (MW)")).toEqual(["23.27", "23.27", ""]);
    expect(screen.queryByText(/Incomplete/)).toBeNull();
  });

  it("switching the distributor experimental changes drain against recharge", async () => {
    render(View);
    // Weapon focused G5 with the fitted Super Conduits, rolled to 100 % (plan_pd_super_conduits).
    await module("Power distributor");
    await pick("Power distributor blueprint", "PowerDistributor_PriorityWeapons");
    await toPlan();
    await waitFor(() => expect(stat("Distributor", "WEP capacity (MJ)")).toEqual(["92.2", "93.7", "+1.5"]));
    expect(stat("Distributor", "WEP recharge (MW)")).toEqual(["9.0", "9.1", "+0.1"]);
    expect(stat("Distributor", "Time to drain")).toEqual(["1:10", "1:18", "+0:08"]);
    // Then Cluster Capacitors (plan_pd_cluster_capacitors): more capacity, slower recharge.
    // Back on the Loadout the module is still selected.
    await fireEvent.click(screen.getByRole("button", { name: "Loadout", pressed: false }));
    await pick("Power distributor experimental", "special_powerdistributor_capacity");
    await toPlan();
    await waitFor(() => expect(stat("Distributor", "WEP capacity (MJ)")).toEqual(["92.2", "105.4", "+13.2"]));
    expect(stat("Distributor", "WEP recharge (MW)")).toEqual(["9.0", "8.6", "-0.4"]);
    expect(stat("Distributor", "Time to drain")).toEqual(["1:10", "1:01", "-0:09"]);
    expect(stat("Distributor", "SYS recharge (MW)")).toEqual(["4.0", "3.7", "-0.3"]);
    expect(asked.at(-1)!.targets).toEqual([
      { slot: "PowerDistributor", blueprint: "PowerDistributor_PriorityWeapons", grade: 5, experimental: "special_powerdistributor_capacity" },
    ]);
  });

  it("switching the FSD blueprint changes the jump range", async () => {
    render(View);
    // Faster boot sequence G5, keeping Mass Manager (plan_fsd_fast_boot).
    await module("Frame shift drive");
    await pick("Frame shift drive blueprint", "FSD_FastBoot");
    await toPlan();
    await waitFor(() => expect(stat("Jump range (ly)", "Max")).toEqual(["24.87", "18.71", "-6.16"]));
    expect(stat("Jump range (ly)", "Full tank")).toEqual(["24.05", "18.09", "-5.96"]);
    expect(stat("Jump range (ly)", "Laden")).toEqual(["23.15", "17.41", "-5.74"]);
    expect(stat("Jump range (ly)", "Total")).toEqual(["163.74", "123.19", "-40.55"]);
    expect(stat("Ship", "Mass (t)")).toEqual(["820", "814", "-6"]);
  });

  it("each thermal-resistant booster raises thermal resistance and lowers shield strength", async () => {
    render(View);
    // Three thermal-resistant boosters instead of two (plan_thermal_boosters_3)...
    await module("Utility 1");
    await pick("Utility 1 blueprint", "ShieldBooster_Thermic");
    await toPlan();
    await waitFor(() => expect(stat("Shields", "Thermal")).toEqual(["36.2", "43.9", "+7.7"]));
    expect(stat("Shields", "Strength (MJ)")).toEqual(["1428", "1197", "-231"]);
    expect(stat("Shields", "Kinetic")).toEqual(["43.4", "41.2", "-2.2"]);
    // ...then four (plan_thermal_boosters_4).
    await fireEvent.click(screen.getByRole("button", { name: "Loadout", pressed: false }));
    await module("Utility 2");
    await pick("Utility 2 blueprint", "ShieldBooster_Thermic");
    await toPlan();
    await waitFor(() => expect(stat("Shields", "Thermal")).toEqual(["36.2", "49.5", "+13.3"]));
    expect(stat("Shields", "Strength (MJ)")).toEqual(["1428", "967", "-461"]);
    expect(stat("Shields", "Explosive")).toEqual(["52.9", "49.0", "-3.9"]);
  });

  it("marks the ship incomplete when the calculations leave something out", async () => {
    const c = structuredClone(ships.cases[0]!.stats) as unknown as StatsComparison;
    c.current.unresolved = ["MainEngines: int_engine_size9_class9"];
    c.planned.unresolved = ["MainEngines: int_engine_size9_class9"];
    override = c;
    location.hash = `#/engineering/${krait}/plan`;
    render(View);
    const warn = await screen.findByText(/Incomplete: 1 item is not in the ship data and is left out/);
    expect(document.getElementById(warn.getAttribute("aria-describedby")!)?.textContent).toBe("MainEngines: int_engine_size9_class9");
  });

  it("shows a hull the ship data lacks as incomplete instead of an error", async () => {
    override = { unknown_hull: "mediumtransport01" };
    location.hash = `#/engineering/${krait}/plan`;
    render(View);
    await screen.findByRole("region", { name: "Ship stats" });
    const warns = await screen.findAllByText(/Incomplete: this hull is not in the ship data/);
    expect(warns.map((w) => document.getElementById(w.getAttribute("aria-describedby")!)?.textContent)).toContain("Hull: mediumtransport01");
    expect(screen.queryByRole("rowgroup", { name: "Shields" })).toBeNull();
  });
});

describe("ship link-outs", () => {
  it("opens the loadout in Coriolis and EDSY, and the system on Spansh and Inara", async () => {
    const opened = vi.spyOn(window, "open").mockImplementation(() => null);
    fleet = structuredClone(ships.fleet) as unknown as Fleet;
    Object.assign(fleet.ships[0]!, { current: false, system: "Shinrarta Dezhra" });
    const links = fleet.ships[0]!.links!;
    render(View);
    await fireEvent.click(await screen.findByRole("button", { name: "Coriolis" }));
    await fireEvent.click(screen.getByRole("button", { name: "EDSY" }));
    await fireEvent.click(screen.getByRole("button", { name: "Spansh" }));
    await fireEvent.click(screen.getByRole("button", { name: "Inara" }));
    expect(opened.mock.calls.map((c) => c[0])).toEqual([
      links.coriolis,
      links.edsy,
      "https://spansh.co.uk/search/Shinrarta%20Dezhra",
      "https://inara.cz/elite/starsystem/?search=Shinrarta%20Dezhra",
    ]);
    expect(links.coriolis).toMatch(/^https:\/\/coriolis\.io\//);
    opened.mockRestore();
  });

  it("links a stored ship's station on Spansh by its market id", async () => {
    const opened = vi.spyOn(window, "open").mockImplementation(() => null);
    fleet = structuredClone(ships.fleet) as unknown as Fleet;
    Object.assign(fleet.ships[0]!, { current: false, system: "Sol", market_id: 128016640 });
    render(View);
    await fireEvent.click(await screen.findByRole("button", { description: "Open the station in Sol on Spansh" }));
    expect(opened.mock.calls.map((c) => c[0])).toEqual(["https://spansh.co.uk/station/128016640"]);
    // Its name is unknown, so there is no Inara station search; the system links stay.
    expect(screen.queryByRole("button", { description: "Open the station in Sol on Inara" })).toBeNull();
    expect(screen.getByRole("button", { description: "Open Sol on Inara" })).toBeTruthy();
    opened.mockRestore();
  });

  it("says to refresh the loadout when the saved one is stale", async () => {
    fleet = structuredClone(ships.fleet) as unknown as Fleet;
    delete fleet.ships[0]!.links;
    render(View);
    expect(await screen.findByText("Re-dock or relog to refresh the Coriolis / EDSY link")).toBeTruthy();
    expect(screen.queryByRole("button", { name: "Coriolis" })).toBeNull();
  });
});

describe("a module in the side panel", () => {
  it("shows the plan's target for it, its materials against what is held, and its engineers", async () => {
    location.hash = "#/engineering/7";
    render(View);
    await module("Frame shift drive");
    const panel = await screen.findByRole("complementary", { name: "Details" });
    expect(within(panel).getByRole("heading", { name: "5A Frame Shift Drive" })).toBeTruthy();
    const target = sample.plan.targets.find((t) => t.slot === "FrameShiftDrive")!;
    expect(await within(panel).findByText(`G${target.grade}`, { selector: ".type-value-xl" })).toBeTruthy();
    const mats = within(panel).getByRole("table", { name: "Materials for 5A Frame Shift Drive" });
    const rows = within(mats).getAllByRole("row").slice(1);
    expect(rows.map((r) => within(r).getAllByRole("cell")[0]!.textContent)).toEqual(
      target.materials.map(([id]) => sample.plan.materials.find((m) => m.id === id)!.name),
    );
    // A short material says so: the plan needs more of it than is held.
    const short = sample.plan.materials.find((m) => target.materials.some(([id]) => id === m.id) && m.short > 0)!;
    const shortRow = rows.find((r) => r.textContent!.includes(short.name))!;
    expect(within(shortRow).getAllByRole("cell")[2]!.textContent).toBe(`!${short.have}`);
    expect(within(panel).getByText(target.engineers.join(", "))).toBeTruthy();
    // ENGINEERS opens Engineers searched for its group.
    expect(within(panel).getByRole("link", { name: "Engineers" }).getAttribute("href")).toBe("#/unlocks?q=Frame%20Shift%20Drive");
  });

  it("ADD TO PLAN plans one grade up on the module's blueprint", async () => {
    localStorage.setItem("astromech.engineering.plan.7", JSON.stringify({ ship_id: 7, targets: [], rolls: [1, 2, 3, 4, 5] }));
    location.hash = "#/engineering/7";
    render(View);
    // The thrusters: Dirty drive tuning at grade 3 with Drag Drives.
    await module("Thrusters");
    sent.mockClear();
    await fireEvent.click(await screen.findByRole("button", { name: "Add to plan" }));
    await waitFor(() => expect(sent.mock.calls.some(([cmd]) => cmd === "engineering_plan")).toBe(true));
    const plans = sent.mock.calls.filter(([cmd]) => cmd === "engineering_plan").map(([, a]) => (a as { plan: Plan }).plan);
    expect(plans.at(-1)!.targets).toEqual([{ slot: "MainEngines", blueprint: "Engine_Dirty", grade: 4, experimental: "special_engine_overloaded" }]);
    expect(screen.getByRole("img", { name: "Grade 3 of 5, grade 4 planned" })).toBeTruthy();
  });

  it("a plan for another blueprint keeps the row's fitted grade", async () => {
    render(View);
    await module("Power distributor");
    const row = screen.getByRole("button", { name: /^Power distributor: / });
    expect(within(row).getByRole("img", { name: "Grade 5 of 5" })).toBeTruthy();
    await pick("Power distributor blueprint", "PowerDistributor_HighFrequency");
    expect(within(row).getByRole("img", { name: "Grade 5 of 5, Charge enhanced grade 5 planned" })).toBeTruthy();
    expect(row.textContent).toContain("Weapon focused");
  });

  it("ADD TO PLAN opens the blueprint choice for a module not yet engineered", async () => {
    location.hash = "#/engineering/7";
    render(View);
    await module("Optional 1");
    await fireEvent.click(await screen.findByRole("button", { name: "Add to plan" }));
    expect(document.activeElement).toBe(screen.getByRole("combobox", { name: "Optional 1 (size 6) blueprint" }));
  });
});

describe("the Plan view", () => {
  it("still shows the shopping list, trades or farming, the engineer trip and material details", async () => {
    location.hash = "#/engineering/7/plan";
    render(View);
    await screen.findByRole("region", { name: "Shopping list" });
    expect(screen.getByRole("region", { name: "Trades" })).toBeTruthy();
    expect(screen.getByRole("region", { name: "Still to collect" })).toBeTruthy();
    await screen.findByRole("region", { name: "Engineer trip" });
    expect(screen.getAllByText("Ship stats are worked out in the app. Outside it, only the sample Krait's sample plans have them.").length).toBeGreaterThan(0);
    const first = document.querySelector<HTMLButtonElement>("button.mrow")!;
    const name = first.querySelector(".mn")!.textContent!.replace(/^G\d /, "").replace(" ⚠", "");
    await fireEvent.click(first);
    const panel = await screen.findByRole("complementary", { name: "Details" });
    expect(within(panel).getByRole("heading", { level: 3, name })).toBeTruthy();
    expect(within(panel).getByText(/^Used by:/)).toBeTruthy();
    await fireEvent.click(first);
    expect(screen.queryByRole("complementary", { name: "Details" })).toBeNull();
  });

  it("hands the engineer trip to the route follower, parking the route it replaces", async () => {
    startPreview("/?core=ready");
    const { followRoute } = await import("../../lib/route");
    await followRoute("Colonia run", "import", [{ system: "Colonia" }]);
    const { planTrip, savedPlan } = await import("../../lib/engineering");
    location.hash = "#/engineering/7/plan";
    render(View);
    await screen.findByRole("region", { name: "Engineer trip" });
    await fireEvent.click(screen.getByRole("button", { name: "Follow" }));
    const trip = (await planTrip(savedPlan(7)))!;
    const first = trip.stops[0]!;
    expect(await screen.findByText("Replaced Colonia run.")).toBeTruthy();
    expect(screen.getByText(`Following. ${first.system} is on your clipboard.`, { exact: false })).toBeTruthy();
    const { view } = await previewAnswer("route");
    expect(view.route.source).toBe("engineer trip");
    expect(view.route.waypoints[0]).toEqual({ system: first.system, label: first.engineer });
    expect(view.parked.name).toBe("Colonia run");
    expect(screen.getByRole("button", { name: "Resume" })).toBeTruthy();
    history.replaceState(null, "", "/");
  });
});

describe("engineer progress poll (astromech-cvxf)", () => {
  it("says beside the trip when engineer progress could not be refreshed, and clears it on the next good read", async () => {
    history.replaceState(null, "", "/");
    location.hash = "#/engineering/7/plan";
    render(View);
    await screen.findByRole("region", { name: "Engineer trip" });
    history.replaceState(null, "", "/?fail=engineering_engineers#/engineering/7/plan");
    expect(await screen.findByText("Could not refresh: preview: engineering_engineers failed on purpose (?fail=).", {}, { timeout: 2500 })).toBeTruthy();
    expect(screen.getByRole("region", { name: "Engineer trip" })).toBeTruthy();
    history.replaceState(null, "", "/#/engineering/7/plan");
    await waitFor(() => expect(screen.queryByText(/Could not refresh/)).toBeNull(), { timeout: 2500 });
  });

  it("says so when it could not read engineer progress at all", async () => {
    history.replaceState(null, "", "/?fail=engineering_engineers#/engineering/7/plan");
    render(View);
    await screen.findByRole("region", { name: "Engineer trip" });
    expect(await screen.findByText("Could not read engineer progress: preview: engineering_engineers failed on purpose (?fail=).")).toBeTruthy();
    history.replaceState(null, "", "/");
  });
});
