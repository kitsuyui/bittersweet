#!/usr/bin/env bash
# Verify that the repository's merge-automation settings match the expected values.
# Exits non-zero and prints a failure summary when any setting has drifted.
# Usage: GH_TOKEN=<token> REPO=<owner/repo> bash scripts/check-repo-settings.sh
set -euo pipefail

REPO="${REPO:-kitsuyui/bittersweet}"

settings=$(gh api "repos/${REPO}" \
  --jq '{
    allow_auto_merge,
    allow_update_branch,
    delete_branch_on_merge,
    allow_squash_merge,
    allow_rebase_merge
  }')

echo "Current settings:"
echo "$settings" | jq .

failures=0

check() {
  local field="$1"
  local expected="$2"
  local actual
  actual=$(echo "$settings" | jq -r ".$field")
  if [ "$actual" != "$expected" ]; then
    echo "FAIL: $field expected=${expected} actual=${actual}"
    failures=$((failures + 1))
  else
    echo "OK:   $field=${actual}"
  fi
}

check allow_auto_merge      true
check allow_update_branch   true
check delete_branch_on_merge true
check allow_squash_merge    true
check allow_rebase_merge    true

if [ "$failures" -gt 0 ]; then
  echo ""
  echo "${failures} setting(s) have drifted from the expected state."
  echo "Restore them with:"
  echo "  gh api repos/${REPO} --method PATCH \\"
  echo "    --field allow_auto_merge=true \\"
  echo "    --field allow_update_branch=true \\"
  echo "    --field delete_branch_on_merge=true \\"
  echo "    --field allow_squash_merge=true \\"
  echo "    --field allow_rebase_merge=true"
  exit 1
fi
