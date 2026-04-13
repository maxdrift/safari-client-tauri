# Safari Client

Desktop app for **Safari Fotosub**–style competitions: load local photos, set category (excluded / fixed points / jury), assign species from the official catalog, reorder slides, and export the **competitor sheet** as CSV (`;`, UTF-8) including `transform_id` (0–7).

Functional requirements: [docs/PRD.md](docs/PRD.md).

## Where data is stored

- **Application state** (slide order, categories, `subject_id`, `transform_id`, file paths): JSON in the OS user data directory under `SafariClient/state.json` (e.g. on macOS: `~/Library/Application Support/SafariClient/state.json`; see `dirs::data_local_dir()` in `src-tauri/src/commands/persistence.rs`).
- **Thumbnails / previews** (cached resized copies at 350 / 512 / 1024 px width): system temp directory + `safari-client-previews` (see `src-tauri/src/imaging/thumbnails.rs`). This is derived cache only, not the original photos.

**Loading behavior:** New files are registered quickly using image dimensions only; preview files are written in the **background** so the UI stays responsive (see `drainThumbnailQueue` in `src/lib/app.svelte.ts` and `regenerate_thumbnails_cmd`). After slide/thumbnail updates, **`slidesRenderEpoch.n`** is incremented so Svelte `$derived.by` in the page re-runs when slide data lives in a separate `.svelte.ts` module (cross-module reactivity).

**Theme:** Light, dark, or system (follows OS) from the navbar; preference is stored under `safari-client-theme` in `localStorage`. Uses Tailwind `dark:` with `class="dark"` on `<html>` (see `src/lib/theme.ts`, `initTheme` in `src/lib/app.svelte.ts`).

## Development

Prerequisites: [Node.js](https://nodejs.org/) LTS, [Rust stable](https://rustup.rs/), and Tauri system dependencies (on Linux: WebKitGTK; see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)).

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

Artifacts are under `src-tauri/target/release/bundle/`.

## Tests and quality

```bash
npm run check    # Svelte + TypeScript
npm run lint     # ESLint
npm test         # Vitest
cd src-tauri && cargo test && cargo clippy -- -D warnings
```

## Release (CI)

- **CI:** runs on every push/PR to `main` (`.github/workflows/ci.yml`).
- **Release:** pushing a `v*` tag creates a **draft** on GitHub Releases with installers for macOS (arm64 + x64), Linux, and Windows (`.github/workflows/release.yml`). Review and publish the draft manually.

## License

[MIT](LICENSE)
