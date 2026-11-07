import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { DEFAULT_FILTERS, type Filters } from "../search";
import SearchFilters from "./SearchFilters.svelte";

const show = (filters: Partial<Filters>) =>
  render(SearchFilters, {
    filters: { ...DEFAULT_FILTERS, ...filters },
    offers: ["distance", "age"],
    hints: { distance: 100, age: 30 },
    onchange: vi.fn(),
  });

const shown = (label: string) => {
  const select = screen.getByText(label).closest("label")!.querySelector("select")!;
  return select.selectedOptions[0]?.textContent;
};

describe("SearchFilters", () => {
  it("shows the default for a value that is the default", () => {
    show({ max_distance_ly: 100, max_age_days: 30 });
    expect(shown("Within")).toBe("100 ly");
    expect(shown("Data age")).toBe("Up to 30 days");
  });

  it("shows the saved values", () => {
    show({ max_distance_ly: 50, max_age_days: 7 });
    expect(shown("Within")).toBe("50 ly");
    expect(shown("Data age")).toBe("Up to 7 days");
  });

  it("shows a saved value the list doesn't offer instead of a blank", () => {
    show({ max_distance_ly: 75, max_age_days: 14 });
    expect(shown("Within")).toBe("75 ly");
    expect(shown("Data age")).toBe("Up to 14 days");
  });

  it("shows the default when nothing is saved", () => {
    show({});
    expect(shown("Within")).toBe("100 ly");
    expect(shown("Data age")).toBe("Up to 30 days");
  });
});
