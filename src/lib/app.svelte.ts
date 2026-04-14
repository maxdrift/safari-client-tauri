import { tick } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import type {
  Category,
  FilterTab,
  PersistedSlide,
  Slide,
  Species,
  ThemePreference,
  ThumbnailPaths,
} from "$lib/types";
import { applyDocumentTheme, THEME_STORAGE_KEY } from "$lib/theme";

export const slides = $state<Slide[]>([]);
/** Incremented on slide list / thumbnail updates so UI `$derived.by` invalidates across module boundaries. Use `.n` mutation only (Svelte forbids reassigning exported `$state` primitives). */
export const slidesRenderEpoch = $state({ n: 0 });

/** Bumped to force-remount `SlideGrid` (see `{#key}` in `+page.svelte`) after bulk import — works around dndzone/Svelte leaving tiles stale until a tab switch. */
export const gridLayoutEpoch = $state({ n: 0 });

function bumpGridLayoutEpoch(): void {
  gridLayoutEpoch.n++;
}

export const ui = $state<{ filterTab: FilterTab; theme: ThemePreference }>({
  filterTab: "all",
  theme: "system",
});

export function setFilterTab(t: FilterTab): void {
  ui.filterTab = t;
}

export function setThemePreference(t: ThemePreference): void {
  ui.theme = t;
  try {
    localStorage.setItem(THEME_STORAGE_KEY, t);
  } catch {
    /* ignore */
  }
  applyDocumentTheme(t);
}

export function initTheme(): void {
  try {
    const raw = localStorage.getItem(THEME_STORAGE_KEY);
    if (raw === "light" || raw === "dark" || raw === "system") {
      ui.theme = raw;
    }
  } catch {
    /* ignore */
  }
  applyDocumentTheme(ui.theme);
  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", () => {
      if (ui.theme === "system") applyDocumentTheme(ui.theme);
    });
}
export const speciesList = $state<Species[]>([]);
/** Selected slide ids (basename) */
export const selectedIds = $state<string[]>([]);

const CATEGORY_ORDER: Category[] = ["excluded", "fixed", "jury"];

/** Tauri may omit `thumbnailsPending`; treat missing as pending so the grid shows a loader until previews exist. */
function normalizeSlide(s: Slide): Slide {
  const snake = (s as Slide & { thumbnails_pending?: boolean }).thumbnails_pending;
  const pending = s.thumbnailsPending ?? snake;
  return {
    ...s,
    thumbnailsPending: pending !== false,
  };
}

function bumpSlidesRenderEpoch(): void {
  slidesRenderEpoch.n++;
}

function toPersisted(s: Slide): PersistedSlide {
  return {
    id: s.id,
    path: s.path,
    category: s.category,
    subjectId: s.subjectId,
    transformId: s.transformId,
  };
}

let saveTimer: ReturnType<typeof setTimeout> | null = null;

export function scheduleSave(): void {
  if (saveTimer) clearTimeout(saveTimer);
  saveTimer = setTimeout(async () => {
    try {
      await invoke("save_app_state_cmd", {
        state: { slides: slides.map(toPersisted) },
      });
    } catch (e) {
      console.error(e);
    }
  }, 400);
}

export async function initApp(): Promise<void> {
  const catalog = await invoke<Species[]>("load_species_catalog_cmd");
  speciesList.length = 0;
  speciesList.push(...catalog);
  try {
    const restored = await invoke<Slide[]>("restore_slides_cmd");
    slides.length = 0;
    slides.push(...restored.map(normalizeSlide));
    bumpSlidesRenderEpoch();
  } catch (e) {
    console.error(e);
  }
  void drainThumbnailQueue();
  if (slides.length > 0) {
    void tick().then(() => {
      requestAnimationFrame(() => bumpGridLayoutEpoch());
    });
  }
}

export function mergeIncoming(existing: Slide[], incoming: Slide[]): Slide[] {
  const out = [...existing];
  for (const raw of incoming) {
    const s = normalizeSlide(raw);
    const i = out.findIndex((x) => x.id === s.id);
    if (i >= 0) out[i] = s;
    else out.push(s);
  }
  return out;
}

const LOAD_BATCH_SIZE = 3;

function chunkPaths(paths: string[], size: number): string[][] {
  const out: string[][] = [];
  for (let i = 0; i < paths.length; i += size) {
    out.push(paths.slice(i, i + size));
  }
  return out;
}

function yieldToMain(): Promise<void> {
  return new Promise((resolve) => {
    requestAnimationFrame(() => resolve());
  });
}

let drainPromise: Promise<void> | null = null;

/** Generates preview files in the background (Rust `regenerate_thumbnails_cmd`); UI stays responsive. */
export function drainThumbnailQueue(): Promise<void> {
  if (drainPromise) return drainPromise;
  drainPromise = (async () => {
    try {
      while (slides.some((s) => s.thumbnailsPending)) {
        const batch = slides.filter((s) => s.thumbnailsPending).slice(0, 2);
        await Promise.all(
          batch.map(async (s) => {
            try {
              const thumbs = await invoke<ThumbnailPaths>("regenerate_thumbnails_cmd", {
                path: s.path,
                transformId: s.transformId,
              });
              const idx = slides.findIndex((x) => x.id === s.id);
              if (idx < 0) return;
              const cur = slides[idx];
              if (!cur.thumbnailsPending) return;
              const next: Slide = {
                ...cur,
                thumbnails: thumbs,
                thumbnailsPending: false,
              };
              slides.splice(idx, 1, next);
              bumpSlidesRenderEpoch();
              await tick();
            } catch (e) {
              console.error(e);
            }
          }),
        );
        await yieldToMain();
      }
    } finally {
      drainPromise = null;
      // If new pending slides appeared while this drain was finishing, run again (covers races with init/load).
      if (slides.some((s) => s.thumbnailsPending)) {
        queueMicrotask(() => {
          void drainThumbnailQueue();
        });
      }
    }
  })();
  return drainPromise;
}

/** Carica le immagini a batch per non bloccare la UI; opzionale callback di avanzamento. */
export async function addImagesFromPaths(
  paths: string[],
  onProgress?: (done: number, total: number) => void,
): Promise<void> {
  if (paths.length === 0) return;
  const total = paths.length;
  const batches = chunkPaths(paths, LOAD_BATCH_SIZE);
  let current = [...slides];
  let done = 0;
  onProgress?.(0, total);

  for (const batch of batches) {
    const loaded = await invoke<Slide[]>("load_slides_from_paths_cmd", { paths: batch });
    current = mergeIncoming(current, loaded);
    slides.length = 0;
    slides.push(...current);
    bumpSlidesRenderEpoch();
    await tick();
    done += batch.length;
    onProgress?.(done, total);
    await yieldToMain();
  }

  scheduleSave();
  void drainThumbnailQueue();
  await tick();
  await new Promise<void>((resolve) => {
    requestAnimationFrame(() => resolve());
  });
  bumpGridLayoutEpoch();
}

export function speciesCommonName(subjectId: number): string {
  if (subjectId === 0) return "";
  const s = speciesList.find((x) => x.id === subjectId);
  return s?.commonName ?? "";
}

export function speciesById(subjectId: number): Species | null {
  if (subjectId === 0) return null;
  return speciesList.find((x) => x.id === subjectId) ?? null;
}

export function toggleSelected(id: string): void {
  const i = selectedIds.indexOf(id);
  if (i >= 0) selectedIds.splice(i, 1);
  else selectedIds.push(id);
}

export function clearSelection(): void {
  selectedIds.length = 0;
}

export function selectAllVisible(visibleIds: string[]): void {
  selectedIds.length = 0;
  selectedIds.push(...visibleIds);
}

/** Rimuove dalla selezione le slide visibili nel filtro corrente (PRD §4.6). */
export function deselectAllInFilter(visibleIds: string[]): void {
  const keep = selectedIds.filter((id) => !visibleIds.includes(id));
  selectedIds.length = 0;
  selectedIds.push(...keep);
}

export function cycleCategoryForSlide(id: string): void {
  const s = slides.find((x) => x.id === id);
  if (!s) return;
  const i = CATEGORY_ORDER.indexOf(s.category);
  s.category = CATEGORY_ORDER[(i + 1) % 3];
  bumpSlidesRenderEpoch();
  scheduleSave();
}

export function setCategoryForSelected(cat: Category, visibleIds: string[]): void {
  const picked = selectedIds.filter((id) => visibleIds.includes(id));
  for (const slide of slides) {
    if (picked.includes(slide.id)) {
      slide.category = cat;
      const j = selectedIds.indexOf(slide.id);
      if (j >= 0) selectedIds.splice(j, 1);
    }
  }
  bumpSlidesRenderEpoch();
  scheduleSave();
}

export function setSpeciesForTargets(targetIds: string[], subjectId: number): void {
  for (const slide of slides) {
    if (targetIds.includes(slide.id)) {
      slide.subjectId = subjectId;
      const j = selectedIds.indexOf(slide.id);
      if (j >= 0) selectedIds.splice(j, 1);
    }
  }
  bumpSlidesRenderEpoch();
  scheduleSave();
}

export function deleteSlidesByIds(ids: string[]): void {
  for (const id of ids) {
    void invoke("remove_slide_cache_cmd", { filename: id });
  }
  const rest = slides.filter((s) => !ids.includes(s.id));
  slides.length = 0;
  slides.push(...rest);
  selectedIds.length = 0;
  bumpSlidesRenderEpoch();
  scheduleSave();
}

export function reorderSlides(newOrder: Slide[]): void {
  slides.length = 0;
  slides.push(...newOrder);
  bumpSlidesRenderEpoch();
  scheduleSave();
}

export async function applyTransform(slide: Slide, action: string): Promise<void> {
  const nextId = await invoke<number>("apply_transform_action_cmd", {
    currentId: slide.transformId,
    action,
  });
  const thumbs = await invoke<ThumbnailPaths>("regenerate_thumbnails_cmd", {
    path: slide.path,
    transformId: nextId,
  });
  const idx = slides.findIndex((x) => x.id === slide.id);
  if (idx < 0) return;
  slides.splice(idx, 1, {
    ...slides[idx],
    transformId: nextId,
    thumbnails: thumbs,
    thumbnailsPending: false,
  });
  bumpSlidesRenderEpoch();
  await tick();
  scheduleSave();
}

export function counts() {
  let excluded = 0;
  let fixed = 0;
  let jury = 0;
  for (const s of slides) {
    if (s.category === "excluded") excluded++;
    else if (s.category === "fixed") fixed++;
    else jury++;
  }
  return { all: slides.length, excluded, fixed, jury };
}

export function visibleSlides(): Slide[] {
  // New array each read so `$derived(visibleSlides())` invalidates when slide fields change (not only reference).
  if (ui.filterTab === "all") return [...slides];
  return slides.filter((s) => s.category === ui.filterTab);
}

export function speciesUsage(): Record<
  number,
  { jury: boolean; fixed: boolean }
> {
  const m: Record<number, { jury: boolean; fixed: boolean }> = {};
  for (const s of slides) {
    if (s.subjectId === 0) continue;
    if (s.category === "excluded") continue;
    const e = m[s.subjectId] ?? { jury: false, fixed: false };
    if (s.category === "jury") e.jury = true;
    if (s.category === "fixed") e.fixed = true;
    m[s.subjectId] = e;
  }
  return m;
}
