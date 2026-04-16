import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;
// @ts-expect-error process is a nodejs global
const isVitest = Boolean(process.env.VITEST);

// https://vite.dev/config/
export default defineConfig(async () => ({
  // Required so `import.meta.env.TAURI_ENV_PLATFORM` exists in the built bundle (see Tauri Vite guide).
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  plugins: [sveltekit()],
  // Vitest must resolve the client Svelte build so `render()` can mount components (not SSR).
  ...(isVitest
    ? {
        resolve: {
          conditions: ["browser", "development"],
        },
      }
    : {}),
  test: {
    environment: "jsdom",
    include: ["src/**/*.{test,spec}.{js,ts}"],
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
