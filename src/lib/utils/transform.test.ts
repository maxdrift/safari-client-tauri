import { describe, expect, it } from "vitest";
import { transformIdToCss } from "./transform";

describe("transformIdToCss", () => {
  it("covers 0–7", () => {
    for (let i = 0; i < 8; i++) {
      expect(transformIdToCss(i)).toBeTruthy();
    }
  });
});
