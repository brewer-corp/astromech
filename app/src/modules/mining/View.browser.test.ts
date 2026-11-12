import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { statValue } from "../../test/stat";
import View from "./View.svelte";

// The preview's core reports the sample run (the JSON the core's mining_status test builds from
// the Rust mining fixture).
describe("Mining run", () => {
  it("shows tons refined by ore and tons per hour", async () => {
    history.replaceState(null, "", "/?core=ready");
    render(View);
    await screen.findByText("Refined");
    expect(statValue("Refined")).toBe("9 t");
    expect(statValue("Per hour")).toBe("18 t/h");
    const rows = screen.getAllByRole("row").map((r) => r.textContent?.replace(/\s+/g, " ").trim());
    expect(rows).toEqual(["Platinum7 t", "Painite2 t"]);
    history.replaceState(null, "", "/");
  });
});
