#!/usr/bin/env bash
# Create and push vX.Y.Z-legacy from the current package.json version (no bump, no commit).
# Triggers release-legacy.yml when the tag is pushed. Use on the legacy line when the
# embedded semver already matches the release you want to ship as legacy.
# Skip push: PUSH=0 make version-tag-legacy
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$ROOT"

git rev-parse --git-dir >/dev/null 2>&1 || {
  echo "Not a git repository." >&2
  exit 1
}

case "${PUSH:-1}" in
0 | false | no | NO) SKIP_PUSH=1 ;;
*) SKIP_PUSH=0 ;;
esac

VER=$(node -p "require('./package.json').version")
TAG="v${VER}-legacy"

if git show-ref --tags --verify --quiet "refs/tags/${TAG}"; then
  echo "Tag ${TAG} already exists. Remove it with: git tag -d ${TAG}" >&2
  exit 1
fi

git tag -a "${TAG}" -m "${TAG}"

if [ "$SKIP_PUSH" -eq 1 ]; then
  echo ""
  echo "Skip push (PUSH=0). To publish the tag and trigger legacy release CI:"
  echo "  git push origin ${TAG}"
  exit 0
fi

git push origin "${TAG}"
echo ""
echo "Tagged and pushed ${TAG} (GitHub Actions: release-legacy on tag)."
