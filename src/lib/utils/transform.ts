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
