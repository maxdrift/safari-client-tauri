<script lang="ts">
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    open: boolean;
    initialName: string;
    onConfirm: (name: string) => void;
    onCancel: () => void;
  }
  let { open, initialName, onConfirm, onCancel }: Props = $props();

  let input = $state("");

  $effect(() => {
    if (open) input = initialName;
  });

  function submit() {
    onConfirm(input);
  }
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
    role="presentation"
    onclick={onCancel}
  >
    <div
      class="w-full max-w-md rounded-lg border border-zinc-300 bg-white p-4 text-zinc-900 shadow-xl dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100"
      role="dialog"
      tabindex="-1"
      aria-labelledby="competitor-export-title"
      onclick={(e) => e.stopPropagation()}
    >
      <h2 id="competitor-export-title" class="mb-2 text-base font-semibold">
        {I18N.competitorExportTitle}
      </h2>
      <p class="mb-3 text-sm text-zinc-600 dark:text-zinc-400">{I18N.competitorExportHint}</p>
      <input
        type="text"
        class="mb-4 w-full rounded border border-zinc-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none ring-emerald-600/30 focus:ring-2 dark:border-zinc-600 dark:bg-zinc-950 dark:text-zinc-100"
        placeholder={I18N.competitorNamePlaceholder}
        bind:value={input}
        onkeydown={(e) => {
          if (e.key === "Enter") {
            e.preventDefault();
            submit();
          }
        }}
      />
      <div class="flex justify-end gap-2">
        <button
          type="button"
          class="rounded border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
          onclick={onCancel}>{I18N.cancel}</button
        >
        <button
          type="button"
          class="rounded bg-emerald-600 px-3 py-1.5 text-sm text-white hover:bg-emerald-500"
          onclick={submit}>{I18N.exportCsv}</button
        >
      </div>
    </div>
  </div>
{/if}
