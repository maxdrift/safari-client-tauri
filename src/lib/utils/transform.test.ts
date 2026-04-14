import { describe, expect, it } from "vitest";
import { displayTransformId, transformIdToCss } from "./transform";

describe("transformIdToCss", () => {
  it("covers 0–7", () => {
    for (let i = 0; i < 8; i++) {
      expect(transformIdToCss(i)).toBeTruthy();
    }
  });
});

describe("displayTransformId", () => {
  it("matches Rust: identity user + EXIF 6 → 1", () => {
    expect(displayTransformId(0, 6)).toBe(1);
  });

  it("defaults missing EXIF to 1 (no extra rotation)", () => {
    expect(displayTransformId(0, undefined)).toBe(0);
  });
});
