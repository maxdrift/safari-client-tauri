#!/usr/bin/env bash
# Bump version, commit, tag vX.Y.Z, and optionally push (for GitHub Actions release on tag v*).
# Usage: bash scripts/release-version.sh patch|minor|major
# Skip push: PUSH=0 make version-release-patch   OR   PUSH=0 bash scripts/release-version.sh patch
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

BUMP="${1:?Usage: release-version.sh patch|minor|major}"
case "$BUMP" in patch | minor | major) ;;
*)
  echo "Invalid bump kind: $BUMP" >&2
  exit 1
  ;;
esac

git rev-parse --git-dir >/dev/null 2>&1 || {
  echo "Not a git repository." >&2
  exit 1
}

case "${PUSH:-1}" in
0 | false | no | NO) SKIP_PUSH=1 ;;
*) SKIP_PUSH=0 ;;
esac

bash "$SCRIPT_DIR/bump-version.sh" "$BUMP"

VER=$(node -p "require('./package.json').version")

git add package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json

if git diff --cached --quiet; then
  echo "Nothing to commit after bump (unexpected)." >&2
  exit 1
fi

git commit -m "chore: release v${VER}"

if git show-ref --tags --verify --quiet "refs/tags/v${VER}"; then
  echo "Tag v${VER} already exists. Remove it with: git tag -d v${VER}" >&2
  exit 1
fi

git tag -a "v${VER}" -m "v${VER}"

if [ "$SKIP_PUSH" -eq 1 ]; then
  echo ""
  echo "Skip push (PUSH=0). To publish the tag and trigger CI:"
  echo "  git push origin HEAD && git push origin v${VER}"
  exit 0
fi

git push origin HEAD
git push origin "v${VER}"
echo ""
echo "Released v${VER} — pushed commit and tag (GitHub Actions: release on v*)."
