#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if ! command -v valgrind >/dev/null; then
    printf 'Valgrind отсутствует. Нужен Linux с valgrind.\n' >&2
    exit 2
fi
mkdir -p target
cargo test --locked --tests --no-run --message-format=json > target/test-executables.json
# Cargo JSON не зависит от хешей имён бинарников. Анализируем сами тесты, не Cargo.
python3 - <<'PY' > target/test-executables.txt
import json
from pathlib import Path
for line in Path("target/test-executables.json").read_text().splitlines():
    item = json.loads(line)
    if item.get("reason") == "compiler-artifact" and item.get("profile", {}).get("test") and item.get("executable"):
        print(item["executable"])
PY
index=0
while IFS= read -r executable; do
    index=$((index + 1))
    bash scripts/record.sh "artifacts/valgrind-$index.txt" valgrind --suppressions=scripts/valgrind.supp --error-exitcode=99 --leak-check=full --show-leak-kinds=all --errors-for-leak-kinds=definite,indirect,possible "$executable" --test-threads=1
done < target/test-executables.txt
test "$index" -gt 0
