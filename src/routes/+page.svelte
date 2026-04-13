<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open, save, message } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";
  import * as app from "$lib/app.svelte";
  import type { FilterTab, Slide } from "$lib/types";
  import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import LoadProgressOverlay from "$lib/components/LoadProgressOverlay.svelte";
  import Lightbox from "$lib/components/Lightbox.svelte";
  import NavBar from "$lib/components/NavBar.svelte";
  import SlideGrid from "$lib/components/SlideGrid.svelte";
  import SpeciesModal from "$lib/components/SpeciesModal.svelte";
  import SpeciesOverview from "$lib/components/SpeciesOverview.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import { I18N } from "$lib/utils/i18n";

  let speciesModalOpen = $state(false);
  let speciesTargets = $state<string[]>([]);
  let lightboxSlide = $state<Slide | null>(null);
  let confirmDeleteOpen = $state(false);
  let loadProgress = $state<{ active: boolean; done: number; total: number }>({
    active: false,
    done: 0,
    total: 0,
  });

  const visible = $derived.by(() => {
    void app.slidesRenderEpoch.n;
    if (app.ui.filterTab === "all") return [...app.slides];
    return app.slides.filter((s) => s.category === app.ui.filterTab);
  });
  const reorderEnabled = $derived(app.ui.filterTab === "all");
  const cnt = $derived.by(() => {
    void app.slidesRenderEpoch.n;
    return app.counts();
  });
  const usage = $derived.by(() => {
    void app.slidesRenderEpoch.n;
    return app.speciesUsage();
  });
  const selectionCount = $derived(app.selectedIds.length);

  const GRID_MIN_KEY = "safari-client-grid-min-cell";
  function readGridMin(): number {
    if (typeof localStorage === "undefined") return 200;
    const n = Number(localStorage.getItem(GRID_MIN_KEY));
    return Number.isFinite(n) && n >= 120 && n <= 400 ? n : 200;
  }
  let gridMinCellPx = $state(readGridMin());

  const lightboxSpecies = $derived(
    lightboxSlide ? app.speciesById(lightboxSlide.subjectId) : null,
  );
  const lightboxSlideSelected = $derived(
    lightboxSlide ? app.selectedIds.includes(lightboxSlide.id) : false,
  );

  onMount(() => {
    void app.initApp();
  });

  $effect(() => {
    localStorage.setItem(GRID_MIN_KEY, String(gridMinCellPx));
  });

  $effect(() => {
    void app.slidesRenderEpoch.n;
    const id = lightboxSlide?.id;
    if (!id) return;
    const fresh = app.slides.find((x) => x.id === id);
    if (fresh) lightboxSlide = fresh;
    else lightboxSlide = null;
  });

  async function pickImages() {
    const sel = await open({
      multiple: true,
      filters: [{ name: "Immagini", extensions: ["jpg", "jpeg", "png", "gif"] }],
    });
    if (sel === null) return;
    const paths = typeof sel === "string" ? [sel] : sel;
    loadProgress = { active: true, done: 0, total: paths.length };
    try {
      await app.addImagesFromPaths(paths, (done, total) => {
        loadProgress = { active: true, done, total };
      });
    } finally {
      loadProgress = { active: false, done: 0, total: 0 };
    }
  }

  async function exportCsv() {
    const path = await save({
      defaultPath: "scheda_concorrente.csv",
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (path === null) return;
    try {
      await invoke("export_csv_cmd", { slides: app.slides, path });
      await message(I18N.exportSuccess, { title: I18N.appTitle });
    } catch (e) {
      await message(String(e), {
        title: I18N.exportErrorTitle,
        kind: "error",
      });
    }
  }

  function openSpeciesFor(id: string) {
    speciesTargets = [id];
    speciesModalOpen = true;
  }

  function openSpeciesBulk() {
    speciesTargets = [...app.selectedIds];
    speciesModalOpen = true;
  }

  function onPickSpecies(subjectId: number) {
    app.setSpeciesForTargets(speciesTargets, subjectId);
    speciesModalOpen = false;
    speciesTargets = [];
  }

  function setFilter(t: FilterTab) {
    app.setFilterTab(t);
  }

  function onReorder(nextVisible: Slide[]) {
    if (app.ui.filterTab !== "all") return;
    app.reorderSlides(nextVisible);
  }

  function syncLightbox() {
    if (!lightboxSlide) return;
    const s = app.slides.find((x) => x.id === lightboxSlide!.id);
    if (s) lightboxSlide = s;
  }

  async function lightboxTransform(action: string) {
    if (!lightboxSlide) return;
    const s = app.slides.find((x) => x.id === lightboxSlide!.id);
    if (!s) return;
    await app.applyTransform(s, action);
    syncLightbox();
  }
</script>

<div class="flex h-screen min-h-0 flex-col bg-zinc-50 dark:bg-zinc-950">
  <NavBar
    {selectionCount}
    hasSlides={app.slides.length > 0}
    onExport={exportCsv}
    onDeselectAll={() => app.deselectAllInFilter(visible.map((s) => s.id))}
    onSelectAll={() => app.selectAllVisible(visible.map((s) => s.id))}
    onSetCategory={(c) => app.setCategoryForSelected(c, visible.map((s) => s.id))}
    onAssignSpecies={openSpeciesBulk}
    onDelete={() => (confirmDeleteOpen = true)}
    onClearSelection={() => app.clearSelection()}
  />

  <TabBar tab={app.ui.filterTab} onChange={setFilter} counts={cnt} />

  {#if app.slides.length > 0}
    <div
      class="flex shrink-0 flex-wrap items-center gap-3 border-b border-zinc-200 bg-white/80 px-4 py-2 dark:border-zinc-800 dark:bg-zinc-950/80"
    >
      <label class="text-xs text-zinc-500 dark:text-zinc-400" for="grid-zoom">{I18N.gridZoom}</label>
      <input
        id="grid-zoom"
        type="range"
        class="h-2 w-40 max-w-[50vw] flex-1 accent-emerald-600 md:w-56"
        min="120"
        max="400"
        step="10"
        bind:value={gridMinCellPx}
      />
      <span class="w-10 tabular-nums text-xs text-zinc-500 dark:text-zinc-500">{gridMinCellPx}px</span>
    </div>
  {/if}

  <div class="flex min-h-0 flex-1 overflow-hidden">
    <div class="flex min-h-0 min-w-0 flex-1 flex-col overflow-y-auto">
      {#if app.slides.length === 0}
        <EmptyState onLoad={pickImages} />
      {:else}
        <SlideGrid
          minCellPx={gridMinCellPx}
          {visible}
          {reorderEnabled}
          selectedIds={app.selectedIds}
          speciesLabelFor={(id) => app.speciesCommonName(id)}
          onToggleSelect={(id) => app.toggleSelected(id)}
          onCycleCategory={(id) => app.cycleCategoryForSlide(id)}
          onOpenSpecies={openSpeciesFor}
          onOpenLightbox={(id) => {
            const s = app.slides.find((x) => x.id === id) ?? null;
            lightboxSlide = s;
          }}
          onReorder={onReorder}
        />
      {/if}
    </div>
    <SpeciesOverview species={app.speciesList} {usage} />
  </div>

  {#if app.slides.length > 0 && selectionCount === 0}
    <button
      type="button"
      class="fixed bottom-6 right-6 z-40 rounded-full bg-emerald-600 px-5 py-3 text-sm font-medium text-white shadow-lg hover:bg-emerald-500"
      onclick={pickImages}>{I18N.loadImages}</button
    >
  {/if}
</div>

<SpeciesModal
  open={speciesModalOpen}
  species={app.speciesList}
  onPick={onPickSpecies}
  onClose={() => {
    speciesModalOpen = false;
    speciesTargets = [];
  }}
/>

<Lightbox
  slide={lightboxSlide}
  slides={app.slides}
  species={lightboxSpecies}
  slideSelected={lightboxSlideSelected}
  onClose={() => (lightboxSlide = null)}
  onNavigate={(s) => (lightboxSlide = s)}
  onTransform={lightboxTransform}
/>

<LoadProgressOverlay
  active={loadProgress.active}
  done={loadProgress.done}
  total={loadProgress.total}
  label={I18N.loadingImages}
/>

<ConfirmDialog
  open={confirmDeleteOpen}
  message={I18N.confirmDelete}
  onCancel={() => (confirmDeleteOpen = false)}
  onConfirm={() => {
    app.deleteSlidesByIds([...app.selectedIds]);
    confirmDeleteOpen = false;
  }}
/>
