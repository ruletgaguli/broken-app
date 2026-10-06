"""Сравнение реальных оценок Criterion; не подменяет отсутствующие замеры."""
import csv
import json
from pathlib import Path

root = Path("artifacts/criterion/comparison")
rows = []
for before in sorted(root.glob("**/new/estimates.json")):
    relative = str(before.relative_to(root))
    if "_before" not in relative:
        continue
    after = root / relative.replace("_before", "_after")
    old = json.loads(before.read_text())["median"]["point_estimate"]
    new = json.loads(after.read_text())["median"]["point_estimate"]
    rows.append((relative.split("/new/")[0], old, new, old / new))
if not rows:
    raise SystemExit("Нет результатов Criterion")
with Path("artifacts/benchmark-summary.csv").open("w", newline="") as output:
    writer = csv.writer(output)
    writer.writerow(["case", "before_ns", "after_ns", "speedup"])
    writer.writerows(rows)
lines = ["# Реальные замеры Criterion", "", "Медианы, наносекунды на вызов. Ускорение = до / после.", "", "| Сценарий | До, нс | После, нс | Ускорение |", "|---|---:|---:|---:|"]
for name, old, new, ratio in rows:
    lines.append(f"| {name} | {old:.1f} | {new:.1f} | {ratio:.2f}x |")
lines += ["", "Источник: criterion.txt и criterion/comparison/**/new/{estimates,sample}.json.", "Сравниваются безопасная точка до оптимизаций и финальная реализация в одном бинарнике."]
Path("artifacts/benchmark-summary.md").write_text("\n".join(lines) + "\n")
print("\n".join(lines))
