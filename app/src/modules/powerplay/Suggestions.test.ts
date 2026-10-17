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

describe("What earns merits here", () => {
  it("lists each activity with how to do it, in the order given", () => {
    render(Suggestions, {
      items: [
        s({ activity: "commit-crimes", name: "Crimes", how: "Murder ships here.", favoured: true }),
        s({ activity: "power-commodities", name: "Power commodities", how: "Pick up Sirius Corporate Contracts." }),
        s({ activity: "power-commodities", name: "Power commodities", how: "Pick up Sirius Franchise Package." }),
      ],
      rules,
    });
    const names = screen.getAllByText(/^(Crimes|Power commodities)$/).map((e) => e.textContent);
    expect(names).toEqual(["Crimes", "Power commodities", "Power commodities"]);
    expect(screen.getByText("Murder ships here.")).toBeTruthy();
    expect(screen.getByText("Pick up Sirius Franchise Package.")).toBeTruthy();
    expect(screen.getByText(/as of 16 Oct 2026 \(rules v1\)/)).toBeTruthy();
  });

  it("tags ethos, weak spots and on-foot activities", () => {
    render(Suggestions, {
      items: [
        s({ activity: "flood-markets", name: "Flood markets", favoured: true, weak_spot: true }),
        s({ activity: "classified-data", name: "Power classified data", odyssey: true }),
      ],
      rules,
    });
    // One tag per row, and one in the key under the list, which says what it means.
    expect(screen.getAllByText("ethos")).toHaveLength(2);
    expect(screen.getAllByText("weak spot")).toHaveLength(2);
    expect(screen.getByText(/your power favours it: bonus merits/)).toBeTruthy();
    expect(screen.getByText(/the controlling power is weak to it/)).toBeTruthy();
    expect(screen.getByText("on foot")).toBeTruthy();
  });

  it("marks unverified rules quietly and explains the mark", () => {
    const { container } = render(Suggestions, {
      items: [s({ activity: "salvage", name: "Salvage", uncertain: "Hand-in place not documented." }), s({ activity: "cartography", name: "Exploration data" })],
      rules,
    });
    expect(container.querySelectorAll(".unverified")).toHaveLength(1);
    // The reason is plain text under the row: no hover needed, read by screen readers.
    expect(screen.getByText("Not checked in game: Hand-in place not documented.").tagName).toBe("P");
    expect(container.querySelector("[title]")).toBeNull();
    expect(screen.getByText(/not yet checked in game/)).toBeTruthy();
  });

  it("leaves out the key when every rule is verified", () => {
    render(Suggestions, { items: [s({ name: "Exploration data" })], rules });
    expect(screen.queryByText(/not yet checked in game/)).toBeNull();
    expect(screen.queryByText(/bonus merits/)).toBeNull();
  });
});
