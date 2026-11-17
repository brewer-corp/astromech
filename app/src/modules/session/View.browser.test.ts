import { describe, expect, it } from "vitest";
import { render, screen, within } from "@testing-library/svelte";
import session from "../../preview/fixtures/session.json";
import View from "./View.svelte";

// Outside the app the summary is the sample history's latest session (worlds/preview/history,
// the history crate's synthetic journals), dumped from the core.
const text = (el: Element) => el.textContent?.replace(/\s+/g, " ").trim();

describe("Session summary", () => {
  it("shows the session's totals", async () => {
    render(View);
    await screen.findByText("Latest session");
    expect(screen.getByText(/2026-09-03 10:00 to 2026-09-03 10:30 UTC · 29 min/)).toBeTruthy();
    expect(text(screen.getByText("cr per hour").parentElement!)).toBe("14,483,231cr per hour");
    expect(text(screen.getByText("cr earned").parentElement!)).toBe("7,221,500cr earned");
    expect(text(screen.getByText("ly").parentElement!)).toBe("0.0ly");
    const by = screen.getByRole("region", { name: "By activity" });
    expect(within(by).getAllByRole("listitem").map(text)).toEqual([
      "Organic data 5,000,000cr 1 payment",
      "Community goals 2,000,000cr 1 payment",
      "Exploration data 100,000cr 1 payment",
      "Combat bonds 60,000cr 1 payment",
      "Search and rescue 40,000cr 1 payment",
      "Odyssey goods 15,000cr 1 payment",
      "Other vouchers 5,000cr 1 payment",
      "Trade 1,500cr 1 payment · 1,500 cr profit",
    ]);
  });

  it("draws each payment on the band at its time", async () => {
    render(View);
    const band = await screen.findByRole("region", { name: "Over time" });
    const marks = within(band).getAllByRole("img");
    expect(marks.map((m) => m.getAttribute("aria-label"))).toEqual(
      session.earnings.flatMap((e) =>
        session.timeline.filter((p) => p.kind === e.kind).map((p) => expect.stringMatching(new RegExp(`, ${p.amount.toLocaleString("en")} cr at ${p.ts.slice(11, 16)} UTC$`))),
      ),
    );
    // Organic data, paid at 10:11, sits 655 s into the session's 1,795 s.
    expect(parseFloat(marks[0]!.style.left)).toBeCloseTo((655 / 1795) * 100, 1);
  });
});
