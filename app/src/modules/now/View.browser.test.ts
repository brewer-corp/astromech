import "../../design/tokens.css";
import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen, waitFor, within } from "@testing-library/svelte";
import App from "../../App.svelte";
import type { CargoSearch } from "../../lib/trade";
import { fakeJump } from "../../preview/fakecore";
import { answer, idle, previewAnswer, startPreview, watch } from "../../test/transport";
import View from "./View.svelte";

// On the recorded replies: the station world's docking at Test Orbital (and its undocking),
// trade.json's cargo answer for a 600 t hold, the history world's session and the powerplay
// world's standing. `?game=running` says Elite is running; without it the status waits for Elite.
const block = async (name: string) => within(await screen.findByRole("region", { name }));
const DESTINATION = { name: "Set LHS 1827 as the destination" };
/** DESIGN.md Grey Dim, as the browser computes it. */
const GREY_DIM = "rgb(106, 106, 106)";
const colour = (el: HTMLElement) => getComputedStyle(el).color;

describe("Now", () => {
  afterEach(() => {
    cleanup();
    vi.useRealTimers();
  });

  it("docked: the station, its services with the absent ones dim, and what the hold fetches", async () => {
    startPreview("/?core=ready&game=running");
    render(View);
    expect(await screen.findByRole("heading", { name: /Test Orbital$/ })).toBeTruthy();
    const services = within(screen.getByRole("list", { name: "Services" }));
    expect(services.getAllByRole("listitem")).toHaveLength(20);
    expect(services.getByRole("listitem", { name: "Commodities" })).toBeTruthy();
    expect(services.getByRole("listitem", { name: "Vista Genomics: not here" })).toBeTruthy();
    // Reported but not in the station menu: not drawn.
    expect(services.queryByText("Contacts")).toBeNull();

    const sale = await block("Sell your cargo");
    expect(await sale.findByText("takes all 600 t")).toBeTruthy();
    expect(sale.getByText("183,674,700")).toBeTruthy();
    expect(sale.getByText("Good Landing")).toBeTruthy();
    expect(sale.getByRole("button", DESTINATION)).toBeTruthy();
  });

  it("asks for the hold again after a jump, keeping the old answer dimmed until the new one comes, and not on an unchanged poll", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    startPreview("/?core=ready&game=running");
    const asks = watch("trade_cargo");
    render(View);
    const sale = await block("Sell your cargo");
    await sale.findByText("takes all 600 t");
    await idle();
    expect(asks).toHaveBeenCalledTimes(1);

    // Polls with nothing changed ask nothing.
    await vi.advanceTimersByTimeAsync(10000);
    await idle();
    expect(asks).toHaveBeenCalledTimes(1);

    // The next answer waits until the test lets it through.
    let release = () => {};
    const held = new Promise<void>((r) => (release = r));
    answer("trade_cargo", async (args: unknown) => {
      await held;
      return previewAnswer<CargoSearch>("trade_cargo", args);
    });
    fakeJump("Deciat");
    await vi.advanceTimersByTimeAsync(5000); // the docked_here poll sees the jump
    await waitFor(() => expect(asks).toHaveBeenCalledTimes(2));
    expect(await sale.findByText("Asking Ardent…")).toBeTruthy();
    expect(colour(sale.getByText("183,674,700")), "the old answer stays, dimmed").toBe(GREY_DIM);
    expect(sale.getByRole("button", DESTINATION)).toHaveProperty("disabled", true);
    expect(screen.getByRole("heading", { name: /^Deciat/ }), "in space in the new system").toBeTruthy();

    release();
    await waitFor(() => expect(sale.queryByText("Asking Ardent…")).toBeNull());
    expect(sale.getByRole("button", DESTINATION)).toHaveProperty("disabled", false);
    expect(colour(sale.getByText("183,674,700"))).not.toBe(GREY_DIM);
  });

  it("a failed status or ship read keeps the blocks and asks nothing again", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    startPreview("/?core=ready&game=running");
    const asks = watch("trade_cargo");
    render(View);
    const sale = await block("Sell your cargo");
    await sale.findByText("takes all 600 t");
    await idle();
    let fail = true;
    answer("ship_status", async () => (fail ? Promise.reject("the core didn't answer") : previewAnswer("ship_status")));
    answer("status", async () => (fail ? Promise.reject("the core didn't answer") : previewAnswer("status")));
    await vi.advanceTimersByTimeAsync(2000);
    await idle();
    expect(screen.getByRole("region", { name: "Sell your cargo" })).toBeTruthy();
    // Each line says what may be out of date, beside it.
    expect(sale.getByText(/Could not read the ship: the core didn't answer/)).toBeTruthy();
    expect(screen.getByText(/Could not read whether Elite is running: the core didn't answer/)).toBeTruthy();
    expect(screen.getByRole("heading", { name: /Test Orbital$/ }), "the place stays").toBeTruthy();
    fail = false;
    await vi.advanceTimersByTimeAsync(2000);
    await idle();
    expect(asks).toHaveBeenCalledTimes(1);
    expect(screen.queryByText(/Could not read/), "gone once the reads answer").toBeNull();
  });

  it("a failed read of the place or of the data at risk keeps what is shown, and says so", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    startPreview("/?core=ready&game=running");
    render(View);
    await screen.findByRole("heading", { name: /Test Orbital$/ });
    await idle();
    answer("docked_here", () => Promise.reject("the journal is locked"));
    answer("exploration_unsold", () => Promise.reject("the history is busy"));
    await vi.advanceTimersByTimeAsync(15000);
    await idle();
    expect(screen.getByRole("heading", { name: /Test Orbital$/ })).toBeTruthy();
    expect(screen.getByText(/Could not read the docked station: the journal is locked/)).toBeTruthy();
    expect(screen.getByText(/Could not read your unsold data: the history is busy/)).toBeTruthy();
  });

  it("says the status could not be read, not that Elite isn't running, when the core never answered", async () => {
    startPreview("/?core=ready&game=running&fail=status");
    render(View);
    expect(await screen.findByText(/Could not read whether Elite is running/)).toBeTruthy();
    expect(screen.queryByRole("heading", { name: "Elite isn't running" })).toBeNull();
  });

  it("says when the hold can't be read, and asks once it can", async () => {
    vi.useFakeTimers({ toFake: ["setInterval", "clearInterval"] });
    let fail = true;
    answer("ship_status", async () => (fail ? Promise.reject("the core didn't answer") : previewAnswer("ship_status")));
    startPreview("/?core=ready&game=running");
    const asks = watch("trade_cargo");
    render(View);
    const sale = await block("Sell your cargo");
    expect(await sale.findByText(/Could not read the ship: the core didn't answer/)).toBeTruthy();
    expect(asks).not.toHaveBeenCalled();
    fail = false;
    await vi.advanceTimersByTimeAsync(2000);
    expect(await (await block("Sell your cargo")).findByText("takes all 600 t")).toBeTruthy();
    expect(asks).toHaveBeenCalledTimes(1);
  });

  it("with no station in reach, says so and offers no destination", async () => {
    // An empty answer the recorded fixtures lack.
    answer("trade_cargo", async (args: unknown) => {
      const r = await previewAnswer<CargoSearch>("trade_cargo", args);
      return { ...r, plan: { ...r.plan, stations: [] } };
    });
    startPreview("/?core=ready&game=running");
    render(View);
    const sale = await block("Sell your cargo");
    expect(await sale.findByText("No station within 100 ly buys your hold.")).toBeTruthy();
    expect(sale.queryByRole("button", { name: /destination|as a stop/ })).toBeNull();
    expect(sale.getByRole("link", { name: "Sell my cargo" })).toBeTruthy();
  });

  it("a failed ask fails that block only", async () => {
    startPreview("/?core=ready&game=running&fail=trade_cargo");
    render(View);
    const sale = await block("Sell your cargo");
    expect(await sale.findByText(/Could not ask Ardent Insight/)).toBeTruthy();
    expect(sale.getByRole("button", { name: "Try again" })).toBeTruthy();
    expect(await (await block("This session")).findByText("14.5")).toBeTruthy();
    expect(await (await block("Powerplay")).findByText("1,829")).toBeTruthy();
  });

  it("in space: the system leads, with no services", async () => {
    startPreview("/?core=ready&game=running&docked=no");
    render(View);
    expect(await screen.findByRole("heading", { name: /^Test System/ })).toBeTruthy();
    expect(await (await block("Sell your cargo")).findByText("takes all 600 t")).toBeTruthy();
    expect(screen.queryByRole("list", { name: "Services" })).toBeNull();
  });

  it("Elite not running: the last place and the last session, and no sale", async () => {
    startPreview("/?core=ready");
    render(View);
    expect(await screen.findByRole("heading", { name: "Elite isn't running" })).toBeTruthy();
    expect(await screen.findByText("Last docked at Test Orbital, Test System")).toBeTruthy();
    expect(await (await block("Last session")).findByText("14.5")).toBeTruthy();
    await idle();
    expect(screen.queryByRole("region", { name: "Sell your cargo" })).toBeNull();
    expect(screen.queryByRole("list", { name: "Services" })).toBeNull();
  });

  it("no game in the journal yet: one line and nothing else", async () => {
    startPreview("/?core=empty");
    render(View);
    expect(await screen.findByText(/^No game in the journal yet/)).toBeTruthy();
    await idle();
    expect(screen.queryAllByRole("region")).toHaveLength(0);
  });

  it("an old #/session link opens Session under NOW", async () => {
    startPreview("/?core=ready");
    location.hash = "#/session";
    render(App);
    await screen.findByText("Earned");
    expect(within(screen.getByRole("banner")).getByRole("button", { name: "Now" }).classList.contains("on")).toBe(true);
    expect(location.hash).toBe("#/session");
    location.hash = "";
  });
});
