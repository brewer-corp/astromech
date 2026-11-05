import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/svelte";
import View from "./View.svelte";

describe("Trade, Commodities", () => {
  it("links Sell, Buy and Sell my cargo into their Places finders instead of listing them, and keeps trade routes", async () => {
    render(View);
    expect(screen.getByRole("link", { name: "Sell" }).getAttribute("href")).toBe("#/places/sell/painite");
    expect(screen.getByRole("link", { name: "Buy" }).getAttribute("href")).toBe("#/places/buy/painite");
    expect(screen.getByRole("link", { name: "Sell my cargo" }).getAttribute("href")).toBe("#/places/sell-cargo");
    expect(screen.queryByRole("tab")).toBeNull();
    expect(screen.queryByRole("combobox", { name: "Commodity" })).toBeNull();
    expect(await screen.findByRole("button", { name: "Find route" })).toBeTruthy();
  });
});
