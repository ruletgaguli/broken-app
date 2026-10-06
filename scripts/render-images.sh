#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
for name in flamegraph-before flamegraph-after perf-flamegraph-before perf-flamegraph-after; do
    rsvg-convert --output "artifacts/$name.svg.png" "artifacts/$name.svg"
done
