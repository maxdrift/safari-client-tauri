<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { nonPassiveWheel } from "$lib/actions/nonPassiveWheel";
  import type { Slide, Species } from "$lib/types";
  import { displayTransformId, transformIdToCss } from "$lib/utils/transform";
  import { categoryLabel, I18N } from "$lib/utils/i18n";

  interface Props {
    slide: Slide | null;
    slides: Slide[];
    /** Full species row when assigned, otherwise null. */
    species: Species | null;
    slideSelected: boolean;
    onClose: () => void;
    onNavigate: (s: Slide) => void;
    onTransform: (action: string) => void;
  }
  let { slide, slides, species, slideSelected, onClose, onNavigate, onTransform }: Props =
    $props();

  let view = $state<{ zoom: number; mode: "fit" | "100" }>({ zoom: 1, mode: "fit" });
  let pan = $state({ x: 0, y: 0 });
  let dragging = $state(false);
  let dragRef = $state({ sx: 0, sy: 0, px: 0, py: 0 });

  let viewportEl = $state<HTMLDivElement | null>(null);
  let dialogEl = $state<HTMLDivElement | null>(null);

  const origUrl = $derived(slide ? convertFileSrc(slide.path) : "");
  const tf = $derived(
    slide
      ? transformIdToCss(displayTransformId(slide.transformId, slide.exifOrientation))
      : "none",
  );

  const navIdx = $derived(
    slide ? slides.findIndex((s) => s.id === slide.id) : -1,
  );
  const canPrev = $derived(navIdx > 0);
  const canNext = $derived(navIdx >= 0 && navIdx < slides.length - 1);

  const canPan = $derived(view.zoom > 1.001 || view.mode === "100");

  $effect(() => {
    if (slide) {
      view = { zoom: 1, mode: "fit" };
      pan = { x: 0, y: 0 };
    }
  });

  $effect(() => {
    if (slide && dialogEl) {
      dialogEl.focus();
    }
  });

  function handleWheel(e: WheelEvent) {
    if (!slide) return;
    e.preventDefault();
    let factor: number;
    if (e.ctrlKey || e.metaKey) {
      factor = e.deltaY < 0 ? 1.06 : 1 / 1.06;
    } else {
      factor = e.deltaY < 0 ? 1.09 : 1 / 1.09;
    }
    view = {
      ...view,
      zoom: Math.min(32, Math.max(0.05, view.zoom * factor)),
    };
  }

  function goPrev() {
    if (!slide || navIdx <= 0) return;
    onNavigate(slides[navIdx - 1]);
  }

  function goNext() {
    if (!slide || navIdx < 0 || navIdx >= slides.length - 1) return;
    onNavigate(slides[navIdx + 1]);
  }

  function onKeyDown(e: KeyboardEvent) {
    if (!slide) return;
    if (e.key === "Escape") {
      e.preventDefault();
      onClose();
      return;
    }
    if (e.key === "ArrowLeft") {
      e.preventDefault();
      goPrev();
      return;
    }
    if (e.key === "ArrowRight") {
      e.preventDefault();
      goNext();
    }
  }

  let pinchStartDist = $state(0);
  let pinchStartZoom = $state(1);

  function touchDistance(t: TouchList): number {
    if (t.length < 2) return 0;
    const a = t[0];
    const b = t[1];
    const dx = a.clientX - b.clientX;
    const dy = a.clientY - b.clientY;
    return Math.hypot(dx, dy);
  }

  function onTouchStart(e: TouchEvent) {
    if (e.touches.length === 2) {
      pinchStartDist = touchDistance(e.touches);
      pinchStartZoom = view.zoom;
    }
  }

  function onTouchMove(e: TouchEvent) {
    if (e.touches.length !== 2 || pinchStartDist <= 0) return;
    e.preventDefault();
    const d = touchDistance(e.touches);
    if (d > 0 && pinchStartDist > 0) {
      const ratio = d / pinchStartDist;
      view = {
        ...view,
        zoom: Math.min(32, Math.max(0.05, pinchStartZoom * ratio)),
      };
    }
  }

  function onTouchEnd(e: TouchEvent) {
    if (e.touches.length < 2) {
      pinchStartDist = 0;
    }
  }

  function onPanMouseDown(e: MouseEvent) {
    if (!slide || !canPan) return;
    if (e.button !== 0) return;
    e.preventDefault();
    dragging = true;
    dragRef = { sx: e.clientX, sy: e.clientY, px: pan.x, py: pan.y };
  }

  function onWindowMouseMove(e: MouseEvent) {
    if (!dragging) return;
    pan = {
      x: dragRef.px + e.clientX - dragRef.sx,
      y: dragRef.py + e.clientY - dragRef.sy,
    };
  }

  function onWindowMouseUp() {
    dragging = false;
  }

  function setFit() {
    view = { zoom: 1, mode: "fit" };
    pan = { x: 0, y: 0 };
    viewportEl?.scrollTo({ left: 0, top: 0 });
  }

  function setActual100() {
    view = { zoom: 1, mode: "100" };
    pan = { x: 0, y: 0 };
    viewportEl?.scrollTo({ left: 0, top: 0 });
  }

  function coeffFmt(c: number): string {
    if (Math.abs(c - Math.round(c)) < 1e-9) return String(Math.round(c));
    return String(c);
  }
</script>

<svelte:window
  onkeydown={onKeyDown}
  onmousemove={onWindowMouseMove}
  onmouseup={onWindowMouseUp}
/>

{#if slide}
  <div
    bind:this={dialogEl}
    class="fixed inset-0 z-[60] flex flex-col bg-black/90 outline-none dark:bg-black/95"
    role="dialog"
    aria-modal="true"
    aria-label="Anteprima immagine"
    tabindex="-1"
  >
    <div
      class="flex flex-wrap items-center justify-between gap-2 border-b border-zinc-700/80 bg-zinc-900/90 p-2 dark:border-zinc-800"
    >
      <div class="flex min-w-0 flex-1 items-center gap-2">
        <button
          type="button"
          class="rounded px-2 py-1 text-sm text-zinc-200 hover:bg-zinc-800 disabled:opacity-30"
          disabled={!canPrev}
          onclick={goPrev}
          title={I18N.prevImage}>←</button
        >
        <button
          type="button"
          class="rounded px-2 py-1 text-sm text-zinc-200 hover:bg-zinc-800 disabled:opacity-30"
          disabled={!canNext}
          onclick={goNext}
          title={I18N.nextImage}>→</button
        >
        <p class="min-w-0 flex-1 truncate text-sm text-zinc-200">{slide.id}</p>
      </div>
      <div class="flex flex-wrap items-center gap-2">
        <button
          type="button"
          class="rounded px-2 py-1 text-xs {view.mode === 'fit'
            ? 'bg-emerald-700 text-white dark:bg-emerald-800'
            : 'bg-zinc-700 text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700'}"
          onclick={setFit}>{I18N.zoomFit}</button
        >
        <button
          type="button"
          class="rounded px-2 py-1 text-xs {view.mode === '100'
            ? 'bg-emerald-700 text-white dark:bg-emerald-800'
            : 'bg-zinc-700 text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700'}"
          onclick={setActual100}>{I18N.zoom100}</button
        >
        <button
          type="button"
          class="rounded px-3 py-1 text-sm text-zinc-200 hover:bg-zinc-800"
          onclick={onClose}>{I18N.close}</button
        >
      </div>
    </div>

    <p class="bg-zinc-900/90 px-3 py-1 text-center text-[11px] text-zinc-400">{I18N.zoomWheelHint}</p>

    <div class="flex min-h-0 flex-1 flex-col md:flex-row">
      <div
        bind:this={viewportEl}
        class="flex min-h-0 min-w-0 flex-1 overflow-hidden overscroll-contain {canPan
          ? dragging
            ? 'cursor-grabbing'
            : 'cursor-grab'
          : ''}"
        use:nonPassiveWheel={handleWheel}
        ontouchstart={onTouchStart}
        ontouchmove={onTouchMove}
        ontouchend={onTouchEnd}
        role="presentation"
      >
        <div class="flex h-full min-h-full min-w-full flex-1 items-center justify-center p-4">
          <div
            class="origin-center will-change-transform select-none"
            style:transform="translate({pan.x}px, {pan.y}px) scale({view.zoom})"
            style:transform-origin="center center"
            onmousedown={onPanMouseDown}
            role="presentation"
          >
            {#if view.mode === "fit"}
              <img
                src={origUrl}
                alt={slide.id}
                class="max-h-[calc(100vh-220px)] max-w-[calc(100vw-32px)] object-contain [image-orientation:none] md:max-h-[calc(100vh-200px)]"
                style:transform={tf}
                style:transform-origin="center center"
                draggable="false"
              />
            {:else}
              <img
                src={origUrl}
                alt={slide.id}
                width={slide.width}
                height={slide.height}
                class="max-w-none [image-orientation:none]"
                style:transform={tf}
                style:transform-origin="center center"
                draggable="false"
              />
            {/if}
          </div>
        </div>
      </div>

      <aside
        class="max-h-[40vh] w-full shrink-0 overflow-y-auto border-t border-zinc-700/80 bg-zinc-900/90 p-3 text-xs text-zinc-200 md:max-h-none md:w-72 md:border-l md:border-t-0 dark:border-zinc-800"
      >
        <dl class="space-y-2">
          <div>
            <dt class="text-zinc-500">{I18N.lightboxDimensions}</dt>
            <dd class="font-mono text-zinc-100">
              {slide.width} × {slide.height}px
            </dd>
          </div>
          <div>
            <dt class="text-zinc-500">{I18N.lightboxCategory}</dt>
            <dd>{categoryLabel(slide.category)}</dd>
          </div>
          <div>
            <dt class="text-zinc-500">{I18N.lightboxSelection}</dt>
            <dd>{slideSelected ? I18N.lightboxSelected : I18N.lightboxNotSelected}</dd>
          </div>
          <div>
            <dt class="text-zinc-500">{I18N.lightboxSpecies}</dt>
            <dd class="space-y-1">
              {#if species}
                <p class="font-medium text-emerald-300 dark:text-emerald-200">{species.commonName}</p>
                {#if species.scientificName}
                  <p class="text-zinc-400">
                    <span class="text-zinc-500">{I18N.lightboxScientific}: </span>{species.scientificName}
                  </p>
                {/if}
                <p>
                  <span class="text-zinc-500">{I18N.lightboxCoefficient}: </span>{coeffFmt(
                    species.coefficient,
                  )}
                </p>
                <p>
                  <span class="text-zinc-500">{I18N.lightboxCatalogVersion}: </span>{species.version}
                </p>
              {:else}
                <p class="text-zinc-500">{I18N.lightboxNoSpecies}</p>
              {/if}
            </dd>
          </div>
        </dl>
      </aside>
    </div>

    <div
      class="flex flex-wrap items-center justify-center gap-2 border-t border-zinc-700/80 bg-zinc-900/90 p-3 dark:border-zinc-800"
    >
      <button
        type="button"
        class="rounded bg-zinc-700 px-3 py-2 text-sm text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700"
        onclick={() => onTransform("rotateCw")}>{I18N.rotateCw}</button
      >
      <button
        type="button"
        class="rounded bg-zinc-700 px-3 py-2 text-sm text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700"
        onclick={() => onTransform("rotateCcw")}>{I18N.rotateCcw}</button
      >
      <button
        type="button"
        class="rounded bg-zinc-700 px-3 py-2 text-sm text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700"
        onclick={() => onTransform("flipH")}>{I18N.flipH}</button
      >
      <button
        type="button"
        class="rounded bg-zinc-700 px-3 py-2 text-sm text-zinc-100 hover:bg-zinc-600 dark:bg-zinc-800 dark:hover:bg-zinc-700"
        onclick={() => onTransform("flipV")}>{I18N.flipV}</button
      >
    </div>
  </div>
{/if}
