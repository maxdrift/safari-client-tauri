# Safari Client — common tasks (requires GNU Make, Node.js, Rust)
# Run `make` or `make help` to list targets.

.PHONY: help install dev dev-web build preview \
	check check-watch lint \
	test test-watch test-rust test-js \
	clippy clippy-fix fmt fmt-check \
	tauri-build tauri-build-debug \
	clean clean-deep ci verify quality \
	rustdoc \
	version-show version-bump \
	version-bump-patch version-bump-minor version-bump-major \
	version-release-patch version-release-minor version-release-major \
	version-release-legacy-patch version-release-legacy-minor version-release-legacy-major \
	version-release-legacy \
	version-tag-legacy

NPM := npm
CARGO := cargo
TAURI_DIR := src-tauri

.DEFAULT_GOAL := help

help:
	@echo "Safari Client — Makefile"
	@echo ""
	@echo "Setup:"
	@echo "  make install          Install Node deps (npm ci)"
	@echo ""
	@echo "Run:"
	@echo "  make dev              Tauri desktop dev (Vite + Rust)"
	@echo "  make dev-web          Frontend only (Vite dev, no Tauri)"
	@echo "  make preview          Preview production frontend (after make build)"
	@echo ""
	@echo "Build:"
	@echo "  make build            Production frontend only (Vite → build/)"
	@echo "  make tauri-build      Full app bundle (release)"
	@echo "  make tauri-build-debug Full app bundle (debug, faster)"
	@echo ""
	@echo "Quality (JS/Svelte):"
	@echo "  make check            svelte-check + TypeScript"
	@echo "  make check-watch      svelte-check --watch"
	@echo "  make lint             ESLint"
	@echo "  make test / test-js   Vitest once"
	@echo "  make test-watch       Vitest watch"
	@echo ""
	@echo "Quality (Rust):"
	@echo "  make test-rust        cargo test in $(TAURI_DIR)"
	@echo "  make clippy           cargo clippy -D warnings"
	@echo "  make clippy-fix       cargo clippy --fix (use with care)"
	@echo "  make fmt              cargo fmt"
	@echo "  make fmt-check        cargo fmt --check"
	@echo "  make rustdoc          cargo doc --open (optional)"
	@echo ""
	@echo "Aggregates:"
	@echo "  make quality          check + lint + test-js + test-rust + clippy"
	@echo "  make ci / verify      quality + tauri-build-debug (matches CI spirit)"
	@echo ""
	@echo "Clean:"
	@echo "  make clean            Remove build/, .svelte-kit, Rust target"
	@echo "  make clean-deep       clean + node_modules (re-run make install)"
	@echo ""
	@echo "Version (before tagging a release):"
	@echo "  make version-show              Print current version (package.json)"
	@echo "  make version-bump-patch        Bump patch, sync npm + Cargo + Tauri + lockfiles"
	@echo "  make version-bump-minor / version-bump-major"
	@echo "  make version-bump BUMP=patch   Same as patch (BUMP=minor|major)"
	@echo "  make version-release-patch     Bump + commit + tag vX.Y.Z + push (omit push: PUSH=0)"
	@echo "  make version-release-minor / version-release-major"
	@echo ""
	@echo "Version — legacy line (backport/tailwind-v3): bump with targets above; tag is vX.Y.Z-legacy only via:"
	@echo "  make version-release-legacy-patch / version-release-legacy-minor / version-release-legacy-major"
	@echo "  make version-release-legacy BUMP=patch   (set PUSH=0 to skip push)"
	@echo "  make version-tag-legacy   Tag vX.Y.Z-legacy from current package.json only (no bump; PUSH=0 to skip push)"

install:
	$(NPM) ci

dev:
	$(NPM) run tauri dev

dev-web:
	$(NPM) run dev

build:
	$(NPM) run build

preview:
	$(NPM) run preview

check:
	$(NPM) run check

check-watch:
	$(NPM) run check:watch

lint:
	$(NPM) run lint

test: test-js

test-js:
	$(NPM) test

test-watch:
	$(NPM) run test:watch

test-rust:
	cd $(TAURI_DIR) && $(CARGO) test

clippy:
	cd $(TAURI_DIR) && $(CARGO) clippy -- -D warnings

clippy-fix:
	cd $(TAURI_DIR) && $(CARGO) clippy --fix --allow-dirty --allow-staged

fmt:
	cd $(TAURI_DIR) && $(CARGO) fmt

fmt-check:
	cd $(TAURI_DIR) && $(CARGO) fmt -- --check

rustdoc:
	cd $(TAURI_DIR) && $(CARGO) doc --open --no-deps

# Same --config merge as .github/workflows/ci.yml: skip signed updater bundles unless TAURI_SIGNING_PRIVATE_KEY is set (see Tauri updater docs).
tauri-build:
	$(NPM) run tauri build -- --config '{"bundle":{"createUpdaterArtifacts":false}}'

tauri-build-debug:
	$(NPM) run tauri build -- --debug --config '{"bundle":{"createUpdaterArtifacts":false}}'

clean:
	rm -rf build .svelte-kit
	cd $(TAURI_DIR) && $(CARGO) clean

clean-deep: clean
	rm -rf node_modules

quality: check lint test-js test-rust clippy
	@echo "quality: OK"

verify: quality tauri-build-debug
	@echo "verify: OK"

ci: verify

version-show:
	@node -p "require('./package.json').version"

version-bump-patch:
	@bash scripts/bump-version.sh patch

version-bump-minor:
	@bash scripts/bump-version.sh minor

version-bump-major:
	@bash scripts/bump-version.sh major

version-bump:
	@test -n "$(BUMP)" || (echo "Usage: make version-bump BUMP=patch|minor|major" >&2; exit 1)
	@bash scripts/bump-version.sh "$(BUMP)"

version-release-patch:
	@bash scripts/release-version.sh patch

version-release-minor:
	@bash scripts/release-version.sh minor

version-release-major:
	@bash scripts/release-version.sh major

# Legacy WebKit backport: same semver bump as mainline; tag is vX.Y.Z-legacy (see scripts/release-version-legacy.sh).
version-release-legacy-patch:
	@bash scripts/release-version-legacy.sh patch

version-release-legacy-minor:
	@bash scripts/release-version-legacy.sh minor

version-release-legacy-major:
	@bash scripts/release-version-legacy.sh major

version-release-legacy:
	@test -n "$(BUMP)" || (echo "Usage: make version-release-legacy BUMP=patch|minor|major" >&2; exit 1)
	@bash scripts/release-version-legacy.sh "$(BUMP)"

# No bump: annotated tag v$(version)-legacy from package.json (same semver as last release work on the branch).
version-tag-legacy:
	@bash scripts/tag-version-legacy.sh
