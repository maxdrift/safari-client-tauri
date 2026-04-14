/**
 * Thumbnail loading regression tests (Vitest + Testing Library).
 *
 * - `drainThumbnailQueue`: asserts mocked `regenerate_thumbnails_cmd` updates `app.slides` state.
 * - `SlideTile`: grid uses `slide.path` (like lightbox); asserts spinner clears after img `load`.
 * - Does not run the real Tauri webview; if these pass but the desktop app still fails, debug IPC/asset protocol next.
 */
import { describe, it, expect, vi, beforeEach } from "vitest";
import { render } from "@testing-library/svelte";
import { tick } from "svelte";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
  /** Stable fake URL so <img src> is deterministic in tests */
  convertFileSrc: (path: string) => `asset://local${path.startsWith("/") ? "" : "/"}${path}`,
}));

import * as app from "$lib/app.svelte";
import { invoke } from "@tauri-apps/api/core";
import SlideTile from "$lib/components/SlideTile.svelte";

function resetAppSlides(): void {
  app.slides.length = 0;
  app.slidesRenderEpoch.n = 0;
  app.gridLayoutEpoch.n = 0;
  app.selectedIds.length = 0;
}

describe("drainThumbnailQueue", () => {
  beforeEach(() => {
    resetAppSlides();
    vi.mocked(invoke).mockReset();
    vi.mocked(invoke).mockImplementation(async (cmd: string) => {
      if (cmd === "regenerate_thumbnails_cmd") {
        return {
          s350: "/cache/350.jpg",
          s512: "/cache/512.jpg",
          s1024: "/cache/1024.jpg",
        };
      }
      throw new Error(`unexpected invoke: ${cmd}`);
    });
  });

  it("sets thumbnailsPending false after regenerate completes", async () => {
    app.slides.push({
      id: "photo.jpg",
      path: "/abs/photo.jpg",
      width: 800,
      height: 600,
      category: "excluded",
      subjectId: 0,
      transformId: 0,
      thumbnails: { s350: "", s512: "", s1024: "" },
      thumbnailsPending: true,
    });
    app.slidesRenderEpoch.n++;

    await app.drainThumbnailQueue();

    expect(app.slides).toHaveLength(1);
    expect(app.slides[0].thumbnailsPending).toBe(false);
    expect(app.slides[0].thumbnails.s350).toBe("/cache/350.jpg");
    expect(vi.mocked(invoke)).toHaveBeenCalledWith(
      "regenerate_thumbnails_cmd",
      expect.objectContaining({ path: "/abs/photo.jpg", transformId: 0 }),
    );
  });

  it("drains every pending slide", async () => {
    for (let i = 0; i < 5; i++) {
      app.slides.push({
        id: `f${i}.jpg`,
        path: `/p/f${i}.jpg`,
        width: 1,
        height: 1,
        category: "excluded",
        subjectId: 0,
        transformId: 0,
        thumbnails: { s350: "", s512: "", s1024: "" },
        thumbnailsPending: true,
      });
    }
    app.slidesRenderEpoch.n++;

    await app.drainThumbnailQueue();

    expect(app.slides.length).toBe(5);
    expect(app.slides.every((s) => s.thumbnailsPending === false)).toBe(true);
    expect(vi.mocked(invoke).mock.calls.filter((c) => c[0] === "regenerate_thumbnails_cmd")).toHaveLength(5);
  });
});

describe("SlideTile (grid uses original path)", () => {
  beforeEach(() => {
    resetAppSlides();
  });

  it("hides spinner after the image fires load (src is slide.path, not cache)", async () => {
    app.slides.push({
      id: "tile-id",
      path: "/photos/sample.jpg",
      width: 1,
      height: 1,
      category: "excluded",
      subjectId: 0,
      transformId: 0,
      thumbnails: { s350: "/thumbs/350.jpg", s512: "", s1024: "" },
      thumbnailsPending: true,
    });
    app.slidesRenderEpoch.n++;

    const { container } = render(SlideTile, {
      props: {
        slideId: "tile-id",
        speciesLabelFor: () => "",
        selected: false,
        reorderEnabled: false,
        onToggleSelect: () => {},
        onCycleCategory: () => {},
        onOpenSpecies: () => {},
        onOpenLightbox: () => {},
      },
    });

    const img = container.querySelector("img");
    expect(img).not.toBeNull();
    expect(img?.getAttribute("src")).toContain("sample.jpg");

    expect(container.querySelector('[aria-busy="true"]')).toBeTruthy();

    img?.dispatchEvent(new Event("load"));
    await tick();

    expect(container.querySelector('[aria-busy="true"]')).toBeNull();
  });

  it("uses slide.path for src after background drain (pending flag unrelated to grid img)", async () => {
    vi.mocked(invoke).mockImplementation(async (cmd: string) => {
      if (cmd === "regenerate_thumbnails_cmd") {
        return {
          s350: "/cache/350.jpg",
          s512: "/cache/512.jpg",
          s1024: "/cache/1024.jpg",
        };
      }
      throw new Error(`unexpected invoke: ${cmd}`);
    });

    app.slides.push({
      id: "after-drain",
      path: "/p/original.jpg",
      width: 2,
      height: 2,
      category: "excluded",
      subjectId: 0,
      transformId: 0,
      thumbnails: { s350: "", s512: "", s1024: "" },
      thumbnailsPending: true,
    });
    app.slidesRenderEpoch.n++;

    await app.drainThumbnailQueue();
    expect(app.slides[0].thumbnailsPending).toBe(false);

    const { container } = render(SlideTile, {
      props: {
        slideId: "after-drain",
        speciesLabelFor: () => "",
        selected: false,
        reorderEnabled: false,
        onToggleSelect: () => {},
        onCycleCategory: () => {},
        onOpenSpecies: () => {},
        onOpenLightbox: () => {},
      },
    });

    const img = container.querySelector("img");
    expect(img?.getAttribute("src")).toContain("original.jpg");
    img?.dispatchEvent(new Event("load"));
    await tick();
    expect(container.querySelector('[aria-busy="true"]')).toBeNull();
  });
});
