#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for version in before after; do
    bash scripts/record.sh "artifacts/perf-$version-command.txt" bash scripts/profile.sh "$version"
done
