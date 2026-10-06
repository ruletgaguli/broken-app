#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
mkdir -p artifacts
bash scripts/record.sh artifacts/baseline_before.txt env BENCH_VERSION=before cargo bench --locked --bench baseline
bash scripts/record.sh artifacts/baseline_after.txt env BENCH_VERSION=after cargo bench --locked --bench baseline
bash scripts/record.sh artifacts/criterion.txt cargo bench --locked --bench criterion -- --noplot
# Включаем сырые выборки и оценки Criterion в сдаваемый репозиторий.
mkdir -p artifacts/criterion
cp -R "${CARGO_TARGET_DIR:-target}/criterion/." artifacts/criterion/
python3 scripts/bench-report.py
