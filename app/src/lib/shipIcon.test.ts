import { expect, it } from "vitest";
import { shipIcon } from "./shipIcon";

it("maps journal hulls to the Qohen Leth cartoons", () => {
  expect(shipIcon("krait_mkii")).toBe("/ships/krait-mk-ii.png");
  expect(shipIcon("anaconda")).toBe("/ships/anaconda.png");
  expect(shipIcon("no_such_ship")).toBeNull();
});
