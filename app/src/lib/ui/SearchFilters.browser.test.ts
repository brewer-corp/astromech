import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { DEFAULT_FILTERS, type Filters } from "../search";
import SearchFilters from "./SearchFilters.svelte";

const rest = (filters: Partial<Filters> = {}) =>
  render(SearchFilters, {
    filters: { ...DEFAULT_FILTERS, ...filters },
    offers: ["distance", "age"],
    hints: { distance: 100, age: 30 },
    summary: "Within 100 ly of Sol · reports up to 30 days old",
    onchange: vi.fn(),
  });
/** The filters with their controls open. */
const show = async (filters: Partial<Filters>) => {
  rest(filters);
  await fireEvent.click(screen.getByRole("button", { name: "Change filters" }));
};

const shown = (label: string) => {
  const select = screen.getByText(label).closest("label")!.querySelector("select")!;
  return select.selectedOptions[0]?.textContent;
};

describe("SearchFilters", () => {
  it("rests as its summary with CHANGE, and opens in place until DONE, Escape or CHANGE again", async () => {
    rest();
    const change = screen.getByRole("button", { name: "Change filters" });
    expect(screen.getByText("Within 100 ly of Sol · reports up to 30 days old")).toBeTruthy();
    expect(screen.queryByRole("combobox")).toBeNull();
    expect(change.getAttribute("aria-expanded")).toBe("false");

    await fireEvent.click(change);
    expect(screen.getAllByRole("combobox")).toHaveLength(2);
    expect(change.getAttribute("aria-expanded")).toBe("true");
    await fireEvent.click(screen.getByRole("button", { name: "Done" }));
    expect(screen.queryByRole("combobox")).toBeNull();
    expect(document.activeElement).toBe(change);

    await fireEvent.click(change);
    await fireEvent.keyDown(screen.getAllByRole("combobox")[0]!, { key: "Escape" });
    expect(screen.queryByRole("combobox")).toBeNull();

    await fireEvent.click(change);
    await fireEvent.click(change);
    expect(screen.queryByRole("combobox")).toBeNull();
  });

  it("shows the default for a value that is the default", async () => {
    await show({ max_distance_ly: 100, max_age_days: 30 });
    expect(shown("Within")).toBe("100 ly");
    expect(shown("Data age")).toBe("Up to 30 days");
  });

  it("shows the saved values", async () => {
    await show({ max_distance_ly: 50, max_age_days: 7 });
    expect(shown("Within")).toBe("50 ly");
    expect(shown("Data age")).toBe("Up to 7 days");
  });

  it("shows a saved value the list doesn't offer instead of a blank", async () => {
    await show({ max_distance_ly: 75, max_age_days: 14 });
    expect(shown("Within")).toBe("75 ly");
    expect(shown("Data age")).toBe("Up to 14 days");
  });

  it("shows the default when nothing is saved", async () => {
    await show({});
    expect(shown("Within")).toBe("100 ly");
    expect(shown("Data age")).toBe("Up to 30 days");
  });
});
