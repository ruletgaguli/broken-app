#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p target
cargo test --locked --test integration --no-run --message-format=json > target/debug-test.json
executable=$(python3 - <<'PY'
import json
from pathlib import Path
for line in Path("target/debug-test.json").read_text().splitlines():
    item = json.loads(line)
    if item.get("reason") == "compiler-artifact" and item.get("executable") and item.get("target", {}).get("name") == "integration":
        print(item["executable"])
PY
)
case "$(uname -s)" in
    Darwin)
        bash scripts/record.sh artifacts/debugger-after.txt lldb --batch \
            -o 'breakpoint set --name broken_app::average_positive' \
            -o 'run averages_only_positive --exact --nocapture' \
            -o 'frame variable' -o 'thread step-out' -o 'register read d0' -o continue "$executable"
        ;;
    Linux)
        bash scripts/record.sh artifacts/debugger-after.txt gdb --batch \
            -ex 'break broken_app::average_positive' \
            -ex 'run averages_only_positive --exact --nocapture' \
            -ex 'info args' -ex finish -ex continue "$executable"
        ;;
    *) exit 2 ;;
esac
