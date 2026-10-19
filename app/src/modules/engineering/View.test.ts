// Ships page: selected ship as the main pane, the rest of the fleet as a bottom rail.
// Stats come from lib/fixtures/ships.json (astromech-ship dump-ships-fixture).
import { afterEach, beforeEach, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import ships from "../../lib/fixtures/ships.json";
import type { Compared, Fleet, Headline } from "../../lib/engineering";
import View from "./View.svelte";

let headlines: Record<string, Headline> = {};
let fleet: Fleet;
let headlinesFail: string | null = null;
let compared: Compared | null = null;

vi.mock("../../lib/engineering", async (importActual) => {
  const real = await importActual<typeof import("../../lib/engineering")>();
  return {
    ...real,
    loadFleet: async () => fleet,
    loadHeadlines: async () => {
      if (headlinesFail) throw new Error(headlinesFail);
      return headlines;
    },
    shipStats: async () => compared,
  };
});

function emptyStats(): Compared {
  return structuredClone(ships.cases[0]!.stats) as unknown as Compared;
}

beforeEach(() => {
  location.hash = "#/engineering";
  headlines = structuredClone(ships.headlines) as unknown as Record<string, Headline>;
  fleet = structuredClone(ships.fleet) as unknown as Fleet;
  headlinesFail = null;
  compared = emptyStats();
});
afterEach(cleanup);

it("shows speed, jump and power on the Info tab", async () => {
  render(View);
  expect(await screen.findByRole("tab", { name: "Info" })).toBeTruthy();
  expect(screen.queryByRole("heading", { name: "Core internals" })).toBeNull();
  expect((await screen.findByLabelText("Speed")).textContent).toMatch(/376/);
  expect(screen.getByLabelText("Jump").textContent).toMatch(/24\.87/);
  expect(screen.getByLabelText("Power").textContent).toMatch(/44\.10/);
  expect(screen.getByLabelText("Pitch / roll / yaw").textContent).toMatch(/36/);
  expect(screen.getByLabelText("Shields").textContent).toMatch(/MJ/);
  expect(screen.getByLabelText("Weapons and hull").textContent).toMatch(/DPS/);
});

it("shows the build planner on the Engineering tab", async () => {
  location.hash = "#/engineering/1/plan";
  render(View);
  expect(await screen.findByRole("heading", { name: "Core internals" })).toBeTruthy();
  expect(screen.queryByLabelText("Speed")).toBeNull();
});

it("switches from Info to Engineering", async () => {
  render(View);
  expect(await screen.findByLabelText("Speed")).toBeTruthy();
  await fireEvent.click(screen.getByRole("tab", { name: "Engineering" }));
  expect(await screen.findByRole("heading", { name: "Core internals" })).toBeTruthy();
});

it("says when the hull is not in the ship data", async () => {
  compared = { unknown_hull: "mediumtransport01" };
  render(View);
  const warn = await screen.findByText(/Incomplete: this hull is not in the ship data/);
  expect(warn.getAttribute("title")).toBe("Hull: mediumtransport01");
});

it("marks a ship incomplete when the calculations leave something out or don't know the hull", async () => {
  const c = emptyStats();
  if ("current" in c) {
    c.current.unresolved = ["Hull: krait_mkii"];
    c.planned.unresolved = ["Hull: krait_mkii"];
  }
  compared = c;
  render(View);
  const warn = await screen.findByText(/Incomplete/);
  expect(warn.getAttribute("title")).toBe("Hull: krait_mkii");
});

it("shows a stored ship's system as a chip that copies its name", async () => {
  const krait = fleet.ships[0]!;
  Object.assign(krait, { current: false, system: "Shinrarta Dezhra" });
  const writeText = vi.fn(async () => {});
  Object.defineProperty(navigator, "clipboard", { value: { writeText }, configurable: true });
  render(View);
  const c = (await screen.findByRole("heading", { name: "Synthetic_Krait" })).closest(".ship") as HTMLElement;
  const copy = within(c).getByRole("button", { name: "Copy Shinrarta Dezhra" });
  expect(copy.closest(".chip")!.textContent).toContain("Shinrarta Dezhra");
  await fireEvent.click(copy);
  expect(writeText).toHaveBeenCalledWith("Shinrarta Dezhra");
  const open = screen.getByRole("link", { name: "Synthetic_Krait, Krait MkII" });
  expect(open.getAttribute("href")).toBe("#/engineering/1");
});

it("says where a ship in transit is going, as a chip", async () => {
  Object.assign(fleet.ships[0]!, { current: false, in_transit: true, system: "Sol" });
  render(View);
  const c = (await screen.findByRole("heading", { name: "Synthetic_Krait" })).closest(".ship") as HTMLElement;
  const where = c.querySelector(".where")!;
  expect(where.textContent!.replace(/\s+/g, " ").trim()).toBe("In transit to Sol");
  // The location wraps and its name may be cut short on a narrow card: the full name is on hover.
  expect(where.getAttribute("title")).toBe("Sol");
  expect(within(c).getByRole("button", { name: "Copy Sol" })).toBeTruthy();
});

it("names each rail link by ship name or ident and type", async () => {
  Object.assign(fleet.ships[0]!, { name: null, ident: "CU-01", type_name: "Imperial Cutter" });
  render(View);
  const link = await screen.findByRole("link", { name: "CU-01, Imperial Cutter" });
  expect(link.textContent).toContain("Unnamed");
});

it("says when the row stats can't be worked out", async () => {
  headlinesFail = "journal folder not found: C:\\nowhere";
  render(View);
  const note = await screen.findByText(
    (_, el) => el?.tagName === "P" && /^! Could not work out shield, DPS and jump range: .*journal folder not found/.test(el.textContent?.trim() ?? ""),
  );
  expect(note).toBeTruthy();
});

it("opens the engineering tab for the selected ship", async () => {
  render(View);
  const plan = await screen.findByRole("tab", { name: "Engineering" });
  expect(plan.getAttribute("href")).toBe("#/engineering/1/plan");
});

it("opens the Info tab for the selected ship", async () => {
  render(View);
  const info = await screen.findByRole("tab", { name: "Info" });
  expect(info.getAttribute("href")).toBe("#/engineering/1");
});
