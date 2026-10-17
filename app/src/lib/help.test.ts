import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import App from "../App.svelte";
import { GUIDE_URL, helpUrl, SETUP_HELP } from "./help";
import { modules } from "./modules";

// That every target file and heading exists is checked by scripts/check-guide.py (it runs in CI),
// which reads the same `help:` values. Here: the shape of each, and the button.
const SHAPE = /^[a-z0-9-]+\.md(#[a-z0-9-]+)?$/;

describe("help links", () => {
  it("gives every page a help link of the form page.md or page.md#heading", () => {
    expect(modules.length).toBeGreaterThan(20);
    const bad = modules.filter((m) => !SHAPE.test(m.help)).map((m) => `${m.id}: ${m.help}`);
    expect(bad).toEqual([]);
  });

  it("has a target for first-run setup", () => {
    expect(SETUP_HELP).toMatch(SHAPE);
  });

  it("uses only the project's GitHub pages, which the shell allows to open", () => {
    expect(GUIDE_URL.startsWith("https://github.com/spadaval/astromech/")).toBe(true);
    expect(helpUrl("trade.md")).toBe("https://github.com/spadaval/astromech/blob/main/docs/guide/trade.md");
  });
});

describe("the Help link in the window", () => {
  afterEach(() => {
    cleanup();
    location.hash = "";
    vi.restoreAllMocks();
  });

  it("opens the guide page of the page you are on", async () => {
    const open = vi.spyOn(window, "open").mockImplementation(() => null);
    location.hash = "#/powerplay";
    render(App);
    await screen.findByText("My standing");
    await fireEvent.click(screen.getByRole("button", { name: "Help" }));
    expect(open).toHaveBeenCalledWith(helpUrl("powerplay.md"), "_blank");
  });

  it("follows the page: Trade opens the Trade guide", async () => {
    const open = vi.spyOn(window, "open").mockImplementation(() => null);
    location.hash = "#/trade";
    render(App);
    await screen.findByRole("button", { name: "Sell" });
    await fireEvent.click(screen.getByRole("button", { name: "Help" }));
    expect(open).toHaveBeenCalledWith(helpUrl("trade.md"), "_blank");
  });
});
