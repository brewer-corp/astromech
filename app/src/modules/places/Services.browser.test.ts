import { afterEach, describe, expect, it, vi } from "vitest";
import { cleanup, render, screen, within } from "@testing-library/svelte";
import { FINDERS } from "../../lib/places";
import { DEFAULT_FILTERS } from "../../lib/search";
import sample from "../../preview/fixtures/stations.json";
import { answer } from "../../test/transport";
import Services from "./Services.svelte";

answer("search_filters", () => DEFAULT_FILTERS);
answer("set_search_filters", () => {});

const finder = FINDERS.find((f) => f.id === "services")!;
const row = async (name: string) => (await screen.findByText(name)).closest('[role="row"]') as HTMLElement;

afterEach(() => {
  cleanup();
  vi.useRealTimers();
  location.hash = "";
});

describe("Services finder", () => {
  it("puts a report older than 30 days in the trip group in Orange, in words, and steps its jumps down", async () => {
    // The recorded answer, read on the day it was fetched.
    vi.useFakeTimers({ toFake: ["Date"] });
    vi.setSystemTime(Date.parse(sample.fetched));
    location.hash = "#/places/services/interstellar_factors";
    render(Services, { finder });

    // Last reported in March 2021: the light years give way to its age.
    const old = await row("Rescue Ship - Li Qing Jao");
    expect(old.classList.contains("stale")).toBe(true);
    const age = within(old).getByText(/^[\d,]+ d old$/);
    expect(age.classList.contains("old")).toBe(true);
    expect(old.querySelector(".trip, .first .ly")).toBeNull();

    // Reported the day before: its light years, and no age on a list with a side panel.
    const fresh = await row("Bakshi Manufacturing Complex");
    expect(fresh.classList.contains("stale")).toBe(false);
    expect(within(fresh).queryByText(/d old$/)).toBeNull();
    expect(fresh.querySelector(".first .ly")?.textContent).toMatch(/ly$/);
  });
});
