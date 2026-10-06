#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
root=$PWD
bash scripts/prepare-original.sh
cd target/original
export CARGO_TARGET_DIR="$root/target/original-linux"
cargo test --locked --test probes --no-run --message-format=json > "$root/target/original-executables.json"
executable=$(python3 - "$root/target/original-executables.json" <<'PY'
import json
import sys
from pathlib import Path
for line in Path(sys.argv[1]).read_text().splitlines():
    item = json.loads(line)
    if item.get("reason") == "compiler-artifact" and item.get("executable") and item.get("target", {}).get("name") == "probes":
        print(item["executable"])
PY
)
if bash "$root/scripts/record.sh" "$root/artifacts/original-valgrind.txt" valgrind --suppressions="$root/scripts/valgrind.supp" --error-exitcode=99 --leak-check=full --show-leak-kinds=all --errors-for-leak-kinds=definite,indirect,possible "$executable" buffer_is_released --exact --test-threads=1; then
    printf 'Исходная утечка не обнаружена.\n' >&2
    exit 1
fi
