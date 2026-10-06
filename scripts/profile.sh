#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
version=${1:-before}
case "$version" in before|after) ;; *) exit 2 ;; esac
mkdir -p artifacts
cargo build --locked --release --example profile
executable="${CARGO_TARGET_DIR:-target}/release/examples/profile"
case "$(uname -s)" in
    Darwin)
        "$executable" "$version" 8 > "artifacts/profile-$version-workload.txt" &
        pid=$!
        trap 'kill "$pid" 2>/dev/null || true' EXIT
        sample "$pid" 3 -file "artifacts/profile-$version.txt"
        wait "$pid"
        trap - EXIT
        if command -v inferno-collapse-sample >/dev/null && command -v inferno-flamegraph >/dev/null; then
            inferno-collapse-sample "artifacts/profile-$version.txt" > "artifacts/stacks-$version.folded"
            inferno-flamegraph --deterministic --title "broken-app: $version (sample)" "artifacts/stacks-$version.folded" > "artifacts/flamegraph-$version.svg"
        fi
        ;;
    Linux)
        perf record -F 99 -e cpu-clock:u -g --call-graph dwarf -o "artifacts/perf-$version.data" -- "$executable" "$version" 8
        perf report --stdio -i "artifacts/perf-$version.data" > "artifacts/perf-$version.txt"
        perf script -i "artifacts/perf-$version.data" > "artifacts/perf-$version-stacks.txt"
        if command -v inferno-collapse-perf >/dev/null && command -v inferno-flamegraph >/dev/null; then
            inferno-collapse-perf "artifacts/perf-$version-stacks.txt" | inferno-flamegraph > "artifacts/perf-flamegraph-$version.svg"
        fi
        ;;
    *) printf 'Профилировщик для этой ОС не настроен.\n' >&2; exit 2 ;;
esac
