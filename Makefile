# Safari Client — common tasks (requires GNU Make, Node.js, Rust)
# Run `make` or `make help` to list targets.

.PHONY: help install dev dev-web build preview \
	check check-watch lint \
	test test-watch test-rust test-js \
	clippy clippy-fix fmt fmt-check \
	tauri-build tauri-build-debug \
	clean clean-deep ci verify quality \
	rustdoc

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

tauri-build:
	$(NPM) run tauri build

tauri-build-debug:
	$(NPM) run tauri build -- --debug

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
