#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for version in before after; do
    inferno-collapse-perf "artifacts/perf-$version-stacks.txt" > "artifacts/perf-$version.folded"
    inferno-flamegraph --deterministic --title "broken-app: $version (perf)" "artifacts/perf-$version.folded" > "artifacts/perf-flamegraph-$version.svg"
    gzip -f "artifacts/perf-$version.data" "artifacts/perf-$version-stacks.txt"
done
