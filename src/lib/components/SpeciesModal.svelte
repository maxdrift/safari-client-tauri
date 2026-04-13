<script lang="ts">
  import type { Species } from "$lib/types";
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    open: boolean;
    species: Species[];
    onPick: (subjectId: number) => void;
    onClose: () => void;
  }
  let { open, species, onPick, onClose }: Props = $props();

  let q = $state("");

  const rows = $derived.by(() => {
    const needle = q.trim().toLowerCase();
    const synthetic: Species = {
      id: 0,
      commonName: I18N.removeSpecies,
      scientificName: "",
      coefficient: 0,
      version: "",
    };
    const base = [synthetic, ...species];
    if (!needle) return base;
    return base.filter(
      (s) =>
        s.commonName.toLowerCase().includes(needle) ||
        s.scientificName.toLowerCase().includes(needle),
    );
  });
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
    role="presentation"
    onclick={onClose}
  >
    <div
      class="flex max-h-[80vh] w-full max-w-lg flex-col rounded-lg border border-zinc-300 bg-white shadow-xl dark:border-zinc-700 dark:bg-zinc-900"
      role="dialog"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="border-b border-zinc-200 p-3 dark:border-zinc-800">
        <h2 class="text-sm font-semibold text-zinc-900 dark:text-zinc-100">{I18N.speciesModalTitle}</h2>
        <input
          class="mt-2 w-full rounded border border-zinc-300 bg-zinc-50 px-2 py-1.5 text-sm text-zinc-900 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100"
          placeholder={I18N.speciesSearch}
          bind:value={q}
        />
      </div>
      <div class="min-h-0 flex-1 overflow-y-auto p-2">
        {#each rows as s (s.id === 0 ? "rimuovi-specie" : s.id)}
          <button
            type="button"
            class="flex w-full flex-col items-start rounded px-2 py-2 text-left text-sm text-zinc-900 hover:bg-zinc-100 dark:text-zinc-100 dark:hover:bg-zinc-800"
            onclick={() => onPick(s.id)}
          >
            <span class="font-medium">{s.commonName}</span>
            {#if s.scientificName}
              <span class="text-xs text-zinc-600 dark:text-zinc-500">{s.scientificName}</span>
            {/if}
          </button>
        {/each}
      </div>
      <div class="border-t border-zinc-200 p-2 text-right dark:border-zinc-800">
        <button
          type="button"
          class="rounded border border-zinc-300 px-3 py-1 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
          onclick={onClose}>{I18N.cancel}</button
        >
      </div>
    </div>
  </div>
{/if}
