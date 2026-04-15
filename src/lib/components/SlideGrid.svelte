<script lang="ts">
  import { flip } from "svelte/animate";
  import {
    dndzone,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
    SHADOW_PLACEHOLDER_ITEM_ID,
  } from "svelte-dnd-action";
  import type { Slide } from "$lib/types";
  import SlideTile from "./SlideTile.svelte";

  const flipDurationMs = 200;

  interface Props {
    /** Minimum column width in px (grid zoom). */
    minCellPx: number;
    visible: Slide[];
    reorderEnabled: boolean;
    selectedIds: string[];
    speciesLabelFor: (subjectId: number) => string;
    onToggleSelect: (id: string) => void;
    onCycleCategory: (id: string) => void;
    onOpenSpecies: (id: string) => void;
    onOpenLightbox: (id: string) => void;
    onReorder: (slides: Slide[]) => void;
  }
  let {
    minCellPx,
    visible,
    reorderEnabled,
    selectedIds,
    speciesLabelFor,
    onToggleSelect,
    onCycleCategory,
    onOpenSpecies,
    onOpenLightbox,
    onReorder,
  }: Props = $props();

  type DndItem = { id: string; slide: Slide };
  let items = $state<DndItem[]>([]);

  /**
   * While true, `items` is owned by svelte-dnd-action (incl. shadow placeholder rows).
   * Parent sync must not run: any `slidesRenderEpoch` bump (thumbnails, saves, etc.) would
   * overwrite `items` and break drag — disappearing tiles, empty drop gaps, stuck state.
   */
  let dndSessionActive = $state(false);

  $effect(() => {
    if (dndSessionActive) return;
    items = visible.map((s) => ({ id: s.id, slide: s }));
  });

  function isRealItem(i: DndItem): boolean {
    if (i.id === SHADOW_PLACEHOLDER_ITEM_ID) return false;
    return !(SHADOW_ITEM_MARKER_PROPERTY_NAME in i && (i as Record<string, unknown>)[SHADOW_ITEM_MARKER_PROPERTY_NAME]);
  }

  function handleConsider(e: CustomEvent<{ items: DndItem[] }>) {
    dndSessionActive = true;
    items = e.detail.items;
  }

  function handleFinalize(e: CustomEvent<{ items: DndItem[] }>) {
    items = e.detail.items;
    try {
      const m = new Map(visible.map((s) => [s.id, s]));
      const reordered = e.detail.items.filter(isRealItem).map((i) => m.get(i.id) ?? i.slide);
      onReorder(reordered);
    } finally {
      dndSessionActive = false;
    }
  }
</script>

<section
  class="grid auto-rows-auto content-start gap-3 p-4"
  style="grid-template-columns: repeat(auto-fill, minmax({minCellPx}px, 1fr));"
  use:dndzone={{
    items,
    flipDurationMs,
    dragDisabled: !reorderEnabled,
    dropTargetStyle: { outline: "rgba(16, 185, 129, 0.35) solid 2px" },
  }}
  onconsider={handleConsider}
  onfinalize={handleFinalize}
>
  {#each items as item (item.id)}
    <div class="min-w-0" animate:flip={{ duration: flipDurationMs }}>
      {#if !isRealItem(item)}
        <div
          class="aspect-square rounded-lg bg-zinc-200/70 ring-2 ring-emerald-500/25 dark:bg-zinc-800/70"
          aria-hidden="true"
        ></div>
      {:else}
        <SlideTile
          slideId={item.id}
          {speciesLabelFor}
          selected={selectedIds.includes(item.id)}
          {reorderEnabled}
          onToggleSelect={() => onToggleSelect(item.id)}
          onCycleCategory={() => onCycleCategory(item.id)}
          onOpenSpecies={() => onOpenSpecies(item.id)}
          onOpenLightbox={() => onOpenLightbox(item.id)}
        />
      {/if}
    </div>
  {/each}
</section>
