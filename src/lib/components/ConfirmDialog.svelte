<script lang="ts">
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    open: boolean;
    message: string;
    onConfirm: () => void;
    onCancel: () => void;
  }
  let { open, message, onConfirm, onCancel }: Props = $props();
</script>

{#if open}
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4"
    role="presentation"
    onclick={onCancel}
  >
    <div
      class="max-w-md rounded-lg border border-zinc-300 bg-white p-4 text-zinc-900 shadow-xl dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100"
      role="dialog"
      onclick={(e) => e.stopPropagation()}
    >
      <p class="mb-4 text-sm">{message}</p>
      <div class="flex justify-end gap-2">
        <button
          type="button"
          class="rounded border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
          onclick={onCancel}>{I18N.cancel}</button
        >
        <button
          type="button"
          class="rounded bg-rose-600 px-3 py-1.5 text-sm hover:bg-rose-500"
          onclick={onConfirm}>{I18N.confirm}</button
        >
      </div>
    </div>
  </div>
{/if}
