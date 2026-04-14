/** CSS preview for transform_id 0–7 (PRD §8.6, same convention as Rust). */
export function transformIdToCss(id: number): string {
  const t = ((id % 8) + 8) % 8;
  const map: Record<number, string> = {
    0: "none",
    1: "rotate(90deg)",
    2: "rotate(180deg)",
    3: "rotate(270deg)",
    4: "scaleX(-1)",
    5: "rotate(90deg) scaleX(-1)",
    6: "rotate(180deg) scaleX(-1)",
    7: "rotate(270deg) scaleX(-1)",
  };
  return map[t] ?? "none";
}

/**
 * EXIF orientation tag (1–8) → D₄ id (see `exif_orientation_to_d4` in Rust). Must stay in sync with `dump_tables_for_typescript`.
 */
const EXIF_TO_D4: Record<number, number> = {
  1: 0,
  2: 4,
  3: 2,
  4: 6,
  5: 5,
  6: 1,
  7: 7,
  8: 3,
};

/** `compose_transform_ids(outer, inner)` from Rust — outer ∘ inner on pixels. */
const COMPOSE_D4: number[][] = [
  [0, 1, 2, 3, 4, 5, 6, 7],
  [1, 2, 3, 0, 7, 4, 5, 6],
  [2, 3, 0, 1, 6, 7, 4, 5],
  [3, 0, 1, 2, 5, 6, 7, 4],
  [4, 5, 6, 7, 0, 1, 2, 3],
  [5, 6, 7, 4, 3, 0, 1, 2],
  [6, 7, 4, 5, 2, 3, 0, 1],
  [7, 4, 5, 6, 1, 2, 3, 0],
];

/**
 * Combined CSS transform for on-screen display: apply EXIF uprighting first, then the user’s `transform_id`.
 * Use with `image-orientation: none` on `<img>` so the browser does not apply EXIF twice.
 */
export function displayTransformId(
  userTransformId: number,
  exifOrientation: number | undefined,
): number {
  const u = ((Math.floor(userTransformId) % 8) + 8) % 8;
  const o = exifOrientation ?? 1;
  const clamped = Math.min(8, Math.max(1, Math.round(o)));
  const inner = EXIF_TO_D4[clamped] ?? 0;
  return COMPOSE_D4[u]?.[inner] ?? 0;
}
