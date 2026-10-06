#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root=$PWD
bash scripts/prepare-original.sh
cd target/original
target=$(rustc +nightly -vV | sed -n 's/^host: //p')
for entry in 'address:owned_value_remains_alive' 'thread:counter_is_synchronized_and_calls_are_independent'; do
    kind=${entry%%:*}
    name=${entry##*:}
    if bash "$root/scripts/record.sh" "$root/artifacts/original-sanitizer-$kind.txt" env CARGO_TARGET_DIR="$root/target/original-sanitizer-$kind" RUSTFLAGS="-Zsanitizer=$kind -Cforce-frame-pointers=yes" cargo +nightly test --locked -Zbuild-std --target "$target" --test probes "$name" -- --exact --test-threads=1; then
        printf 'Исходная диагностика %s неожиданно прошла.\n' "$kind" >&2
        exit 1
    fi
done
