#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
{
    date -u '+# UTC: %Y-%m-%dT%H:%M:%SZ'
    printf '# Исходный broken-app (архив до изменений):\n'
    shasum -a 256 artifacts/original.tar.gz
    printf '# Эталон: %s\n' "$(cd ../reference-app && pwd)"
    printf '# Коммит эталона: '
    git -C ../reference-app rev-parse --verify HEAD 2>/dev/null || printf 'отсутствует\n'
    find ../reference-app/src ../reference-app/tests ../reference-app/benches ../reference-app/scripts -type f -print | LC_ALL=C sort | while read -r file; do
        shasum -a 256 "$file"
    done
    shasum -a 256 ../reference-app/Cargo.toml ../reference-app/Cargo.lock ../reference-app/.gitignore
} > artifacts/source-identity.txt
