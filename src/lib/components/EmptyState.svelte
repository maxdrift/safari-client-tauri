<script lang="ts">
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    onLoad: () => void;
  }
  let { onLoad }: Props = $props();
</script>

<div class="flex flex-1 flex-col items-center justify-center gap-4 p-8 text-center">
  <p class="max-w-md text-zinc-600 dark:text-zinc-400">{I18N.emptyHint}</p>
  <button
    type="button"
    class="rounded-lg bg-emerald-600 px-6 py-3 font-medium text-white shadow hover:bg-emerald-500"
    onclick={onLoad}>{I18N.loadImages}</button
  >
</div>
