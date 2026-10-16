import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, within } from "@testing-library/svelte";
import View from "./View.svelte";

describe("Engineer unlock tree", () => {
  it("marks progress and names what is left for each engineer", async () => {
    render(View);
    const card = async (name: string) => within(await screen.findByRole("article", { name }));
    expect((await card("Felicity Farseer")).getByText("Unlocked · rank 5")).toBeTruthy();
    expect((await card("Marco Qwent")).getByText("Known")).toBeTruthy();
    expect((await card("Marco Qwent")).getByText("Next: Invitation: Ally reputation with Sirius Corporation.")).toBeTruthy();
    expect((await card("Lori Jameson")).getByText("Locked")).toBeTruthy();
    expect((await card("Lori Jameson")).getByText("Unlock Marco Qwent first.")).toBeTruthy();
    expect((await card("Lori Jameson")).getByText("← Marco Qwent")).toBeTruthy();
    expect((await card("Hera Tani")).getByText(/^Next: Introduction from Liz Ryder/)).toBeTruthy();
    expect(screen.getByText(/4 of 25 unlocked/)).toBeTruthy();

    await fireEvent.click(screen.getByRole("tab", { name: "Suit engineers" }));
    expect((await card("Kit Fowler")).getByText("Invited")).toBeTruthy();
    expect((await card("Terra Velasquez")).getByText("Next: Referral from Jude Navarro: Provide 5 Genetic Repair Meds.")).toBeTruthy();
    expect(screen.getByText(/2 of 13 unlocked/)).toBeTruthy();
  });
});
