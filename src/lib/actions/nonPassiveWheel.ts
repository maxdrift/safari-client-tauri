import type { Action } from "svelte/action";

/** `wheel` with `{ passive: false }` so `preventDefault()` works (zoom). */
export const nonPassiveWheel: Action<HTMLElement, (e: WheelEvent) => void> = (
  node,
  initial,
) => {
  let handler = initial;
  const wrapped = (e: WheelEvent) => handler(e);
  node.addEventListener("wheel", wrapped, { passive: false });
  return {
    update(next: (e: WheelEvent) => void) {
      handler = next;
    },
    destroy() {
      node.removeEventListener("wheel", wrapped);
    },
  };
};
