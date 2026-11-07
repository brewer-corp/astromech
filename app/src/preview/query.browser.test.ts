import { afterEach, expect, test, vi } from "vitest";
import { previewQuery } from "./query";

afterEach(() => {
  vi.unstubAllEnvs();
  history.replaceState(null, "", "/");
});

test("the page's own query wins over a preview build's", () => {
  vi.stubEnv("VITE_PREVIEW_QUERY", "core=empty");
  history.replaceState(null, "", "/?core=ready");
  expect(previewQuery().get("core")).toBe("ready");
});

test("with no query, a preview build's baked-in one answers", () => {
  vi.stubEnv("VITE_PREVIEW_QUERY", "core=ready&route=exact");
  expect(previewQuery().get("core")).toBe("ready");
  expect(previewQuery().get("route")).toBe("exact");
});

test("the page's own query adds to a preview build's, key by key", () => {
  vi.stubEnv("VITE_PREVIEW_QUERY", "core=ready&route=exact");
  history.replaceState(null, "", "/?fail=all");
  expect(previewQuery().get("core")).toBe("ready");
  expect(previewQuery().get("fail")).toBe("all");
});

test("with neither, nothing is set", () => {
  expect(previewQuery().get("core")).toBeNull();
});
