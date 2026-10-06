#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
kind=${1:?Укажите address или thread}
case "$kind" in address|thread) ;; *) exit 2 ;; esac
target=$(rustc +nightly -vV | sed -n 's/^host: //p')
export CARGO_TARGET_DIR="target/sanitizer-$kind"
export RUSTFLAGS="-Zsanitizer=$kind -Cforce-frame-pointers=yes"
# Пересобираем std: TSan должен видеть синхронизацию внутри стандартной библиотеки.
bash scripts/record.sh "artifacts/sanitizer-$kind.txt" cargo +nightly test --locked -Zbuild-std --target "$target" --tests -- --test-threads=1
