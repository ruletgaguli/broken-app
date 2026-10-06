#!/usr/bin/env bash
set -euo pipefail

# Сохраняем команду, окружение, вывод и настоящий код возврата.
log=$1
shift
mkdir -p "$(dirname "$log")"
{
    date -u '+UTC: %Y-%m-%dT%H:%M:%SZ'
    uname -a
    rustc -Vv
    rustup toolchain list
    printf 'Команда:'
    printf ' %q' "$@"
    printf '\n'
} > "$log"
set +e
"$@" 2>&1 | tee -a "$log"
status=${PIPESTATUS[0]}
set -e
printf '\nEXIT_STATUS=%s\n' "$status" | tee -a "$log"
exit "$status"
