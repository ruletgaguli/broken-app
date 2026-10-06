#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target/original
tar -xzf artifacts/original.tar.gz -C target/original
cp scripts/original-probes.rs target/original/tests/probes.rs
