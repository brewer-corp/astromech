import { describe, expect, it } from "vitest";
import shapes from "./core-reply-shapes.json";
import { fakeCall, resetFakeCore } from "./fakecore";

// The core's real replies, pinned in core-reply-shapes.json by a Rust test
// (crates/astromech-core/src/app.rs `setup_and_calibration_reply_shapes_match_the_window_fake`).
// The fake core answers the setup wizard in tests, so it must have the same keys.

/** Each object's keys, recursively; arrays and values are leaves (as in the Rust test). */
function shape(v: unknown): unknown {
  if (Array.isArray(v)) return "array";
  if (v !== null && typeof v === "object") return Object.fromEntries(Object.entries(v).map(([k, x]) => [k, shape(x)]));
  return "value";
}

describe("fake core replies match the real core's shapes", () => {
  it("answers each setup and calibration command with the real keys", async () => {
    history.replaceState(null, "", "/?core=ready");
    resetFakeCore();
    const got: Record<string, unknown> = {};
    got.find_elite = shape(await fakeCall("find_elite"));
    got.mic_list = shape(await fakeCall("mic_list"));
    got.calibrate_start = shape(await fakeCall("calibrate_start"));
    got.calibrate_record = shape(await fakeCall("calibrate_record"));
    for (let i = 0; i < 3; i++) await fakeCall("calibrate_record");
    got.calibrate_finish = shape(await fakeCall("calibrate_finish"));
    got.calibrate_cancel = shape(await fakeCall("calibrate_cancel"));
    got.mic_check = shape(await fakeCall("mic_check"));
    expect(got).toEqual(shapes);
  });
});
