#!/usr/bin/env bash
# Usage: ./inject-version.sh <version> [dir]
# Example: ./inject-version.sh 0.4.2           # defaults to ./definitions
#          ./inject-version.sh 0.4.2 ./defs

set -euo pipefail
IFS=$'\n\t'

# ---- args & checks ----
if [ $# -lt 1 ]; then
  echo "Usage: $0 <version> [dir]" >&2
  exit 1
fi

V="$1"
DIR="${2:-definitions}"

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required" >&2
  exit 1
fi

if [[ ! "$V" =~ ^[0-9]+(\.[0-9]+)*$ ]]; then
  echo "Invalid version: $V" >&2
  exit 1
fi

if [ ! -d "$DIR" ]; then
  echo "Folder not found: $DIR" >&2
  exit 1
fi

# ---- process ----
updated=0
skipped=0

# Disable exit-on-error inside loop so non-critical skips don’t abort the script
set +e
while IFS= read -r -d '' f; do
  # Validate JSON first
  if ! jq -e . "$f" >/dev/null 2>&1; then
    echo "INVALID JSON (skipped): $f" >&2
    ((skipped+=1)) || true
    continue
  fi

  # Only modify if top-level is an object
  if jq -e 'type=="object"' "$f" >/dev/null 2>&1; then
    tmp="$(mktemp --tmpdir="$(dirname "$f")" .inject.XXXXXX)"
    # Write to temp first; only replace on success
    if jq --arg v "$V" '.version=$v' "$f" >"$tmp"; then
      mv -f "$tmp" "$f"
      echo "updated: $f"
      ((updated+=1)) || true
    else
      echo "ERROR processing: $f" >&2
      rm -f "$tmp"
      ((skipped+=1)) || true
    fi
  else
    echo "skipped (non-object): $f"
    ((skipped+=1)) || true
  fi
done < <(find "$DIR" -type f -name '*.json' -print0)
set -e

echo "Done. Updated: $updated  Skipped: $skipped"
exit 0
