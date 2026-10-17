import { describe, expect, it } from "vitest";
import { render, screen, fireEvent } from "@testing-library/svelte";
import Callouts from "./Callouts.svelte";
import { fakeCall, resetFakeCore } from "../../lib/fakecore";

const find = (r: Record<string, unknown>, id: string) =>
  (r.callouts as { id: string; enabled: boolean; cooldown_s: number }[]).find((c) => c.id === id)!;

describe("Settings → Callouts", () => {
  it("lists callouts, saves a switch and a cooldown, resets, and refuses a bad value", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    render(Callouts);
    expect(await screen.findByText("Scans")).toBeTruthy();
    const toggle = (await screen.findByLabelText("Scans on or off")) as HTMLButtonElement;
    expect(toggle.getAttribute("aria-pressed")).toBe("true");
    expect((await screen.findByLabelText("Powerplay arrival on or off")).getAttribute("aria-pressed")).toBe("false");

    await fireEvent.click(toggle);
    await screen.findByText(/applies the next time Elite starts/);
    expect(find(await fakeCall("callouts"), "scan").enabled).toBe(false);

    const cd = screen.getByLabelText("Scans cooldown, seconds") as HTMLInputElement;
    expect(cd.value).toBe("30");
    await fireEvent.change(cd, { target: { value: "90" } });
    await screen.findByRole("button", { name: "Reset to 30 s" });
    expect(find(await fakeCall("callouts"), "scan").cooldown_s).toBe(90);

    await fireEvent.change(cd, { target: { value: "soon" } });
    expect(await screen.findByText(/enter a number of seconds from 0 to 86400/)).toBeTruthy();
    expect(find(await fakeCall("callouts"), "scan").cooldown_s).toBe(90);

    await fireEvent.click(screen.getByRole("button", { name: "Reset to 30 s" }));
    await screen.findByText(/applies the next time/);
    expect(find(await fakeCall("callouts"), "scan").cooldown_s).toBe(30);
  });
});
