#!/bin/sh
# Verify that the release tag matches the version declared in Cargo.toml.
# Usage: check-version.sh <tag>   e.g. check-version.sh v0.3.0
set -eu

tag="${1:-}"
if [ -z "$tag" ]; then
  echo "Usage: $0 <release-tag>" >&2
  exit 1
fi

cargo_version=$(grep -m1 '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
expected_tag="v${cargo_version}"

if [ "$tag" != "$expected_tag" ]; then
  echo "Version mismatch: release tag=${tag}, Cargo.toml expects tag=${expected_tag}" >&2
  exit 1
fi

echo "Version check passed: tag=${tag} matches Cargo.toml version=${cargo_version}"
