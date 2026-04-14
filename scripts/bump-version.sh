#!/usr/bin/env bash
# Bump Safari Client version everywhere npm/cargo/tauri expect it.
# Usage (from repo root): bash scripts/bump-version.sh patch|minor|major
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

BUMP="${1:?Usage: bump-version.sh patch|minor|major}"
case "$BUMP" in patch | minor | major) ;;
*)
  echo "Invalid bump kind: $BUMP (use patch, minor, or major)" >&2
  exit 1
  ;;
esac

command -v npm >/dev/null || {
  echo "npm not found" >&2
  exit 1
}
command -v node >/dev/null || {
  echo "node not found" >&2
  exit 1
}
command -v cargo >/dev/null || {
  echo "cargo not found" >&2
  exit 1
}

# npm prints the new version to stdout (e.g. v0.1.2); we echo a single summary below.
npm version "$BUMP" --no-git-tag-version >/dev/null

VER=$(node -p "require('./package.json').version")
export VER

node <<'NODE'
const fs = require("fs");
const ver = process.env.VER;

let cargo = fs.readFileSync("src-tauri/Cargo.toml", "utf8");
if (!/^version = "/m.test(cargo)) {
  throw new Error("src-tauri/Cargo.toml: missing version = line");
}
cargo = cargo.replace(/^version = "[^"]*"/m, `version = "${ver}"`);
fs.writeFileSync("src-tauri/Cargo.toml", cargo);

const tauriPath = "src-tauri/tauri.conf.json";
const tauri = JSON.parse(fs.readFileSync(tauriPath, "utf8"));
tauri.version = ver;
fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + "\n");
NODE

(cd src-tauri && cargo check -q)

echo "Bumped Safari Client to ${VER} (package.json, package-lock.json, Cargo.toml, Cargo.lock, tauri.conf.json)."
