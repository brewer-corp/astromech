import { afterEach, describe, expect, it } from "vitest";
import { userEvent } from "vitest/browser";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import IconButton from "./IconButton.svelte";
import Button from "./Button.svelte";
import { tip } from "./tip";
import { createRawSnippet } from "svelte";
import { createAttachmentKey } from "svelte/attachments";

const shown = () => [...document.querySelectorAll<HTMLElement>(".tip")].filter((t) => !t.hidden);
const words = (s: string) => createRawSnippet(() => ({ render: () => `<span>${s}</span>` }));

describe("Tip", () => {
  afterEach(() => cleanup());

  it("shows at once on hover, swaps in place on the next target, and goes when the pointer leaves", async () => {
    render(IconButton, { icon: "add-stop", label: "Add Sol as a stop" });
    render(IconButton, { icon: "destination", label: "Set Sol as the destination", tip: "Set Sol as the destination (replaces the route)" });
    const [add, dest] = screen.getAllByRole("button");

    await fireEvent.pointerEnter(add!);
    expect(shown().map((t) => t.textContent)).toEqual(["Add Sol as a stop"]);
    const box = shown()[0];

    await fireEvent.pointerLeave(add!);
    await fireEvent.pointerEnter(dest!);
    expect(shown()).toEqual([box]);
    expect(box!.textContent).toBe("Set Sol as the destination (replaces the route)");

    await fireEvent.pointerLeave(dest!);
    expect(shown()).toEqual([]);
  });

  it("goes on Escape and on scroll", async () => {
    render(IconButton, { icon: "copy", label: "Copy Sol" });
    const copy = screen.getByRole("button");

    await fireEvent.pointerEnter(copy);
    await fireEvent.keyDown(document, { key: "Escape" });
    expect(shown()).toEqual([]);

    await fireEvent.pointerEnter(copy);
    await fireEvent.scroll(window);
    expect(shown()).toEqual([]);
  });

  it("shows on keyboard focus, and screen readers get the same words", async () => {
    render(Button, { children: words("Spansh"), [createAttachmentKey()]: tip("Open Sol on Spansh") });
    render(IconButton, { icon: "copy", label: "Copy Sol" });

    await userEvent.tab();
    const spansh = screen.getByRole("button", { name: "Spansh", description: "Open Sol on Spansh" });
    expect(document.activeElement).toBe(spansh);
    expect(shown().map((t) => t.textContent)).toEqual(["Open Sol on Spansh"]);

    await userEvent.tab();
    expect(shown().map((t) => t.textContent)).toEqual(["Copy Sol"]);
    // Its tip is its name: it isn't read twice.
    expect(screen.getByRole("button", { name: "Copy Sol" }).hasAttribute("aria-describedby")).toBe(false);
  });

  it("gives a plain element a tab stop, and a repeated mark none", async () => {
    const host = document.createElement("div");
    host.innerHTML = `<span id="fact">12.4</span><span id="mark" role="img" aria-label="Coriolis Starport"></span>`;
    document.body.append(host);
    const offFact = tip("Full tank and full cargo")(host.querySelector("#fact")!);
    const offMark = tip({ title: "Coriolis Starport", stop: false })(host.querySelector("#mark")!);

    expect(host.querySelector<HTMLElement>("#fact")!.tabIndex).toBe(0);
    expect(host.querySelector("#mark")!.hasAttribute("tabindex")).toBe(false);

    if (offFact) offFact();
    if (offMark) offMark();
    host.remove();
  });
});
