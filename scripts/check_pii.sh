#!/usr/bin/env bash
set -euo pipefail

if [ "$#" -eq 0 ]; then
  exit 0
fi

allowlist_file="${PII_ALLOWLIST_FILE:-.pii-allowlist}"
allowlist_regex=""
if [ -f "$allowlist_file" ]; then
  allowlist_regex="$(tr '\n' '|' < "$allowlist_file" | sed 's/|$//')"
fi

pattern='([A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,})|(\b\d{3}-\d{2}-\d{4}\b)|(\b\+?[1-9]\d{7,14}\b)|(\b\d{1,3}(?:\.\d{1,3}){3}\b)'

failed=0
for file in "$@"; do
  [ -f "$file" ] || continue
  matches="$(grep -nE "$pattern" "$file" || true)"
  if [ -n "$matches" ] && [ -n "$allowlist_regex" ]; then
    matches="$(printf '%s\n' "$matches" | grep -vE "$allowlist_regex" || true)"
  fi
  if [ -n "$matches" ]; then
    printf 'PII detected in %s\n' "$file"
    printf '%s\n' "$matches"
    failed=1
  fi
done

exit "$failed"
