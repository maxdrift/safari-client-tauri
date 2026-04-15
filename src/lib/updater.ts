import { ask } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

/**
 * Checks GitHub Releases `latest.json` (from tauri-action) and prompts to install.
 * Skips in dev and in non-Tauri (browser) builds.
 */
export async function maybeOfferUpdate(): Promise<void> {
  if (!import.meta.env.TAURI_ENV_PLATFORM || import.meta.env.DEV) return;

  try {
    const update = await check();
    if (!update) return;

    const notes = update.body?.trim();
    const prompt = notes
      ? `È disponibile la versione ${update.version}.\n\n${notes}\n\nScaricare e installare ora?`
      : `È disponibile la versione ${update.version}. Scaricare e installare ora?`;

    const ok = await ask(prompt, {
      title: "Aggiornamento Safari Client",
      kind: "info",
    });
    if (!ok) return;

    await update.downloadAndInstall();
    await relaunch();
  } catch (e) {
    console.error("Aggiornamento: verifica non riuscita", e);
  }
}
