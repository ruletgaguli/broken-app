# broken-app

## Проектная работа модуля 5. Поиск ошибок и оптимизация

Исправление приложения с выходом за границы, use-after-free, утечкой памяти, логическими ошибками и гонкой данных. Корректность проверяется тестами, эталоном, LLDB, Miri, Valgrind и sanitizer'ами; производительность сравнивается воспроизводимыми бенчмарками и профилями.

Разработка и локальные проверки выполнялись на macOS. Поэтому для отладки использован LLDB, а для профилирования на хосте — `sample`. Проверки Valgrind и профилирование с `perf` выполнялись в Linux-контейнере через Docker; ниже приведены команды для обоих окружений.

В основной библиотеке нет `unsafe`; это закреплено `#![forbid(unsafe_code)]`. Использованы изученные конструкции: срезы, итераторы, стандартные коллекции, владение, потоки, `Arc` и атомарные операции. Производственных зависимостей нет, Criterion и снимок эталона используются только при разработке.

## Сборка

Нужны Rust 1.88 или новее и системный линкер. Сохранённые замеры выполнены на Rust 1.96.0; CI использует эту же версию для воспроизводимости. `Cargo.lock` включён в проект.

Из корня `broken-app`:

```bash
cargo build --locked --workspace
cargo run --locked --bin demo
```

Ожидаемый вывод:

```text
sum_even: 6
non-zero bytes: 3
normalize: helloworld
fib(20): 6765
dedup: [1, 2, 3, 4]
```

## Исправления

| Дефект | Исправление | Регрессионная проверка |
|---|---|---|
| `sum_even`: индекс `len` в `get_unchecked` | Безопасный проход по срезу | Пустой срез, единственный и последний чётный элемент |
| `use_after_free`: чтение освобождённого `Box` | Вычисление при живом владельце, безопасный API | Результат `84`, Miri и ASan |
| `leak_buffer`: потеря владельца `Box` | Подсчёт по исходному срезу без выделения памяти | Повторные вызовы, Miri и Valgrind |
| `normalize`: не удаляются табуляции и Unicode-пробелы | `split_whitespace`, корректный Unicode-регистр | Переводы строк, NBSP, греческая сигма, турецкая `İ` |
| `average_positive`: отрицательные и нулевые элементы включены в среднее | Один проход по положительным значениям | Нет положительных, смешанный вход, большие `i64` |
| `COUNTER`: несинхронизированный `static mut` | Локальный `Arc<AtomicU64>` каждого вызова, `join`, атомарный опубликованный снимок | Параллельные вызовы, сброс, нулевые параметры, Miri и TSan |
| `slow_dedup`: поиск и сортировка при каждой вставке | Одно копирование, одна сортировка и `dedup` | Состав и отсортированный порядок, сверка с эталоном |
| `slow_fib`: экспоненциальная рекурсия | Итеративное вычисление за O(n), O(1) памяти | Значения 0..93, явное отклонение 94 |

Исторические имена функций сохранены. `sum_even` явно паникует при переполнении аккумулятора в любом профиле; `slow_fib` отклоняет `n > 93`. Среднее накапливается в `f64`, без переполнения суммы `i64`; действуют обычные ограничения точности floating-point.

`race_increment` возвращает результат собственного вызова, даже если другие вызовы выполняются одновременно. `read_after_sleep` больше не задерживает поток и читает последнее опубликованное значение. Сброс изменяет снимок, но не отменяет уже работающие вызовы: они могут опубликовать новый результат после сброса.

## Проверки

```bash
cargo fmt --package broken-app -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace --all-targets
```

Всего 18 тестов: 6 исходных, 10 регрессионных, сверка с эталоном и проверка безопасной версии «до». В режиме `--all-targets` дополнительно проверяются бенчмарки и примеры. Форматирование ограничено нашим пакетом: неизменённая эталонная фикстура содержит исходный стиль и предупреждение о неиспользуемом `Arc`.

## Динамический анализ

Miri и sanitizer'ы требуют nightly. Его версия для выполненных прогонов записана в [tool-versions.txt](artifacts/tool-versions.txt). Прогоны ниже проверяют тестовые сценарии, а не являются математическим доказательством отсутствия всех возможных ошибок.

```bash
rustup toolchain install nightly --profile minimal --component miri --component rust-src
bash scripts/record.sh artifacts/miri-after.txt cargo +nightly miri test --locked -- --test-threads=1
bash scripts/sanitizers.sh address
bash scripts/sanitizers.sh thread
```

Sanitizer'ы пересобирают стандартную библиотеку через `-Zbuild-std`; это важно для видимости синхронизации в TSan. Их отдельные каталоги сборки исключают смешение инструментированных и обычных бинарников.

На Linux установите `valgrind` и Python 3:

```bash
bash scripts/valgrind.sh
```

Скрипт получает исполняемые тесты из Cargo JSON и запускает Valgrind на них, а не на процессе Cargo. Включён `--error-exitcode=99` и проверяются definite, indirect и possible leaks. В [valgrind.supp](scripts/valgrind.supp) есть одно узкое исключение для контекста `std::sync::mpmc` тестового раннера Rust 1.96. Сохранён [нефильтрованный отчёт](artifacts/valgrind-unfiltered.txt): 48 байт possible leak внутри std. После исключения ошибки проекта не обнаружены; 544 байта в глобальной таблице std остаются `still reachable`, не потеряны. Исходная утечка проекта с тем же исключением обнаруживается: [original-valgrind.txt](artifacts/original-valgrind.txt), 64 байта definite leak.

На macOS Valgrind выполнен в Linux-контейнере:

```bash
docker build -f scripts/linux.Dockerfile -t broken-app-analysis .
docker run --rm -v "$PWD:/work" -v broken-app-cargo:/usr/local/cargo \
  -e CARGO_TARGET_DIR=/work/target/linux \
  broken-app-analysis bash scripts/valgrind.sh
```

## Отладчик

[debugger-before.txt](artifacts/debugger-before.txt) фиксирует остановку LLDB на `average_positive`: вход длиной 3 и возвращаемое значение `5` вместо `10`. Контрольный отчёт: [debugger-after.txt](artifacts/debugger-after.txt).

```bash
bash scripts/debug.sh
```

Скрипт автоматически находит тестовый бинарник; на macOS используется LLDB, на Linux GDB. Исходный тест имеет вход `[-5, 5, 15]`. LLDB сам завершился с кодом 0 и после `continue` в исходном отчёте явно показывает код тестового процесса 101.

## Бенчмарки

```bash
bash scripts/compare.sh
```

Сначала запускаются текстовые замеры «до/после», затем Criterion сравнивает обе версии внутри одного release-бинарника. Вход и результат защищены `std::hint::black_box`; выделение входных данных вне измерения. Возвращённый `Vec`/`String` освобождается внутри измеряемого вызова одинаково для обеих версий.

UB нельзя использовать как точку сравнения производительности. Поэтому [benches/support/before.rs](benches/support/before.rs) хранит безопасную версию до оптимизаций: старые тела дедупликации и Фибоначчи, копирование буфера с корректным освобождением, нормализация и среднее с промежуточными буферами. Неизменный исходник с дефектами отдельно сохранён в архиве. Соответствие безопасной точки «до» проверяется тестами и Miri.

Покрыты дедупликация на 100 и 10 000 элементах, Фибоначчи 20 и 32, миллион байтов, длинная строка и 50 000 чисел. Criterion: 20 выборок, прогрев 200 мс и измерение не менее 1 секунды. Все результаты: [benchmark-summary.md](artifacts/benchmark-summary.md), [CSV](artifacts/benchmark-summary.csv), сырые выборки и доверительные интервалы в `artifacts/criterion/`.

На этой машине дедупликация 10 000 элементов ускорилась примерно в 981 раз, Фибоначчи 32 примерно в 405 000 раз; подсчёт миллиона байтов примерно на 12%. Эти коэффициенты относятся к конкретным входам и окружению, не к приложению вообще. Среднее почти не изменилось по времени, но перестало выделять промежуточный `Vec`. Нормализация на этом ASCII-входе примерно на 3% медленнее при переиспользовании буфера. Проверяемая сумма примерно в 4 раза медленнее векторизуемой версии «до» из-за явной проверки переполнения; ускорение для неё не заявляется. Уменьшение аллокаций описано по коду, отдельный счётчик аллокаций не применялся.

Повторяйте замеры без параллельных сборок и профилировщиков. Строки Criterion `change:` сравнивают повторные прогоны одного benchmark ID, а не версии before/after: для сравнения версий используется отдельный CSV-отчёт.

## Профилирование

```bash
cargo install inferno --locked
bash scripts/profile.sh before
bash scripts/profile.sh after
```

На macOS используется `sample`, на Linux `perf` с событием `cpu-clock:u`, 99 Гц и DWARF-стеками. Release и bench содержат debug-символы. Нагрузка выполняется 8 секунд, каждый цикл включает все измеряемые функции. Профили показывают доли сэмплов, не точное время каждого вызова.

Linux через Docker требует возможностей perf/ptrace и отключения seccomp для этого контейнера:

```bash
docker run --rm --cap-add PERFMON --cap-add SYS_PTRACE \
  --security-opt seccomp=unconfined \
  -v "$PWD:/work" -v broken-app-cargo:/usr/local/cargo \
  -e CARGO_TARGET_DIR=/work/target/linux \
  broken-app-analysis bash scripts/linux-profile.sh
bash scripts/render-perf.sh
docker run --rm -v "$PWD:/work" broken-app-analysis bash scripts/render-images.sh
```

Разрешения нужны только контейнеру профилирования; не используйте их для обычного запуска приложения. На нативном Linux ограничения `perf_event_paranoid` могут потребовать согласованного с администратором изменения окружения. Текстовые отчёты и стеки сохраняются даже без Inferno. `render-perf.sh` строит SVG на хосте и сжимает perf data/стеки без потери данных; перед повторным построением распакуйте `*.gz` либо повторите сбор профиля.

В [perf-before.txt](artifacts/perf-before.txt) дедупликация занимает 91.93% сэмплов, её повторная сортировка 88.90%, Фибоначчи 7.69%. Именно эти горячие пути оптимизированы. После оптимизации [perf-after.txt](artifacts/perf-after.txt) нагрузка распределена между подсчётом байтов, средним, нормализацией и одной сортировкой. За примерно 8 секунд записаны 114 циклов «до» и 22 506 «после»; это наблюдение профилировщика, отдельные функции оцениваются Criterion.

Flamegraph до/после: [SVG до](artifacts/perf-flamegraph-before.svg), [SVG после](artifacts/perf-flamegraph-after.svg), [PNG до](artifacts/perf-flamegraph-before.svg.png), [PNG после](artifacts/perf-flamegraph-after.svg.png). Профили `sample` и их flamegraph также включены в `artifacts/`.

## Исходник и эталон

На момент начала оба репозитория не имели коммитов и remote. Исходный `broken-app` сохранён в [original.tar.gz](artifacts/original.tar.gz), хеши исходника и всех файлов эталона в [source-identity.txt](artifacts/source-identity.txt). Исходный `../reference-app` не изменён. Его `src/` и `Cargo.toml` скопированы побайтово в `tests/fixtures/reference-app`, чтобы проект собирался без соседнего репозитория.

Проверка эталона при наличии соседнего проекта:

```bash
shasum -a 256 -c artifacts/source-identity.txt
diff -r ../reference-app/src tests/fixtures/reference-app/src
```

Для воспроизведения дефектов используется отдельная распакованная копия в `target/original`, не рабочие исходники:

```bash
bash scripts/original-miri.sh
bash scripts/original-sanitizers.sh
```

`scripts/original-valgrind.sh` запускается в Linux-окружении аналогично основному Valgrind. Эти диагностические скрипты ожидают ошибки исходника; в логах остаются реальные ненулевые коды. `scripts/original-probes.rs` намеренно вызывает старый unsafe API только внутри исходной копии и не входит в финальную библиотеку.

## CI

GitHub Actions запускается при `push` и `pull_request`: сначала форматирование и Clippy, затем сборка, тесты, проверки бенчмарков и demo. Отдельная задача с `needs: lint` выполняет Miri, Valgrind, ASan и TSan и публикует артефакты даже при ошибке. Настройка workflow готова; удалённый запуск GitHub Actions ещё не проверен.

## Справочники инструментов

- [Miri](https://github.com/rust-lang/miri)
- [sanitizer'ы Rust](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html)
- [Valgrind](https://valgrind.org/docs/manual/manual-core.html)
- [Criterion](https://bheisler.github.io/criterion.rs/book/user_guide/comparing_functions.html), [Inferno](https://github.com/jonhoo/inferno)
