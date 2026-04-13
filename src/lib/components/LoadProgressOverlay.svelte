<script lang="ts">
  interface Props {
    active: boolean;
    done: number;
    total: number;
    label: string;
  }
  let { active, done, total, label }: Props = $props();

  const pct = $derived(total > 0 ? Math.round((done / total) * 100) : 0);
</script>

{#if active}
  <div
    class="fixed inset-0 z-[70] flex items-center justify-center bg-black/50 backdrop-blur-sm"
    role="status"
    aria-live="polite"
    aria-busy="true"
  >
    <div
      class="flex min-w-[280px] flex-col gap-3 rounded-lg border border-zinc-300 bg-white px-6 py-4 text-zinc-900 shadow-xl dark:border-zinc-600 dark:bg-zinc-900 dark:text-zinc-100"
    >
      <p class="text-center text-sm font-medium">{label}</p>
      <div class="h-2 w-full overflow-hidden rounded-full bg-zinc-200 dark:bg-zinc-800">
        <div
          class="h-full rounded-full bg-emerald-600 transition-[width] duration-150"
          style:width="{pct}%"
        ></div>
      </div>
      <p class="text-center text-xs text-zinc-600 dark:text-zinc-400">{done} / {total} ({pct}%)</p>
    </div>
  </div>
{/if}
