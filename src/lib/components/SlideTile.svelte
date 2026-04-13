<script lang="ts">
  import { convertFileSrc } from "@tauri-apps/api/core";
  import type { Slide } from "$lib/types";
  import { transformIdToCss } from "$lib/utils/transform";
  import { categoryLabel } from "$lib/utils/i18n";

  interface Props {
    slide: Slide;
    /** Nome comune specie se assegnata; stringa vuota se assente. */
    speciesLabel: string;
    selected: boolean;
    reorderEnabled: boolean;
    onToggleSelect: () => void;
    onCycleCategory: () => void;
    onOpenSpecies: () => void;
    onOpenLightbox: () => void;
  }
  let {
    slide,
    speciesLabel,
    selected,
    reorderEnabled,
    onToggleSelect,
    onCycleCategory,
    onOpenSpecies,
    onOpenLightbox,
  }: Props = $props();

  const thumbUrl = $derived(convertFileSrc(slide.thumbnails.s350));
  const tf = $derived(transformIdToCss(slide.transformId));
  const thumbReady = $derived(slide.thumbnailsPending === false);

  const ring = $derived(
    slide.category === "excluded"
      ? "ring-zinc-400 dark:ring-zinc-600"
      : slide.category === "fixed"
        ? "ring-amber-500"
        : "ring-emerald-500",
  );
</script>

<div
  class="group relative aspect-square overflow-hidden rounded-lg bg-zinc-200 ring-2 dark:bg-zinc-900 {ring} {selected
    ? 'ring-offset-2 ring-offset-zinc-50 dark:ring-offset-zinc-950'
    : ''}"
>
  <button
    type="button"
    class="absolute inset-0 z-0"
    onclick={onOpenLightbox}
    aria-label="Apri"
  >
    {#key `${slide.id}-${slide.thumbnailsPending === false ? "ready" : "wait"}`}
      {#if thumbReady}
        <img
          src={thumbUrl}
          alt={slide.id}
          class="h-full w-full object-contain"
          style:transform={tf}
          style:transform-origin="center center"
          draggable="false"
        />
      {:else}
        <div
          class="flex h-full w-full items-center justify-center bg-zinc-300/90 dark:bg-zinc-800/90"
          aria-busy="true"
        >
          <div
            class="h-8 w-8 animate-spin rounded-full border-2 border-zinc-500 border-t-emerald-600 dark:border-zinc-600 dark:border-t-emerald-500"
          ></div>
        </div>
      {/if}
    {/key}
  </button>

  <div class="absolute left-2 top-2 z-10 flex flex-col gap-1 opacity-0 transition group-hover:opacity-100">
    <button
      type="button"
      class="pointer-events-auto rounded bg-black/60 px-2 py-0.5 text-xs text-white hover:bg-black/80"
      onclick={(e) => {
        e.stopPropagation();
        onCycleCategory();
      }}>{categoryLabel(slide.category)}</button
    >
    <button
      type="button"
      class="pointer-events-auto rounded bg-black/60 px-2 py-0.5 text-xs text-white hover:bg-black/80"
      onclick={(e) => {
        e.stopPropagation();
        onOpenSpecies();
      }}>Specie</button
    >
  </div>

  <button
    type="button"
    class="absolute right-2 top-2 z-10 rounded bg-black/60 px-2 py-0.5 text-xs text-white hover:bg-black/80"
    aria-label={selected ? "Deseleziona" : "Seleziona"}
    title={selected ? "Selezionata" : "Seleziona"}
    onclick={(e) => {
      e.stopPropagation();
      onToggleSelect();
    }}>{selected ? "✓" : "□"}</button
  >

  {#if speciesLabel}
    <div
      class="pointer-events-none absolute bottom-1 left-1 right-10 z-10 truncate rounded bg-black/55 px-1.5 py-0.5 text-left text-[10px] font-medium leading-tight text-emerald-200"
      title={speciesLabel}
    >
      {speciesLabel}
    </div>
  {/if}

  {#if reorderEnabled}
    <div
      class="absolute bottom-2 right-2 rounded bg-zinc-200/90 px-1 text-[10px] text-zinc-500 dark:bg-zinc-800/90 dark:text-zinc-400"
      title="Trascina per riordinare"
    >
      ⋮⋮
    </div>
  {/if}
</div>
