<script lang="ts">
  import type { Species } from "$lib/types";
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    species: Species[];
    usage: Record<number, { jury: boolean; fixed: boolean }>;
  }
  let { species, usage }: Props = $props();

  let q = $state("");

  const rows = $derived.by(() => {
    const needle = q.trim().toLowerCase();
    if (!needle) return species;
    return species.filter(
      (s) =>
        s.commonName.toLowerCase().includes(needle) ||
        s.scientificName.toLowerCase().includes(needle),
    );
  });

  function rowClass(id: number): string {
    const u = usage[id];
    if (!u) return "bg-transparent";
    if (u.jury) return "bg-emerald-200/70 dark:bg-emerald-900/50";
    if (u.fixed) return "bg-amber-200/70 dark:bg-amber-900/50";
    return "bg-transparent";
  }
</script>

<div
  class="flex w-72 shrink-0 flex-col border-l border-zinc-200 bg-zinc-100/80 dark:border-zinc-800 dark:bg-zinc-900/50"
>
  <div class="border-b border-zinc-200 p-2 dark:border-zinc-800">
    <h2 class="text-xs font-semibold uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
      {I18N.speciesOverview}
    </h2>
    <input
      class="mt-2 w-full rounded border border-zinc-300 bg-white px-2 py-1 text-xs text-zinc-900 dark:border-zinc-700 dark:bg-zinc-950 dark:text-zinc-100"
      placeholder={I18N.speciesSearch}
      bind:value={q}
    />
  </div>
  <div class="min-h-0 flex-1 overflow-y-auto p-1 text-xs">
    {#each rows as s (s.id)}
      <div class="rounded px-2 py-1.5 {rowClass(s.id)}">
        <div class="font-medium text-zinc-900 dark:text-inherit">{s.commonName}</div>
        <div class="text-zinc-600 dark:text-zinc-500">{s.scientificName}</div>
      </div>
    {/each}
  </div>
</div>
