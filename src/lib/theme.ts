import type { ThemePreference } from "$lib/types";

export const THEME_STORAGE_KEY = "safari-client-theme";

export function effectiveThemeIsDark(pref: ThemePreference): boolean {
  if (pref === "dark") return true;
  if (pref === "light") return false;
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

/** Sets `class="dark"` on `<html>` for Tailwind `dark:` utilities. */
export function applyDocumentTheme(pref: ThemePreference): void {
  document.documentElement.classList.toggle("dark", effectiveThemeIsDark(pref));
}
