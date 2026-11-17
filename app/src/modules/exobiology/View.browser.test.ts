import { describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";
import { millions, shortName, valueRange } from "../../lib/exobiology";

// Outside the app the page shows the synthetic sample system (the core's view of a replayed
// system: two bodies, one mapped, a Stratum being sampled, and what others reported).
/** The cards: the list's own items, not the genus lines inside them. */
const cardsOf = (list: HTMLElement) => within(list).getAllByRole("listitem").filter((el) => el.parentElement === list);

describe("Biology", () => {
  it("ranks the system's bodies by value with their genera", async () => {
    render(View);
    const list = await screen.findByRole("list", { name: "Bodies with biology" });
    const cards = cardsOf(list);
    expect(cards).toHaveLength(2);
    expect(cards[0]!.textContent).toContain("A 2");
    expect(cards[0]!.textContent).toContain("Stratum");
    expect(cards[0]!.textContent).toContain("Tectonicas");
    expect(cards[0]!.textContent).toContain("20 M");
    expect(cards[1]!.textContent).toContain("B 3");
    // A 2's genera come from the DSS; B 3's are predicted from its scan, which their Tip says.
    const said = (el: HTMLElement) => document.getElementById(el.getAttribute("aria-describedby") ?? "")?.textContent ?? "";
    expect(said(within(cards[0]!).getByText("Stratum"))).toBe("");
    expect(said(within(cards[1]!).getByText("Electricae"))).toMatch(/^Predicted/);
    expect(screen.getByText(/biology on 2 bodies/)).toBeTruthy();
  });

  it("shows the sampling in progress against the colony range", async () => {
    render(View);
    const block = await screen.findByLabelText("Sampling");
    expect(block.textContent).toContain("Stratum Tectonicas");
    expect(within(block).getByRole("img", { name: "1 of 3" })).toBeTruthy();
    expect(block.textContent).toContain("209 m of 500 m");
    expect(block.textContent).toContain("291 m to go");
  });

  it("lists what other players reported", async () => {
    render(View);
    // Spansh's reports merge into the commander's cards: here they add each body's reported range.
    const list = await screen.findByRole("list", { name: "Bodies with biology" });
    const [a2, b3] = cardsOf(list);
    expect(a2!.textContent).toContain("reported 20–28 M");
    expect(b3!.textContent).toContain("reported 8.3–34 M");
  });

  it("formats values and names", () => {
    expect(millions(19_010_800)).toBe("19 M");
    expect(millions(1_362_000)).toBe("1.4 M");
    expect(valueRange([1_000_000, 19_010_800])).toBe("1–19 M");
    expect(valueRange([6_284_600, 6_284_600])).toBe("6.3 M");
    expect(shortName("Synthetic Sector AB-C d1 A 2", "Synthetic Sector AB-C d1")).toBe("A 2");
    expect(shortName("Other 1", "Synthetic")).toBe("Other 1");
  });
});
