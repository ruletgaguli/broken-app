#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root=$PWD
bash scripts/prepare-original.sh
cd target/original
if bash "$root/scripts/record.sh" "$root/artifacts/original-miri-bounds.txt" cargo +nightly miri test --locked --test integration sums_even_numbers -- --exact; then
    exit 1
fi
for entry in 'owned_value_remains_alive:uaf' 'buffer_is_released:leak' 'counter_is_synchronized_and_calls_are_independent:race'; do
    name=${entry%%:*}
    label=${entry##*:}
    if bash "$root/scripts/record.sh" "$root/artifacts/original-miri-$label.txt" cargo +nightly miri test --locked --test probes "$name" -- --exact; then
        printf 'Диагностика %s неожиданно прошла; проверьте, что это исходник.\n' "$label" >&2
        exit 1
    fi
done
