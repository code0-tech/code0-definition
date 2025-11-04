#!/usr/bin/env bash
# Usage: ./inject-version.sh <version> [dir]
# Example: ./inject-version.sh 0.4.2           # defaults to ./definitions
#          ./inject-version.sh 0.4.2 ./defs

set -euo pipefail
IFS=$'\n\t'

# ---- args & checks ----
[ $# -ge 1 ] || { echo "Usage: $0 <version> [dir]" >&2; exit 1; }
V="$1"
DIR="${2:-definitions}"

command -v jq >/dev/null 2>&1 || { echo "jq is required" >&2; exit 1; }
[[ "$V" =~ ^[0-9]+(\.[0-9]+)*$ ]] || { echo "Invalid version: $V" >&2; exit 1; }
[ -d "$DIR" ] || { echo "Folder not found: $DIR" >&2; exit 1; }

# ---- process ----
updated=0
skipped=0
while IFS= read -r -d '' f; do
  # Validate JSON first
  if ! jq -e . "$f" >/dev/null 2>&1; then
    echo "INVALID JSON (skipped): $f" >&2
    ((skipped++))
    continue
  fi

  # Only modify if top-level is an object
  if jq -e 'type=="object"' "$f" >/dev/null 2>&1; then
    tmp="$(mktemp --tmpdir="$(dirname "$f")" .inject.XXXXXX)"
    # Write to temp first; only replace on success
    if jq --arg v "$V" '.version=$v' "$f" >"$tmp"; then
      mv -f "$tmp" "$f"
      echo "updated: $f"
      ((updated++))
    else
      echo "ERROR processing: $f" >&2
      rm -f "$tmp"
      ((skipped++))
    fi
  else
    echo "skipped (non-object): $f"
    ((skipped++))
  fi
done < <(find "$DIR" -type f -name '*.json' -print0)

echo "Done. Updated: $updated  Skipped: $skipped"
