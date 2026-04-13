<script lang="ts">
  import type { FilterTab } from "$lib/types";
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    tab: FilterTab;
    onChange: (t: FilterTab) => void;
    counts: { all: number; excluded: number; fixed: number; jury: number };
  }
  let { tab, onChange, counts }: Props = $props();

  const tabs: { key: FilterTab; label: string; n: () => number }[] = [
    { key: "all", label: I18N.all, n: () => counts.all },
    { key: "excluded", label: I18N.excluded, n: () => counts.excluded },
    { key: "fixed", label: I18N.fixed, n: () => counts.fixed },
    { key: "jury", label: I18N.jury, n: () => counts.jury },
  ];
</script>

<div
  class="flex flex-wrap gap-2 border-b border-zinc-200 px-4 py-2 dark:border-zinc-800"
>
  {#each tabs as t (t.key)}
    <button
      type="button"
      class="rounded px-3 py-1.5 text-sm font-medium transition-colors {tab === t.key
        ? 'bg-zinc-200 text-zinc-900 dark:bg-zinc-800 dark:text-white'
        : 'text-zinc-600 hover:bg-zinc-100 dark:text-zinc-400 dark:hover:bg-zinc-800/60'}"
      onclick={() => onChange(t.key)}>{t.label} ({t.n()})</button
    >
  {/each}
</div>
