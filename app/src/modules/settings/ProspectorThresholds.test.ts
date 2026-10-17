import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import ProspectorThresholds from "./ProspectorThresholds.svelte";
import { fakeCall, resetFakeCore } from "../../lib/fakecore";

describe("Settings → Prospector callouts", () => {
  it("lists each ore's threshold and saves a change, an off and a reset", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(ProspectorThresholds);
    const plat = (await screen.findByLabelText("Platinum threshold, percent")) as HTMLInputElement;
    expect(plat.value).toBe("20");
    await fireEvent.change(plat, { target: { value: "35" } });
    await screen.findByRole("button", { name: "Reset to 20%" });
    let saved = await fakeCall("prospector_thresholds");
    expect(saved.ores.find((o: { ore: string }) => o.ore === "platinum").threshold).toBe(35);
    // Empty turns it off; a bad value is refused with a message.
    await fireEvent.change(plat, { target: { value: "" } });
    saved = await fakeCall("prospector_thresholds");
    expect(saved.ores.find((o: { ore: string }) => o.ore === "platinum").threshold).toBeNull();
    await fireEvent.change(screen.getByLabelText("Painite threshold, percent"), { target: { value: "150" } });
    expect(await screen.findByText(/enter a percentage from 0 to 100/)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "Reset to 20%" }));
    saved = await fakeCall("prospector_thresholds");
    expect(saved.ores.find((o: { ore: string }) => o.ore === "platinum").threshold).toBe(20);
  });
});
