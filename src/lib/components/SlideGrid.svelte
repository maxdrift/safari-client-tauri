<script lang="ts">
  /* eslint-disable svelte/prefer-writable-derived -- svelte-dnd-action needs a mutable items array */
  import { dndzone } from "svelte-dnd-action";
  import type { Slide } from "$lib/types";
  import SlideTile from "./SlideTile.svelte";

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

  $effect(() => {
    items = visible.map((s) => ({ id: s.id, slide: s }));
  });

  function handleConsider(e: CustomEvent<{ items: DndItem[] }>) {
    items = e.detail.items;
  }

  function handleFinalize(e: CustomEvent<{ items: DndItem[] }>) {
    items = e.detail.items;
    const m = new Map(visible.map((s) => [s.id, s]));
    onReorder(e.detail.items.map((i) => m.get(i.id) ?? i.slide));
  }
</script>

<section
  class="grid auto-rows-auto content-start gap-3 p-4"
  style="grid-template-columns: repeat(auto-fill, minmax({minCellPx}px, 1fr));"
  use:dndzone={{
    items,
    flipDurationMs: 200,
    dragDisabled: !reorderEnabled,
    dropTargetStyle: { outline: "rgba(16, 185, 129, 0.35) solid 2px" },
  }}
  onconsider={handleConsider}
  onfinalize={handleFinalize}
>
  {#each items as item (item.id)}
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
  {/each}
</section>
