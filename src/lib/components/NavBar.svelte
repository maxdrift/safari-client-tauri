<script lang="ts">
  import * as app from "$lib/app.svelte";
  import type { ThemePreference } from "$lib/types";
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    selectionCount: number;
    hasSlides: boolean;
    onImportCsv: () => void;
    onExport: () => void;
    onDeselectAll: () => void;
    onSelectAll: () => void;
    onSetCategory: (c: "excluded" | "fixed" | "jury") => void;
    onAssignSpecies: () => void;
    onDelete: () => void;
    onClearSelection: () => void;
  }
  let {
    selectionCount,
    hasSlides,
    onImportCsv,
    onExport,
    onDeselectAll,
    onSelectAll,
    onSetCategory,
    onAssignSpecies,
    onDelete,
    onClearSelection,
  }: Props = $props();

  const selectionMode = $derived(selectionCount > 0);
</script>

<header
  class="flex min-h-14 flex-wrap items-center gap-2 border-b border-zinc-200 bg-white/95 px-4 py-2 dark:border-zinc-800 dark:bg-zinc-900/95"
>
  {#if selectionMode}
    <span class="text-sm text-zinc-700 dark:text-zinc-300">{I18N.selectedOne(selectionCount)}</span>
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={onDeselectAll}>{I18N.deselectAll}</button
    >
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={onSelectAll}>{I18N.selectAll}</button
    >
    <span class="text-xs text-zinc-500">{I18N.judgeMenu}:</span>
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={() => onSetCategory("excluded")}>{I18N.excluded}</button
    >
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={() => onSetCategory("fixed")}>{I18N.fixed}</button
    >
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={() => onSetCategory("jury")}>{I18N.jury}</button
    >
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={onAssignSpecies}>{I18N.assignSpecies}</button
    >
    <button
      type="button"
      class="rounded border border-rose-700 px-2 py-1 text-xs text-rose-700 hover:bg-rose-50 dark:text-rose-300 dark:hover:bg-rose-950"
      onclick={onDelete}>{I18N.delete}</button
    >
    <button
      type="button"
      class="rounded border border-zinc-300 px-2 py-1 text-xs hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
      onclick={onClearSelection}>{I18N.clearSelection}</button
    >
  {:else}
    <h1 class="mr-auto text-lg font-semibold tracking-tight">{I18N.appTitle}</h1>
    <label class="sr-only" for="theme-select">{I18N.theme}</label>
    <select
      id="theme-select"
      class="rounded border border-zinc-300 bg-white px-2 py-1 text-xs text-zinc-800 dark:border-zinc-600 dark:bg-zinc-900 dark:text-zinc-200"
      value={app.ui.theme}
      onchange={(e) =>
        app.setThemePreference(e.currentTarget.value as ThemePreference)}
    >
      <option value="light">{I18N.themeLight}</option>
      <option value="dark">{I18N.themeDark}</option>
      <option value="system">{I18N.themeSystem}</option>
    </select>
    <button
      type="button"
      class="rounded-lg border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 disabled:opacity-40 dark:border-zinc-600 dark:hover:bg-zinc-800"
      disabled={!hasSlides}
      onclick={onImportCsv}>{I18N.importCsv}</button
    >
    <button
      type="button"
      class="rounded-lg border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 disabled:opacity-40 dark:border-zinc-600 dark:hover:bg-zinc-800"
      disabled={!hasSlides}
      onclick={onExport}>{I18N.exportCsv}</button
    >
  {/if}
</header>
