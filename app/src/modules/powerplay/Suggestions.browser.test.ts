import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import type { Suggestion } from "../../lib/powerplay";
import Suggestions from "./Suggestions.svelte";

const s = (o: Partial<Suggestion>): Suggestion => ({
  activity: "x",
  name: "X",
  how: "Do x here.",
  odyssey: false,
  favoured: false,
  weak_spot: false,
  uncertain: null,
  ...o,
});

const rules = { version: 1, fetched: "2026-10-16" };

/** The words a Tip gives the element, as a screen reader hears them. */
const described = (el: HTMLElement) => document.getElementById(el.getAttribute("aria-describedby") ?? "")?.textContent;

describe("What earns merits here", () => {
  it("lists each activity in the order given, with how to do it in its Tip", () => {
    render(Suggestions, {
      items: [
        s({ activity: "commit-crimes", name: "Crimes", how: "Murder ships here.", favoured: true }),
        s({ activity: "power-commodities", name: "Power commodities", how: "Pick up Sirius Corporate Contracts." }),
        s({ activity: "power-commodities", name: "Power commodities", how: "Pick up Sirius Franchise Package." }),
      ],
      rules,
    });
    const names = screen.getAllByText(/^(Crimes|Power commodities)$/);
    expect(names.map((e) => e.textContent)).toEqual(["Crimes", "Power commodities", "Power commodities"]);
    expect(described(names[2]!)).toContain("Pick up Sirius Franchise Package.");
    expect(screen.queryByText("Murder ships here.")).toBeNull();
    expect(screen.getByText(/as of 16 Oct 2026 \(rules v1\)/)).toBeTruthy();
  });

  it("tags ethos, weak spots and on-foot activities, each tag explained in its Tip", () => {
    render(Suggestions, {
      items: [
        s({ activity: "flood-markets", name: "Flood markets", favoured: true, weak_spot: true }),
        s({ activity: "classified-data", name: "Power classified data", odyssey: true }),
      ],
      rules,
    });
    expect(described(screen.getByText("Ethos"))).toMatch(/bonus merits/);
    expect(described(screen.getByText("Weak spot"))).toMatch(/weak to it/);
    expect(screen.getByText("On foot")).toBeTruthy();
  });

  it("marks unverified rules quietly, with the reason in the activity's Tip", () => {
    const { container } = render(Suggestions, {
      items: [s({ activity: "salvage", name: "Salvage", uncertain: "Hand-in place not documented." }), s({ activity: "cartography", name: "Exploration data" })],
      rules,
    });
    expect(container.querySelectorAll(".unverified")).toHaveLength(1);
    expect(described(screen.getByText("Salvage"))).toContain("Not checked in game: Hand-in place not documented.");
    expect(described(screen.getByText("Exploration data"))).not.toContain("Not checked");
  });
});
