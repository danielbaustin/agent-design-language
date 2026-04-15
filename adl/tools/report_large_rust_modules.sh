#!/usr/bin/env bash
set -euo pipefail

ROOTS=("adl/src" "adl/tests")
THRESHOLD_WATCH=800
THRESHOLD_REVIEW=1000
THRESHOLD_RATIONALE=1500
FORMAT="table"

usage() {
  cat <<'EOF'
Usage: adl/tools/report_large_rust_modules.sh [options]

Report large Rust implementation modules without failing the build.

Options:
  --root <path>                Root to scan; repeatable (default: adl/src and adl/tests)
  --threshold-watch <n>        Watch threshold in lines (default: 800)
  --threshold-review <n>       Review threshold in lines (default: 1000)
  --threshold-rationale <n>    Rationale threshold in lines (default: 1500)
  --format <table|tsv>         Output format (default: table)
  -h, --help                   Show this help

The script exits 0 for reporting use cases. It is a review aid, not a CI blocker.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --root)
      if [[ "${ROOTS[*]}" == "adl/src adl/tests" ]]; then
        ROOTS=()
      fi
      ROOTS+=("$2")
      shift 2
      ;;
    --threshold-watch)
      THRESHOLD_WATCH="$2"
      shift 2
      ;;
    --threshold-review)
      THRESHOLD_REVIEW="$2"
      shift 2
      ;;
    --threshold-rationale)
      THRESHOLD_RATIONALE="$2"
      shift 2
      ;;
    --format)
      FORMAT="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

for root in "${ROOTS[@]}"; do
  if [[ ! -d "$root" ]]; then
    echo "Scan root does not exist: $root" >&2
    exit 2
  fi
done

if [[ "$FORMAT" != "table" && "$FORMAT" != "tsv" ]]; then
  echo "Unsupported format: $FORMAT" >&2
  exit 2
fi

python3 - "$THRESHOLD_WATCH" "$THRESHOLD_REVIEW" "$THRESHOLD_RATIONALE" "$FORMAT" "${ROOTS[@]}" <<'PY'
import sys
from pathlib import Path

watch = int(sys.argv[1])
review = int(sys.argv[2])
rationale = int(sys.argv[3])
fmt = sys.argv[4]
roots = [Path(arg) for arg in sys.argv[5:]]

rows = []
seen = set()
for root in roots:
    for path in sorted(root.rglob("*.rs")):
        if path in seen:
            continue
        seen.add(path)
        with path.open("r", encoding="utf-8") as handle:
            loc = sum(1 for _ in handle)
        if loc < watch:
            continue
        if loc >= rationale:
            level = "RATIONALE"
        elif loc >= review:
            level = "REVIEW"
        else:
            level = "WATCH"
        rows.append((str(path), loc, level))

rows.sort(key=lambda row: (-row[1], row[0]))

if fmt == "tsv":
    print("path\tloc\tlevel")
    for path, loc, level in rows:
        print(f"{path}\t{loc}\t{level}")
    sys.exit(0)

print("Rust module size watch report")
print(f"scan roots: {', '.join(str(root) for root in roots)}")
print(f"thresholds: watch>={watch}, review>={review}, rationale>={rationale}")
print("")

if not rows:
    print("No modules exceeded the configured watch threshold.")
    sys.exit(0)

path_width = max(len("Path"), max(len(path) for path, _, _ in rows))
loc_width = max(len("LoC"), max(len(str(loc)) for _, loc, _ in rows))
level_width = max(len("Level"), max(len(level) for _, _, level in rows))

header = f"{'Path':<{path_width}}  {'LoC':>{loc_width}}  {'Level':<{level_width}}"
print(header)
print(f"{'-' * path_width}  {'-' * loc_width}  {'-' * level_width}")
for path, loc, level in rows:
    print(f"{path:<{path_width}}  {loc:>{loc_width}}  {level:<{level_width}}")
PY
