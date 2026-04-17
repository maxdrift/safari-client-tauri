<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open as openFileDialog, message } from "@tauri-apps/plugin-dialog";
  import * as app from "$lib/app.svelte";
  import type { Species } from "$lib/types";
  import ConfirmDialog from "$lib/components/ConfirmDialog.svelte";
  import { I18N } from "$lib/utils/i18n";

  interface Props {
    settingsOpen: boolean;
    onClose: () => void;
  }
  let { settingsOpen, onClose }: Props = $props();

  let confirmRestoreOpen = $state(false);

  async function savePrefsAndClose() {
    try {
      await app.persistPreferences();
    } catch (e) {
      console.error(e);
    }
    onClose();
  }

  async function importSpeciesCatalog() {
    const path = await openFileDialog({
      filters: [{ name: "CSV", extensions: ["csv"] }],
    });
    if (path === null) return;
    const p = typeof path === "string" ? path : path[0];
    if (!p) return;
    try {
      const rows = await invoke<Species[]>("import_species_catalog_cmd", { path: p });
      app.replaceSpeciesCatalog(rows);
      await message(I18N.settingsSpeciesImportSuccess, { title: I18N.appTitle });
      const missing = app.missingSubjectIdsInCatalog();
      if (missing.length > 0) {
        await message(I18N.settingsSpeciesOrphanWarning(missing), { title: I18N.appTitle });
      }
    } catch (e) {
      await message(String(e), {
        title: I18N.settingsSpeciesImportErrorTitle,
        kind: "error",
      });
    }
  }

  function askRestoreDefault() {
    confirmRestoreOpen = true;
  }

  async function doRestoreDefault() {
    confirmRestoreOpen = false;
    try {
      const rows = await invoke<Species[]>("restore_default_species_catalog_cmd");
      app.replaceSpeciesCatalog(rows);
      await message(I18N.settingsSpeciesImportSuccess, { title: I18N.appTitle });
      const missing = app.missingSubjectIdsInCatalog();
      if (missing.length > 0) {
        await message(I18N.settingsSpeciesOrphanWarning(missing), { title: I18N.appTitle });
      }
    } catch (e) {
      await message(String(e), {
        title: I18N.settingsSpeciesImportErrorTitle,
        kind: "error",
      });
    }
  }
</script>

{#if settingsOpen}
  <div
    class="fixed inset-0 z-[52] flex items-center justify-center bg-black/60 p-4"
    role="presentation"
    onclick={savePrefsAndClose}
  >
    <div
      class="flex max-h-[90vh] w-full max-w-lg flex-col overflow-hidden rounded-lg border border-zinc-300 bg-white text-zinc-900 shadow-xl dark:border-zinc-700 dark:bg-zinc-900 dark:text-zinc-100"
      role="dialog"
      aria-labelledby="settings-title"
      tabindex="-1"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="border-b border-zinc-200 px-4 py-3 dark:border-zinc-700">
        <h2 id="settings-title" class="text-base font-semibold">{I18N.settingsTitle}</h2>
      </div>
      <div class="min-h-0 flex-1 overflow-y-auto px-4 py-3 text-sm">
        <section class="mb-6">
          <h3 class="mb-2 font-medium text-zinc-800 dark:text-zinc-200">
            {I18N.settingsSectionSpecies}
          </h3>
          <p class="mb-2 text-zinc-600 dark:text-zinc-400">{I18N.settingsSpeciesBlurb}</p>
          <p class="mb-3 text-xs text-zinc-500 dark:text-zinc-500">{I18N.settingsSpeciesFormatHint}</p>
          <div class="flex flex-wrap gap-2">
            <button
              type="button"
              class="rounded-lg border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
              onclick={importSpeciesCatalog}>{I18N.settingsSpeciesImport}</button
            >
            <button
              type="button"
              class="rounded-lg border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
              onclick={askRestoreDefault}>{I18N.settingsSpeciesRestore}</button
            >
          </div>
        </section>
        <section>
          <h3 class="mb-2 font-medium text-zinc-800 dark:text-zinc-200">
            {I18N.settingsSectionExport}
          </h3>
          <p class="mb-2 text-zinc-600 dark:text-zinc-400">{I18N.settingsExportNameHint}</p>
          <label class="sr-only" for="settings-export-name">{I18N.competitorExportTitle}</label>
          <input
            id="settings-export-name"
            type="text"
            class="w-full rounded border border-zinc-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none ring-emerald-600/30 focus:ring-2 dark:border-zinc-600 dark:bg-zinc-950 dark:text-zinc-100"
            placeholder={I18N.competitorNamePlaceholder}
            value={app.exportPrefs.defaultExportName}
            oninput={(e) => app.setDefaultExportName(e.currentTarget.value)}
          />
        </section>
      </div>
      <div class="flex justify-end gap-2 border-t border-zinc-200 px-4 py-3 dark:border-zinc-700">
        <button
          type="button"
          class="rounded border border-zinc-300 px-3 py-1.5 text-sm hover:bg-zinc-100 dark:border-zinc-600 dark:hover:bg-zinc-800"
          onclick={savePrefsAndClose}>{I18N.close}</button
        >
      </div>
    </div>
  </div>
{/if}

<ConfirmDialog
  open={confirmRestoreOpen}
  message={I18N.settingsSpeciesRestoreConfirm}
  overlayZClass="z-[60]"
  onCancel={() => (confirmRestoreOpen = false)}
  onConfirm={doRestoreDefault}
/>
